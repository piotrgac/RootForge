<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryStats } from '$lib/categories.js';

  let data = $state(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      data = await invoke('get_dashboard_stats');
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  let challenges = $derived(data?.challenges || []);
  let completed = $derived(challenges.filter(c => c.completed).length);
  let total = $derived(challenges.length);
  let progress = $derived(total > 0 ? Math.round((completed / total) * 100) : 0);
  let projectsDone = $derived(data?.projects?.filter(p => p.completed).length || 0);
  let projectsTotal = $derived(data?.projects?.length || 0);
  let quizzesDone = $derived(data?.quiz_results?.length || 0);
  let catStats = $derived(getCategoryStats(challenges));
  let sessions = $derived(data?.sessions || []);
  let dailyGoal = $derived(data?.daily_goal_minutes || 30);
  let todaySessions = $derived.by(() => {
    const today = new Date().toISOString().slice(0, 10);
    return sessions.filter(s => s.date === today);
  });
  let todayMinutes = $derived(todaySessions.reduce((acc, s) => acc + s.duration_minutes, 0));
</script>

<div class="dashboard">
  <header class="page-header">
    <h1>Dashboard</h1>
    <p>Twoja ścieżka do zostania Linux System Administratorem</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="study-plan">
      <h2>📅 Plan na dziś</h2>
      <div class="plan-card">
        <div class="plan-progress">
          <div class="plan-ring">
            <svg viewBox="0 0 36 36" class="plan-svg">
              <path class="plan-bg" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
              <path class="plan-fill" stroke-dasharray="{Math.min(todayMinutes / dailyGoal, 1) * 100}, 100" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
            </svg>
            <span class="plan-pct">{Math.round(Math.min(todayMinutes / dailyGoal, 1) * 100)}%</span>
          </div>
        </div>
        <div class="plan-info">
          <span class="plan-minutes">{todayMinutes}/{dailyGoal} min</span>
          {#if todayMinutes === 0}
            <span class="plan-tip">🌅 Pora na dzienną sesję! Otwórz <a href="/challenges">Wyzwania</a></span>
          {:else if todayMinutes < dailyGoal}
            <span class="plan-tip">⏳ Jeszcze {dailyGoal - todayMinutes} min do celu. Spróbuj <a href="/quiz">Quiz</a>!</span>
          {:else}
            <span class="plan-tip">🎉 Cel dzienny osiągnięty! Świetna robota.</span>
          {/if}
        </div>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card level">
        <div class="stat-icon">⭐</div>
        <div class="stat-info">
          <span class="stat-value">{data?.level || 1}</span>
          <span class="stat-label">Poziom</span>
        </div>
      </div>
      <div class="stat-card xp">
        <div class="stat-icon">⚡</div>
        <div class="stat-info">
          <span class="stat-value">{data?.xp || 0} XP</span>
          <span class="stat-label">{100 - ((data?.xp || 0) % 100)} XP do następnego poziomu</span>
        </div>
      </div>
      <div class="stat-card challenges">
        <div class="stat-icon">🎯</div>
        <div class="stat-info">
          <span class="stat-value">{completed}/{total}</span>
          <span class="stat-label">Wyzwania ukończone</span>
        </div>
      </div>
      <div class="stat-card projects">
        <div class="stat-icon">💻</div>
        <div class="stat-info">
          <span class="stat-value">{projectsDone}/{projectsTotal}</span>
          <span class="stat-label">Projekty ukończone</span>
        </div>
      </div>
      <div class="stat-card quizzes">
        <div class="stat-icon">🧠</div>
        <div class="stat-info">
          <span class="stat-value">{quizzesDone}</span>
          <span class="stat-label">Quizy rozwiązane</span>
        </div>
      </div>
    </div>

    <div class="progress-section">
      <h2>Ogólny postęp</h2>
      <div class="progress-bar-container">
        <div class="progress-bar" style="width: {progress}%"></div>
        <span class="progress-text">{progress}%</span>
      </div>
    </div>

    <div class="category-breakdown">
      <h2>Postęp według kategorii</h2>
      <div class="category-grid">
        {#each catStats as cat}
          <div class="category-card">
            <div class="cat-header">
              <span class="cat-dot" style="background: {cat.color}"></span>
              <span class="cat-name">{cat.name}</span>
            </div>
            <div class="cat-bar-bg">
              <div class="cat-bar-fill" style="width: {cat.percent}%; background: {cat.color}"></div>
            </div>
            <span class="cat-count">{cat.done}/{cat.total}</span>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .dashboard {
    max-width: 1000px;
  }

  .page-header h1 {
    font-size: 28px;
    font-weight: 700;
    color: #f1f5f9;
    margin-bottom: 4px;
  }

  .page-header p {
    color: #64748b;
    margin-bottom: 24px;
  }

  .loading {
    color: #64748b;
    font-size: 16px;
  }

  .study-plan { margin-bottom: 28px; }
  .study-plan h2 { font-size: 18px; font-weight: 600; margin-bottom: 12px; color: #e2e8f0; }
  .plan-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; display: flex; align-items: center; gap: 20px; }
  .plan-ring { width: 64px; height: 64px; position: relative; }
  .plan-svg { width: 64px; height: 64px; transform: rotate(-90deg); }
  .plan-bg { fill: none; stroke: #334155; stroke-width: 3; }
  .plan-fill { fill: none; stroke: #0ea5e9; stroke-width: 3; stroke-linecap: round; transition: stroke-dasharray 0.5s; }
  .plan-pct { position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 700; color: #38bdf8; }
  .plan-info { flex: 1; }
  .plan-minutes { display: block; font-size: 20px; font-weight: 700; color: #f1f5f9; margin-bottom: 4px; }
  .plan-tip { font-size: 13px; color: #94a3b8; }
  .plan-tip a { color: #38bdf8; text-decoration: none; font-weight: 600; }
  .plan-tip a:hover { text-decoration: underline; }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 16px;
    margin-bottom: 28px;
  }

  .stat-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 12px;
    padding: 20px;
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .stat-icon {
    font-size: 32px;
  }

  .stat-value {
    display: block;
    font-size: 24px;
    font-weight: 700;
    color: #f1f5f9;
  }

  .stat-label {
    font-size: 12px;
    color: #64748b;
  }

  .progress-section {
    margin-bottom: 28px;
  }

  .progress-section h2 {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 12px;
    color: #e2e8f0;
  }

  .progress-bar-container {
    background: #1e293b;
    border-radius: 8px;
    height: 28px;
    position: relative;
    overflow: hidden;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #0ea5e9, #38bdf8);
    border-radius: 8px;
    transition: width 0.5s ease;
  }

  .progress-text {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    font-weight: 600;
    color: #f1f5f9;
  }

  .category-breakdown h2 {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 12px;
    color: #e2e8f0;
  }

  .category-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 12px;
  }

  .category-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 10px;
    padding: 16px;
  }

  .cat-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .cat-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
  }

  .cat-name {
    font-weight: 600;
    color: #e2e8f0;
    font-size: 14px;
  }

  .cat-bar-bg {
    height: 8px;
    background: #334155;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 6px;
  }

  .cat-bar-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.5s ease;
  }

  .cat-count {
    font-size: 12px;
    color: #64748b;
  }
</style>
