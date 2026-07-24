<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

  let scenarios = $state([]);
  let completed = $state([]);
  let loading = $state(true);
  let currentScenario = $state(null);
  let revealedHints = $state(0);
  let showSolution = $state(false);
  let solved = $state(false);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      scenarios = data.troubleshoot || [];
      completed = data.troubleshoot_results || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  function openScenario(s) {
    currentScenario = s;
    revealedHints = 0;
    showSolution = false;
    solved = false;
  }

  function revealHint() {
    if (revealedHints < currentScenario.hints.length) revealedHints++;
  }

  async function markSolved() {
    await invoke('complete_troubleshoot', { id: currentScenario.id });
    solved = true;
    completed = [...completed, currentScenario.id];
  }

  function closeScenario() {
    currentScenario = null;
  }

  function difficultyStars(n) {
    return '★'.repeat(n) + '☆'.repeat(5 - n);
  }

  let totalSolved = $derived(completed.length);
  let totalScenarios = $derived(scenarios.length);
</script>

<div class="troubleshoot-page">
  <header class="page-header">
    <h1>🔧 Troubleshooting</h1>
    <p>Diagnozuj i rozwiązuj prawdziwe problemy administracyjne</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>

  {:else if currentScenario}
    {@const cat = getCategoryInfo(currentScenario.category)}
    <div class="ts-card card">
      <div class="ts-header">
        <span class="ts-badge" style="background: {cat.color}20; color: {cat.color}">{cat.name}</span>
        <span class="ts-diff">{difficultyStars(currentScenario.difficulty)}</span>
        <button class="ts-close" onclick={closeScenario}>✕</button>
      </div>

      <h2 class="ts-title">{currentScenario.title}</h2>

      <div class="ts-scenario">
        <h3>📋 Problem:</h3>
        <p>{currentScenario.scenario}</p>
      </div>

      {#if !solved && !showSolution}
        <div class="ts-hints">
          <h3>💡 Podpowiedzi ({revealedHints}/{currentScenario.hints.length})</h3>
          {#each currentScenario.hints as hint, i}
            {#if i < revealedHints}
              <div class="ts-hint">{hint}</div>
            {/if}
          {/each}
          {#if revealedHints < currentScenario.hints.length}
            <button class="hint-btn" onclick={revealHint}>🔍 Pokaż podpowiedź {revealedHints + 1}</button>
          {:else}
            <button class="solve-btn" onclick={markSolved}>✅ Rozwiązałem!</button>
            <button class="solution-btn" onclick={() => showSolution = true}>👁️ Pokaż rozwiązanie</button>
          {/if}
        </div>
      {:else if showSolution}
        <div class="ts-solution">
          <h3>✅ Rozwiązanie:</h3>
          <pre class="sol-content">{currentScenario.solution}</pre>
          {#if !solved}
            <button class="solve-btn" onclick={markSolved}>✅ Rozwiązałem!</button>
          {:else}
            <div class="solved-badge">✓ Rozwiązane (+25 XP)</div>
          {/if}
        </div>
      {:else}
        <div class="ts-solved">
          <div class="solved-icon">🎉</div>
          <h3>Rozwiązane!</h3>
          <p>Zdobywasz +25 XP za samodzielne rozwiązanie problemu.</p>
          <button class="hint-btn" onclick={() => showSolution = true}>👁️ Zobacz rozwiązanie</button>
        </div>
      {/if}

      {#if solved}
        <button class="next-btn" onclick={closeScenario}>➡️ Powrót do listy</button>
      {/if}
    </div>

  {:else}
    <div class="ts-stats">
      <div class="ts-stat-card card">
        <span class="ts-stat-val">{totalSolved}/{totalScenarios}</span>
        <span class="ts-stat-label">Rozwiązane</span>
      </div>
      <div class="ts-stat-card card">
        <span class="ts-stat-val">{totalScenarios - totalSolved}</span>
        <span class="ts-stat-label">Pozostało</span>
      </div>
      <div class="ts-stat-card card">
        <span class="ts-stat-val">{totalSolved > 0 ? Math.round(totalSolved / totalScenarios * 100) : 0}%</span>
        <span class="ts-stat-label">Postęp</span>
      </div>
    </div>

    <div class="ts-grid">
      {#each scenarios as s}
        {@const cat = getCategoryInfo(s.category)}
        {@const done = completed.includes(s.id)}
        <button class="ts-item card" class:done onclick={() => openScenario(s)}>
          <div class="ts-item-header">
            <span class="ts-item-badge" style="background: {cat.color}20; color: {cat.color}">{cat.name}</span>
            <span class="ts-item-diff">{difficultyStars(s.difficulty)}</span>
          </div>
          <h3 class="ts-item-title">{s.title}</h3>
          <p class="ts-item-desc">{s.scenario.slice(0, 120)}...</p>
          <span class="ts-item-status">{done ? '✅ Rozwiązane' : '🔧 Do rozwiązania'}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .troubleshoot-page { max-width: 800px; }

  .ts-stats { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; margin-bottom: 20px; }
  .ts-stat-card { text-align: center; padding: 16px; }
  .ts-stat-val { display: block; font-size: 24px; font-weight: 700; color: #38bdf8; }
  .ts-stat-label { font-size: 12px; color: #64748b; }

  .ts-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 12px; }
  .ts-item { text-align: left; cursor: pointer; transition: all 0.15s; border-color: #334155; }
  .ts-item:hover { border-color: #475569; background: #263548; }
  .ts-item.done { opacity: 0.7; border-color: #166534; }
  .ts-item-header { display: flex; gap: 8px; align-items: center; margin-bottom: 8px; }
  .ts-item-badge { font-size: 10px; font-weight: 600; padding: 3px 8px; border-radius: 4px; text-transform: uppercase; }
  .ts-item-diff { color: #f59e0b; font-size: 11px; letter-spacing: 1px; }
  .ts-item-title { font-size: 15px; font-weight: 600; color: #f1f5f9; margin-bottom: 6px; }
  .ts-item-desc { font-size: 12px; color: #64748b; line-height: 1.5; margin-bottom: 8px; }
  .ts-item-status { font-size: 11px; color: #94a3b8; }

  .ts-card { padding: 24px; }
  .ts-header { display: flex; align-items: center; gap: 8px; margin-bottom: 12px; }
  .ts-badge { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }
  .ts-diff { color: #f59e0b; font-size: 13px; letter-spacing: 1px; flex: 1; }
  .ts-close { background: none; border: none; color: #64748b; font-size: 18px; cursor: pointer; }
  .ts-close:hover { color: #e2e8f0; }

  .ts-title { font-size: 20px; font-weight: 700; color: #f1f5f9; margin-bottom: 16px; }
  .ts-scenario { margin-bottom: 20px; }
  .ts-scenario h3 { font-size: 14px; font-weight: 600; color: #94a3b8; margin-bottom: 8px; }
  .ts-scenario p { font-size: 14px; color: #cbd5e1; line-height: 1.7; }

  .ts-hints { margin-bottom: 16px; }
  .ts-hints h3 { font-size: 14px; font-weight: 600; color: #94a3b8; margin-bottom: 8px; }
  .ts-hint { padding: 8px 12px; background: #2d1b4e; border: 1px solid #7c3aed; border-radius: 8px; color: #d8b4fe; font-size: 13px; margin-bottom: 6px; }

  .hint-btn, .solve-btn, .solution-btn { padding: 8px 16px; border-radius: 8px; border: none; font-size: 13px; font-weight: 600; cursor: pointer; margin-right: 6px; margin-top: 6px; }
  .hint-btn { background: #7c3aed; color: #fff; }
  .hint-btn:hover { background: #6d28d9; }
  .solve-btn { background: #22c55e; color: #fff; }
  .solve-btn:hover { background: #16a34a; }
  .solution-btn { background: #1e293b; color: #94a3b8; border: 1px solid #334155; }
  .solution-btn:hover { background: #334155; color: #e2e8f0; }

  .ts-solution { margin-bottom: 16px; }
  .ts-solution h3 { font-size: 14px; font-weight: 600; color: #22c55e; margin-bottom: 8px; }
  .sol-content { background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 14px; font-size: 13px; color: #cbd5e1; white-space: pre-wrap; margin-bottom: 12px; }

  .ts-solved { text-align: center; padding: 24px; margin-bottom: 12px; }
  .solved-icon { font-size: 48px; margin-bottom: 8px; }
  .ts-solved h3 { font-size: 18px; font-weight: 700; color: #22c55e; margin-bottom: 8px; }
  .ts-solved p { color: #94a3b8; font-size: 14px; margin-bottom: 12px; }
  .solved-badge { display: inline-block; padding: 6px 14px; background: #22c55e20; border: 1px solid #22c55e; border-radius: 8px; color: #22c55e; font-weight: 600; font-size: 13px; }

  .next-btn { width: 100%; padding: 12px; background: #1e293b; color: #38bdf8; border: 1px solid #334155; border-radius: 10px; font-size: 14px; font-weight: 600; cursor: pointer; transition: all 0.15s; }
  .next-btn:hover { background: #263548; }
</style>
