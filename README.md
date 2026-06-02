# 中文课 Flashcards

A modern spaced-repetition flashcard app for Chinese vocabulary and grammar, built as a single-page app deployable on Netlify (free tier).

## Features

- **622 vocabulary cards** + **32 grammar patterns** from 28 class sessions (Aug 2025 to Jun 2026)
- **3 quiz modes**: From Character, From Pinyin, From English/French + Mixed mode
- **Spaced repetition** (SM-2 algorithm): cards are scheduled based on your performance
- **Cross-device sync** via Netlify Blobs (no database, no account needed)
- **Filter by course** or card type (vocab/grammar)
- **Browse view** with search, SRS status, and next review dates
- **Keyboard shortcuts**: Space/Enter to flip, 1-4 to rate
- **Export/Import** progress as JSON for backup
- **Works offline**: falls back to localStorage when no network

## How Sync Works

Instead of user accounts, you choose a **passphrase** (any string, min. 3 characters). This passphrase is your unique key in the cloud store.

- Use the **same passphrase** on all your devices (laptop, phone, tablet)
- Progress syncs automatically after each card review (3-second debounce)
- If two devices conflict, the one with more reviews wins
- No signup, no email, no password reset. Just your secret phrase.

**Example passphrases**: `julien-chinese`, `moncours2025`, `zhongwen42`

## Project Structure

```
chinese_flashcards_app/
├── index.html                    # Single-page app (all UI + embedded data)
├── netlify.toml                  # Netlify configuration
├── package.json                  # Dependencies (@netlify/blobs)
├── README.md                     # This file
└── netlify/functions/
    ├── get-progress.mjs          # GET: load progress from cloud
    └── save-progress.mjs         # POST: save progress to cloud
```

## Deploy to Netlify

### Option A: Git deploy (recommended)

1. Push this folder to a GitHub/GitLab repo
2. Go to [app.netlify.com](https://app.netlify.com) and click "Add new site" > "Import an existing project"
3. Connect your repo
4. Build settings:
   - **Build command**: (leave empty)
   - **Publish directory**: `.`
5. Click "Deploy site"
6. Done! Your app is live at `https://your-site-name.netlify.app`

### Option B: Drag and drop

1. Run `npm install` locally (to get node_modules for the functions)
2. Zip the entire folder
3. Go to [app.netlify.com/drop](https://app.netlify.com/drop)
4. Drag the zip file

### Option C: Netlify CLI

```bash
npm install
npx netlify-cli deploy --prod
```

## Local Development

The app works locally without Netlify Functions (uses localStorage only):

```bash
# Just open index.html in a browser
open index.html
```

For full sync testing with functions:

```bash
npm install
npx netlify dev
```

## Free Tier Limits

This app fits comfortably within Netlify's free tier:
- **Functions**: 125,000 invocations/month (each card review = 1 call, debounced)
- **Blobs storage**: Included, no extra cost
- **Bandwidth**: 100GB/month (the app is ~100KB)

## Tech Stack

- Pure HTML/CSS/JavaScript (no framework, no build step)
- Font Awesome 6 for icons
- Netlify Functions (serverless, Node.js)
- Netlify Blobs (key-value store)

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space / Enter | Flip card |
| 1 | Again (forgot) |
| 2 | Hard |
| 3 | Good |
| 4 | Easy |

## Data Source

Vocabulary and grammar extracted from 28 Chinese class sessions (New Practical Chinese Reader / NPCR, HSK 1-2 level), covering:
- Greetings, introductions, numbers
- Dates, time, daily activities
- Food, restaurant, shopping
- Weather, seasons, travel
- Directions, professions, sports
- Comparisons, duration, emphasis structures
