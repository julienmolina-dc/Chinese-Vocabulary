mod auth;
mod data;
mod stories;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use data::{get_all_words, Word};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;

fn load_cards(pool: &PgPool, words: &[Word]) -> tokio::task::JoinHandle<()> {
    // Initialize cards in the database for new words
    let pool = pool.clone();
    let words = words.to_vec();
    tokio::spawn(async move {
        for word in words {
            let _ = sqlx::query(
                "INSERT INTO srs_cards (word_id, ease_factor, interval, repetitions, next_review, box_level)
                 VALUES ($1, 2.5, 0, 0, 0, 0)
                 ON CONFLICT (word_id) DO NOTHING"
            )
            .bind(word.id as i32)
            .execute(&pool)
            .await;
        }
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SrsCard {
    word_id: u32,
    ease_factor: f64,
    interval: u32, // days
    repetitions: u32,
    next_review: i64,    // unix timestamp
    box_level: u8,       // Leitner box 0-4
    is_relearning: bool, // Flag set when card drops from mastered to learning
}

impl SrsCard {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        Ok(SrsCard {
            word_id: row.try_get::<i32, _>("word_id")? as u32,
            ease_factor: row.try_get("ease_factor")?,
            interval: row.try_get::<i32, _>("interval")? as u32,
            repetitions: row.try_get::<i32, _>("repetitions")? as u32,
            next_review: row.try_get("next_review")?,
            box_level: row.try_get::<i16, _>("box_level")? as u8,
            is_relearning: row.try_get("is_relearning")?,
        })
    }

    // SM-2 algorithm adapted for Leitner boxes
    fn review(&mut self, rating: u8) {
        let now = chrono::Utc::now().timestamp();
        let was_mastered = self.box_level >= 4;

        match rating {
            1 => {
                // Again - reset
                self.repetitions = 0;
                self.interval = 0;
                self.box_level = 0;
                self.ease_factor = (self.ease_factor - 0.2).max(1.3);
                if was_mastered {
                    self.is_relearning = true;
                }
            }
            2 => {
                // Hard
                self.repetitions += 1;
                self.interval = (self.interval as f64 * 1.2).max(1.0) as u32;
                self.ease_factor = (self.ease_factor - 0.15).max(1.3);
                self.box_level = self.box_level.min(2);
                if was_mastered {
                    self.is_relearning = true;
                }
            }
            3 => {
                // Good
                self.repetitions += 1;
                if self.repetitions == 1 {
                    self.interval = 1;
                } else if self.repetitions == 2 {
                    self.interval = 3;
                } else {
                    self.interval = (self.interval as f64 * self.ease_factor) as u32;
                }
                self.box_level = (self.box_level + 1).min(4);
                if !was_mastered || self.box_level >= 4 {
                    self.is_relearning = false;
                }
            }
            4 => {
                // Easy
                self.repetitions += 1;
                if self.repetitions == 1 {
                    self.interval = 3;
                } else {
                    self.interval = (self.interval as f64 * self.ease_factor * 1.3) as u32;
                }
                self.ease_factor += 0.15;
                self.box_level = (self.box_level + 1).min(4);
                if !was_mastered || self.box_level >= 4 {
                    self.is_relearning = false;
                }
            }
            _ => {}
        }
        self.next_review = now + (self.interval as i64 * 86400);
    }
}

#[derive(Serialize)]
struct ProgressStats {
    total: usize,
    new: usize,
    learning: usize,
    review: usize,
    mastered: usize,
    relearning: usize,
    due_today: usize,
    studied_today: usize,
    retention_rate_7d: f64,
    study_streak: u32,
}

struct AppState {
    words: Vec<Word>,
    pool: Option<PgPool>,
}

#[get("/api/words")]
async fn get_words(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.words)
}

#[derive(Deserialize)]
struct LevelQuery {
    level: Option<u8>,
}

#[get("/api/words/level")]
async fn get_words_by_level(
    state: web::Data<AppState>,
    query: web::Query<LevelQuery>,
) -> impl Responder {
    let words: Vec<&Word> = match query.level {
        Some(l) => state.words.iter().filter(|w| w.level == l).collect(),
        None => state.words.iter().collect(),
    };
    HttpResponse::Ok().json(words)
}

#[get("/api/stats")]
async fn get_stats(state: web::Data<AppState>) -> impl Responder {
    match &state.pool {
        Some(pool) => {
            let now = chrono::Utc::now().timestamp();
            let total = state.words.len();

            let mastered =
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM srs_cards WHERE box_level >= 4")
                    .fetch_one(pool)
                    .await
                    .unwrap_or(0);

            let learning = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE repetitions > 0 AND box_level < 2",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let due = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE next_review <= $1",
            )
            .bind(now)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            HttpResponse::Ok().json(serde_json::json!({
                "total": total,
                "mastered": mastered as usize,
                "learning": learning as usize,
                "due": due as usize,
            }))
        }
        None => {
            // Mock data for testing
            HttpResponse::Ok().json(serde_json::json!({
                "total": state.words.len(),
                "mastered": 8,
                "learning": 15,
                "due": 12,
            }))
        }
    }
}

#[get("/api/progress-stats")]
async fn get_progress_stats(state: web::Data<AppState>) -> impl Responder {
    match &state.pool {
        Some(pool) => {
            // Real database implementation
            let now = chrono::Utc::now().timestamp();
            let today_start = now - (now % 86400);

            let total = state.words.len() as i64;

            let existing_new = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE repetitions = 0 AND next_review = 0",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let existing_cards = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM srs_cards")
                .fetch_one(pool)
                .await
                .unwrap_or(0);

            let missing_words = (total - existing_cards).max(0);
            let new = existing_new + missing_words;

            let learning = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE repetitions > 0 AND box_level < 2",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let review = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE box_level >= 2 AND box_level < 4",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let mastered =
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM srs_cards WHERE box_level >= 4")
                    .fetch_one(pool)
                    .await
                    .unwrap_or(0);

            let relearning = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE is_relearning = true",
            )
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let due_today = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM srs_cards WHERE next_review <= $1",
            )
            .bind(now)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let studied_today = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM review_log WHERE reviewed_at >= $1",
            )
            .bind(today_start)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let seven_days_ago = now - (7 * 24 * 60 * 60);
            let total_reviews_7d = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM review_log WHERE reviewed_at >= $1",
            )
            .bind(seven_days_ago)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let good_reviews_7d = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM review_log WHERE reviewed_at >= $1 AND rating >= 3",
            )
            .bind(seven_days_ago)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let retention_rate_7d = if total_reviews_7d > 0 {
                (good_reviews_7d as f64 / total_reviews_7d as f64) * 100.0
            } else {
                0.0
            };

            let study_streak = 0;

            HttpResponse::Ok().json(ProgressStats {
                total: state.words.len(),
                new: new as usize,
                learning: learning as usize,
                review: review as usize,
                mastered: mastered as usize,
                relearning: relearning as usize,
                due_today: due_today as usize,
                studied_today: studied_today as usize,
                retention_rate_7d,
                study_streak,
            })
        }
        None => {
            // Mock data for testing without database
            HttpResponse::Ok().json(ProgressStats {
                total: state.words.len(),
                new: 45,
                learning: 23,
                review: 12,
                mastered: 8,
                relearning: 2,
                due_today: 15,
                studied_today: 8,
                retention_rate_7d: 85.5,
                study_streak: 5,
            })
        }
    }
}
#[derive(Deserialize)]
struct ReviewQuery {
    level: Option<u8>,
    limit: Option<usize>,
    session_type: Option<String>, // "standard", "new_only", "mixed"
}

fn select_review_words<'a>(
    session_type: &str,
    limit: usize,
    candidate_words: &[&'a Word],
    cards: &[SrsCard],
    all_srs_word_ids: &HashSet<u32>,
) -> Vec<&'a Word> {
    let mut review_words: Vec<&Word> = candidate_words
        .iter()
        .filter(|w| cards.iter().any(|c| c.word_id == w.id))
        .cloned()
        .collect();

    if session_type == "new_only" || session_type == "mixed" {
        let mut existing_ids: HashSet<u32> =
            cards.iter().map(|c| c.word_id).collect();

        for word in candidate_words.iter() {
            if review_words.len() >= limit {
                break;
            }
            if existing_ids.contains(&word.id) {
                continue;
            }
            if !all_srs_word_ids.contains(&word.id) {
                review_words.push(*word);
                existing_ids.insert(word.id);
            }
        }
    }

    review_words
}

#[get("/api/review")]
async fn get_review_cards(
    state: web::Data<AppState>,
    query: web::Query<ReviewQuery>,
) -> impl Responder {
    match &state.pool {
        Some(pool) => {
            let now = chrono::Utc::now().timestamp();
            let limit = query.limit.unwrap_or(20);
            let session_type = query.session_type.as_deref().unwrap_or("standard");

            let all_srs_word_ids: HashSet<u32> =
                sqlx::query_scalar::<_, i32>("SELECT word_id FROM srs_cards")
                    .fetch_all(pool)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|id| id as u32)
                    .collect();

            let rows = match session_type {
                "new_only" => {
                    sqlx::query(
                        "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level, is_relearning
                         FROM srs_cards
                         WHERE repetitions = 0 AND next_review = 0
                         ORDER BY word_id ASC
                         LIMIT $1"
                    )
                    .bind(limit as i32)
                    .fetch_all(pool)
                    .await
                },
                "mixed" => {
                    let due_limit = limit / 2;
                    let new_limit = limit - due_limit;

                    let due_rows = sqlx::query(
                        "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level, is_relearning
                         FROM srs_cards
                         WHERE next_review <= $1 AND repetitions > 0
                         ORDER BY box_level ASC, next_review ASC
                         LIMIT $2"
                    )
                    .bind(now)
                    .bind(due_limit as i32)
                    .fetch_all(pool)
                    .await;

                    let new_rows = sqlx::query(
                        "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level, is_relearning
                         FROM srs_cards
                         WHERE repetitions = 0 AND next_review = 0
                         ORDER BY word_id ASC
                         LIMIT $1"
                    )
                    .bind(new_limit as i32)
                    .fetch_all(pool)
                    .await;

                    match (due_rows, new_rows) {
                        (Ok(mut due), Ok(new)) => {
                            due.extend(new);
                            use rand::seq::SliceRandom;
                            let mut rng = rand::thread_rng();
                            due.shuffle(&mut rng);
                            Ok(due)
                        },
                        _ => Ok(Vec::new()),
                    }
                },
                _ => {
                    sqlx::query(
                        "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level, is_relearning
                         FROM srs_cards
                         WHERE next_review <= $1
                         ORDER BY box_level ASC, next_review ASC
                         LIMIT $2"
                    )
                    .bind(now)
                    .bind(limit as i32)
                    .fetch_all(pool)
                    .await
                }
            }
            .unwrap_or_default();

            let cards: Vec<SrsCard> = rows
                .iter()
                .filter_map(|row| SrsCard::from_row(row).ok())
                .collect();

            let candidate_words: Vec<&Word> = if let Some(level) = query.level {
                state.words.iter().filter(|w| w.level == level).collect()
            } else {
                state.words.iter().collect()
            };

            let filtered_cards: Vec<SrsCard> = cards
                .into_iter()
                .filter(|c| candidate_words.iter().any(|w| w.id == c.word_id))
                .collect();

            let review_words = select_review_words(
                session_type,
                limit,
                &candidate_words,
                &filtered_cards,
                &all_srs_word_ids,
            );

            HttpResponse::Ok().json(review_words)
        }
        None => {
            let mock_words: Vec<&Word> = state.words.iter().take(10).collect();
            HttpResponse::Ok().json(mock_words)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_word(id: u32) -> Word {
        Word {
            id,
            hanzi: format!("汉字{}", id),
            pinyin: format!("pinyin{}", id),
            english: format!("word{}", id),
            level: 1,
            sentence_cn: String::new(),
            sentence_pinyin: String::new(),
            sentence_en: String::new(),
        }
    }

    fn make_test_card(word_id: u32) -> SrsCard {
        SrsCard {
            word_id,
            ease_factor: 2.5,
            interval: 0,
            repetitions: 0,
            next_review: 0,
            box_level: 0,
            is_relearning: false,
        }
    }

    #[test]
    fn new_only_includes_words_missing_from_srs_cards() {
        let words = vec![
            make_test_word(1),
            make_test_word(2),
            make_test_word(3),
        ];
        let candidate_words: Vec<&Word> = words.iter().collect();
        let cards = vec![make_test_card(1)];
        let all_srs_word_ids: HashSet<u32> = HashSet::from([1]);

        let selected = select_review_words("new_only", 5, &candidate_words, &cards, &all_srs_word_ids);

        assert!(selected.iter().any(|w| w.id == 2));
        assert!(selected.iter().any(|w| w.id == 3));
        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn mixed_session_avoids_duplicate_cards_and_still_adds_missing_words() {
        let words = vec![
            make_test_word(1),
            make_test_word(2),
            make_test_word(3),
            make_test_word(4),
        ];
        let candidate_words: Vec<&Word> = words.iter().collect();
        let cards = vec![make_test_card(1), make_test_card(2)];
        let all_srs_word_ids: HashSet<u32> = HashSet::from([1, 2]);

        let selected = select_review_words("mixed", 4, &candidate_words, &cards, &all_srs_word_ids);

        assert!(selected.iter().any(|w| w.id == 1));
        assert!(selected.iter().any(|w| w.id == 2));
        assert!(selected.iter().any(|w| w.id == 3));
        assert!(selected.iter().any(|w| w.id == 4));
        assert_eq!(selected.len(), 4);
    }
}

#[derive(Deserialize)]
struct ReviewSubmit {
    word_id: u32,
    rating: u8,
}

#[post("/api/review")]
async fn submit_review(
    state: web::Data<AppState>,
    body: web::Json<ReviewSubmit>,
) -> impl Responder {
    match &state.pool {
        Some(pool) => {
            // Get current card data from database
            let row = sqlx::query(
                "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level, is_relearning
                 FROM srs_cards WHERE word_id = $1"
            )
            .bind(body.word_id as i32)
            .fetch_optional(pool)
            .await
            .unwrap_or(None);

            if let Some(row) = row {
                if let Ok(mut card) = SrsCard::from_row(&row) {
                    let _previous_box_level = card.box_level;
                    card.review(body.rating);

                    // Log the review
                    let _ = sqlx::query(
                        "INSERT INTO review_log (word_id, rating, reviewed_at, was_relearning)
                         VALUES ($1, $2, $3, $4)",
                    )
                    .bind(body.word_id as i32)
                    .bind(body.rating as i32)
                    .bind(chrono::Utc::now().timestamp())
                    .bind(card.is_relearning)
                    .execute(pool)
                    .await;

                    // Update in database
                    let _ = sqlx::query(
                        "UPDATE srs_cards SET ease_factor = $1, interval = $2, repetitions = $3,
                         next_review = $4, box_level = $5, is_relearning = $6, updated_at = CURRENT_TIMESTAMP
                         WHERE word_id = $7"
                    )
                    .bind(card.ease_factor)
                    .bind(card.interval as i32)
                    .bind(card.repetitions as i32)
                    .bind(card.next_review)
                    .bind(card.box_level as i16)
                    .bind(card.is_relearning)
                    .bind(body.word_id as i32)
                    .execute(pool)
                    .await;
                }
            }
            HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
        }
        None => {
            // Mock response for testing
            HttpResponse::Ok().json(serde_json::json!({"status": "ok", "mock": true}))
        }
    }
}

#[derive(Deserialize)]
struct QuizQuery {
    mode: Option<String>,
    count: Option<usize>,
    level: Option<u8>,
}

#[get("/api/quiz")]
async fn get_quiz(state: web::Data<AppState>, query: web::Query<QuizQuery>) -> impl Responder {
    let count = query.count.unwrap_or(10);
    let mut rng = rand::thread_rng();

    let filtered: Vec<&Word> = match query.level {
        Some(l) => state.words.iter().filter(|w| w.level == l).collect(),
        None => state.words.iter().collect(),
    };

    let mut selected: Vec<&&Word> = filtered.iter().collect();
    selected.shuffle(&mut rng);
    selected.truncate(count);

    #[derive(Serialize)]
    struct QuizItem {
        word: Word,
        choices: Vec<String>,
        correct_index: usize,
    }

    let mode = query.mode.as_deref().unwrap_or("en_to_cn");

    let items: Vec<QuizItem> = selected
        .iter()
        .map(|&&ref word| {
            let correct_answer = match mode {
                "en_to_cn" => word.hanzi.clone(),
                "en_to_pinyin" => word.pinyin.clone(),
                "cn_to_en" => word.english.clone(),
                "cn_to_pinyin" => word.pinyin.clone(),
                _ => word.hanzi.clone(),
            };

            // Generate 3 wrong choices
            let mut wrong: Vec<String> = filtered
                .iter()
                .filter(|w| w.id != word.id)
                .map(|w| match mode {
                    "en_to_cn" => w.hanzi.clone(),
                    "en_to_pinyin" => w.pinyin.clone(),
                    "cn_to_en" => w.english.clone(),
                    "cn_to_pinyin" => w.pinyin.clone(),
                    _ => w.hanzi.clone(),
                })
                .collect();
            wrong.shuffle(&mut rng);
            wrong.truncate(3);

            let correct_index = rand::random::<usize>() % 4;
            let mut choices = wrong;
            choices.insert(correct_index, correct_answer);

            QuizItem {
                word: (*word).clone(),
                choices,
                correct_index,
            }
        })
        .collect();

    HttpResponse::Ok().json(items)
}

// ---- Embedded frontend files ----
const INDEX_HTML: &str = include_str!("../frontend/index.html");
const APP_JS: &str = include_str!("../frontend/app.js");
const STYLE_CSS: &str = include_str!("../frontend/style.css");

fn is_authenticated(req: &HttpRequest) -> bool {
    req.cookie("hsk_auth")
        .map(|c| c.value() == "1")
        .unwrap_or(false)
}

fn redirect_login() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header(("Location", "/login"))
        .finish()
}

#[get("/app.js")]
async fn serve_js(req: HttpRequest) -> HttpResponse {
    if !is_authenticated(&req) {
        return redirect_login();
    }
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(APP_JS)
}

#[get("/style.css")]
async fn serve_css(req: HttpRequest) -> HttpResponse {
    if !is_authenticated(&req) {
        return redirect_login();
    }
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(STYLE_CSS)
}

async fn serve_index(req: HttpRequest) -> HttpResponse {
    if !is_authenticated(&req) {
        return redirect_login();
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_HTML)
}

#[get("/api/stories")]
async fn get_stories() -> impl Responder {
    HttpResponse::Ok().json(stories::get_stories_meta())
}

#[get("/api/stories/{id}")]
async fn get_story(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let all = stories::get_all_stories();
    match all.into_iter().find(|s| s.id == id) {
        Some(story) => HttpResponse::Ok().json(story),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Story not found"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/hsk".to_string());

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(p) => {
            println!("✅ Connected to database");
            Some(p)
        }
        Err(e) => {
            println!(
                "⚠️  Database connection failed: {}. Running in test mode with mock data.",
                e
            );
            None
        }
    };

    // Only run migrations if we have a database connection
    if let Some(ref pool) = pool {
        // Run migrations
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS srs_cards (
                word_id INTEGER PRIMARY KEY,
                ease_factor FLOAT NOT NULL DEFAULT 2.5,
                interval INTEGER NOT NULL DEFAULT 0,
                repetitions INTEGER NOT NULL DEFAULT 0,
                next_review BIGINT NOT NULL DEFAULT 0,
                box_level SMALLINT NOT NULL DEFAULT 0,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(pool)
        .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_next_review ON srs_cards(next_review)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_box_level ON srs_cards(box_level)")
            .execute(pool)
            .await;

        let words = get_all_words();
        let _init = load_cards(pool, &words);
    }

    let words = get_all_words();

    let state = web::Data::new(AppState {
        words,
        pool: pool.clone(),
    });

    let password = std::env::var("HSK_PASSWORD").unwrap_or_else(|_| "hsk2025".to_string());
    let secret = Arc::new(password);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Starting server on http://{}", addr);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::Data::new(secret.clone()))
            .route("/login", web::get().to(auth::login_get))
            .route("/login", web::post().to(auth::login_post))
            .service(get_words)
            .service(get_words_by_level)
            .service(get_stats)
            .service(get_review_cards)
            .service(submit_review)
            .service(get_quiz)
            .service(get_stories)
            .service(get_story)
            .service(serve_js)
            .service(serve_css)
            .service(get_progress_stats)
            .default_service(web::get().to(serve_index))
    })
    .bind(&addr);

    match server {
        Ok(server) => {
            println!("✅ Successfully bound to {}", addr);
            let result = server
                .workers(2)
                .shutdown_timeout(300) // 5 minutes
                .run()
                .await;
            println!("Server run result: {:?}", result);
            result?;
            println!("Server stopped normally");
        }
        Err(e) => {
            println!("❌ Failed to bind to {}: {}", addr, e);
            return Err(e.into());
        }
    }

    Ok(())
}
