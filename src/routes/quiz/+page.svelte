<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo, categoryInfo } from '$lib/categories';
  import QuizOption from '$lib/components/QuizOption.svelte';
  import ResultBox from '$lib/components/ResultBox.svelte';

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
  let stageFilter = $state(0);
  let mode = $state('browse');
  let sessionWrong = $state([]);
  let sessionTotal = $state(0);
  let reviewWrong = $state(null);
  let reviewIdx = $state(0);
  let showingHint = $state(false);

  let categories = $derived(Object.keys(categoryInfo));
  let stages = $derived.by(() => {
    const s = new Set(quizzes.map(q => q.stage || 0).filter(v => v > 0));
    return [...s].sort((a, b) => a - b);
  });

  let filteredQuizzes = $derived(
    categoryFilter === 'all' && stageFilter <= 0
      ? quizzes
      : quizzes.filter(q =>
          (categoryFilter === 'all' || q.category === categoryFilter) &&
          (stageFilter <= 0 || (q.stage || 0) === stageFilter)
        )
  );

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
        {#if currentQuiz.stage}
          <span class="quiz-stage">Etap {currentQuiz.stage}</span>
        {/if}
        <span class="quiz-num">{isReview ? 'Powtórka' : 'Pytanie'} {currentQuiz.id}/{quizzes.length}</span>
      </div>
      <h2 class="quiz-question">{currentQuiz.question}</h2>
      <div class="quiz-options">
        {#each currentQuiz.options as opt, i}
          <QuizOption
            text={opt}
            index={i}
            correctIndex={currentQuiz.correct_index}
            selected={selectedAnswer}
            {showResult}
            onselect={(idx) => selectedAnswer = idx}
          />
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
        <ResultBox correct={isCorrect} {explanation} />
        <div class="review-nav">
          <button class="next-btn" onclick={nextReview}>➡️ Dalej ({reviewIdx + 1}/{reviewWrong.length})</button>
          <button class="skip-btn" onclick={skipReview}>⏭️ Zakończ powtórkę</button>
        </div>
      {:else}
        {#snippet hintContent()}
          <button class="hint-toggle" onclick={() => showingHint = !showingHint}>
            💡 {showingHint ? 'Ukryj podpowiedź' : 'Pokaż podpowiedź'}
          </button>
          {#if showingHint}
            <div class="hint-box">{currentQuiz.hint}</div>
          {/if}
        {/snippet}
        <ResultBox correct={isCorrect} {explanation} hint={currentQuiz.hint}>
          {hintContent}
        </ResultBox>
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
        <span class="qs-value">{quizResults.filter(r => !r.correct).length}</span>
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

      {#if stages.length > 0}
        <div class="stage-filters">
          <button class="stage-btn" class:active={stageFilter === 0} onclick={() => stageFilter = 0}>Wszystkie</button>
          {#each stages as s}
            <button class="stage-btn" class:active={stageFilter === s} onclick={() => stageFilter = s}>
              Etap {s}
            </button>
          {/each}
        </div>
      {/if}

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
              {#if q.stage}
                <span class="qi-stage">Etap {q.stage}</span>
              {/if}
              <span class="qi-question">{q.question}</span>
            </button>
          {/each}
        </div>
      {/if}
  {/if}
</div>

<style>
  .quiz-page { max-width: 700px; }
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

  .submit-btn, .next-btn { width: 100%; padding: 14px; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .submit-btn { background: #0ea5e9; color: #fff; }
  .submit-btn:hover:not(:disabled) { background: #0284c7; }
  .submit-btn:disabled { opacity: 0.5; cursor: default; }
  .next-btn { background: #1e293b; color: #38bdf8; border: 1px solid #334155; margin-top: 12px; }
  .next-btn:hover { background: #263548; }
  .skip-btn { background: none; border: 1px solid #475569; color: #94a3b8; padding: 8px 16px; border-radius: 8px; cursor: pointer; font-size: 13px; }
  .skip-btn:hover { color: #e2e8f0; border-color: #64748b; }

  .review-nav { display: flex; gap: 8px; }
  .review-nav .next-btn { margin-top: 0; flex: 1; }

  .stage-filters { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 12px; }
  .stage-btn { padding: 4px 12px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 6px; cursor: pointer; font-size: 11px; font-weight: 500; transition: all 0.15s; }
  .stage-btn:hover { background: #334155; color: #e2e8f0; }
  .stage-btn.active { background: #7c3aed; color: #fff; border-color: #7c3aed; }
  .qi-stage { font-size: 10px; font-weight: 600; background: #7c3aed20; color: #a78bfa; padding: 2px 6px; border-radius: 4px; flex-shrink: 0; }
  .quiz-stage { font-size: 10px; font-weight: 600; background: #7c3aed20; color: #a78bfa; padding: 3px 8px; border-radius: 4px; text-transform: uppercase; }
  .hint-toggle { display: block; margin-top: 8px; background: none; border: 1px dashed #7c3aed; color: #a78bfa; padding: 6px 12px; border-radius: 6px; cursor: pointer; font-size: 12px; transition: all 0.15s; }
  .hint-toggle:hover { background: #7c3aed20; }
  .hint-box { margin-top: 8px; padding: 10px 14px; background: #2d1b4e; border: 1px solid #7c3aed; border-radius: 8px; color: #d8b4fe; font-size: 13px; line-height: 1.5; }

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
