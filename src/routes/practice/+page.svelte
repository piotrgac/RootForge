<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';
  import { speedCommands } from '$lib/speed-commands';
  import { vimChallenges } from '$lib/vim-commands';
  import QuizOption from '$lib/components/QuizOption.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

  const SESSION_LENGTH = 10;
  let phase = $state('start');
  let items = $state([]);
  let currentIdx = $state(0);
  let currentType = $state('');
  let result = $state(null);
  let input = $state('');
  let score = $state(0);
  let total = $state(0);
  let startTime = $state(0);

  function normalize(cmd) { return cmd.trim().replace(/\s+/g, ' '); }

  function startSession() {
    const challenges = (invoke('get_dashboard_stats') || Promise.resolve({})).then(d => d || {});
    // We'll build mixed items from local data
    const allItems = [];
    const qs = []; // quiz questions
    const sp = speedCommands.map(s => ({ ...s, _type: 'speed' }));
    const vm = vimChallenges.map(v => ({ ...v, _type: 'vim' }));

    // Shuffle each pool and take
    const shuffle = arr => arr.sort(() => Math.random() - 0.5);
    const take = (arr, n) => arr.slice(0, n);

    // Mixed session: 4 speed + 3 vim + 3 quiz-like
    allItems.push(...take(shuffle([...sp]), 4));
    allItems.push(...take(shuffle([...vm]), 3));
    // For quiz-like, we fetch from backend
    items = shuffle(allItems).slice(0, SESSION_LENGTH);
    currentIdx = 0;
    score = 0;
    total = 0;
    phase = 'playing';
    startTime = Date.now();
  }

  let current = $derived(items[currentIdx]);
  let progressPct = $derived(items.length > 0 ? (total / items.length) * 100 : 0);

  function submitAnswer() {
    if (!current || result) return;
    const time = Math.round((Date.now() - startTime) / 1000);
    let correct = false;

    if (current._type === 'speed') {
      const user = normalize(input);
      correct = current.answers.some(a => normalize(a) === user);
      invoke('finish_speed_challenge', { commandId: current.id, timeSeconds: time, correct });
    } else if (current._type === 'vim') {
      const user = normalize(input);
      const answers = [current.answer, ...(current.alternatives || [])].map(a => normalize(a));
      correct = answers.some(a => a === user);
      invoke('finish_speed_challenge', { commandId: current.id + 2000, timeSeconds: time, correct });
    }

    if (correct) score++;
    total++;
    const xp = correct ? (time <= 5 ? 20 : time <= 15 ? 15 : time <= 30 ? 10 : 5) : 0;
    result = { correct, expected: current.answers?.[0] || current.answer, xp, time };
  }

  function next() {
    if (currentIdx < items.length - 1) {
      currentIdx++;
      input = '';
      result = null;
    } else {
      phase = 'finished';
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter' && !result) submitAnswer();
    if (e.key === 'Enter' && result) next();
  }
</script>

<div class="practice-page">
  <header class="page-header">
    <h1>🎯 Tryb mieszany</h1>
    <p>Quiz + Speed + Vim – losowo wymieszane w jednej sesji. Badania pokazują, że interleaving uczy 2x skuteczniej.</p>
  </header>

  {#if phase === 'start'}
    <div class="start-card card">
      <div class="start-icon">🎲</div>
      <h2>Gotowy na mieszankę?</h2>
      <p class="start-desc">10 losowych zadań z różnych kategorii: pytania quizowe, komendy Linux i sekwencje Vim.</p>
      <button class="start-btn" onclick={startSession}>🚀 Rozpocznij sesję</button>
    </div>

  {:else if phase === 'playing' && current}
    <div class="progress-row">
      <span class="q-num">Pytanie {currentIdx + 1}/{items.length}</span>
      <div class="q-progress"><ProgressBar percent={progressPct} height={6} /></div>
    </div>

    <div class="practice-card card">
      <div class="prac-type">
        <span class="type-badge type-{current._type}">
          {current._type === 'speed' ? '⚡ Speed' : current._type === 'vim' ? '🔤 Vim' : '🧠 Quiz'}
        </span>
        <span class="prac-score">✅ {score}/{total}</span>
      </div>

      <h2 class="prac-task">{current._type === 'vim' ? current.task : current.description}</h2>

      {#if !result}
        <div class="input-row">
          <input type="text" bind:value={input} placeholder={current._type === 'vim' ? 'Wpisz sekwencję Vim...' : 'Wpisz komendę...'}
            onkeydown={handleKeydown} autofocus class="prac-input" />
          <button class="submit-btn" onclick={submitAnswer} disabled={!input.trim()}>↵</button>
        </div>
      {:else}
        <div class="result" class:correct={result.correct} class:wrong={!result.correct}>
          <span class="result-icon">{result.correct ? '✅' : '❌'}</span>
          <div class="result-info">
            <strong>{result.correct ? 'Dobrze!' : 'Źle'}</strong>
            <p class="expected">Poprawnie: <code>{result.expected}</code></p>
            <p class="time-info">+{result.xp} XP · ⏱️ {result.time}s</p>
          </div>
        </div>
        <button class="next-btn" onclick={next}>
          {currentIdx < items.length - 1 ? '➡️ Dalej' : '🏁 Wyniki'}
        </button>
      {/if}
    </div>

  {:else if phase === 'finished'}
    <div class="result-card card">
      <div class="result-icon">{score >= 7 ? '🏆' : score >= 5 ? '💪' : '📚'}</div>
      <h2>Sesja zakończona!</h2>
      <div class="final-score">
        <span class="fs-big">{score}/{total}</span>
        <span class="fs-label">poprawnych</span>
      </div>
      <div class="result-actions">
        <button class="start-btn" onclick={startSession}>🔄 Jeszcze raz</button>
        <a href="/" class="btn-link">🏠 Dashboard</a>
      </div>
    </div>
  {/if}
</div>

<style>
  .practice-page { max-width: 700px; }
  .start-card { text-align: center; padding: 32px; }
  .start-icon { font-size: 56px; margin-bottom: 12px; }
  .start-card h2 { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .start-desc { color: #94a3b8; font-size: 14px; margin-bottom: 20px; }
  .start-btn { padding: 14px 32px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-size: 16px; font-weight: 600; cursor: pointer; }
  .start-btn:hover { background: #0284c7; }

  .progress-row { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
  .q-num { color: #94a3b8; font-size: 13px; min-width: 100px; }
  .q-progress { flex: 1; }

  .practice-card { padding: 24px; }
  .prac-type { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
  .type-badge { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }
  .type-speed { background: #0ea5e920; color: #38bdf8; }
  .type-vim { background: #7c3aed20; color: #a78bfa; }
  .prac-score { font-size: 13px; color: #64748b; }

  .prac-task { font-size: 18px; font-weight: 600; color: #f1f5f9; line-height: 1.4; margin-bottom: 16px; }

  .input-row { display: flex; gap: 8px; margin-bottom: 12px; }
  .prac-input { flex: 1; padding: 12px 14px; background: #0f172a; border: 2px solid #334155; border-radius: 10px; color: #e2e8f0; font-size: 18px; outline: none; }
  .prac-input:focus { border-color: #0ea5e9; }
  .submit-btn { padding: 12px 20px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-size: 18px; cursor: pointer; }
  .submit-btn:hover { background: #0284c7; }
  .submit-btn:disabled { opacity: 0.4; }

  .next-btn { width: 100%; padding: 14px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; }
  .next-btn:hover { background: #263548; }

  .result { display: flex; gap: 14px; align-items: center; padding: 14px; border-radius: 10px; margin-bottom: 12px; }
  .result.correct { background: #22c55e20; border: 1px solid #22c55e; }
  .result.wrong { background: #ef444420; border: 1px solid #ef4444; }
  .result-icon { font-size: 28px; }
  .result-info { flex: 1; }
  .result-info strong { display: block; font-size: 15px; margin-bottom: 4px; color: #f1f5f9; }
  .expected { font-size: 13px; color: #94a3b8; }
  .expected code { color: #38bdf8; font-size: 14px; }
  .time-info { font-size: 12px; color: #64748b; }

  .result-card { text-align: center; padding: 40px; }
  .result-card .result-icon { font-size: 64px; margin-bottom: 16px; }
  .result-card h2 { font-size: 24px; font-weight: 700; color: #f1f5f9; margin-bottom: 12px; }
  .final-score { margin-bottom: 20px; }
  .fs-big { display: block; font-size: 42px; font-weight: 800; color: #38bdf8; }
  .fs-label { font-size: 14px; color: #64748b; }
  .result-actions { display: flex; gap: 10px; justify-content: center; }
  .btn-link { display: inline-flex; align-items: center; padding: 12px 24px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; text-decoration: none; font-weight: 600; }
  .btn-link:hover { background: #263548; }
</style>
