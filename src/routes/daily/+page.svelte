<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';

  let status = $state('loading'); // loading | ready | claimed | error
  let streak = $state(0);
  let xpReward = $state(20);
  let today = $state('');
  let data = $state(null);

  onMount(async () => {
    today = new Date().toISOString().slice(0, 10);
    try {
      const full = await invoke('get_dashboard_stats');
      data = full;
      // Pick a challenge based on today's date
      if (full.challenges?.length) {
        const startOfYear = new Date(new Date().getFullYear(), 0, 0).getTime();
        const dayOfYear = Math.floor((Date.now() - startOfYear) / 86400000);
        const idx = dayOfYear % full.challenges.length;
        data.dailyChallenge = full.challenges[idx];
      }
      // Check if already claimed
      if (full.last_daily_date === today) {
        status = 'claimed';
        streak = full.daily_streak || 0;
      } else {
        status = 'ready';
        streak = full.daily_streak || 0;
        xpReward = 20 + streak * 5;
      }
    } catch (e) {
      console.error(e);
      status = 'error';
    }
  });

  async function claimDaily() {
    try {
      const [result, newStreak, xp] = await invoke('claim_daily');
      streak = newStreak;
      xpReward = xp;
      status = result === 'claimed' ? 'claimed' : 'already_claimed';
    } catch (e) {
      console.error(e);
    }
  }

  function difficultyStars(n) {
    return '★'.repeat(n) + '☆'.repeat(5 - n);
  }
</script>

<div class="daily-page">
  <header class="page-header">
    <h1>📅 Daily Challenge</h1>
    <p>Codzienne wyzwanie – zbuduj nawyk i zdobywaj bonusowe XP</p>
  </header>

  {#if status === 'loading'}
    <div class="loading">Ładowanie...</div>

  {:else if status === 'claimed'}
    <div class="claimed-card card">
      <div class="claimed-icon">✅</div>
      <h2>Dzisiejsze wyzwanie odebrane!</h2>
      <p>Streak: <strong>{streak} dni</strong> z rzędu</p>
      <p class="claimed-note">Wróć jutro po kolejną dawkę wiedzy. 🔥</p>
      <a href="/challenges" class="btn-link">📖 Przeglądaj wyzwania</a>
    </div>

  {:else if status === 'ready' && data?.dailyChallenge}
    {@const ch = data.dailyChallenge}
    {@const cat = getCategoryInfo(ch.category)}
    <div class="daily-card card">
      <div class="daily-streak-row">
        <span class="streak-badge">🔥 Streak: {streak} dni</span>
        <span class="reward-badge">🎁 +{xpReward} XP</span>
      </div>

      <div class="challenge-badge" style="background: {cat.color}20; color: {cat.color}">
        {cat.name}
      </div>
      <h2 class="daily-title">{ch.title}</h2>
      <p class="daily-desc">{ch.description}</p>
      <div class="daily-meta">
        <span class="daily-diff">{difficultyStars(ch.difficulty)}</span>
        <span class="daily-stage">{ch.stage ? `Etap ${ch.stage}` : ''}</span>
      </div>

      <div class="daily-actions">
        <a href="/challenges" class="btn-primary">📖 Idź do wyzwania</a>
        <button class="btn-secondary" onclick={claimDaily}>
          ✅ Odbierz nagrodę dzienną
        </button>
      </div>
    </div>

  {:else if status === 'ready'}
    <div class="empty-card card">
      <p>Brak wyzwań w systemie. Dodaj je najpierw!</p>
    </div>
  {/if}
</div>

<style>
  .daily-page { max-width: 600px; }

  .claimed-card { text-align: center; padding: 40px; }
  .claimed-icon { font-size: 64px; margin-bottom: 16px; }
  .claimed-card h2 { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .claimed-card p { color: #94a3b8; font-size: 14px; margin-bottom: 4px; }
  .claimed-note { margin-bottom: 24px; }

  .daily-card { padding: 28px; }
  .daily-streak-row { display: flex; gap: 8px; margin-bottom: 16px; }
  .streak-badge, .reward-badge { padding: 4px 10px; border-radius: 6px; font-size: 12px; font-weight: 600; }
  .streak-badge { background: #f59e0b20; color: #fbbf24; }
  .reward-badge { background: #22c55e20; color: #22c55e; }

  .challenge-badge { display: inline-block; font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; margin-bottom: 12px; }
  .daily-title { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .daily-desc { font-size: 14px; color: #94a3b8; line-height: 1.5; margin-bottom: 12px; }
  .daily-meta { display: flex; gap: 8px; margin-bottom: 20px; }
  .daily-diff { color: #f59e0b; font-size: 13px; letter-spacing: 1px; }
  .daily-stage { font-size: 11px; font-weight: 600; background: #7c3aed20; color: #a78bfa; padding: 2px 8px; border-radius: 4px; }

  .daily-actions { display: flex; gap: 8px; }
  .btn-primary { padding: 10px 20px; background: #0ea5e9; color: #fff; border-radius: 8px; text-decoration: none; font-size: 13px; font-weight: 600; }
  .btn-primary:hover { background: #0284c7; }
  .btn-secondary { padding: 10px 20px; background: #1e293b; border: 1px solid #334155; color: #94a3b8; border-radius: 8px; cursor: pointer; font-size: 13px; font-weight: 600; }
  .btn-secondary:hover { background: #334155; color: #e2e8f0; }
  .empty-card { padding: 40px; text-align: center; color: #64748b; }
</style>
