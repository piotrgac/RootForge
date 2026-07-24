<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { Chart, registerables } from 'chart.js';
  import { getCategoryStats } from '$lib/categories';

  Chart.register(...registerables);

  let data = $state(null);
  let loading = $state(true);
  let chartInstances = [];
  let currentStreak = $state(0);
  let totalHours = $state(0);
  let todayMinutes = $state(0);
  let dailyGoal = $state(30);
  let stageStats = $derived.by(() => {
    if (!data?.challenges) return [];
    const map = {};
    for (const ch of data.challenges) {
      const s = ch.stage || 0;
      if (s === 0) continue;
      if (!map[s]) map[s] = { stage: s, total: 0, done: 0 };
      map[s].total++;
      if (ch.completed) map[s].done++;
    }
    return Object.values(map).sort((a, b) => a.stage - b.stage);
  });

  function computeStreak(sessions) {
    if (!sessions.length) return 0;
    const daySet = new Set(sessions.map(s => s.date));
    let streak = 0;
    const today = new Date();
    for (let i = 0; i < 365; i++) {
      const d = new Date(today);
      d.setDate(d.getDate() - i);
      const key = d.toISOString().slice(0, 10);
      if (daySet.has(key)) {
        streak++;
      } else {
        break;
      }
    }
    return streak;
  }

  function computeDailyMinutes(sessions) {
    const map = {};
    for (const s of sessions) {
      map[s.date] = (map[s.date] || 0) + s.duration_minutes;
    }
    return map;
  }

  function getLastNDays(n) {
    const result = [];
    const today = new Date();
    for (let i = n - 1; i >= 0; i--) {
      const d = new Date(today);
      d.setDate(d.getDate() - i);
      result.push(d.toISOString().slice(0, 10));
    }
    return result;
  }

  onMount(async () => {
    try {
      data = await invoke('get_dashboard_stats');
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  $effect(() => {
    if (loading || !data) return;
    for (const c of chartInstances) c.destroy();
    chartInstances = [];

    const cats = getCategoryStats(data.challenges);

    // --- Category doughnut ---
    const catCanvas = document.getElementById('categoryChart');
    if (catCanvas) {
      const ctx = /** @type {HTMLCanvasElement} */ (catCanvas).getContext('2d');
      const chart = new Chart(ctx, {
        type: 'doughnut',
        data: {
          labels: cats.map(c => c.name),
          datasets: [{
            data: cats.map(c => c.total),
            backgroundColor: cats.map(c => c.color),
            borderColor: '#0f172a',
            borderWidth: 2,
          }],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: { position: 'bottom', labels: { color: '#94a3b8', padding: 16 } },
            title: { display: true, text: 'Wyzwania według kategorii', color: '#e2e8f0', font: { size: 16 } },
          },
        },
      });
      chartInstances.push(chart);
    }

    // --- Progress bar chart ---
    const progCanvas = document.getElementById('progressChart');
    if (progCanvas) {
      const ctx = /** @type {HTMLCanvasElement} */ (progCanvas).getContext('2d');
      const chart = new Chart(ctx, {
        type: 'bar',
        data: {
          labels: cats.map(c => c.name),
          datasets: [
            {
              label: 'Ukończone',
              data: cats.map(c => c.done),
              backgroundColor: cats.map(c => c.color),
              borderRadius: 4,
            },
            {
              label: 'Pozostało',
              data: cats.map(c => c.total - c.done),
              backgroundColor: '#334155',
              borderRadius: 4,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: {
            x: { ticks: { color: '#94a3b8' }, grid: { color: '#1e293b' } },
            y: { ticks: { color: '#94a3b8', stepSize: 1 }, grid: { color: '#1e293b' }, beginAtZero: true },
          },
          plugins: {
            legend: { position: 'bottom', labels: { color: '#94a3b8', padding: 16 } },
            title: { display: true, text: 'Postęp w kategoriach', color: '#e2e8f0', font: { size: 16 } },
          },
        },
      });
      chartInstances.push(chart);
    }

    // --- Stage charts ---
    const stageMap = {};
    for (const ch of data.challenges) {
      const s = ch.stage || 0;
      if (s === 0) continue;
      if (!stageMap[s]) stageMap[s] = { stage: s, total: 0, done: 0, label: `Etap ${s}` };
      stageMap[s].total++;
      if (ch.completed) stageMap[s].done++;
    }
    const stageStats = Object.values(stageMap).sort((a, b) => a.stage - b.stage);
    const stageColors = ['#7c3aed', '#a78bfa', '#c4b5fd', '#8b5cf6', '#6d28d9'];

    const stageCanvas = document.getElementById('stageChart');
    if (stageCanvas && stageStats.length > 0) {
      const ctx = /** @type {HTMLCanvasElement} */ (stageCanvas).getContext('2d');
      const chart = new Chart(ctx, {
        type: 'bar',
        data: {
          labels: stageStats.map(s => s.label),
          datasets: [
            {
              label: 'Ukończone',
              data: stageStats.map(s => s.done),
              backgroundColor: stageStats.map((_, i) => stageColors[i % stageColors.length]),
              borderRadius: 4,
            },
            {
              label: 'Pozostało',
              data: stageStats.map(s => s.total - s.done),
              backgroundColor: '#334155',
              borderRadius: 4,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: {
            x: { ticks: { color: '#94a3b8' }, grid: { color: '#1e293b' } },
            y: { ticks: { color: '#94a3b8', stepSize: 1 }, grid: { color: '#1e293b' }, beginAtZero: true },
          },
          plugins: {
            legend: { position: 'bottom', labels: { color: '#94a3b8', padding: 16 } },
            title: { display: true, text: 'Postęp według etapów', color: '#e2e8f0', font: { size: 16 } },
          },
        },
      });
      chartInstances.push(chart);
    }

    // --- Study time bar chart (last 14 days) ---
    const sessions = data.sessions || [];
    dailyGoal = data.daily_goal_minutes || 30;

    currentStreak = computeStreak(sessions);
    const dayMap = computeDailyMinutes(sessions);
    totalHours = Math.round(sessions.reduce((a, s) => a + s.duration_minutes, 0) / 60 * 10) / 10;
    const todayKey = new Date().toISOString().slice(0, 10);
    todayMinutes = dayMap[todayKey] || 0;

    const last14 = getLastNDays(14);
    const dayLabels = last14.map(d => {
      const parts = d.split('-');
      return `${parts[2]}.${parts[1]}`;
    });
    const dayData = last14.map(d => dayMap[d] || 0);

    const studyCanvas = document.getElementById('studyChart');
    if (studyCanvas) {
      const ctx = /** @type {HTMLCanvasElement} */ (studyCanvas).getContext('2d');
      const chart = new Chart(ctx, {
        type: 'bar',
        data: {
          labels: dayLabels,
          datasets: [{
            label: 'Minuty nauki',
            data: dayData,
            backgroundColor: dayData.map(v => v >= dailyGoal ? '#22c55e' : '#0ea5e9'),
            borderRadius: 4,
          }],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: {
            x: { ticks: { color: '#94a3b8', maxRotation: 0 }, grid: { color: '#1e293b' } },
            y: { ticks: { color: '#94a3b8' }, grid: { color: '#1e293b' }, beginAtZero: true },
          },
          plugins: {
            legend: { display: false },
            title: { display: true, text: 'Czas nauki – ostatnie 14 dni', color: '#e2e8f0', font: { size: 16 } },
          },
        },
      });
      chartInstances.push(chart);
    }

    return () => {
      for (const c of chartInstances) c.destroy();
      chartInstances = [];
    };
  });
</script>

<div class="progress-page">
  <header class="page-header">
    <h1>📈 Postępy</h1>
    <p>Wizualizacja Twoich osiągnięć i postępów w nauce</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <!-- Streak & summary cards -->
    <div class="summary-row">
      <div class="summary-card">
        <span class="summary-icon">🔥</span>
        <span class="summary-val">{currentStreak}</span>
        <span class="summary-label">dni z rzędu</span>
      </div>
      <div class="summary-card">
        <span class="summary-icon">⏱️</span>
        <span class="summary-val">{totalHours}h</span>
        <span class="summary-label">łączny czas nauki</span>
      </div>
      <div class="summary-card">
        <span class="summary-icon">📅</span>
        <span class="summary-val">{todayMinutes} / {dailyGoal} min</span>
        <span class="summary-label">dzisiaj</span>
      </div>
      <div class="summary-card" class:goal-met={todayMinutes >= dailyGoal}>
        <span class="summary-icon">{todayMinutes >= dailyGoal ? '✅' : '🎯'}</span>
        <span class="summary-val">{todayMinutes >= dailyGoal ? 'Cel osiągnięty' : 'Do celu: ' + (dailyGoal - todayMinutes) + ' min'}</span>
        <span class="summary-label">dzienny cel</span>
      </div>
    </div>

    <div class="charts-grid">
      <div class="chart-container">
        <canvas id="categoryChart"></canvas>
      </div>
      <div class="chart-container">
        <canvas id="progressChart"></canvas>
      </div>
    </div>

    <div class="chart-full">
      <div class="chart-container">
        <canvas id="studyChart"></canvas>
      </div>
    </div>

    {#if stageStats.length > 0}
      <div class="chart-full">
        <div class="chart-container">
          <canvas id="stageChart"></canvas>
        </div>
      </div>
    {/if}

    <div class="stats-table">
      <h2>Szczegółowe statystyki</h2>
      <div class="table-grid">
        <div class="table-row header">
          <span>Kategoria</span>
          <span>Ukończone</span>
          <span>Wszystkie</span>
          <span>Postęp</span>
        </div>
        {#each getCategoryStats(data.challenges) as cat}
          <div class="table-row">
            <span class="cat-label">
              <span class="cat-dot" style="background: {cat.color}"></span>
              {cat.name}
            </span>
            <span>{cat.done}</span>
            <span>{cat.total}</span>
            <span>
              <div class="tbl-bar-bg">
                <div class="tbl-bar-fill" style="width: {cat.percent}%; background: {cat.color}"></div>
              </div>
            </span>
          </div>
        {/each}
      </div>
    </div>

    {#if stageStats.length > 0}
      <div class="stats-table" style="margin-top: 20px;">
        <h2>Postęp według etapów</h2>
        <div class="table-grid">
          <div class="table-row header">
            <span>Etap</span>
            <span>Ukończone</span>
            <span>Wszystkie</span>
            <span>Postęp</span>
          </div>
          {#each stageStats as sp}
            <div class="table-row">
              <span class="cat-label">
                <span class="cat-dot" style="background: #7c3aed"></span>
                Etap {sp.stage}
              </span>
              <span>{sp.done}</span>
              <span>{sp.total}</span>
              <span>
                <div class="tbl-bar-bg">
                  <div class="tbl-bar-fill" style="width: {sp.done / sp.total * 100}%; background: #7c3aed"></div>
                </div>
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .progress-page { max-width: 1000px; }
  .summary-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 24px; }
  .summary-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 16px; text-align: center; }
  .summary-card.goal-met { border-color: #166534; }
  .summary-icon { font-size: 24px; display: block; margin-bottom: 4px; }
  .summary-val { display: block; font-size: 22px; font-weight: 800; color: #f1f5f9; }
  .summary-label { font-size: 12px; color: #64748b; margin-top: 2px; }

  .charts-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 20px; }
  .chart-full { margin-bottom: 32px; }
  .chart-container { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; height: 320px; }

  .stats-table h2 { font-size: 18px; font-weight: 600; color: #e2e8f0; margin-bottom: 12px; }
  .table-grid { background: #1e293b; border: 1px solid #334155; border-radius: 12px; overflow: hidden; }
  .table-row { display: grid; grid-template-columns: 1fr 100px 100px 1fr; gap: 12px; padding: 12px 20px; align-items: center; font-size: 14px; border-bottom: 1px solid #334155; }
  .table-row:last-child { border-bottom: none; }
  .table-row.header { color: #64748b; font-weight: 600; font-size: 12px; text-transform: uppercase; }
  .table-row:not(.header) { color: #e2e8f0; }
  .cat-label { display: flex; align-items: center; gap: 8px; }
  .cat-dot { width: 10px; height: 10px; border-radius: 50%; }
  .tbl-bar-bg { height: 8px; background: #334155; border-radius: 4px; overflow: hidden; }
  .tbl-bar-fill { height: 100%; border-radius: 4px; }

  @media (max-width: 768px) {
    .charts-grid { grid-template-columns: 1fr; }
    .summary-row { grid-template-columns: 1fr 1fr; }
  }
</style>
