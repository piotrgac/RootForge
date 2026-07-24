<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';
  import QuizOption from '$lib/components/QuizOption.svelte';
  import ResultBox from '$lib/components/ResultBox.svelte';

  let quizzes = $state([]);
  let quizResults = $state([]);
  let wrongAnswers = $state([]);
  let loading = $state(true);
  let currentIndex = $state(0);
  let selectedAnswer = $state(null);
  let showResult = $state(false);
  let isCorrect = $state(false);
  let filterMode = $state('due');
  let refreshKey = $state(0);

  let today = $derived(new Date().toISOString().slice(0, 10));

  let dueWrongIds = $derived.by(() => {
    if (filterMode === 'all') {
      return new Set(wrongAnswers.map(w => w.quiz_id));
    }
    return new Set(wrongAnswers.filter(w => w.next_review <= today).map(w => w.quiz_id));
  });

  let wrongQuizzes = $derived(quizzes.filter(q => dueWrongIds.has(q.id)));
  let currentQuiz = $derived(wrongQuizzes[currentIndex] || null);

  let filteredWrong = $derived(quizResults.filter(r => !r.correct));

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      quizzes = data.quizzes || [];
      quizResults = data.quiz_results || [];
      wrongAnswers = (await invoke('get_review_questions')) || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function refreshWrongAnswers() {
    wrongAnswers = (await invoke('get_review_questions')) || [];
  }

  async function submitAnswer() {
    if (selectedAnswer === null || !currentQuiz) return;
    const [, correct] = await invoke('submit_quiz', { id: currentQuiz.id, answer: selectedAnswer });
    isCorrect = correct;
    showResult = true;
    if (correct) {
      await invoke('mark_quiz_correct_in_review', { id: currentQuiz.id });
      await refreshWrongAnswers();
      const r = quizResults.find(r => r.quiz_id === currentQuiz.id);
      if (r) r.correct = true;
    }
  }

  function next() {
    if (isCorrect) {
      currentIndex = 0;
      refreshKey++;
    } else {
      if (currentIndex < wrongQuizzes.length - 1) {
        currentIndex++;
      }
    }
    selectedAnswer = null;
    showResult = false;
    isCorrect = false;
  }
</script>

<div class="review-page">
  <header class="page-header">
    <h1>🔄 Powtórki</h1>
    <p>Algorytm spaced repetition – pokazuje pytania wg odstępów 0/1/3/7/14 dni od ostatniej pomyłki</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="filter-bar">
      <button class="filter-btn" class:active={filterMode === 'due'} onclick={() => { filterMode = 'due'; currentIndex = 0; }}>
        📅 Zaległe ({wrongAnswers.filter(w => w.next_review <= today).length})
      </button>
      <button class="filter-btn" class:active={filterMode === 'all'} onclick={() => { filterMode = 'all'; currentIndex = 0; }}>
        📋 Wszystkie błędne ({wrongAnswers.length})
      </button>
    </div>

    {#if wrongQuizzes.length === 0}
      <div class="empty-state">
        <div class="empty-icon">🎉</div>
        <h2>Brak powtórek na dziś!</h2>
        <p>{filterMode === 'due' ? 'Wszystkie pytania są opanowane. Sprawdź zakładkę "Wszystkie błędne" lub wróć do quizu.' : 'Nie ma żadnych błędnych odpowiedzi.'}</p>
        <a href="/quiz" class="btn-link">➡️ Wróć do quizu</a>
      </div>
    {:else if currentQuiz}
      {@const cat = getCategoryInfo(currentQuiz.category)}
      {@const wa = wrongAnswers.find(w => w.quiz_id === currentQuiz.id)}
      <div class="review-header">
        <span class="review-count">Pytanie {currentIndex + 1} z {wrongQuizzes.length}</span>
        <span class="review-category" style="background: {cat.color}20; color: {cat.color}">
          {cat.name}
        </span>
      </div>
      {#if wa}
        <div class="review-meta">
          <span class="rm-item">❌ Błędów: {wa.wrong_count}</span>
          <span class="rm-item">📅 Ostatnio: {wa.last_wrong}</span>
          <span class="rm-item">⏭️ Kolejna: {wa.next_review}</span>
        </div>
      {/if}

      <div class="quiz-card">
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
          <button class="submit-btn" disabled={selectedAnswer === null} onclick={submitAnswer}>
            Sprawdź odpowiedź
          </button>
        {:else}
          <ResultBox correct={isCorrect} explanation={currentQuiz.explanation} />
          <button class="next-btn" onclick={next}>
            {isCorrect
              ? wrongQuizzes.filter((_, i) => i !== currentIndex).length === 0
                ? '🎉 Wszystkie poprawione!'
                : '➡️ Następne pytanie'
              : currentIndex < wrongQuizzes.length - 1
                ? '⏭️ Pomiń'
                : '🏁 Koniec'}
          </button>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .review-page { max-width: 700px; }
  .review-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
  .review-count { font-size: 13px; color: #94a3b8; }
  .review-category { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }

  .review-meta { display: flex; gap: 16px; margin-bottom: 16px; }
  .rm-item { font-size: 12px; color: #64748b; background: #1e293b; padding: 4px 10px; border-radius: 6px; border: 1px solid #334155; }

  .empty-state { text-align: center; padding: 60px 20px; }
  .empty-icon { font-size: 64px; margin-bottom: 16px; }
  .empty-state h2 { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .empty-state p { color: #64748b; margin-bottom: 20px; }
  .btn-link { display: inline-block; padding: 12px 24px; background: #0ea5e9; color: #fff; border-radius: 8px; text-decoration: none; font-weight: 600; }
  .btn-link:hover { background: #0284c7; }

  .quiz-card { background: #1e293b; border: 1px solid #334155; border-radius: 14px; padding: 28px; }
  .quiz-question { font-size: 20px; font-weight: 600; color: #f1f5f9; margin-bottom: 20px; line-height: 1.4; }
  .quiz-options { display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px; }

  .submit-btn, .next-btn { width: 100%; padding: 14px; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .submit-btn { background: #0ea5e9; color: #fff; }
  .submit-btn:hover:not(:disabled) { background: #0284c7; }
  .submit-btn:disabled { opacity: 0.5; cursor: default; }
  .next-btn { background: #1e293b; color: #38bdf8; border: 1px solid #334155; margin-top: 12px; }
  .next-btn:hover { background: #263548; }
</style>
