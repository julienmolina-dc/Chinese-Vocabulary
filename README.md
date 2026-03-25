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

## Deploy to Render

1. Create a new Web Service on Render
2. Connect your GitHub repo
3. Set build command: `cargo build --release`
4. Set start command: `./target/release/julien-chinese-vocab`
5. Add environment variable: `HSK_PASSWORD` with your desired password
6. Deploy

Your app will be at the URL provided by Render

## Features

- Dashboard with progress tracking
- SRS flashcards (Mixed / EN→汉字 / EN→Pinyin / 汉字→EN / 汉字→Pinyin)
- Multiple-choice quiz
- Browsable word list (HSK 1 / HSK 2 / Class vocabulary)
- 5 annotated Chinese stories (hover for pinyin, click for translation)
- Password-protected access
- Progress persists to local file (note: data may be lost on redeploys if not using persistent storage)
