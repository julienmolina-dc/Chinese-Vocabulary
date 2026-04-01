# 🀄 Julien's Chinese Vocabulary

A spaced-repetition vocabulary learning app for HSK 1/2 + class vocabulary (410 words) with a Rust backend and vanilla JS frontend.

## Learning Method

Based on research-backed techniques:
- Spaced Repetition (SM-2 + Leitner boxes)
- Active Recall through flashcards
- Context Learning with example sentences
- Multi-directional practice (4 modes + mixed)
- Reading practice with annotated Chinese stories

## Running Locally

```bash
cargo run
```
Open `http://localhost:8080` — default password: `hsk2025`

Set a custom password:
```bash
HSK_PASSWORD=mypassword cargo run
```

## Deploy to Render with Persistent Storage (Supabase)

### 1. Set up Supabase (Free PostgreSQL)

1. Go to [supabase.com](https://supabase.com)
2. Click "Start your project" and sign in with GitHub
3. Create a new project with default settings
4. In the "SQL Editor" tab, run the migration from `migrations/001_init.sql`
   - Click "New query"
   - Paste the contents of `migrations/001_init.sql`
   - Click "Run"
5. In the "Settings > Database" section, copy your connection string:
   - Copy the URI under "Connection String"
   - It should look like: `postgresql://postgres.[project-id]:[password]@aws-0-[region].pooler.supabase.com:6543/postgres`

### 2. Deploy to Render

1. Create a new Web Service on Render
2. Connect your GitHub repo
3. Set build command: `cargo build --release`
4. Set start command: `./target/release/julien-chinese-vocab`
5. Add environment variables:
   - `DATABASE_URL`: Paste your Supabase connection string
   - `HSK_PASSWORD`: Your desired login password
   - `PORT`: `8080` (default)
6. Deploy

Your app will be at the URL provided by Render, with persistent progress storage via Supabase!

### Local Development with Supabase

1. Copy `.env.example` to `.env`
2. Update `DATABASE_URL` with your Supabase connection string
3. Run `cargo run` to start locally
4. Progress will now persist to your Supabase database instead of local files

## Features

- Dashboard with progress tracking
- SRS flashcards (Mixed / EN→汉字 / EN→Pinyin / 汉字→EN / 汉字→Pinyin)
- Multiple-choice quiz
- Browsable word list (HSK 1 / HSK 2 / Class vocabulary)
- 5 annotated Chinese stories (hover for pinyin, click for translation)
- Password-protected access
- **Persistent progress storage** (PostgreSQL via Supabase for production, local SQLite optional for development)
