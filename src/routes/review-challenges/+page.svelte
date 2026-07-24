<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let data = $state(null);
  let loading = $state(true);
  let reviewedToday = $state([]);

  const REVIEW_INTERVALS = [7, 14, 30];

  onMount(async () => {
    try {
      data = await invoke('get_dashboard_stats');
      const saved = localStorage.getItem('reviewed-challenges');
      if (saved) reviewedToday = JSON.parse(saved);
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  let dueChallenges = $derived.by(() => {
    if (!data?.challenges) return [];
    const today = new Date().toISOString().slice(0, 10);
    return data.challenges.filter(c => {
      if (!c.completed) return false;
      if (reviewedToday.includes(c.id)) return false;
      if (!c.last_reviewed) return c.completed; // completed but never reviewed
      const last = new Date(c.last_reviewed);
      const days = Math.floor((Date.now() - last.getTime()) / 86400000);
      const interval = REVIEW_INTERVALS[c.difficulty > 3 ? 2 : c.difficulty > 1 ? 1 : 0];
      return days >= interval;
    });
  });

  let todayReviewed = $derived(reviewedToday.length);

  function markReviewed(id) {
    reviewedToday = [...reviewedToday, id];
    localStorage.setItem('reviewed-challenges', JSON.stringify(reviewedToday));
    const ch = data.challenges.find(c => c.id === id);
    if (ch) ch.last_reviewed = new Date().toISOString().slice(0, 10);
  }

  function resetAll() {
    reviewedToday = [];
    localStorage.removeItem('reviewed-challenges');
  }

  function difficultyStars(n) {
    return '★'.repeat(n) + '☆'.repeat(5 - n);
  }

  let totalCompleted = $derived(data?.challenges?.filter(c => c.completed).length || 0);
</script>

<div class="review-ch-page">
  <header class="page-header">
    <h1>🔄 Powtórki wyzwań</h1>
    <p>Challenge'e do odświeżenia – krzywa zapominania Ebbinghausa</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="rc-summary card">
      <div class="rc-summary-row">
        <span class="rc-stat">📚 Ukończone wyzwania: <strong>{totalCompleted}</strong></span>
        <span class="rc-stat">🔄 Dzisiaj powtórzone: <strong>{todayReviewed}</strong></span>
        <span class="rc-stat">⏳ Do powtórki: <strong>{dueChallenges.length}</strong></span>
      </div>
      {#if dueChallenges.length === 0 && totalCompleted > 0}
        <div class="rc-all-done">
          <span class="rc-check-icon">✅</span>
          <span>Wszystkie wyzwania powtórzone! Wróć jutro.</span>
        </div>
      {/if}
      {#if todayReviewed > 0}
        <button class="rc-reset" onclick={resetAll}>🔄 Resetuj dzisiejsze powtórki</button>
      {/if}
    </div>

    {#if dueChallenges.length > 0}
      <div class="rc-list">
        {#each dueChallenges as ch}
          <div class="rc-item card">
            <div class="rc-item-top">
              <h3>{ch.title}</h3>
              <span class="rc-diff">{difficultyStars(ch.difficulty)}</span>
            </div>
            <p class="rc-desc">{ch.description}</p>
            <div class="rc-meta">
              <span class="rc-category">{ch.category}</span>
              <span class="rc-interval">
                {#if ch.last_reviewed}
                  Ostatnia powtórka: {ch.last_reviewed}
                {:else}
                  Nigdy nie powtórzone
                {/if}
              </span>
            </div>
            <div class="rc-actions">
              <a href="/challenges" class="rc-link">📖 Otwórz wyzwanie</a>
              <button class="rc-done" onclick={() => markReviewed(ch.id)}>✅ Powtórzone</button>
            </div>
          </div>
        {/each}
      </div>
    {:else if totalCompleted === 0}
      <div class="rc-empty card">
        <p>Nie masz jeszcze ukończonych wyzwań. Wróć po ukończeniu pierwszego!</p>
        <a href="/challenges" class="rc-empty-link">📖 Idź do wyzwań</a>
      </div>
    {/if}
  {/if}
</div>

<style>
  .review-ch-page { max-width: 700px; }

  .rc-summary { padding: 16px; margin-bottom: 16px; }
  .rc-summary-row { display: flex; flex-wrap: wrap; gap: 12px; justify-content: space-between; }
  .rc-stat { font-size: 13px; color: #94a3b8; }
  .rc-stat strong { color: #e2e8f0; }
  .rc-all-done { text-align: center; padding: 12px; margin-top: 8px; background: #22c55e10; border: 1px solid #22c55e40; border-radius: 8px; font-size: 14px; color: #22c55e; }
  .rc-check-icon { font-size: 20px; margin-right: 8px; }
  .rc-reset { display: block; margin: 8px auto 0; background: none; border: 1px dashed #334155; color: #64748b; padding: 6px 14px; border-radius: 6px; cursor: pointer; font-size: 12px; }
  .rc-reset:hover { border-color: #475569; color: #94a3b8; }

  .rc-list { display: flex; flex-direction: column; gap: 10px; }
  .rc-item { padding: 16px; }
  .rc-item-top { display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px; }
  .rc-item-top h3 { font-size: 15px; font-weight: 600; color: #f1f5f9; }
  .rc-diff { color: #f59e0b; font-size: 12px; letter-spacing: 1px; }
  .rc-desc { font-size: 13px; color: #64748b; line-height: 1.5; margin-bottom: 8px; }
  .rc-meta { display: flex; gap: 12px; font-size: 11px; color: #64748b; margin-bottom: 10px; }
  .rc-category { padding: 2px 8px; background: #334155; border-radius: 4px; }
  .rc-interval { font-style: italic; }
  .rc-actions { display: flex; gap: 8px; }
  .rc-link { padding: 6px 14px; background: #0ea5e9; color: #fff; border-radius: 6px; text-decoration: none; font-size: 12px; font-weight: 600; }
  .rc-link:hover { background: #0284c7; }
  .rc-done { padding: 6px 14px; background: #22c55e; color: #fff; border: none; border-radius: 6px; font-size: 12px; font-weight: 600; cursor: pointer; }
  .rc-done:hover { background: #16a34a; }

  .rc-empty { text-align: center; padding: 40px; }
  .rc-empty p { color: #64748b; margin-bottom: 16px; font-size: 14px; }
  .rc-empty-link { display: inline-block; padding: 10px 20px; background: #0ea5e9; color: #fff; border-radius: 8px; text-decoration: none; font-weight: 600; font-size: 13px; }
</style>
