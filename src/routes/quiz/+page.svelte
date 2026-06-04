<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo, categoryInfo } from '$lib/categories.js';

  let quizzes = $state([]);
  let quizResults = $state([]);
  let loading = $state(true);
  let currentQuiz = $state(null);
  let selectedAnswer = $state(null);
  let showResult = $state(false);
  let isCorrect = $state(false);
  let explanation = $state('');
  let doneIds = $state(new Set());
  let seenIds = $state(new Set());
  let categoryFilter = $state('all');
  let mode = $state('browse');
  let sessionWrong = $state([]);
  let sessionTotal = $state(0);
  let reviewWrong = $state(null);
  let reviewIdx = $state(0);

  let categories = $derived(Object.keys(categoryInfo));

  let filteredQuizzes = $derived(
    categoryFilter === 'all'
      ? quizzes
      : quizzes.filter(q => q.category === categoryFilter)
  );

  let filteredDone = $derived(filteredQuizzes.filter(q => doneIds.has(q.id)));
  let filteredWrong = $derived(quizResults.filter(r => !r.correct && quizzes.some(q => q.id === r.quiz_id && (categoryFilter === 'all' || q.category === categoryFilter))));

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      quizzes = data.quizzes || [];
      quizResults = data.quiz_results || [];
      doneIds = new Set(quizResults.map(r => r.quiz_id));
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  function startQuiz(q) {
    currentQuiz = q;
    selectedAnswer = null;
    showResult = false;
    isCorrect = false;
    explanation = '';
  }

  async function submitAnswer() {
    if (selectedAnswer === null || !currentQuiz) return;
    const [, correct, expl] = await invoke('submit_quiz', {
      id: currentQuiz.id,
      answer: selectedAnswer,
    });
    isCorrect = correct;
    explanation = expl;
    showResult = true;
    seenIds.add(currentQuiz.id);
    sessionTotal++;
    if (correct) {
      doneIds.add(currentQuiz.id);
    } else {
      sessionWrong.push(currentQuiz.id);
      const existing = quizResults.find(r => r.quiz_id === currentQuiz.id);
      if (existing) existing.correct = false;
      else quizResults.push({ quiz_id: currentQuiz.id, correct: false });
    }
  }

  function nextQuiz() {
    const remaining = filteredQuizzes.filter(q => !seenIds.has(q.id));
    if (remaining.length > 0) {
      startQuiz(remaining[0]);
    } else {
      if (sessionWrong.length > 0) {
        reviewWrong = [...sessionWrong];
        reviewIdx = 0;
        reviewQuiz(reviewWrong[0]);
      } else {
        currentQuiz = null;
      }
    }
  }

  function reviewQuiz(id) {
    const q = quizzes.find(x => x.id === id);
    if (q) {
      currentQuiz = q;
      selectedAnswer = null;
      showResult = false;
      isCorrect = false;
      explanation = '';
    }
  }

  function nextReview() {
    reviewIdx++;
    if (reviewIdx < reviewWrong.length) {
      reviewQuiz(reviewWrong[reviewIdx]);
    } else {
      reviewWrong = null;
      currentQuiz = null;
    }
  }

  function skipReview() {
    reviewWrong = null;
    currentQuiz = null;
  }

  function setCategory(cat) {
    categoryFilter = cat;
    currentQuiz = null;
    sessionWrong = [];
    sessionTotal = 0;
    seenIds = new Set();
    reviewWrong = null;
  }
</script>

<div class="quiz-page">
  <header class="page-header">
    <h1>🧠 Quiz</h1>
    <p>Testuj swoją wiedzę z Linuksa i administracji systemem</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else if currentQuiz}
    {@const cat = getCategoryInfo(currentQuiz.category)}
    {@const isReview = reviewWrong !== null}
    <div class="quiz-card">
      <div class="quiz-meta">
        <span class="quiz-category" style="background: {cat.color}20; color: {cat.color}">
          {cat.name}
        </span>
        <span class="quiz-num">{isReview ? 'Powtórka' : 'Pytanie'} {currentQuiz.id}/{quizzes.length}</span>
      </div>
      <h2 class="quiz-question">{currentQuiz.question}</h2>
      <div class="quiz-options">
        {#each currentQuiz.options as opt, i}
          <button
            class="quiz-option"
            class:selected={selectedAnswer === i}
            class:correct={showResult && i === currentQuiz.correct_index}
            class:wrong={showResult && selectedAnswer === i && i !== currentQuiz.correct_index}
            onclick={() => { if (!showResult) selectedAnswer = i; }}
            disabled={showResult}
          >
            <span class="opt-letter">{String.fromCharCode(65 + i)}</span>
            <span class="opt-text">{opt}</span>
            {#if showResult && i === currentQuiz.correct_index}
              <span class="opt-icon">✓</span>
            {:else if showResult && selectedAnswer === i && i !== currentQuiz.correct_index}
              <span class="opt-icon">✗</span>
            {/if}
          </button>
        {/each}
      </div>

      {#if !showResult}
        <button
          class="submit-btn"
          disabled={selectedAnswer === null}
          onclick={submitAnswer}
        >
          Sprawdź odpowiedź
        </button>
      {:else if isReview}
        <div class="result-box" class:correct={isCorrect} class:wrong={!isCorrect}>
          <strong>{isCorrect ? '✅ Poprawnie!' : '❌ Nadal błędnie'}</strong>
          <p>{explanation}</p>
        </div>
        <div class="review-nav">
          <button class="next-btn" onclick={nextReview}>➡️ Dalej ({reviewIdx + 1}/{reviewWrong.length})</button>
          <button class="skip-btn" onclick={skipReview}>⏭️ Zakończ powtórkę</button>
        </div>
      {:else}
        <div class="result-box" class:correct={isCorrect} class:wrong={!isCorrect}>
          <strong>{isCorrect ? '✅ Poprawna odpowiedź!' : '❌ Niepoprawna'}</strong>
          <p>{explanation}</p>
        </div>
        <button class="next-btn" onclick={nextQuiz}>
          {filteredQuizzes.filter(q => !seenIds.has(q.id)).length === 0 && sessionWrong.length === 0 ? '🏁 Wróć do listy' : '➡️ Następne pytanie'}
        </button>
      {/if}
    </div>
  {:else if reviewWrong !== null && sessionWrong.length > 0}
    <div class="session-summary">
      <h2>📊 Sesja zakończona</h2>
      <p>Przerobiłeś <strong>{sessionTotal}</strong> pytań. Powtórz błędne odpowiedzi:</p>
      <div class="summary-bar">
        <div class="sum-correct" style="width: {sessionTotal > 0 ? (sessionTotal - sessionWrong.length) / sessionTotal * 100 : 0}%"></div>
        <div class="sum-wrong" style="width: {sessionTotal > 0 ? sessionWrong.length / sessionTotal * 100 : 0}%"></div>
      </div>
      <div class="summary-stats">
        <span class="sum-stat ok">✅ {sessionTotal - sessionWrong.length} poprawnych</span>
        <span class="sum-stat no">❌ {sessionWrong.length} błędnych</span>
      </div>
      <div class="summary-actions">
        <button class="start-btn" onclick={() => nextQuiz()}>🔄 Powtórz błędne ({sessionWrong.length})</button>
        <button class="skip-btn" onclick={() => { reviewWrong = null; }}>🏁 Wróć do listy</button>
      </div>
    </div>
  {/if}
  {#if !currentQuiz}
    <div class="quiz-stats">
      <div class="quiz-stat-card">
        <span class="qs-value">{quizResults.filter(r => r.correct).length}/{quizzes.length}</span>
        <span class="qs-label">Poprawne odpowiedzi</span>
      </div>
      <div class="quiz-stat-card">
        <span class="qs-value">{quizzes.length - doneIds.size}</span>
        <span class="qs-label">Pozostało pytań</span>
      </div>
      <div class="quiz-stat-card">
        <span class="qs-value">{filteredWrong.length}</span>
        <span class="qs-label">Błędne odpowiedzi</span>
      </div>
    </div>

    <div class="cat-filters">
      <button class="cat-btn" class:active={categoryFilter === 'all'} onclick={() => setCategory('all')}>Wszystkie ({quizzes.length})</button>
      {#each categories as cat}
        {@const info = getCategoryInfo(cat)}
        {@const count = quizzes.filter(q => q.category === cat).length}
        <button class="cat-btn" style="--cat-color: {info.color}" class:active={categoryFilter === cat} onclick={() => setCategory(cat)}>
          {info.name} ({count})
        </button>
      {/each}
    </div>

    {#if filteredQuizzes.length === 0}
      <div class="empty">Brak pytań w tej kategorii</div>
    {:else}
      <div class="quiz-list">
        {#each filteredQuizzes as q (q.id)}
          {@const cat = getCategoryInfo(q.category)}
          {@const result = quizResults.find(r => r.quiz_id === q.id)}
          <button class="quiz-item" onclick={() => startQuiz(q)}>
            <span class="qi-status">
              {#if doneIds.has(q.id) && result?.correct}✅
              {:else if doneIds.has(q.id)}❌
              {:else}⬜
              {/if}
            </span>
            <span class="qi-category" style="color: {cat.color}">{cat.name}</span>
            <span class="qi-question">{q.question}</span>
          </button>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .quiz-page { max-width: 700px; }
  .page-header h1 { font-size: 28px; font-weight: 700; color: #f1f5f9; margin-bottom: 4px; }
  .page-header p { color: #64748b; margin-bottom: 20px; }
  .loading { color: #64748b; }

  .quiz-stats { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; margin-bottom: 20px; }
  .quiz-stat-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 16px; text-align: center; }
  .qs-value { display: block; font-size: 24px; font-weight: 700; color: #38bdf8; }
  .qs-label { font-size: 12px; color: #64748b; }

  .cat-filters { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 16px; }
  .cat-btn { padding: 6px 14px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 6px; cursor: pointer; font-size: 12px; font-weight: 500; transition: all 0.15s; }
  .cat-btn:hover { background: #334155; color: #e2e8f0; }
  .cat-btn.active { background: var(--cat-color); color: #fff; border-color: var(--cat-color); }

  .empty { color: #64748b; text-align: center; padding: 40px; }

  .quiz-list { display: flex; flex-direction: column; gap: 6px; }
  .quiz-item { display: flex; align-items: center; gap: 12px; width: 100%; padding: 12px 16px; background: #1e293b; border: 1px solid #334155; border-radius: 10px; color: #e2e8f0; cursor: pointer; text-align: left; transition: all 0.15s; font-size: 14px; }
  .quiz-item:hover { border-color: #475569; background: #263548; }
  .qi-status { font-size: 16px; width: 24px; text-align: center; }
  .qi-category { font-size: 10px; font-weight: 600; text-transform: uppercase; min-width: 70px; }
  .qi-question { color: #94a3b8; }

  .quiz-card { background: #1e293b; border: 1px solid #334155; border-radius: 14px; padding: 28px; }
  .quiz-meta { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
  .quiz-category { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }
  .quiz-num { font-size: 13px; color: #64748b; }
  .quiz-question { font-size: 20px; font-weight: 600; color: #f1f5f9; margin-bottom: 20px; line-height: 1.4; }
  .quiz-options { display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px; }
  .quiz-option { display: flex; align-items: center; gap: 12px; padding: 14px 16px; background: #0f172a; border: 1px solid #334155; border-radius: 10px; color: #e2e8f0; cursor: pointer; text-align: left; transition: all 0.15s; font-size: 14px; }
  .quiz-option:hover:not(:disabled) { border-color: #475569; background: #1a2332; }
  .quiz-option.selected { border-color: #0ea5e9; background: #0ea5e920; }
  .quiz-option.correct { border-color: #22c55e; background: #22c55e20; }
  .quiz-option.wrong { border-color: #ef4444; background: #ef444420; }
  .quiz-option:disabled { cursor: default; }
  .opt-letter { width: 28px; height: 28px; display: flex; align-items: center; justify-content: center; border-radius: 50%; background: #334155; font-weight: 700; font-size: 13px; flex-shrink: 0; }
  .quiz-option.correct .opt-letter { background: #22c55e; color: #fff; }
  .quiz-option.wrong .opt-letter { background: #ef4444; color: #fff; }
  .opt-text { flex: 1; }
  .opt-icon { font-weight: 700; font-size: 16px; }

  .submit-btn, .next-btn { width: 100%; padding: 14px; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .submit-btn { background: #0ea5e9; color: #fff; }
  .submit-btn:hover:not(:disabled) { background: #0284c7; }
  .submit-btn:disabled { opacity: 0.5; cursor: default; }
  .next-btn { background: #1e293b; color: #38bdf8; border: 1px solid #334155; margin-top: 12px; }
  .next-btn:hover { background: #263548; }
  .skip-btn { background: none; border: 1px solid #475569; color: #94a3b8; padding: 8px 16px; border-radius: 8px; cursor: pointer; font-size: 13px; }
  .skip-btn:hover { color: #e2e8f0; border-color: #64748b; }

  .result-box { padding: 16px; border-radius: 10px; margin-bottom: 12px; font-size: 14px; line-height: 1.6; }
  .result-box.correct { background: #22c55e20; border: 1px solid #22c55e; color: #bbf7d0; }
  .result-box.wrong { background: #ef444420; border: 1px solid #ef4444; color: #fecaca; }
  .result-box strong { display: block; margin-bottom: 8px; font-size: 16px; }

  .review-nav { display: flex; gap: 8px; }
  .review-nav .next-btn { margin-top: 0; flex: 1; }

  .session-summary { background: #1e293b; border: 1px solid #334155; border-radius: 14px; padding: 32px; text-align: center; }
  .session-summary h2 { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .session-summary p { color: #94a3b8; margin-bottom: 20px; font-size: 14px; }
  .summary-bar { display: flex; height: 12px; border-radius: 6px; overflow: hidden; margin-bottom: 12px; }
  .sum-correct { background: #22c55e; transition: width 0.3s; }
  .sum-wrong { background: #ef4444; transition: width 0.3s; }
  .summary-stats { display: flex; justify-content: center; gap: 24px; margin-bottom: 24px; }
  .sum-stat { font-size: 14px; font-weight: 600; }
  .sum-stat.ok { color: #22c55e; }
  .sum-stat.no { color: #ef4444; }
  .summary-actions { display: flex; gap: 12px; justify-content: center; }
  .start-btn { padding: 12px 24px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-weight: 600; cursor: pointer; }
  .start-btn:hover { background: #0284c7; }
</style>
