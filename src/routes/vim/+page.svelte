<script>
  import { invoke } from '@tauri-apps/api/core';
  import { vimChallenges, vimLevelNames, getVimByLevel } from '$lib/vim-commands';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

  let phase = $state('start');
  let currentLevel = $state(1);
  let currentIdx = $state(0);
  let input = $state('');
  let result = $state(null);
  let scoring = $state(false);
  let correctCount = $state(0);
  let totalAnswered = $state(0);
  let sessionResults = $state([]);
  let cheatMode = $state(false);
  let showHint = $state(false);

  let filtered = $derived(vimChallenges.filter(c => c.level === currentLevel));
  let current = $derived(filtered[currentIdx]);

  function startSession() {
    const shuffled = [...filtered].sort(() => Math.random() - 0.5);
    const all = [...shuffled];
    currentIdx = 0;
    correctCount = 0;
    totalAnswered = 0;
    sessionResults = [];
    phase = 'playing';
    input = '';
    result = null;
    showHint = false;
  }

  function checkAnswer() {
    if (!current || scoring) return;
    scoring = true;
    const user = input.trim();
    const normalizedAnswers = [current.answer, ...(current.alternatives || [])]
      .map(a => a.trim().toLowerCase());
    const isCorrect = normalizedAnswers.some(a => a === user.toLowerCase()) ||
      normalizedAnswers.some(a => normalizeVim(user) === normalizeVim(a));
    sessionResults.push({ id: current.id, correct: isCorrect });
    if (isCorrect) correctCount++;
    totalAnswered++;
    result = { correct: isCorrect, answer: current.answer, task: current.task };
    if (isCorrect) {
      invoke('finish_speed_challenge', { commandId: current.id + 1000, timeSeconds: 0, correct: true });
    }
    scoring = false;
  }

  function normalizeVim(s) {
    return s.replace(/\s+/g, '').replace(/[<>]/g, '').toLowerCase();
  }

  function next() {
    if (currentIdx < filtered.length - 1) {
      currentIdx++;
      input = '';
      result = null;
      showHint = false;
    } else {
      phase = 'finished';
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter' && !result) checkAnswer();
    if (e.key === 'Enter' && result) next();
  }

  let progressPct = $derived(filtered.length > 0 ? (totalAnswered / filtered.length) * 100 : 0);
  let totalCorrect = $derived(sessionResults.filter(r => r.correct).length);
</script>

<div class="vim-page">
  <header class="page-header">
    <h1>🔤 Vim Master</h1>
    <p>Opanuj Vima – wpisz sekwencję klawiszy z pamięci</p>
  </header>

  {#if phase === 'start'}
    <div class="start-card card">
      <div class="start-icon">🔤</div>
      <h2>Tryb nauki Vim</h2>
      <p class="start-desc">Widzisz zadanie po polsku – wpisz sekwencję klawiszy Vim, która je wykonuje.</p>

      <div class="level-info">
        <h3>Poziomy:</h3>
        <div class="level-grid">
          {#each Object.entries(vimLevelNames) as [lvl, name]}
            <button class="level-btn" class:active={currentLevel === parseInt(lvl)} onclick={() => currentLevel = parseInt(lvl)}>
              {name}
              <span class="level-count">{getVimByLevel(parseInt(lvl)).length}</span>
            </button>
          {/each}
        </div>
      </div>

      <div class="start-actions">
        <button class="start-btn" onclick={startSession}>🚀 Rozpocznij ({filtered.length} zadań)</button>
      </div>

      {#if cheatMode}
        <div class="cheatsheet">
          <h3>📋 Ściąga Vim</h3>
          <div class="cs-grid">
            {#each vimChallenges.filter(c => c.level <= currentLevel) as ch}
              <div class="cs-item">
                <code class="cs-key">{ch.answer}</code>
                <span class="cs-task">{ch.task}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
      <button class="cheat-toggle" onclick={() => cheatMode = !cheatMode}>
        {cheatMode ? '🙈 Ukryj ściągę' : '📖 Pokaż ściągę'}
      </button>
    </div>

  {:else if phase === 'playing' && current}
    <div class="progress-row">
      <span class="q-num">Pytanie {currentIdx + 1}/{filtered.length}</span>
      <div class="q-progress">
        <ProgressBar percent={progressPct} height={6} />
      </div>
    </div>

    <div class="vim-card card">
      <div class="vim-meta">
        <span class="vim-level">Poziom {current.level}: {vimLevelNames[current.level]}</span>
        <span class="vim-id">#{current.id}</span>
      </div>

      <h2 class="vim-task">{current.task}</h2>

      {#if showHint && current.hint}
        <div class="vim-hint">💡 {current.hint}</div>
      {/if}

      <div class="input-row">
        <input
          type="text"
          bind:value={input}
          placeholder="Wpisz sekwencję..."
          onkeydown={handleKeydown}
          disabled={result !== null}
          autofocus
          class="vim-input"
        />
        {#if !result}
          <button class="submit-btn" onclick={checkAnswer} disabled={!input.trim()}>↵</button>
        {:else}
          <button class="next-btn" onclick={next}>
            {currentIdx < filtered.length - 1 ? '➡️ Dalej' : '🏁 Wyniki'}
          </button>
        {/if}
      </div>

      {#if !result && !showHint}
        <button class="hint-toggle" onclick={() => showHint = true}>💡 Podpowiedź</button>
      {/if}

      {#if result}
        <div class="result" class:correct={result.correct} class:wrong={!result.correct}>
          <div class="result-icon">{result.correct ? '✅' : '❌'}</div>
          <div class="result-info">
            <strong>{result.correct ? 'Dobrze!' : 'Źle'}</strong>
            {#if !result.correct}
              <p class="expected">Prawidłowa sekwencja: <code>{result.answer}</code></p>
            {/if}
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
      <div class="result-icon">{totalCorrect >= filtered.length * 0.8 ? '🏆' : totalCorrect >= filtered.length * 0.5 ? '💪' : '📚'}</div>
      <h2>Koniec sesji!</h2>
      <div class="final-score">
        <span class="fs-big">{totalCorrect}/{filtered.length}</span>
        <span class="fs-label">poprawnych sekwencji</span>
      </div>
      <div class="result-actions">
        <button class="start-btn" onclick={startSession}>🔄 Jeszcze raz</button>
        <button class="start-btn secondary" onclick={() => { phase = 'start'; currentLevel = currentLevel < 5 ? currentLevel + 1 : 1; }}>
          {currentLevel < 5 ? '➡️ Następny poziom' : '🔁 Od nowa'}
        </button>
        <a href="/" class="btn-link">🏠 Dashboard</a>
      </div>
    </div>
  {/if}
</div>

<style>
  .vim-page { max-width: 700px; }

  .start-card { text-align: center; padding: 28px; }
  .start-icon { font-size: 56px; margin-bottom: 12px; }
  .start-card h2 { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .start-desc { color: #94a3b8; font-size: 14px; margin-bottom: 20px; }

  .level-info { margin-bottom: 20px; }
  .level-info h3 { font-size: 13px; color: #94a3b8; margin-bottom: 8px; }
  .level-grid { display: flex; gap: 6px; flex-wrap: wrap; justify-content: center; }
  .level-btn { padding: 8px 14px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 8px; cursor: pointer; font-size: 13px; transition: all 0.15s; }
  .level-btn:hover { background: #334155; color: #e2e8f0; }
  .level-btn.active { background: #7c3aed; color: #fff; border-color: #7c3aed; }
  .level-count { font-size: 10px; background: rgba(255,255,255,0.15); padding: 1px 6px; border-radius: 4px; margin-left: 4px; }

  .start-actions { margin-bottom: 12px; }
  .start-btn { padding: 12px 28px; background: #0ea5e9; color: #fff; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: background 0.15s; }
  .start-btn:hover { background: #0284c7; }
  .start-btn.secondary { background: #7c3aed; }
  .start-btn.secondary:hover { background: #6d28d9; }

  .cheat-toggle { padding: 6px 14px; background: none; border: 1px dashed #334155; color: #64748b; border-radius: 6px; cursor: pointer; font-size: 12px; margin-top: 8px; }

  .cheatsheet { margin-top: 16px; text-align: left; }
  .cheatsheet h3 { font-size: 14px; color: #e2e8f0; margin-bottom: 8px; }
  .cs-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 4px; max-height: 300px; overflow-y: auto; padding: 8px; background: #0f172a; border-radius: 8px; }
  .cs-item { display: flex; gap: 8px; align-items: center; padding: 4px 8px; border-radius: 4px; }
  .cs-key { font-family: 'JetBrains Mono', monospace; font-size: 12px; color: #38bdf8; background: #1e293b; padding: 2px 6px; border-radius: 4px; min-width: 60px; text-align: center; }
  .cs-task { font-size: 12px; color: #94a3b8; }

  .progress-row { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
  .q-num { color: #94a3b8; font-size: 13px; min-width: 120px; }
  .q-progress { flex: 1; }

  .vim-card { padding: 24px; }
  .vim-meta { display: flex; justify-content: space-between; margin-bottom: 12px; }
  .vim-level { font-size: 12px; color: #7c3aed; font-weight: 600; }
  .vim-id { font-size: 11px; color: #64748b; }
  .vim-task { font-size: 18px; font-weight: 600; color: #f1f5f9; line-height: 1.4; margin-bottom: 16px; }
  .vim-hint { padding: 10px 14px; background: #2d1b4e; border: 1px solid #7c3aed; border-radius: 8px; color: #d8b4fe; font-size: 13px; margin-bottom: 12px; }

  .input-row { display: flex; gap: 8px; margin-bottom: 12px; }
  .vim-input {
    flex: 1;
    padding: 12px 14px;
    background: #0f172a;
    border: 2px solid #334155;
    border-radius: 10px;
    color: #e2e8f0;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 18px;
    outline: none;
    transition: border-color 0.15s;
    letter-spacing: 2px;
  }
  .vim-input:focus { border-color: #7c3aed; }
  .vim-input:disabled { opacity: 0.6; }

  .submit-btn { padding: 12px 20px; background: #7c3aed; color: #fff; border: none; border-radius: 10px; font-size: 18px; cursor: pointer; transition: background 0.15s; }
  .submit-btn:hover { background: #6d28d9; }
  .submit-btn:disabled { opacity: 0.4; }

  .next-btn { flex: 1; padding: 12px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; }
  .next-btn:hover { background: #263548; }

  .hint-toggle { display: block; background: none; border: 1px dashed #7c3aed; color: #a78bfa; padding: 6px 12px; border-radius: 6px; cursor: pointer; font-size: 12px; margin-bottom: 12px; }
  .hint-toggle:hover { background: #7c3aed20; }

  .result { display: flex; gap: 14px; align-items: center; padding: 14px; border-radius: 10px; }
  .result.correct { background: #22c55e20; border: 1px solid #22c55e; }
  .result.wrong { background: #ef444420; border: 1px solid #ef4444; }
  .result-icon { font-size: 28px; }
  .result-info { flex: 1; }
  .result-info strong { display: block; font-size: 15px; margin-bottom: 4px; color: #f1f5f9; }
  .expected { font-size: 13px; color: #94a3b8; }
  .expected code { color: #38bdf8; font-family: 'JetBrains Mono', monospace; font-size: 14px; }

  .live-stats { display: flex; gap: 16px; margin-top: 16px; justify-content: center; }
  .live-stat { padding: 6px 14px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; font-size: 13px; color: #94a3b8; }

  .result-card { text-align: center; padding: 40px; }
  .result-card .result-icon { font-size: 64px; margin-bottom: 16px; }
  .result-card h2 { font-size: 24px; font-weight: 700; color: #f1f5f9; margin-bottom: 12px; }
  .final-score { margin-bottom: 20px; }
  .fs-big { display: block; font-size: 42px; font-weight: 800; color: #7c3aed; }
  .fs-label { font-size: 14px; color: #64748b; }
  .result-actions { display: flex; gap: 10px; justify-content: center; flex-wrap: wrap; }
  .btn-link { display: inline-flex; align-items: center; padding: 12px 24px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; text-decoration: none; font-weight: 600; font-size: 14px; }
  .btn-link:hover { background: #263548; }
</style>
