<script>
  import { invoke } from '@tauri-apps/api/core';
  import { getCategoryInfo } from '$lib/categories';
  import { speedCommands } from '$lib/speed-commands';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

  let sessionActive = $state(false);
  let currentIdx = $state(0);
  let input = $state('');
  let result = $state(null);
  let scoring = $state(false);
  let startTime = $state(0);
  let elapsed = $state(0);
  let correctCount = $state(0);
  let totalAnswered = $state(0);
  let sessionResults = $state([]);
  let phase = $state('start'); // start | playing | finished

  let sessionSize = $state(10);
  let sessionQuestions = $state([]);

  let current = $derived(sessionQuestions[currentIdx]);

  function startSession() {
    const shuffled = [...speedCommands].sort(() => Math.random() - 0.5);
    sessionQuestions = shuffled.slice(0, sessionSize);
    currentIdx = 0;
    correctCount = 0;
    totalAnswered = 0;
    sessionResults = [];
    phase = 'playing';
    sessionActive = true;
    input = '';
    result = null;
    startTime = Date.now();
    elapsed = 0;
  }

  function normalize(cmd) {
    return cmd.trim().replace(/\s+/g, ' ');
  }

  function checkAnswer() {
    if (!current || scoring) return;
    scoring = true;
    const time = Math.round((Date.now() - startTime) / 1000);
    elapsed = time;
    const userCmd = normalize(input);
    const isCorrect = current.answers.some(a => normalize(a) === userCmd);
    sessionResults.push({ command_id: current.id, correct: isCorrect, time });
    if (isCorrect) correctCount++;
    totalAnswered++;
    let feedback = '';
    if (!isCorrect) {
      if (current.feedback) {
        feedback = current.feedback;
      } else {
        const firstExpected = current.answers[0];
        const userParts = userCmd.split(' ');
        const expectedParts = firstExpected.split(' ');
        const common = userParts.filter(p => expectedParts.includes(p));
        const missing = expectedParts.filter(p => !userParts.includes(p));
        if (missing.length > 0) {
          feedback = `Brakuje: ${missing.join(' ')}. `;
        }
        if (common.length > 0) {
          feedback += `Dobrze użyłeś: ${common.join(' ')}.`;
        }
        if (!feedback) {
          feedback = `Spróbuj: ${firstExpected}`;
        }
      }
    }
    result = { correct: isCorrect, time, expected: current.answers[0], feedback };
    invoke('finish_speed_challenge', { commandId: current.id, timeSeconds: time, correct: isCorrect });
    scoring = false;
  }

  function next() {
    if (currentIdx < sessionQuestions.length - 1) {
      currentIdx++;
      input = '';
      result = null;
      startTime = Date.now();
    } else {
      phase = 'finished';
      sessionActive = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter' && !result) checkAnswer();
    if (e.key === 'Enter' && result) next();
  }

  let progressPct = $derived(sessionQuestions.length > 0 ? (totalAnswered / sessionQuestions.length) * 100 : 0);
  let avgTime = $derived(sessionResults.length > 0 ? Math.round(sessionResults.reduce((a, r) => a + r.time, 0) / sessionResults.length) : 0);
</script>

<div class="speed-page">
  <header class="page-header">
    <h1>⚡ Speed Challenge</h1>
    <p>Wpisz komendę z pamięci – licz się czas i celność</p>
  </header>

  {#if phase === 'start'}
    <div class="start-card card">
      <div class="start-icon">⌨️</div>
      <h2>Jak to działa?</h2>
      <ul class="rules">
        <li>📋 Zobaczysz opis zadania po polsku</li>
        <li>⌨️ Wpisz poprawną komendę Linux z pamięci</li>
        <li>⚡ Im szybciej i celniej, tym więcej XP</li>
        <li>🏆 do 20 XP za zadanie (≤5s = 20XP, ≤15s = 15XP, ≤30s = 10XP, więcej = 5XP)</li>
        <li>🎯 Jeśli nie znasz komendy – poddaj się, zobaczysz odpowiedź i nauczysz się na następny raz</li>
      </ul>
      <div class="session-size">
        <label>Liczba pytań:</label>
        <select bind:value={sessionSize}>
          <option value={5}>5 (szybka rozgrzewka)</option>
          <option value={10}>10 (standard)</option>
          <option value={20}>20 (trening)</option>
          <option value={50}>50 (maraton)</option>
        </select>
      </div>
      <button class="start-btn" onclick={startSession}>🚀 Rozpocznij</button>
    </div>

  {:else if phase === 'playing' && current}
    {@const cat = getCategoryInfo(current.category)}
    <div class="progress-row">
      <span class="q-num">Pytanie {currentIdx + 1}/{sessionQuestions.length}</span>
      <div class="q-progress">
        <ProgressBar percent={progressPct} height={6} />
      </div>
    </div>

    <div class="challenge-card card">
      <div class="challenge-meta">
        <span class="challenge-badge" style="background: {cat.color}20; color: {cat.color}">{cat.name}</span>
        <span class="challenge-diff">{'★'.repeat(current.difficulty)}{'☆'.repeat(5 - current.difficulty)}</span>
      </div>
      <h2 class="challenge-desc">{current.description}</h2>

      {#if current.hint && !result}
        <p class="hint">💡 {current.hint}</p>
      {/if}

      <div class="input-row">
        <input
          type="text"
          bind:value={input}
          placeholder="Wpisz komendę..."
          onkeydown={handleKeydown}
          disabled={result !== null}
          autofocus
          class="cmd-input"
        />
        {#if !result}
          <button class="submit-btn" onclick={checkAnswer} disabled={!input.trim()}>↵</button>
        {:else}
          <button class="next-btn" onclick={next}>
            {currentIdx < sessionQuestions.length - 1 ? '➡️ Dalej' : '🏁 Zobacz wyniki'}
          </button>
        {/if}
      </div>

      {#if result}
        <div class="result" class:correct={result.correct} class:wrong={!result.correct}>
          <div class="result-icon">{result.correct ? '✅' : '❌'}</div>
          <div class="result-info">
            <strong>{result.correct ? 'Dobrze!' : 'Źle'}</strong>
            {#if !result.correct}
              <p class="expected">Poprawna komenda: <code>{result.expected}</code></p>
              {#if result.feedback}
                <p class="feedback">{result.feedback}</p>
              {/if}
            {/if}
            <p class="time-info">
              {result.correct ? `+${result.time <= 5 ? 20 : result.time <= 15 ? 15 : result.time <= 30 ? 10 : 5} XP` : '0 XP'}
              · ⏱️ {result.time}s
            </p>
          </div>
        </div>
      {/if}
    </div>

    <div class="live-stats">
      <span class="live-stat">✅ {correctCount}/{totalAnswered}</span>
      <span class="live-stat">🎯 {totalAnswered > 0 ? Math.round(correctCount / totalAnswered * 100) : 0}%</span>
    </div>

  {:else if phase === 'finished'}
    <div class="result-card card">
      <div class="result-icon">{correctCount >= sessionQuestions.length * 0.7 ? '🏆' : '💪'}</div>
      <h2>Koniec sesji!</h2>
      <div class="final-score">
        <span class="fs-big">{correctCount}/{sessionQuestions.length}</span>
        <span class="fs-label">poprawnych odpowiedzi</span>
      </div>
      <div class="final-stats">
        <div class="fs-item">
          <span class="fs-val">{Math.round(correctCount / Math.max(totalAnswered, 1) * 100)}%</span>
          <span class="fs-lbl">celność</span>
        </div>
        <div class="fs-item">
          <span class="fs-val">{avgTime}s</span>
          <span class="fs-lbl">średni czas</span>
        </div>
        <div class="fs-item">
          <span class="fs-val">{sessionResults.filter(r => r.time <= 5).length}</span>
          <span class="fs-lbl">błyskawiczne (≤5s)</span>
        </div>
      </div>
      <div class="result-actions">
        <button class="start-btn" onclick={startSession}>🔄 Jeszcze raz</button>
        <a href="/" class="btn-link">🏠 Dashboard</a>
      </div>
    </div>
  {/if}
</div>

<style>
  .speed-page { max-width: 700px; }

  .start-card { text-align: center; padding: 32px; }
  .start-icon { font-size: 64px; margin-bottom: 16px; }
  .start-card h2 { font-size: 24px; font-weight: 700; color: #f1f5f9; margin-bottom: 20px; }
  .rules { text-align: left; list-style: none; padding: 0; margin: 0 auto 24px; max-width: 480px; }
  .rules li { padding: 8px 0; color: #cbd5e1; font-size: 14px; border-bottom: 1px solid #334155; }
  .rules li:last-child { border-bottom: none; }

  .session-size { margin-bottom: 24px; display: flex; align-items: center; justify-content: center; gap: 12px; }
  .session-size label { color: #94a3b8; font-size: 14px; }
  .session-size select {
    padding: 8px 12px;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    color: #e2e8f0;
    font-size: 14px;
    cursor: pointer;
  }
  .session-size select:focus { outline: none; border-color: #0ea5e9; }

  .start-btn { padding: 14px 32px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-size: 16px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .start-btn:hover { background: #0284c7; }

  .progress-row { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
  .q-num { color: #94a3b8; font-size: 13px; min-width: 120px; }
  .q-progress { flex: 1; }

  .challenge-card { padding: 28px; }
  .challenge-meta { display: flex; gap: 8px; align-items: center; margin-bottom: 16px; }
  .challenge-badge { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }
  .challenge-diff { color: #f59e0b; font-size: 13px; letter-spacing: 1px; }
  .challenge-desc { font-size: 20px; font-weight: 600; color: #f1f5f9; line-height: 1.4; margin-bottom: 20px; }
  .hint { font-size: 13px; color: #a78bfa; margin-bottom: 16px; padding: 10px 14px; background: #2d1b4e; border: 1px solid #7c3aed; border-radius: 8px; }

  .input-row { display: flex; gap: 8px; margin-bottom: 16px; }
  .cmd-input {
    flex: 1;
    padding: 14px 16px;
    background: #0f172a;
    border: 2px solid #334155;
    border-radius: 10px;
    color: #e2e8f0;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 16px;
    outline: none;
    transition: border-color 0.15s;
  }
  .cmd-input:focus { border-color: #0ea5e9; }
  .cmd-input:disabled { opacity: 0.6; }

  .submit-btn { padding: 14px 20px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-size: 18px; cursor: pointer; transition: background 0.15s; }
  .submit-btn:hover { background: #0284c7; }
  .submit-btn:disabled { opacity: 0.4; }

  .next-btn { flex: 1; padding: 14px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: all 0.15s; }
  .next-btn:hover { background: #263548; }

  .result { display: flex; gap: 16px; align-items: center; padding: 16px; border-radius: 12px; }
  .result.correct { background: #22c55e20; border: 1px solid #22c55e; }
  .result.wrong { background: #ef444420; border: 1px solid #ef4444; }
  .result-icon { font-size: 32px; }
  .result-info { flex: 1; }
  .result-info strong { display: block; font-size: 16px; margin-bottom: 4px; color: #f1f5f9; }
  .expected { font-size: 13px; color: #94a3b8; margin-bottom: 4px; }
  .expected code { color: #38bdf8; font-family: 'JetBrains Mono', monospace; font-size: 14px; }
  .feedback { font-size: 12px; color: #a78bfa; margin-top: 6px; padding: 8px 10px; background: #2d1b4e; border-radius: 6px; line-height: 1.5; }
  .time-info { font-size: 12px; color: #64748b; }

  .live-stats { display: flex; gap: 16px; margin-top: 16px; justify-content: center; }
  .live-stat { padding: 6px 14px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; font-size: 13px; color: #94a3b8; }

  .result-card { text-align: center; padding: 40px; }
  .result-card .result-icon { font-size: 72px; margin-bottom: 16px; }
  .result-card h2 { font-size: 24px; font-weight: 700; color: #f1f5f9; margin-bottom: 16px; }
  .final-score { margin-bottom: 24px; }
  .fs-big { display: block; font-size: 48px; font-weight: 800; color: #38bdf8; }
  .fs-label { font-size: 14px; color: #64748b; }

  .final-stats { display: flex; gap: 24px; justify-content: center; margin-bottom: 24px; }
  .fs-item { text-align: center; }
  .fs-val { display: block; font-size: 22px; font-weight: 700; color: #f1f5f9; }
  .fs-lbl { font-size: 12px; color: #64748b; }

  .result-actions { display: flex; gap: 12px; justify-content: center; }
  .btn-link { display: inline-flex; align-items: center; padding: 12px 24px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; text-decoration: none; font-weight: 600; font-size: 14px; }
  .btn-link:hover { background: #263548; }
</style>
