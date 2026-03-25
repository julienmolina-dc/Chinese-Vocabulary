const API = '/api';

// ---- State ----
let allWords = [];
let currentLevel = 'all';
let studyMode = 'mixed';
const ALL_MODES = ['en_to_cn', 'en_to_pinyin', 'cn_to_en', 'cn_to_pinyin'];
let cardMode = 'en_to_cn'; // per-card mode when mixed
let reviewQueue = [];
let reviewIndex = 0;
let cardFlipped = false;

// Quiz state
let quizItems = [];
let quizIndex = 0;
let quizScore = 0;
let quizWrong = [];

// ---- Init ----
document.addEventListener('DOMContentLoaded', async () => {
  await loadWords();
  await loadStories();
  updateDashboard();
  setupNav();
  setupStudy();
  setupQuiz();
  setupBrowse();
});

async function loadWords() {
  try {
    const res = await fetch(`${API}/words`);
    allWords = await res.json();
  } catch {
    // Fallback: load from embedded data if backend is down
    console.warn('Backend not available, using empty dataset');
    allWords = [];
  }
}

// ---- Navigation ----
function setupNav() {
  document.querySelectorAll('.nav-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      document.querySelectorAll('.view').forEach(v => v.classList.remove('active'));
      document.getElementById(btn.dataset.view).classList.add('active');
    });
  });

  document.querySelectorAll('.filter-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      currentLevel = btn.dataset.level;
      updateDashboard();
    });
  });
}

// ---- Dashboard ----
async function updateDashboard() {
  try {
    const res = await fetch(`${API}/stats`);
    const stats = await res.json();
    document.getElementById('total-words').textContent = stats.total;
    document.getElementById('mastered-words').textContent = stats.mastered;
    document.getElementById('learning-words').textContent = stats.learning;
    document.getElementById('due-words').textContent = stats.due;

    const pct = stats.total > 0 ? Math.round((stats.mastered / stats.total) * 100) : 0;
    document.getElementById('progress-bar').style.width = pct + '%';
    document.getElementById('progress-text').textContent = pct + '% mastered';
  } catch {
    // Use local data
    const filtered = getFilteredWords();
    document.getElementById('total-words').textContent = filtered.length;
    document.getElementById('mastered-words').textContent = 0;
    document.getElementById('learning-words').textContent = 0;
    document.getElementById('due-words').textContent = filtered.length;
  }

  document.getElementById('start-review').onclick = () => {
    document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));
    document.querySelector('[data-view="study"]').classList.add('active');
    document.querySelectorAll('.view').forEach(v => v.classList.remove('active'));
    document.getElementById('study').classList.add('active');
    startReview();
  };
}

function getFilteredWords() {
  if (currentLevel === 'all') return allWords;
  return allWords.filter(w => w.level === parseInt(currentLevel));
}

// ---- Study / Flashcards with SRS ----
function setupStudy() {
  document.querySelectorAll('.mode-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.mode-btn').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      studyMode = btn.dataset.mode;
    });
  });

  document.getElementById('flashcard').addEventListener('click', flipCard);

  document.querySelectorAll('.rating-btn').forEach(btn => {
    btn.addEventListener('click', () => rateCard(parseInt(btn.dataset.rating)));
  });
}

async function startReview() {
  try {
    const levelParam = currentLevel !== 'all' ? `&level=${currentLevel}` : '';
    const res = await fetch(`${API}/review?limit=20${levelParam}`);
    reviewQueue = await res.json();
  } catch {
    reviewQueue = [...getFilteredWords()];
  }

  // Always shuffle so we don't learn in order
  reviewQueue = shuffle(reviewQueue);
  if (reviewQueue.length > 20) reviewQueue = reviewQueue.slice(0, 20);

  reviewIndex = 0;
  if (reviewQueue.length === 0) {
    document.getElementById('card-prompt').textContent = 'No cards due for review! 🎉';
    document.getElementById('card-context').textContent = '';
    document.getElementById('card-actions').classList.add('hidden');
    document.getElementById('card-counter').textContent = '';
    return;
  }
  showCard();
}

function showCard() {
  if (reviewIndex >= reviewQueue.length) {
    document.getElementById('card-prompt').textContent = 'Review complete! 🎉';
    document.getElementById('card-context').textContent = '';
    document.getElementById('card-back').classList.add('hidden');
    document.getElementById('card-front').classList.remove('hidden');
    document.getElementById('card-actions').classList.add('hidden');
    document.getElementById('card-counter').textContent = '';
    updateDashboard();
    return;
  }

  const word = reviewQueue[reviewIndex];
  cardFlipped = false;

  // Pick direction: random if mixed, otherwise use selected mode
  cardMode = studyMode === 'mixed'
    ? ALL_MODES[Math.floor(Math.random() * ALL_MODES.length)]
    : studyMode;

  // Show front
  document.getElementById('card-front').classList.remove('hidden');
  document.getElementById('card-back').classList.add('hidden');
  document.getElementById('card-actions').classList.add('hidden');

  const prompt = getPrompt(word, cardMode);
  const context = getContext(word, cardMode);
  document.getElementById('card-prompt').textContent = prompt;
  document.getElementById('card-context').textContent = context;
  document.getElementById('card-counter').textContent =
    `${reviewIndex + 1} / ${reviewQueue.length}`;
}

function getPrompt(word, mode) {
  switch (mode) {
    case 'en_to_cn': return word.english;
    case 'en_to_pinyin': return word.english;
    case 'cn_to_en': return word.hanzi;
    case 'cn_to_pinyin': return word.hanzi;
    default: return word.english;
  }
}

function getContext(word, mode) {
  switch (mode) {
    case 'en_to_cn': return word.sentence_en;
    case 'en_to_pinyin': return word.sentence_en;
    case 'cn_to_en': return word.sentence_cn;
    case 'cn_to_pinyin': return word.sentence_cn;
    default: return '';
  }
}

function flipCard() {
  if (cardFlipped || reviewIndex >= reviewQueue.length) return;
  cardFlipped = true;

  const word = reviewQueue[reviewIndex];
  document.getElementById('card-front').classList.add('hidden');
  document.getElementById('card-back').classList.remove('hidden');
  document.getElementById('card-actions').classList.remove('hidden');

  let answer, extra;
  switch (cardMode) {
    case 'en_to_cn':
      answer = word.hanzi;
      extra = word.pinyin;
      break;
    case 'en_to_pinyin':
      answer = word.pinyin;
      extra = word.hanzi;
      break;
    case 'cn_to_en':
      answer = word.english;
      extra = word.pinyin;
      break;
    case 'cn_to_pinyin':
      answer = word.pinyin;
      extra = word.english;
      break;
    default:
      answer = word.hanzi;
      extra = word.pinyin;
  }

  document.getElementById('card-answer').textContent = answer;
  document.getElementById('card-extra').textContent = extra;
  document.getElementById('card-sentence').textContent =
    `${word.sentence_cn}\n${word.sentence_pinyin}\n${word.sentence_en}`;
}

async function rateCard(rating) {
  const word = reviewQueue[reviewIndex];
  try {
    await fetch(`${API}/review`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ word_id: word.id, rating }),
    });
  } catch {
    // offline mode - just continue
  }
  reviewIndex++;
  showCard();
}

// ---- Quiz ----
function setupQuiz() {
  document.getElementById('start-quiz').addEventListener('click', startQuiz);
  document.getElementById('quiz-next').addEventListener('click', nextQuizQuestion);
  document.getElementById('quiz-restart').addEventListener('click', () => {
    document.getElementById('quiz-results').classList.add('hidden');
    document.getElementById('quiz-setup').classList.remove('hidden');
  });
}

async function startQuiz() {
  const mode = document.getElementById('quiz-mode').value;
  const count = parseInt(document.getElementById('quiz-count').value);

  try {
    const levelParam = currentLevel !== 'all' ? `&level=${currentLevel}` : '';
    const res = await fetch(`${API}/quiz?mode=${mode}&count=${count}${levelParam}`);
    quizItems = await res.json();
  } catch {
    // Generate quiz locally
    quizItems = generateLocalQuiz(mode, count);
  }

  quizIndex = 0;
  quizScore = 0;
  quizWrong = [];

  document.getElementById('quiz-setup').classList.add('hidden');
  document.getElementById('quiz-active').classList.remove('hidden');
  document.getElementById('quiz-results').classList.add('hidden');
  showQuizQuestion();
}

function generateLocalQuiz(mode, count) {
  const filtered = getFilteredWords();
  const selected = shuffle([...filtered]).slice(0, count);

  return selected.map(word => {
    const correctAnswer = getAnswerForMode(word, mode);
    const wrong = shuffle(filtered.filter(w => w.id !== word.id))
      .slice(0, 3)
      .map(w => getAnswerForMode(w, mode));

    const correctIndex = Math.floor(Math.random() * 4);
    const choices = [...wrong];
    choices.splice(correctIndex, 0, correctAnswer);

    return { word, choices, correct_index: correctIndex };
  });
}

function getAnswerForMode(word, mode) {
  switch (mode) {
    case 'en_to_cn': return word.hanzi;
    case 'en_to_pinyin': return word.pinyin;
    case 'cn_to_en': return word.english;
    case 'cn_to_pinyin': return word.pinyin;
    default: return word.hanzi;
  }
}

function showQuizQuestion() {
  if (quizIndex >= quizItems.length) {
    showQuizResults();
    return;
  }

  const item = quizItems[quizIndex];
  const mode = document.getElementById('quiz-mode').value;

  document.getElementById('quiz-progress-text').textContent =
    `${quizIndex + 1} / ${quizItems.length}`;
  document.getElementById('quiz-score-text').textContent = `Score: ${quizScore}`;

  // Show prompt
  let prompt, contextSentence;
  switch (mode) {
    case 'en_to_cn':
    case 'en_to_pinyin':
      prompt = item.word.english;
      contextSentence = item.word.sentence_en;
      break;
    case 'cn_to_en':
    case 'cn_to_pinyin':
      prompt = item.word.hanzi;
      contextSentence = item.word.sentence_cn;
      break;
    default:
      prompt = item.word.english;
      contextSentence = item.word.sentence_en;
  }

  document.getElementById('quiz-prompt').textContent = prompt;
  document.getElementById('quiz-context-sentence').textContent = contextSentence;

  // Show choices
  const choicesEl = document.getElementById('quiz-choices');
  choicesEl.innerHTML = '';
  item.choices.forEach((choice, i) => {
    const btn = document.createElement('button');
    btn.className = 'quiz-choice';
    btn.textContent = choice;
    btn.addEventListener('click', () => selectQuizAnswer(i));
    choicesEl.appendChild(btn);
  });

  document.getElementById('quiz-feedback').classList.add('hidden');
  document.getElementById('quiz-next').classList.add('hidden');
}

function selectQuizAnswer(index) {
  const item = quizItems[quizIndex];
  const buttons = document.querySelectorAll('.quiz-choice');
  const feedback = document.getElementById('quiz-feedback');

  buttons.forEach(b => b.classList.add('disabled'));
  buttons[item.correct_index].classList.add('correct');

  if (index === item.correct_index) {
    quizScore++;
    feedback.textContent = '✓ Correct!';
    feedback.style.color = '#2ecc71';
  } else {
    buttons[index].classList.add('wrong');
    feedback.textContent = `✗ The answer was: ${item.choices[item.correct_index]}`;
    feedback.style.color = '#e74c3c';
    quizWrong.push({
      prompt: document.getElementById('quiz-prompt').textContent,
      answer: item.choices[item.correct_index],
    });
  }

  feedback.classList.remove('hidden');
  document.getElementById('quiz-next').classList.remove('hidden');
  document.getElementById('quiz-score-text').textContent = `Score: ${quizScore}`;
}

function nextQuizQuestion() {
  quizIndex++;
  showQuizQuestion();
}

function showQuizResults() {
  document.getElementById('quiz-active').classList.add('hidden');
  document.getElementById('quiz-results').classList.remove('hidden');

  const pct = Math.round((quizScore / quizItems.length) * 100);
  document.getElementById('quiz-final-score').textContent =
    `${quizScore} / ${quizItems.length} (${pct}%)`;

  const wrongList = document.getElementById('quiz-wrong-list');
  wrongList.innerHTML = '';
  if (quizWrong.length > 0) {
    const h4 = document.createElement('h4');
    h4.textContent = 'Review these:';
    h4.style.color = '#888';
    h4.style.marginBottom = '10px';
    wrongList.appendChild(h4);

    quizWrong.forEach(item => {
      const div = document.createElement('div');
      div.className = 'wrong-item';
      div.innerHTML = `<span class="wrong-q">${item.prompt}</span>
                        <span class="wrong-a">${item.answer}</span>`;
      wrongList.appendChild(div);
    });
  }
}

// ---- Browse ----
function setupBrowse() {
  renderWordList(allWords);

  document.getElementById('search-input').addEventListener('input', filterBrowse);
  document.getElementById('browse-level').addEventListener('change', filterBrowse);
}

function filterBrowse() {
  const search = document.getElementById('search-input').value.toLowerCase();
  const level = document.getElementById('browse-level').value;

  let filtered = allWords;
  if (level !== 'all') {
    filtered = filtered.filter(w => w.level === parseInt(level));
  }
  if (search) {
    filtered = filtered.filter(w =>
      w.hanzi.includes(search) ||
      w.pinyin.toLowerCase().includes(search) ||
      w.english.toLowerCase().includes(search)
    );
  }
  renderWordList(filtered);
}

function renderWordList(words) {
  const list = document.getElementById('word-list');
  list.innerHTML = '';
  words.forEach(w => {
    const div = document.createElement('div');
    div.className = 'word-item';
    div.innerHTML = `
      <span class="word-hanzi">${w.hanzi}</span>
      <span class="word-pinyin">${w.pinyin}</span>
      <span class="word-english">${w.english}</span>
      <span class="word-level">${w.level === 3 ? 'Class' : 'HSK ' + w.level}</span>
    `;
    div.title = `${w.sentence_cn}\n${w.sentence_pinyin}\n${w.sentence_en}`;
    list.appendChild(div);
  });
}

// ---- Utility ----
function shuffle(arr) {
  for (let i = arr.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [arr[i], arr[j]] = [arr[j], arr[i]];
  }
  return arr;
}

// ---- Stories ----
let storiesData = [];

async function loadStories() {
  try {
    const res = await fetch(`${API}/stories`);
    storiesData = await res.json();
    console.log('Stories loaded:', storiesData.length);
  } catch (e) {
    console.error('Failed to load stories:', e);
    storiesData = [];
  }
  renderStoryList();
}

function renderStoryList() {
  const list = document.getElementById('story-list');
  list.innerHTML = '';
  if (storiesData.length === 0) {
    list.innerHTML = '<p style="text-align:center;color:#888;">No stories available. Make sure the server is running.</p>';
    return;
  }
  storiesData.forEach(s => {
    const card = document.createElement('div');
    card.className = 'story-card';
    card.innerHTML = `
      <div class="story-card-title">${s.title_cn} — ${s.title}</div>
      <div class="story-card-subtitle">${s.description}</div>
    `;
    card.addEventListener('click', () => openStory(s.id));
    list.appendChild(card);
  });
}

async function openStory(id) {
  try {
    const res = await fetch(`${API}/stories/${id}`);
    const story = await res.json();
    renderStoryReader(story);
  } catch {
    console.error('Failed to load story');
  }
}

function renderStoryReader(story) {
  document.getElementById('story-list').classList.add('hidden');
  const reader = document.getElementById('story-reader');
  reader.classList.remove('hidden');

  document.getElementById('story-title').textContent =
    `${story.title_cn} — ${story.title}`;

  // Extra vocab
  const extraSection = document.getElementById('story-extra-vocab');
  const extraWords = document.getElementById('story-extra-words');
  if (story.extra_vocab && story.extra_vocab.length > 0) {
    extraSection.classList.remove('hidden');
    extraWords.innerHTML = '';
    story.extra_vocab.forEach(w => {
      const span = document.createElement('span');
      span.className = 'extra-word';
      span.innerHTML = `<span class="ew-hanzi">${w.hanzi}</span><span class="ew-pinyin">${w.pinyin}</span><span class="ew-en">${w.english}</span>`;
      extraWords.appendChild(span);
    });
  } else {
    extraSection.classList.add('hidden');
  }

  // Render paragraphs with annotated characters, grouping multi-char words
  const textEl = document.getElementById('story-text');
  textEl.innerHTML = '';

  story.paragraphs.forEach(para => {
    const p = document.createElement('p');
    p.className = 'story-paragraph';

    // Group consecutive chars that share the same english meaning into words
    const groups = [];
    let i = 0;
    while (i < para.chars.length) {
      const c = para.chars[i];
      const group = { chars: [c] };
      // Look ahead: merge consecutive chars with same english
      while (i + 1 < para.chars.length
        && para.chars[i + 1].english === c.english
        && c.english !== ',' && c.english !== '.' && c.english !== '!'
        && c.english !== '?' && c.english !== ':' && c.english !== '"'
        && c.english !== 'open quote' && c.english !== 'close quote'
        && c.english !== 'quote') {
        i++;
        group.chars.push(para.chars[i]);
      }
      groups.push(group);
      i++;
    }

    groups.forEach(group => {
      const wordText = group.chars.map(c => c.ch).join('');
      const wordPinyin = group.chars.map(c => c.pinyin).join('');
      const wordEnglish = group.chars[0].english;

      const span = document.createElement('span');
      span.className = 'story-char';
      span.textContent = wordText;

      // Hover tooltip: pinyin of the full word
      const tooltip = document.createElement('span');
      tooltip.className = 'char-tooltip';
      tooltip.innerHTML = `<span class="tt-pinyin">${wordPinyin}</span>`;
      span.appendChild(tooltip);

      // Click: popup with full word pinyin + english
      span.addEventListener('click', (e) => {
        e.stopPropagation();
        showCharPopup(wordText, wordPinyin, wordEnglish);
      });

      p.appendChild(span);
    });

    textEl.appendChild(p);
  });

  // Back button
  document.getElementById('story-back').onclick = () => {
    reader.classList.add('hidden');
    document.getElementById('story-list').classList.remove('hidden');
  };
}

function showCharPopup(ch, pinyin, english) {
  // Remove existing
  closeCharPopup();

  const overlay = document.createElement('div');
  overlay.className = 'popup-overlay';
  overlay.addEventListener('click', closeCharPopup);
  document.body.appendChild(overlay);

  const popup = document.getElementById('char-popup');
  document.getElementById('popup-pinyin').textContent = `${ch}  ${pinyin}`;
  document.getElementById('popup-english').textContent = english;
  popup.classList.remove('hidden');
}

function closeCharPopup() {
  document.getElementById('char-popup').classList.add('hidden');
  const overlay = document.querySelector('.popup-overlay');
  if (overlay) overlay.remove();
}
