mod auth;
mod data;
mod stories;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use data::{get_all_words, Word};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
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
    next_review: i64, // unix timestamp
    box_level: u8,    // Leitner box 0-4
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
        })
    }

    fn new(word_id: u32) -> Self {
        SrsCard {
            word_id,
            ease_factor: 2.5,
            interval: 0,
            repetitions: 0,
            next_review: 0,
            box_level: 0,
        }
    }

    // SM-2 algorithm adapted for Leitner boxes
    fn review(&mut self, rating: u8) {
        let now = chrono::Utc::now().timestamp();
        match rating {
            1 => {
                // Again - reset
                self.repetitions = 0;
                self.interval = 0;
                self.box_level = 0;
                self.ease_factor = (self.ease_factor - 0.2).max(1.3);
            }
            2 => {
                // Hard
                self.repetitions += 1;
                self.interval = (self.interval as f64 * 1.2).max(1.0) as u32;
                self.ease_factor = (self.ease_factor - 0.15).max(1.3);
                self.box_level = self.box_level.min(2);
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
            }
            _ => {}
        }
        self.next_review = now + (self.interval as i64 * 86400);
    }
}

struct AppState {
    words: Vec<Word>,
    pool: PgPool,
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
    let now = chrono::Utc::now().timestamp();
    let total = state.words.len();

    let mastered = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE box_level >= 4"
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    let learning = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE repetitions > 0 AND box_level < 4"
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    let due = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE next_review <= $1"
    )
    .bind(now)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(0);

    HttpResponse::Ok().json(serde_json::json!({
        "total": total,
        "mastered": mastered as usize,
        "learning": learning as usize,
        "due": due as usize,
    }))
}

#[derive(Deserialize)]
struct ReviewQuery {
    level: Option<u8>,
    limit: Option<usize>,
}

#[get("/api/review")]
async fn get_review_cards(
    state: web::Data<AppState>,
    query: web::Query<ReviewQuery>,
) -> impl Responder {
    let now = chrono::Utc::now().timestamp();
    let limit = query.limit.unwrap_or(20);

    let rows = sqlx::query(
        "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level 
         FROM srs_cards 
         WHERE next_review <= $1
         ORDER BY box_level ASC, next_review ASC
         LIMIT $2"
    )
    .bind(now)
    .bind(limit as i32)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let mut due_cards: Vec<SrsCard> = rows
        .iter()
        .filter_map(|row| SrsCard::from_row(row).ok())
        .collect();

    // Filter by level if specified
    let filtered_cards: Vec<SrsCard> = if let Some(level) = query.level {
        due_cards
            .into_iter()
            .filter(|c| state.words.iter().any(|w| w.id == c.word_id && w.level == level))
            .collect()
    } else {
        due_cards
    };

    let mut rng = rand::thread_rng();
    let mut shuffled = filtered_cards;
    shuffled.shuffle(&mut rng);

    let word_ids: Vec<u32> = shuffled.iter().map(|c| c.word_id).collect();
    let review_words: Vec<&Word> = state
        .words
        .iter()
        .filter(|w| word_ids.contains(&w.id))
        .collect();

    HttpResponse::Ok().json(review_words)
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
    // Get current card data from database
    let row = sqlx::query(
        "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level 
         FROM srs_cards WHERE word_id = $1"
    )
    .bind(body.word_id as i32)
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None);

    if let Some(row) = row {
        if let Ok(mut card) = SrsCard::from_row(&row) {
            card.review(body.rating);

            // Update in database
            let _ = sqlx::query(
                "UPDATE srs_cards SET ease_factor = $1, interval = $2, repetitions = $3, 
                 next_review = $4, box_level = $5, updated_at = CURRENT_TIMESTAMP 
                 WHERE word_id = $6"
            )
            .bind(card.ease_factor)
            .bind(card.interval as i32)
            .bind(card.repetitions as i32)
            .bind(card.next_review)
            .bind(card.box_level as i16)
            .bind(body.word_id as i32)
            .execute(&state.pool)
            .await;
        }
    }

    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

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
        )"
    )
    .execute(&pool)
    .await;

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_next_review ON srs_cards(next_review)"
    )
    .execute(&pool)
    .await;

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_box_level ON srs_cards(box_level)"
    )
    .execute(&pool)
    .await;

    let words = get_all_words();
    let _init = load_cards(&pool, &words);

    let state = web::Data::new(AppState {
        words,
        pool: pool.clone(),
    });

    let password = std::env::var("HSK_PASSWORD").unwrap_or_else(|_| "hsk2025".to_string());
    let secret = Arc::new(password);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Starting server on http://{}", addr);

    HttpServer::new(move || {
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
            .default_service(web::get().to(serve_index))
    })
    .bind(&addr)?
    .run()
    .await
}
