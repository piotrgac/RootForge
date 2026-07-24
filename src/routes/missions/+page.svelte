<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

  let missions = $state([]);
  let challenges = $state([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      missions = data.missions || [];
      challenges = data.challenges || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function completeMission(id) {
    await invoke('complete_mission', { id });
    const m = missions.find(x => x.id === id);
    if (m) m.completed = true;
  }

  function missionProgress(m) {
    const done = m.steps.filter(sid => challenges.some(c => c.id === sid && c.completed)).length;
    const total = m.steps.length;
    return { done, total, percent: total > 0 ? Math.round(done / total * 100) : 0 };
  }

  function isUnlockable(m) {
    return m.steps.some(sid => challenges.some(c => c.id === sid));
  }
</script>

<div class="missions-page">
  <header class="page-header">
    <h1>🎯 Misje</h1>
    <p>Scenariusze łączące wiele wyzwań – tak jak prawdziwa praca administratora</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="mission-list">
      {#each missions as m (m.id)}
        {@const prog = missionProgress(m)}
        {@const cat = m.category ? getCategoryInfo(m.category) : null}
        <div class="mission-card card" class:completed={m.completed}>
          <div class="mission-header">
            <span class="mission-icon">{m.completed ? '✅' : m.icon}</span>
            <div class="mission-info">
              <h3>{m.title}</h3>
              <p class="mission-desc">{m.description}</p>
            </div>
          </div>

          <div class="mission-progress">
            <ProgressBar percent={prog.percent} height={6} color={prog.percent >= 100 ? '#22c55e' : undefined} />
            <span class="mission-count">{prog.done}/{prog.total} kroków</span>
          </div>

          <div class="mission-steps">
            {#each m.steps as stepId}
              {@const ch = challenges.find(c => c.id === stepId)}
              {#if ch}
                <span class="mission-step" class:done={ch.completed}>
                  {ch.completed ? '✅' : '⬜'} {ch.title}
                </span>
              {/if}
            {/each}
          </div>

          <div class="mission-footer">
            {#if cat}
              <span class="mission-cat" style="color: {cat.color}">{cat.name}</span>
            {/if}
            {#if !m.completed && prog.percent >= 100}
              <button class="mission-claim" onclick={() => completeMission(m.id)}>
                🎁 Odbierz {m.xp_reward} XP
              </button>
            {:else if !m.completed}
              <span class="mission-hint">Wykonaj wszystkie kroki, aby odebrać +{m.xp_reward} XP</span>
            {:else}
              <span class="mission-done">✓ Ukończono</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .missions-page { max-width: 800px; }

  .mission-list { display: flex; flex-direction: column; gap: 16px; }

  .mission-card { padding: 20px; }
  .mission-card.completed { border-color: #166534; opacity: 0.8; }

  .mission-header { display: flex; gap: 12px; margin-bottom: 12px; }
  .mission-icon { font-size: 28px; }
  .mission-info { flex: 1; }
  .mission-info h3 { font-size: 16px; font-weight: 600; color: #f1f5f9; margin-bottom: 4px; }
  .mission-desc { font-size: 13px; color: #64748b; line-height: 1.5; }

  .mission-progress { display: flex; align-items: center; gap: 10px; margin-bottom: 12px; }
  .mission-progress :global(.progress-bar-bg) { flex: 1; }
  .mission-count { font-size: 12px; color: #64748b; white-space: nowrap; }

  .mission-steps { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 12px; }
  .mission-step { font-size: 11px; padding: 3px 8px; background: #334155; border-radius: 4px; color: #94a3b8; }
  .mission-step.done { background: #16653430; color: #22c55e; }

  .mission-footer { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 8px; }
  .mission-cat { font-size: 11px; font-weight: 600; text-transform: uppercase; }

  .mission-claim { padding: 8px 16px; background: #22c55e; color: #fff; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; font-size: 13px; transition: background 0.15s; }
  .mission-claim:hover { background: #16a34a; }

  .mission-hint { font-size: 12px; color: #64748b; }
  .mission-done { font-size: 12px; color: #22c55e; font-weight: 600; }
</style>
