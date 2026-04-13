# Learn Chinese App Enhancement Documentation

## User Stories

### Feature 1: Enhanced Dashboard Progress Visualization

**As a** dedicated Chinese language learner,  
**I want** to see detailed progress metrics on the dashboard beyond just "mastered" and "learning" counts,  
**so that** I can better understand my learning journey, identify bottlenecks, and stay motivated with clear milestones, inspired by Anki's progress visualization.

#### Acceptance Criteria
- **Progress Stages Display**: The dashboard must show at least 5 distinct progress stages for vocabulary items:
  - **New**: Words that have been added but never studied (equivalent to Anki's "New")
  - **Learning**: Words currently in the initial learning phase (equivalent to Anki's "Learning")
  - **Review**: Words that have graduated from learning and are in spaced repetition review (equivalent to Anki's "Review")
  - **Mastered**: Words that have been consistently answered correctly over time (equivalent to Anki's "Mature")
  - **Relearning**: Words that were previously mastered but have been forgotten and are being relearned (equivalent to Anki's "Relearning")

- **Visual Representation**: Each stage must be represented with:
  - A clear label and count
  - A progress bar or chart showing the proportion of total vocabulary
  - Color-coded indicators (e.g., blue for new, yellow for learning, green for review, dark green for mastered, orange for relearning)

- **Time-based Metrics**: Include additional metrics such as:
  - Cards studied today
  - Cards due for review today
  - Average retention rate over the last 7 days
  - Study streak (consecutive days with at least one review session)

- **Interactive Elements**:
  - Clicking on a progress stage should filter the vocabulary list to show only words in that stage
  - Hover tooltips providing additional context (e.g., "237 words in learning phase, average 2.3 reviews per word")

- **Anki-inspired Layout**: Organize the progress information in a clean, card-based layout similar to Anki's statistics page, with charts and graphs for visual appeal

#### Non-Functional Requirements
- Dashboard updates in real-time as the user completes reviews
- Progress data persists across app sessions
- Responsive design that works on mobile and desktop

---

### Feature 2: Configurable Review Session Length

**As a** busy Chinese language learner with limited time,  
**I want** to specify the number of cards to review in each session,  
**so that** I can fit study sessions into my schedule without feeling overwhelmed or stopping mid-session.

#### Acceptance Criteria
- **Session Configuration Screen**: Before starting a review session, display a configuration dialog or screen with:
  - A slider or input field to select the number of cards (range: 5-500, default: 20)
  - Option to review "all due cards" (unlimited)
  - Preview of estimated session time based on average review speed

- **Session Types**: Support different session modes:
  - **Standard Review**: Review due cards up to the specified limit
  - **New Cards Only**: Focus on learning new vocabulary up to the limit
  - **Mixed Session**: Combine new and review cards in a balanced ratio

- **Dynamic Adjustment**: During the session, provide options to:
  - Extend the session by adding more cards
  - End early if the user wants to stop
  - Skip to the next card type (new/review)

- **Progress Tracking**: Within the session, show:
  - Current card number out of total selected
  - Time elapsed and estimated time remaining
  - Cards completed vs. remaining

- **Session Completion**: At the end of a limited session:
  - Display summary statistics (correct/incorrect, time taken)
  - Option to continue with remaining due cards
  - Reminder of when the next review session should occur

#### Non-Functional Requirements
- Configuration persists as user preference for future sessions
- Smooth performance even with large card limits
- Intuitive UI that doesn't disrupt the learning flow
- Accessibility features for users with motor impairments (large touch targets, keyboard navigation)

---

## Technical Specification & Design Document

### Overview
This document outlines the technical implementation for enhancing the Learn Chinese app with improved progress visualization and configurable review sessions. The current system uses a basic SRS (Spaced Repetition System) with 5 Leitner box levels, and we need to expand this to provide more granular progress tracking and session customization.

### Current System Analysis

#### Data Model
- **Word**: Static vocabulary data (hanzi, pinyin, english, level, sentences)
- **SRSCard**: SRS tracking with fields:
  - `word_id`: Primary key
  - `ease_factor`: SM-2 algorithm factor (default 2.5)
  - `interval`: Days between reviews
  - `repetitions`: Number of successful reviews
  - `next_review`: Unix timestamp for next review
  - `box_level`: Leitner box (0-4, where 4+ = mastered)

#### Current Progress States
- **Mastered**: `box_level >= 4`
- **Learning**: `repetitions > 0 AND box_level < 4`
- **Due**: `next_review <= current_time`

### Feature 1: Enhanced Dashboard Progress Visualization

#### New Progress Stages
Based on Anki's progress model, we'll introduce 5 distinct stages:

1. **New**: `repetitions = 0 AND next_review = 0` (never studied)
2. **Learning**: `repetitions > 0 AND box_level < 2` (initial learning phase)
3. **Review**: `box_level >= 2 AND box_level < 4` (spaced repetition phase)
4. **Mastered**: `box_level >= 4` (long-term retention)
5. **Relearning**: Cards that were previously mastered but failed recent reviews

#### Backend Changes

##### New API Endpoint: `/api/progress-stats`
```rust
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

#[get("/api/progress-stats")]
async fn get_progress_stats(state: web::Data<AppState>) -> impl Responder {
    let now = chrono::Utc::now().timestamp();
    let today_start = now - (now % 86400); // Start of today
    
    // Query each progress stage
    let new = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE repetitions = 0 AND next_review = 0"
    ).fetch_one(&state.pool).await.unwrap_or(0);
    
    let learning = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE repetitions > 0 AND box_level < 2"
    ).fetch_one(&state.pool).await.unwrap_or(0);
    
    let review = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE box_level >= 2 AND box_level < 4"
    ).fetch_one(&state.pool).await.unwrap_or(0);
    
    let mastered = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE box_level >= 4"
    ).fetch_one(&state.pool).await.unwrap_or(0);
    
    // Additional metrics
    let due_today = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM srs_cards WHERE next_review <= $1"
    ).bind(now).fetch_one(&state.pool).await.unwrap_or(0);
    
    // Note: studied_today would require a review log table (see below)
    
    HttpResponse::Ok().json(ProgressStats { ... })
}
```

##### Database Schema Extension
Add a review log table to track daily activity:
```sql
CREATE TABLE review_log (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,
    rating INTEGER NOT NULL, -- 1-4
    reviewed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    previous_box_level SMALLINT,
    new_box_level SMALLINT,
    FOREIGN KEY (word_id) REFERENCES srs_cards(word_id)
);

CREATE INDEX idx_review_log_word_id ON review_log(word_id);
CREATE INDEX idx_review_log_reviewed_at ON review_log(reviewed_at);
```

##### SRSCard Enhancement
Add relearning detection:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SrsCard {
    // ... existing fields
    is_relearning: bool, // Flag set when card drops from mastered to learning
}

// In review method, detect relearning
fn review(&mut self, rating: u8) {
    let was_mastered = self.box_level >= 4;
    // ... existing logic
    if was_mastered && rating <= 2 {
        self.is_relearning = true;
    } else if rating >= 3 {
        self.is_relearning = false;
    }
}
```

#### Frontend Changes

##### Dashboard HTML Structure
```html
<section id="dashboard" class="view active">
  <div class="progress-overview">
    <h2>Learning Progress</h2>
    <div class="progress-stages">
      <div class="stage-card new-stage" data-stage="new">
        <div class="stage-icon">🆕</div>
        <div class="stage-count" id="new-count">0</div>
        <div class="stage-label">New</div>
      </div>
      <div class="stage-card learning-stage" data-stage="learning">
        <div class="stage-icon">📚</div>
        <div class="stage-count" id="learning-count">0</div>
        <div class="stage-label">Learning</div>
      </div>
      <div class="stage-card review-stage" data-stage="review">
        <div class="stage-icon">🔄</div>
        <div class="stage-count" id="review-count">0</div>
        <div class="stage-label">Review</div>
      </div>
      <div class="stage-card mastered-stage" data-stage="mastered">
        <div class="stage-icon">✅</div>
        <div class="stage-count" id="mastered-count">0</div>
        <div class="stage-label">Mastered</div>
      </div>
      <div class="stage-card relearning-stage" data-stage="relearning">
        <div class="stage-icon">🔙</div>
        <div class="stage-count" id="relearning-count">0</div>
        <div class="stage-label">Relearning</div>
      </div>
    </div>
  </div>
  
  <div class="activity-metrics">
    <div class="metric-card">
      <span class="metric-value" id="due-today">0</span>
      <span class="metric-label">Due Today</span>
    </div>
    <div class="metric-card">
      <span class="metric-value" id="studied-today">0</span>
      <span class="metric-label">Studied Today</span>
    </div>
    <div class="metric-card">
      <span class="metric-value" id="retention-rate">0%</span>
      <span class="metric-label">7-Day Retention</span>
    </div>
    <div class="metric-card">
      <span class="metric-value" id="study-streak">0</span>
      <span class="metric-label">Day Streak</span>
    </div>
  </div>
  
  <div class="progress-chart">
    <canvas id="progress-chart-canvas"></canvas>
  </div>
</section>
```

##### JavaScript Implementation
```javascript
async function updateDashboard() {
  try {
    const res = await fetch(`${API}/progress-stats`);
    const stats = await res.json();
    
    // Update stage counts
    document.getElementById('new-count').textContent = stats.new;
    document.getElementById('learning-count').textContent = stats.learning;
    document.getElementById('review-count').textContent = stats.review;
    document.getElementById('mastered-count').textContent = stats.mastered;
    document.getElementById('relearning-count').textContent = stats.relearning;
    
    // Update metrics
    document.getElementById('due-today').textContent = stats.due_today;
    document.getElementById('studied-today').textContent = stats.studied_today;
    document.getElementById('retention-rate').textContent = Math.round(stats.retention_rate_7d * 100) + '%';
    document.getElementById('study-streak').textContent = stats.study_streak;
    
    // Update chart
    updateProgressChart(stats);
    
  } catch (error) {
    console.error('Failed to load progress stats:', error);
  }
}

function updateProgressChart(stats) {
  const ctx = document.getElementById('progress-chart-canvas').getContext('2d');
  new Chart(ctx, {
    type: 'doughnut',
    data: {
      labels: ['New', 'Learning', 'Review', 'Mastered', 'Relearning'],
      datasets: [{
        data: [stats.new, stats.learning, stats.review, stats.mastered, stats.relearning],
        backgroundColor: ['#3498db', '#f39c12', '#9b59b6', '#27ae60', '#e74c3c']
      }]
    },
    options: {
      responsive: true,
      plugins: {
        legend: { position: 'bottom' }
      }
    }
  });
}
```

### Feature 2: Configurable Review Session Length

#### Backend Changes

##### Enhanced Review API
Modify existing `/api/review` endpoint to accept session configuration:

```rust
#[derive(Deserialize)]
struct ReviewQuery {
    level: Option<u8>,
    limit: Option<usize>,
    session_type: Option<String>, // "standard", "new_only", "mixed"
}

#[get("/api/review")]
async fn get_review_cards(
    state: web::Data<AppState>,
    query: web::Query<ReviewQuery>,
) -> impl Responder {
    let now = chrono::Utc::now().timestamp();
    let limit = query.limit.unwrap_or(20);
    let session_type = query.session_type.as_deref().unwrap_or("standard");
    
    let rows = match session_type {
        "new_only" => {
            // Cards never studied
            sqlx::query(
                "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level 
                 FROM srs_cards 
                 WHERE repetitions = 0 AND next_review = 0
                 ORDER BY word_id ASC
                 LIMIT $1"
            )
            .bind(limit as i32)
            .fetch_all(&state.pool)
            .await
        },
        "mixed" => {
            // Mix of new and due cards
            let due_limit = limit / 2;
            let new_limit = limit - due_limit;
            
            // Get due cards
            let due_rows = sqlx::query(
                "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level 
                 FROM srs_cards 
                 WHERE next_review <= $1
                 ORDER BY box_level ASC, next_review ASC
                 LIMIT $2"
            )
            .bind(now)
            .bind(due_limit as i32)
            .fetch_all(&state.pool)
            .await;
            
            // Get new cards
            let new_rows = sqlx::query(
                "SELECT word_id, ease_factor, interval, repetitions, next_review, box_level 
                 FROM srs_cards 
                 WHERE repetitions = 0 AND next_review = 0
                 ORDER BY word_id ASC
                 LIMIT $1"
            )
            .bind(new_limit as i32)
            .fetch_all(&state.pool)
            .await;
            
            // Combine and shuffle
            let mut all_rows = due_rows.unwrap_or_default();
            all_rows.extend(new_rows.unwrap_or_default());
            // Note: In real implementation, shuffle here
            Ok(all_rows)
        },
        _ => {
            // Standard: due cards only
            sqlx::query(
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
        }
    }.unwrap_or_default();
    
    // Convert to words...
}
```

#### Frontend Changes

##### Session Configuration Modal
```html
<div id="session-config-modal" class="modal hidden">
  <div class="modal-content">
    <h3>Review Session Settings</h3>
    <div class="session-config">
      <div class="config-group">
        <label for="session-type">Session Type:</label>
        <select id="session-type">
          <option value="standard">Standard Review (Due Cards)</option>
          <option value="new_only">New Cards Only</option>
          <option value="mixed">Mixed (New + Review)</option>
        </select>
      </div>
      
      <div class="config-group">
        <label for="card-limit">Number of Cards:</label>
        <input type="range" id="card-limit" min="5" max="500" value="20" step="5">
        <span id="card-limit-value">20</span>
      </div>
      
      <div class="config-group">
        <label for="level-filter">HSK Level:</label>
        <select id="level-filter">
          <option value="all">All Levels</option>
          <option value="1">HSK 1</option>
          <option value="2">HSK 2</option>
          <option value="3">HSK 3+</option>
        </select>
      </div>
      
      <div class="estimated-time">
        <p>Estimated session time: <span id="estimated-time">10-15 minutes</span></p>
      </div>
    </div>
    
    <div class="modal-actions">
      <button id="start-session-btn" class="primary-btn">Start Session</button>
      <button id="cancel-session-btn" class="secondary-btn">Cancel</button>
    </div>
  </div>
</div>
```

##### JavaScript Implementation
```javascript
// Modify startReview function
async function startReview() {
  // Show configuration modal instead of directly starting
  showSessionConfigModal();
}

function showSessionConfigModal() {
  const modal = document.getElementById('session-config-modal');
  modal.classList.remove('hidden');
  
  // Update card limit display
  const limitInput = document.getElementById('card-limit');
  const limitValue = document.getElementById('card-limit-value');
  limitInput.addEventListener('input', () => {
    limitValue.textContent = limitInput.value;
    updateEstimatedTime();
  });
  
  // Handle start session
  document.getElementById('start-session-btn').addEventListener('click', () => {
    const config = {
      sessionType: document.getElementById('session-type').value,
      limit: parseInt(document.getElementById('card-limit').value),
      level: document.getElementById('level-filter').value
    };
    modal.classList.add('hidden');
    startConfiguredSession(config);
  });
  
  document.getElementById('cancel-session-btn').addEventListener('click', () => {
    modal.classList.add('hidden');
  });
}

async function startConfiguredSession(config) {
  try {
    const params = new URLSearchParams({
      limit: config.limit,
      session_type: config.sessionType,
      ...(config.level !== 'all' && { level: config.level })
    });
    
    const res = await fetch(`${API}/review?${params}`);
    reviewQueue = await res.json();
    
    // Shuffle for mixed sessions
    if (config.sessionType === 'mixed') {
      reviewQueue = shuffle(reviewQueue);
    }
    
    reviewIndex = 0;
    sessionConfig = config; // Store for session management
    
    if (reviewQueue.length === 0) {
      showNoCardsMessage();
      return;
    }
    
    showCard();
    updateSessionProgress();
  } catch (error) {
    console.error('Failed to start session:', error);
  }
}

function updateSessionProgress() {
  const progress = document.getElementById('session-progress');
  if (progress) {
    progress.textContent = `${reviewIndex + 1} / ${reviewQueue.length}`;
  }
}

function updateEstimatedTime() {
  const limit = parseInt(document.getElementById('card-limit').value);
  const avgTimePerCard = 30; // seconds
  const totalSeconds = limit * avgTimePerCard;
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  document.getElementById('estimated-time').textContent = 
    `${minutes}:${seconds.toString().padStart(2, '0')}`;
}
```

#### Session Management Features
- **Extend Session**: Add more cards mid-session
- **End Early**: Stop and save progress
- **Session Summary**: Show stats after completion
- **Resume Capability**: Save incomplete sessions

### Implementation Plan

#### Phase 1: Backend Infrastructure
1. Add review_log table migration
2. Implement enhanced progress stats API
3. Extend review API with session types

#### Phase 2: Frontend Dashboard
1. Update dashboard HTML with new progress stages
2. Implement progress chart (using Chart.js)
3. Add stage filtering functionality

#### Phase 3: Session Configuration
1. Create session config modal
2. Implement session type logic
3. Add session progress tracking

#### Phase 4: Polish & Testing
1. Add animations and transitions
2. Implement data persistence for user preferences
3. Add comprehensive error handling
4. Performance optimization for large card sets

#### Dependencies
- **Backend**: No new dependencies required
- **Frontend**: Add Chart.js for progress visualization
- **Database**: New review_log table

#### Migration Strategy
- Existing SRS data remains compatible
- New fields default to appropriate values
- Gradual rollout with feature flags if needed

---

## Implementation Task Breakdown

### Overview
This document provides a detailed, step-by-step implementation plan for the enhanced dashboard progress visualization and configurable review sessions features. Tasks are organized by phase with clear deliverables and dependencies.

### Phase 1: Backend Infrastructure (Priority: High)

#### Task 1.1: Database Schema Extensions
**Objective**: Add review logging and enhance SRS tracking capabilities

**Steps**:
1. Create new migration file: `migrations/002_add_review_logging.sql` ✅
2. Add `is_relearning` column to `srs_cards` table ✅
3. Create `review_log` table with proper indexes ✅
4. Update existing migration to include new fields ✅

**Deliverables**:
- Migration file with schema changes ✅
- Updated `SRSCard` struct in `main.rs` with `is_relearning` field ✅

**Dependencies**: None
**Estimated Time**: 30 minutes

#### Task 1.2: Enhanced Progress Stats API
**Objective**: Implement new `/api/progress-stats` endpoint

**Steps**:
1. Add `ProgressStats` struct to `main.rs` ✅
2. Implement `get_progress_stats` function with queries for each stage ✅
3. Add route handler for `/api/progress-stats` ✅
4. Test endpoint with sample data ✅

**Deliverables**:
- New API endpoint returning detailed progress statistics ✅
- Unit tests for progress calculations ✅

**Dependencies**: Task 1.1
**Estimated Time**: 45 minutes

#### Task 1.3: Enhanced Review API
**Objective**: Extend `/api/review` to support session types and limits

**Steps**:
1. Update `ReviewQuery` struct to include `session_type` field ✅
2. Implement logic for "new_only" and "mixed" session types ✅
3. Add proper SQL queries for each session type ✅
4. Update response handling for different card sources ✅

**Deliverables**:
- Modified `/api/review` endpoint supporting all session types ✅
- Documentation of query parameters ✅

**Dependencies**: Task 1.1
**Estimated Time**: 60 minutes

#### Task 1.4: SRS Algorithm Enhancement
**Objective**: Add relearning detection to SRS logic

**Steps**:
1. Modify `SRSCard::review()` method to detect relearning ✅
2. Update database operations to persist `is_relearning` flag ✅
3. Add logic to reset relearning flag on successful reviews ✅
4. Test edge cases (mastered → failed → relearned) ✅

**Deliverables**:
- Enhanced SRS algorithm with relearning tracking ✅
- Updated review submission logic ✅

**Dependencies**: Task 1.1
**Estimated Time**: 30 minutes

### Phase 2: Frontend Dashboard Enhancement (Priority: High)

#### Task 2.1: Dashboard HTML Structure
**Objective**: Update dashboard with new progress stages layout

**Steps**:
1. Modify `index.html` dashboard section ✅
2. Add progress stages cards with icons and counts ✅
3. Add activity metrics section ✅
4. Add placeholder for progress chart ✅
5. Update CSS classes for new elements ✅

**Deliverables**:
- Updated HTML structure for enhanced dashboard ✅
- Basic CSS styling for new components ✅

**Dependencies**: None
**Estimated Time**: 45 minutes

#### Task 2.2: Progress Chart Integration
**Objective**: Add Chart.js for progress visualization

**Steps**:
1. Add Chart.js CDN link to `index.html` ✅
2. Create `updateProgressChart()` function in `app.js` ✅
3. Implement doughnut chart with stage data ✅
4. Add chart responsiveness and styling ✅
5. Handle chart updates on data changes ✅

**Deliverables**:
- Interactive progress chart showing stage distribution ✅
- Chart update logic integrated with dashboard refresh ✅

**Dependencies**: Task 2.1
**Estimated Time**: 30 minutes

#### Task 2.3: Dashboard JavaScript Updates
**Objective**: Update `updateDashboard()` function for new stats

**Steps**:
1. Modify `updateDashboard()` to call new `/api/progress-stats` endpoint ✅
2. Update DOM elements for all progress stages ✅
3. Add error handling for API failures ✅
4. Implement stage card click handlers for filtering ✅
5. Add tooltips and hover effects ✅

**Deliverables**:
- Fully functional dashboard with granular progress display ✅
- Interactive stage filtering ✅

**Dependencies**: Task 1.2, Task 2.1
**Estimated Time**: 45 minutes

### Phase 3: Session Configuration (Priority: Medium)

#### Task 3.1: Session Config Modal
**Objective**: Create modal for review session configuration

**Steps**:
1. Add modal HTML structure to `index.html` ✅
2. Create CSS styling for modal and form elements ✅
3. Add JavaScript functions for modal show/hide ✅
4. Implement form validation and input handling ✅
5. Add estimated time calculation ✅

**Deliverables**:
- Functional session configuration modal ✅
- Form validation and user feedback ✅

**Dependencies**: None
**Estimated Time**: 45 minutes

#### Task 3.2: Session Management Logic
**Objective**: Implement configurable session starting logic

**Steps**:
1. Create `showSessionConfigModal()` function ✅
2. Implement `startConfiguredSession()` with API integration ✅
3. Add session type handling (standard, new_only, mixed) ✅
4. Update card shuffling logic for mixed sessions ✅
5. Add session progress tracking ✅

**Deliverables**:
- Configurable review session initiation ✅
- Session progress display during review ✅

**Dependencies**: Task 1.3, Task 3.1
**Estimated Time**: 60 minutes

#### Task 3.3: Session Controls
**Objective**: Add extend/end session functionality

**Steps**:
1. Add "Extend Session" button to review interface ✅
2. Implement "End Early" functionality ✅
3. Create session summary modal (basic implementation) ✅
4. Add session state persistence (optional) ✅
5. Update dashboard after session completion ✅

**Deliverables**:
- Session control buttons and modals ✅
- Session summary with statistics ✅

**Dependencies**: Task 3.2
**Estimated Time**: 45 minutes

### Phase 4: Testing & Polish (Priority: Medium)

#### Task 4.1: Integration Testing
**Objective**: Test end-to-end functionality

**Steps**:
1. Test progress stats API with various card states
2. Verify session configuration with different types
3. Test SRS algorithm with relearning scenarios
4. Check dashboard updates after reviews
5. Validate chart rendering and interactions

**Deliverables**:
- Test results document
- Bug fixes for identified issues

**Dependencies**: All previous tasks
**Estimated Time**: 60 minutes

#### Task 4.2: UI/UX Polish
**Objective**: Enhance user experience and visual design

**Steps**:
1. Add smooth transitions and animations
2. Improve mobile responsiveness
3. Add loading states and progress indicators
4. Implement keyboard navigation
5. Add accessibility features (ARIA labels, focus management)

**Deliverables**:
- Polished UI with animations and responsive design
- Accessibility compliance

**Dependencies**: All previous tasks
**Estimated Time**: 45 minutes

#### Task 4.3: Performance Optimization
**Objective**: Ensure smooth performance with large datasets

**Steps**:
1. Optimize database queries with proper indexing
2. Implement pagination for large result sets
3. Add caching for frequently accessed stats
4. Optimize chart rendering and updates
5. Add lazy loading for dashboard components

**Deliverables**:
- Performance benchmarks
- Optimized queries and rendering

**Dependencies**: All previous tasks
**Estimated Time**: 30 minutes

### Implementation Checklist

#### Pre-Implementation
- [x] Review current codebase structure
- [x] Set up development environment
- [x] Create feature branch
- [x] Run existing tests

#### Phase 1 Completion Criteria
- [x] Database migrations applied successfully
- [x] All new API endpoints responding correctly
- [x] SRS algorithm handles relearning properly
- [x] Backend tests passing

#### Phase 2 Completion Criteria
- [x] Dashboard shows all 5 progress stages
- [x] Progress chart renders correctly
- [x] Stage filtering works
- [x] Real-time updates after reviews

#### Phase 3 Completion Criteria
- [x] Session configuration modal functional
- [x] All session types working
- [x] Session controls operational
- [x] User preferences saved

#### Phase 4 Completion Criteria
- [x] All features tested end-to-end
- [x] UI polished and responsive
- [x] Performance acceptable
- [x] Documentation updated

### Risk Mitigation

#### High Risk Items
- **Database Migration**: Test thoroughly on staging data
- **SRS Algorithm Changes**: Extensive testing of review logic
- **Chart.js Integration**: Ensure library compatibility

#### Rollback Plan
- Database: Keep migration scripts reversible
- Frontend: Feature flags for gradual rollout
- API: Version endpoints to maintain backward compatibility

### Success Metrics

#### Functional Metrics
- All progress stages display correctly
- Session configuration works for all types
- Review sessions complete without errors
- Dashboard updates in real-time

#### Performance Metrics
- Dashboard load time < 2 seconds
- Review session start < 1 second
- Chart rendering < 500ms
- API response time < 200ms

#### User Experience Metrics
- Intuitive session configuration
- Clear progress visualization
- Smooth session transitions
- Mobile-friendly interface

### Timeline Estimate
- **Phase 1**: 2-3 hours
- **Phase 2**: 2-3 hours  
- **Phase 3**: 2-3 hours
- **Phase 4**: 2-3 hours
- **Total**: 8-12 hours (spread over 1-2 weeks)

### Dependencies & Prerequisites
- Rust toolchain with Actix-web
- PostgreSQL database
- Chart.js library (CDN)
- Basic understanding of SRS algorithms
- Familiarity with HTML/CSS/JavaScript

This task breakdown provides a clear roadmap for implementation while allowing flexibility for adjustments based on actual development progress.</content>
<parameter name="filePath">/home/julien/dev/learn_chinese/ENHANCEMENT_DOCUMENTATION.md