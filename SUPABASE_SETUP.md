# Supabase Setup Guide for Persistent Storage

This app has been updated to use **PostgreSQL** via **Supabase** for persistent progress storage, making it production-ready on Render's free tier.

## Why Supabase?

- ✅ **Free PostgreSQL** (500MB) - no data loss on redeploys
- ✅ **Industry standard** - PostgreSQL is battle-tested
- ✅ **Easy migration** - trivial to move to another host if needed
- ✅ **No vendor lock-in** - standard SQL, works everywhere
- ✅ **Long-term stable** - Supabase is well-funded and open-source based

## Getting Started

### Local Development

1. **Install Supabase CLI (optional, for local testing)**:
   ```bash
   brew install supabase/tap/supabase
   # or install via Docker
   ```

2. **Get a free Supabase project**:
   - Go to https://supabase.com
   - Sign in with GitHub
   - Create a new project
   - Note your Project URL and database password

3. **Create the database tables**:
   - In Supabase dashboard, go to **SQL Editor**
   - Click **New Query**
   - Paste the contents of `migrations/001_init.sql`
   - Click **Run**

4. **Set up your local `.env` file**:
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` and update:
   ```
   DATABASE_URL=postgresql://postgres.[project-id]:[password]@aws-0-[region].pooler.supabase.com:6543/postgres
   HSK_PASSWORD=hsk2025
   PORT=8080
   ```
   
   Find your `DATABASE_URL` in Supabase → Settings → Database → Connection String

5. **Run locally**:
   ```bash
   cargo run
   ```
   
   Your progress will now persist to Supabase!

### Production Deployment (Render)

1. **Push your code to GitHub**

2. **Create a new Web Service on Render**:
   - Go to https://render.com
   - Click "New +" → "Web Service"
   - Connect your GitHub repository
   - Set build command: `cargo build --release`
   - Set start command: `./target/release/julien-chinese-vocab`

3. **Add environment variables in Render**:
   - Click "Environment"
   - Add `DATABASE_URL`: Your Supabase connection string
   - Add `HSK_PASSWORD`: Your desired login password
   - Add `PORT`: `8080`

4. **Deploy**

Your app will now have **persistent storage across redeploys**!

## Data Structure

The app stores progress in the `srs_cards` table with:
- `word_id`: Which word (matches word ID in your vocabulary)
- `ease_factor`: SM-2 algorithm ease value (difficulty)
- `interval`: Days until next review
- `repetitions`: Total times reviewed
- `next_review`: Unix timestamp of when to show next
- `box_level`: Leitner box (0-4 = not learned to mastered)

## Troubleshooting

### Connection String Not Working?
- Make sure it's the full PostgreSQL URI from Supabase, not just the URL
- In Supabase, go to Settings → Database → Connection Strings → URI
- Include the password in the URL

### "Failed to connect to database"
- Check that `DATABASE_URL` is set correctly
- Verify it's a PostgreSQL connection string starting with `postgresql://`
- Make sure your Supabase project is active

### Local dev works but Render fails?
- Check Render logs: click your Web Service → Logs
- Verify all environment variables are set in Render
- Make sure the Supabase project is still active

## Migrating from Old File-Based Storage

If you were using the old file-based storage:
1. The old `srs_cards.json` file is no longer used
2. All new progress is stored in PostgreSQL
3. Old progress won't be migrated (start fresh or manually copy)
