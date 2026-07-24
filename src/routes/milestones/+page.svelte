<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let milestones = $state([]);
  let challenges = $state([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      milestones = data.milestones || [];
      challenges = data.challenges || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  function milestoneProgress(m) {
    const total = m.challenge_ids.length;
    const done = m.challenge_ids.filter(id => challenges.some(c => c.id === id && c.completed)).length;
    return { done, total, percent: total > 0 ? Math.round((done / total) * 100) : 0 };
  }

  function isUnlocked(m) {
    return m.id === 1 || milestones.some(prev => prev.id === m.id - 1 && prev.completed);
  }
</script>

<div class="milestones-page">
  <header class="page-header">
    <h1>🏆 Kamienie milowe</h1>
    <p>Odblokowuj kolejne kamienie milowe, wykonując wyzwania</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="milestone-list">
      {#each milestones as m (m.id)}
        {@const prog = milestoneProgress(m)}
        {@const unlocked = isUnlocked(m)}
        <div class="milestone-card" class:completed={m.completed} class:locked={!unlocked}>
          <div class="ms-left">
            <div class="ms-icon">{m.completed ? '✅' : unlocked ? '🔓' : '🔒'}</div>
          </div>
          <div class="ms-content">
            <h3>{m.title}</h3>
            <p>{m.description}</p>
            <div class="ms-progress">
              <div class="ms-bar-bg">
                <div class="ms-bar-fill" style="width: {prog.percent}%"></div>
              </div>
              <span class="ms-count">{prog.done}/{prog.total} wyzwań</span>
            </div>
            <div class="ms-challenges">
              {#each m.challenge_ids as cid}
                {@const ch = challenges.find(c => c.id === cid)}
                {#if ch}
                  <span class="ms-ch-tag" class:done={ch.completed}>
                    {ch.completed ? '✓' : '○'} {ch.title}
                  </span>
                {/if}
              {/each}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .milestones-page {
    max-width: 800px;
  }

  .milestone-list {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .milestone-card {
    display: flex;
    gap: 16px;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 12px;
    padding: 20px;
    transition: all 0.2s;
  }

  .milestone-card.completed {
    border-color: #166534;
    background: #1a2e1a;
  }

  .milestone-card.locked {
    opacity: 0.5;
  }

  .ms-left {
    min-width: 40px;
  }

  .ms-icon {
    font-size: 28px;
  }

  .ms-content {
    flex: 1;
  }

  .ms-content h3 {
    font-size: 18px;
    font-weight: 600;
    color: #f1f5f9;
    margin-bottom: 4px;
  }

  .ms-content p {
    font-size: 13px;
    color: #64748b;
    margin-bottom: 12px;
  }

  .ms-progress {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }

  .ms-bar-bg {
    flex: 1;
    height: 8px;
    background: #334155;
    border-radius: 4px;
    overflow: hidden;
  }

  .ms-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #0ea5e9, #38bdf8);
    border-radius: 4px;
    transition: width 0.5s ease;
  }

  .ms-count {
    font-size: 12px;
    color: #94a3b8;
    white-space: nowrap;
  }

  .ms-challenges {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .ms-ch-tag {
    font-size: 11px;
    padding: 4px 8px;
    background: #334155;
    border-radius: 6px;
    color: #94a3b8;
  }

  .ms-ch-tag.done {
    background: #16653430;
    color: #22c55e;
  }
</style>
