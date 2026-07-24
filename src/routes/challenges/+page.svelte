<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';
  import ChallengeModal from '$lib/components/ChallengeModal.svelte';

  let challenges = $state([]);
  let loading = $state(true);
  let filter = $state('all');
  let detailChallenge = $state(null);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      challenges = data.challenges || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  let filtered = $derived(filter === 'all' ? challenges
    : filter === 'completed' ? challenges.filter(c => c.completed)
    : challenges.filter(c => !c.completed));

  async function toggleComplete(ch) {
    const [success, xp, level] = await invoke('complete_challenge', { id: ch.id });
    if (success) {
      ch.completed = true;
    }
  }

  function difficultyStars(n) {
    return '★'.repeat(n) + '☆'.repeat(5 - n);
  }

  // formatDetails, handleCopyClick, stepMode, toggleStepMode,
  // extractCodeBlocks, toggleStep — moved to ChallengeModal component
</script>

<div class="challenges-page">
  <header class="page-header">
    <h1>🎯 Wyzwania</h1>
    <p>Wykonuj wyzwania, zdobywaj XP i odblokowuj kamienie milowe</p>
  </header>

  <div class="filters">
    <button class="filter-btn" class:active={filter === 'all'} onclick={() => filter = 'all'}>Wszystkie</button>
    <button class="filter-btn" class:active={filter === 'active'} onclick={() => filter = 'active'}>Aktywne</button>
    <button class="filter-btn" class:active={filter === 'completed'} onclick={() => filter = 'completed'}>Ukończone</button>
  </div>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="challenge-grid">
      {#each filtered as ch (ch.id)}
        {@const cat = getCategoryInfo(ch.category)}
        <div class="challenge-card" class:completed={ch.completed}>
          <div class="ch-top">
            <span class="ch-category" style="background: {cat.color}20; color: {cat.color}">
              {cat.name}
            </span>
            <div class="ch-badges">
              {#if ch.stage}
                <span class="ch-stage">Etap {ch.stage}</span>
              {/if}
              <span class="ch-difficulty">{difficultyStars(ch.difficulty)}</span>
            </div>
          </div>
          <h3 class="ch-title">{ch.title}</h3>
          <p class="ch-desc">{ch.description}</p>
          <div class="ch-footer">
            <span class="ch-xp">+{ch.difficulty * 10} XP</span>
            <div class="ch-actions">
              {#if ch.details}
                <button class="details-btn" onclick={() => detailChallenge = ch}>📖</button>
              {/if}
              {#if !ch.completed}
                <button class="complete-btn" onclick={() => toggleComplete(ch)}>
                  Oznacz jako ukończone
                </button>
              {:else}
                <span class="completed-badge">✓ Ukończono</span>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if detailChallenge}
  <ChallengeModal challenge={detailChallenge} onclose={() => detailChallenge = null} />
{/if}

<style>
  .challenges-page { max-width: 1000px; }
  .filters { display: flex; gap: 8px; margin-bottom: 20px; }

  .challenge-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(360px, 1fr)); gap: 16px; }

  .challenge-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; transition: all 0.2s; }
  .challenge-card:hover { border-color: #475569; }
  .challenge-card.completed { opacity: 0.7; border-color: #166534; }

  .ch-top { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
  .ch-category { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; letter-spacing: 0.5px; }
  .ch-badges { display: flex; align-items: center; gap: 6px; }
  .ch-stage { font-size: 10px; font-weight: 600; background: #7c3aed20; color: #a78bfa; padding: 2px 8px; border-radius: 4px; }
  .ch-difficulty { color: #f59e0b; font-size: 13px; letter-spacing: 1px; }
  .ch-title { font-size: 16px; font-weight: 600; color: #f1f5f9; margin-bottom: 8px; }
  .ch-desc { font-size: 13px; color: #94a3b8; line-height: 1.5; margin-bottom: 16px; }

  .ch-footer { display: flex; justify-content: space-between; align-items: center; }
  .ch-xp { font-size: 13px; font-weight: 600; color: #38bdf8; }
  .ch-actions { display: flex; align-items: center; gap: 6px; }
  .details-btn { background: #334155; border: none; color: #94a3b8; cursor: pointer; padding: 6px 10px; border-radius: 6px; font-size: 14px; transition: all 0.15s; }
  .details-btn:hover { background: #475569; color: #e2e8f0; }
  .complete-btn { padding: 8px 14px; background: #0ea5e9; color: #fff; border: none; border-radius: 8px; cursor: pointer; font-size: 12px; font-weight: 600; transition: background 0.15s; }
  .complete-btn:hover { background: #0284c7; }
  .completed-badge { color: #22c55e; font-weight: 600; font-size: 13px; }
</style>
