<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';

  onDestroy(() => {
    if (timerInterval) clearInterval(timerInterval);
  });
  import { getCategoryInfo } from '$lib/categories';
  import QuizOption from '$lib/components/QuizOption.svelte';
  import ResultBox from '$lib/components/ResultBox.svelte';

  let quizzes = $state([]);
  let examQuestions = $state([]);
  let currentIdx = $state(0);
  let selectedAnswer = $state(null);
  let showResult = $state(false);
  let isCorrect = $state(false);
  let explanation = $state('');
  let showingHint = $state(false);
  let correctCount = $state(0);
  let totalAnswered = $state(0);
  let phase = $state('start'); // start | exam | result
  let timeLeft = $state(30 * 60);
  let timerInterval = $state(null);
  let examAttempts = $state([]);
  let loading = $state(true);

  let currentQ = $derived(examQuestions[currentIdx] || null);
  let progressPct = $derived(totalAnswered > 0 ? (totalAnswered / 15) * 100 : 0);
  let minutes = $derived(Math.floor(timeLeft / 60));
  let seconds = $derived(timeLeft % 60);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      quizzes = data.quizzes || [];
      examAttempts = data.exam_attempts || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  function startExam() {
    const shuffled = [...quizzes].sort(() => Math.random() - 0.5);
    examQuestions = shuffled.slice(0, 15);
    currentIdx = 0;
    correctCount = 0;
    totalAnswered = 0;
    selectedAnswer = null;
    showResult = false;
    showingHint = false;
    phase = 'exam';
    timeLeft = 30 * 60;

    timerInterval = setInterval(() => {
      timeLeft--;
      if (timeLeft <= 0) {
        clearInterval(timerInterval);
        finishExam();
      }
    }, 1000);
  }

  async function submitAnswer() {
    if (selectedAnswer === null || !currentQ) return;
    const [, correct, expl] = await invoke('submit_quiz', { id: currentQ.id, answer: selectedAnswer });
    isCorrect = correct;
    explanation = expl;
    showResult = true;
    if (correct) correctCount++;
    totalAnswered++;
  }

  function nextQuestion() {
    if (currentIdx < 14) {
      currentIdx++;
      selectedAnswer = null;
      showResult = false;
      isCorrect = false;
      explanation = '';
      showingHint = false;
    } else {
      finishExam();
    }
  }

  async function finishExam() {
    if (timerInterval) clearInterval(timerInterval);
    phase = 'result';
    await invoke('finish_exam', { score: correctCount, total: totalAnswered });
    const data = await invoke('get_dashboard_stats');
    examAttempts = data.exam_attempts || [];
  }

  function formatTime(m, s) {
    return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  }

  $effect(() => {
    if (phase === 'exam' && timeLeft <= 0) {
      finishExam();
    }
  });

  let bestExam = $derived.by(() => {
    if (examAttempts.length === 0) return null;
    return examAttempts.reduce((best, a) => a.score > best.score ? a : best, examAttempts[0]);
  });
</script>

<div class="exam-page">
  <header class="page-header">
    <h1>📝 Egzamin RHCSA (mock)</h1>
    <p>15 losowych pytań · 30 minut · próg zaliczenia: 70%</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>

  {:else if phase === 'start'}
    <div class="start-card">
      <div class="start-icon">🎯</div>
      <h2>Gotowy do egzaminu?</h2>
      <ul class="rules">
        <li>📌 15 pytań z wszystkich kategorii</li>
        <li>⏱️ Limit czasu: <strong>30 minut</strong></li>
        <li>✅ Próg zaliczenia: <strong>70%</strong> (11/15)</li>
        <li>🏆 Zaliczenie daje <strong>+30 XP</strong></li>
        <li>📊 Po egzaminie zobaczysz historię prób</li>
      </ul>
      <button class="start-btn" onclick={startExam}>🚀 Rozpocznij egzamin</button>

      {#if examAttempts.length > 0}
        <div class="history">
          <h3>Historia egzaminów</h3>
          <div class="history-list">
            {#each [...examAttempts].reverse() as att}
              <div class="history-item" class:passed={att.passed} class:failed={!att.passed}>
                <span class="hi-date">{att.date}</span>
                <span class="hi-score" class:pass={att.passed} class:fail={!att.passed}>{att.score}/{att.total}</span>
                <span class="hi-badge">{att.passed ? '✅ ZALICZONY' : '❌ NIEZALICZONY'}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

  {:else if phase === 'exam' && currentQ}
    {@const cat = getCategoryInfo(currentQ.category)}
    <div class="exam-header">
      <div class="timer" class:urgent={timeLeft < 300}>
        ⏱️ {formatTime(minutes, seconds)}
      </div>
      <div class="exam-progress">
        <div class="progress-bar">
          <div class="progress-fill" style="width: {progressPct}%"></div>
        </div>
        <span class="progress-label">{totalAnswered}/15</span>
      </div>
      {#if currentQ.stage}
        <span class="exam-stage">Etap {currentQ.stage}</span>
      {/if}
      <div class="exam-category" style="background: {cat.color}20; color: {cat.color}">
        {cat.name}
      </div>
    </div>

    <div class="exam-card">
      <h2 class="exam-question">
        <span class="q-num">Pytanie {currentIdx + 1}.</span>
        {currentQ.question}
      </h2>
      <div class="quiz-options">
        {#each currentQ.options as opt, i}
          <QuizOption
            text={opt}
            index={i}
            correctIndex={currentQ.correct_index}
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
        {#snippet hintContent()}
          <button class="hint-toggle" onclick={() => showingHint = !showingHint}>
            💡 {showingHint ? 'Ukryj podpowiedź' : 'Pokaż podpowiedź'}
          </button>
          {#if showingHint}
            <div class="hint-box">{currentQ.hint}</div>
          {/if}
        {/snippet}
        <ResultBox correct={isCorrect} {explanation} hint={currentQ.hint}>
          {hintContent}
        </ResultBox>
        <button class="next-btn" onclick={nextQuestion}>
          {currentIdx < 14 ? '➡️ Dalej' : '🏁 Zakończ egzamin'}
        </button>
      {/if}
    </div>

  {:else if phase === 'result'}
    <div class="result-card">
      <div class="result-icon">{correctCount >= 11 ? '🎉' : '😞'}</div>
      <h2>{correctCount >= 11 ? 'Egzamin zaliczony!' : 'Egzamin niezaliczony'}</h2>
      <div class="result-score">
        <span class="rs-value">{correctCount}</span>
        <span class="rs-sep">/</span>
        <span class="rs-total">{totalAnswered}</span>
      </div>
      <div class="result-pct">{Math.round(correctCount / Math.max(totalAnswered, 1) * 100)}%</div>
      {#if correctCount >= 11}
        <p class="result-msg">Gratulacje! Zdobywasz +30 XP. Jesteś gotowy na RHCSA!</p>
      {:else}
        <p class="result-msg">Potrzebujesz {11 - correctCount} więcej poprawnych odpowiedzi. Spróbuj ponownie!</p>
      {/if}
      <div class="result-actions">
        <button class="start-btn" onclick={startExam}>🔄 Spróbuj ponownie</button>
        <a href="/quiz" class="btn-link">📚 Wróć do quizu</a>
      </div>
    </div>
  {/if}
</div>

<style>
  .exam-page { max-width: 700px; }
  .start-card { background: #1e293b; border: 1px solid #334155; border-radius: 16px; padding: 32px; text-align: center; }
  .start-icon { font-size: 64px; margin-bottom: 16px; }
  .start-card h2 { font-size: 24px; font-weight: 700; color: #f1f5f9; margin-bottom: 20px; }
  .rules { text-align: left; list-style: none; padding: 0; margin: 0 auto 24px; max-width: 340px; }
  .rules li { padding: 8px 0; color: #cbd5e1; font-size: 14px; border-bottom: 1px solid #334155; }
  .rules li:last-child { border-bottom: none; }

  .start-btn { padding: 14px 32px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-size: 16px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .start-btn:hover { background: #0284c7; }

  .history { margin-top: 28px; text-align: left; }
  .history h3 { font-size: 16px; font-weight: 600; color: #94a3b8; margin-bottom: 12px; }
  .history-list { display: flex; flex-direction: column; gap: 6px; }
  .history-item { display: flex; align-items: center; gap: 12px; padding: 10px 14px; border-radius: 8px; font-size: 13px; }
  .history-item.passed { background: #22c55e10; border: 1px solid #22c55e40; }
  .history-item.failed { background: #ef444410; border: 1px solid #ef444440; }
  .hi-date { color: #94a3b8; min-width: 90px; }
  .hi-score { font-weight: 700; font-size: 15px; min-width: 50px; }
  .hi-score.pass { color: #22c55e; }
  .hi-score.fail { color: #ef4444; }
  .hi-badge { font-weight: 600; font-size: 11px; }

  .exam-header { display: flex; align-items: center; gap: 16px; margin-bottom: 16px; }
  .timer { font-size: 22px; font-weight: 700; color: #38bdf8; font-variant-numeric: tabular-nums; min-width: 80px; }
  .timer.urgent { color: #ef4444; animation: pulse 1s infinite; }
  @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.6; } }
  .exam-progress { flex: 1; display: flex; align-items: center; gap: 10px; }
  .progress-bar { flex: 1; height: 8px; background: #1e293b; border-radius: 4px; overflow: hidden; }
  .progress-fill { height: 100%; background: #0ea5e9; border-radius: 4px; transition: width 0.3s; }
  .progress-label { font-size: 13px; color: #94a3b8; font-weight: 600; min-width: 30px; }
  .exam-category { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }
  .exam-stage { font-size: 10px; font-weight: 600; background: #7c3aed20; color: #a78bfa; padding: 3px 8px; border-radius: 4px; text-transform: uppercase; }
  .hint-toggle { display: block; margin-top: 8px; background: none; border: 1px dashed #7c3aed; color: #a78bfa; padding: 6px 12px; border-radius: 6px; cursor: pointer; font-size: 12px; transition: all 0.15s; }
  .hint-toggle:hover { background: #7c3aed20; }
  .hint-box { margin-top: 8px; padding: 10px 14px; background: #2d1b4e; border: 1px solid #7c3aed; border-radius: 8px; color: #d8b4fe; font-size: 13px; line-height: 1.5; }

  .exam-card { background: #1e293b; border: 1px solid #334155; border-radius: 14px; padding: 28px; }
  .exam-question { font-size: 20px; font-weight: 600; color: #f1f5f9; margin-bottom: 20px; line-height: 1.4; }
  .q-num { color: #0ea5e9; margin-right: 6px; }
  .quiz-options { display: flex; flex-direction: column; gap: 10px; margin-bottom: 20px; }

  .submit-btn, .next-btn { width: 100%; padding: 14px; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .submit-btn { background: #0ea5e9; color: #fff; }
  .submit-btn:hover:not(:disabled) { background: #0284c7; }
  .submit-btn:disabled { opacity: 0.5; cursor: default; }
  .next-btn { background: #1e293b; color: #38bdf8; border: 1px solid #334155; margin-top: 12px; }
  .next-btn:hover { background: #263548; }

  .result-card { background: #1e293b; border: 1px solid #334155; border-radius: 16px; padding: 40px; text-align: center; }
  .result-icon { font-size: 72px; margin-bottom: 16px; }
  .result-card h2 { font-size: 24px; font-weight: 700; color: #f1f5f9; margin-bottom: 16px; }
  .result-score { font-size: 48px; font-weight: 700; color: #38bdf8; margin-bottom: 4px; }
  .rs-sep { color: #475569; margin: 0 4px; }
  .rs-total { color: #64748b; }
  .result-pct { font-size: 18px; color: #94a3b8; margin-bottom: 16px; }
  .result-msg { color: #cbd5e1; font-size: 14px; margin-bottom: 24px; }
  .result-actions { display: flex; gap: 12px; justify-content: center; }
  .btn-link { display: inline-flex; align-items: center; padding: 12px 24px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; text-decoration: none; font-weight: 600; font-size: 14px; }
  .btn-link:hover { background: #263548; }
</style>
