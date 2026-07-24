<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryStats, getCategoryInfo } from '$lib/categories';

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
  let stageProgress = $derived.by(() => {
    const byStage = {};
    for (const ch of challenges) {
      const s = ch.stage || 0;
      if (s === 0) continue;
      if (!byStage[s]) byStage[s] = { stage: s, total: 0, done: 0 };
      byStage[s].total++;
      if (ch.completed) byStage[s].done++;
    }
    return Object.values(byStage).sort((a, b) => a.stage - b.stage);
  });
  let sessions = $derived(data?.sessions || []);
  let dailyGoal = $derived(data?.daily_goal_minutes || 30);
  let todaySessions = $derived.by(() => {
    const today = new Date().toISOString().slice(0, 10);
    return sessions.filter(s => s.date === today);
  });
  let todayMinutes = $derived(todaySessions.reduce((acc, s) => acc + s.duration_minutes, 0));

  function computeCategoryPerformance(d) {
    const quizzes = d.quizzes || [];
    const results = d.quiz_results || [];
    const perf = {};
    for (const r of results) {
      const q = quizzes.find(x => x.id === r.quiz_id);
      const cat = q ? q.category : 'unknown';
      if (!perf[cat]) perf[cat] = { name: cat, color: '#38bdf8', total: 0, correct: 0 };
      perf[cat].total++;
      if (r.correct) perf[cat].correct++;
    }
    return Object.entries(perf).map(([key, c]) => {
      const inf = getCategoryInfo(key);
      return { ...c, name: inf.name, color: inf.color, accuracy: c.total > 0 ? Math.round(c.correct / c.total * 100) : 0 };
    });
  }
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

    {#if data?.quiz_results?.length}
      {@const catPerf = computeCategoryPerformance(data)}
      {@const weakAreas = catPerf.filter(c => c.accuracy < 60).sort((a, b) => a.accuracy - b.accuracy)}
      {#if weakAreas.length > 0}
        <div class="weak-section">
          <h2>⚠️ Obszary do poprawy</h2>
          <p class="weak-subtitle">Najsłabiej wypadasz w tych kategoriach – skup się na nich:</p>
          <div class="weak-grid">
            {#each weakAreas as area}
              <div class="weak-card">
                <div class="weak-header">
                  <span class="weak-dot" style="background: {area.color}"></span>
                  <span class="weak-name">{area.name}</span>
                  <span class="weak-pct">{area.accuracy}%</span>
                </div>
                <div class="weak-bar-bg">
                  <div class="weak-bar-fill" style="width: {area.accuracy}%; background: {area.color}"></div>
                </div>
                <div class="weak-count">{area.correct}/{area.total} poprawnych</div>
                <div class="weak-actions">
                  <a href="/challenges" class="weak-link">📖 Wyzwania</a>
                  <a href="/quiz" class="weak-link">🧠 Quiz</a>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {:else if catPerf.length > 0}
        <div class="weak-section">
          <h2>🎉 Wszystkie kategorie opanowane!</h2>
          <p class="weak-subtitle">Świetna robota. Spróbuj Speed Challenge, aby utrwalić wiedzę.</p>
          <a href="/speed" class="btn-primary">⚡ Speed Challenge</a>
        </div>
      {/if}
    {/if}

    {#if data?.achievements?.length}
      <div class="achievements-section">
        <h2>🏆 Osiągnięcia</h2>
        <div class="achievement-grid">
          {#each data.achievements as ach}
            <div class="achievement" class:unlocked={ach.unlocked} class:locked={!ach.unlocked}>
              <span class="ach-icon">{ach.unlocked ? ach.icon : '🔒'}</span>
              <div class="ach-info">
                <span class="ach-title">{ach.title}</span>
                <span class="ach-desc">{ach.description}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <div class="progress-section">
      <h2>Ogólny postęp</h2>
      <div class="progress-bar-container">
        <div class="progress-bar" style="width: {progress}%"></div>
        <span class="progress-text">{progress}%</span>
      </div>
    </div>

    {#if stageProgress.length > 0}
      <div class="stage-section">
        <h2>Postęp według etapów</h2>
        <div class="stage-grid">
          {#each stageProgress as sp}
            <div class="stage-card">
              <div class="stage-header">
                <span class="stage-badge">Etap {sp.stage}</span>
                <span class="stage-pct">{Math.round(sp.done / sp.total * 100)}%</span>
              </div>
              <div class="stage-bar-bg">
                <div class="stage-bar-fill" style="width: {sp.done / sp.total * 100}%"></div>
              </div>
              <span class="stage-count">{sp.done}/{sp.total} wyzwań</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

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

  .weak-section { margin-bottom: 28px; }
  .weak-section h2 { font-size: 18px; font-weight: 600; margin-bottom: 4px; color: #e2e8f0; }
  .weak-subtitle { font-size: 13px; color: #64748b; margin-bottom: 12px; }
  .weak-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 12px; }
  .weak-card { background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 14px; }
  .weak-header { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
  .weak-dot { width: 10px; height: 10px; border-radius: 50%; }
  .weak-name { flex: 1; font-weight: 600; color: #e2e8f0; font-size: 13px; }
  .weak-pct { font-size: 16px; font-weight: 700; color: #ef4444; }
  .weak-bar-bg { height: 6px; background: #334155; border-radius: 3px; overflow: hidden; margin-bottom: 6px; }
  .weak-bar-fill { height: 100%; border-radius: 3px; transition: width 0.3s; }
  .weak-count { font-size: 11px; color: #64748b; margin-bottom: 8px; }
  .weak-actions { display: flex; gap: 6px; }
  .weak-link { font-size: 11px; font-weight: 600; color: #38bdf8; text-decoration: none; padding: 3px 8px; background: #0ea5e920; border-radius: 4px; }
  .weak-link:hover { background: #0ea5e930; }

  .achievements-section { margin-bottom: 28px; }
  .achievements-section h2 { font-size: 18px; font-weight: 600; margin-bottom: 12px; color: #e2e8f0; }
  .achievement-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 8px; }
  .achievement { display: flex; align-items: center; gap: 10px; padding: 10px 14px; background: #1e293b; border: 1px solid #334155; border-radius: 10px; transition: all 0.2s; }
  .achievement.unlocked { border-color: #7c3aed; background: #7c3aed10; }
  .achievement.locked { opacity: 0.5; }
  .ach-icon { font-size: 24px; min-width: 32px; text-align: center; }
  .ach-info { flex: 1; }
  .ach-title { display: block; font-size: 13px; font-weight: 600; color: #f1f5f9; }
  .ach-desc { display: block; font-size: 11px; color: #64748b; }

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

  .stage-section { margin-bottom: 28px; }
  .stage-section h2 { font-size: 18px; font-weight: 600; margin-bottom: 12px; color: #e2e8f0; }
  .stage-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 12px; }
  .stage-card { background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 16px; }
  .stage-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }
  .stage-badge { font-size: 12px; font-weight: 700; background: #7c3aed20; color: #a78bfa; padding: 3px 10px; border-radius: 5px; }
  .stage-pct { font-size: 16px; font-weight: 700; color: #38bdf8; }
  .stage-bar-bg { height: 8px; background: #334155; border-radius: 4px; overflow: hidden; margin-bottom: 6px; }
  .stage-bar-fill { height: 100%; background: linear-gradient(90deg, #7c3aed, #a78bfa); border-radius: 4px; transition: width 0.5s ease; }
  .stage-count { font-size: 12px; color: #64748b; }

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
