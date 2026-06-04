<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let token = $state('');
  let gistId = $state('');
  let dailyGoal = $state(30);
  let sessionMinutes = $state(0);
  let sessionRunning = $state(false);
  let sessionTimer = $state(0);
  let status = $state('');
  let markdown = $state('');
  let loading = $state(true);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      if (data.github_token) token = data.github_token;
      if (data.github_gist_id) gistId = data.github_gist_id;
      if (data.daily_goal_minutes) dailyGoal = data.daily_goal_minutes;
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function saveToken() {
    try {
      await invoke('save_github_token', { token });
      status = 'Token zapisany';
    } catch (e) {
      status = `Błąd: ${e}`;
    }
  }

  async function saveGoal() {
    try {
      await invoke('save_daily_goal', { minutes: dailyGoal });
      status = 'Cel dzienny zapisany';
    } catch (e) {
      status = `Błąd: ${e}`;
    }
  }

  async function backup() {
    try {
      const id = await invoke('backup_to_github');
      gistId = id;
      status = `Backup zapisany! Gist ID: ${id}`;
    } catch (e) {
      status = `Błąd backupu: ${e}`;
    }
  }

  async function restore() {
    try {
      const msg = await invoke('restore_from_github');
      status = msg;
    } catch (e) {
      status = `Błąd przywracania: ${e}`;
    }
  }

  async function exportMd() {
    try {
      markdown = await invoke('export_progress_markdown');
      status = 'Markdown wygenerowany';
    } catch (e) {
      status = `Błąd: ${e}`;
    }
  }

  function copyMd() {
    navigator.clipboard.writeText(markdown);
    status = 'Skopiowano do schowka';
  }

  async function testNotify() {
    try {
      await invoke('send_test_notification');
      status = 'Powiadomienie wysłane (sprawdź pulpit)';
    } catch (e) {
      status = `Błąd: ${e}`;
    }
  }

  let interval;
  function startSession() {
    sessionRunning = true;
    sessionTimer = 0;
    interval = setInterval(() => {
      sessionTimer++;
    }, 60000);
  }

  async function stopSession() {
    sessionRunning = false;
    clearInterval(interval);
    const mins = sessionTimer;
    try {
      await invoke('log_study_session', { minutes: mins });
      status = `Sesja zakończona: ${mins} min`;
      sessionMinutes = mins;
    } catch (e) {
      status = `Błąd: ${e}`;
    }
  }
</script>

<div class="settings-page">
  <header class="page-header">
    <h1>⚙️ Ustawienia</h1>
    <p>Konfiguracja konta, backupu i sesji nauki</p>
  </header>

  <div class="settings-grid">
    <section class="card">
      <h2>🔑 GitHub Token</h2>
      <p class="card-desc">Token z dostępem do <strong>gist</strong>. Utwórz go na GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens.</p>
      <input type="password" class="input" bind:value={token} placeholder="ghp_..." />
      <button class="btn" onclick={saveToken}>Zapisz token</button>
    </section>

    <section class="card">
      <h2>☁️ Backup & Przywracanie</h2>
      <p class="card-desc">Backup danych na GitHub Gist. Po pierwszym backupie możesz przywracać.</p>
      {#if gistId}
        <p class="gist-id">Gist ID: {gistId}</p>
      {/if}
      <div class="btn-row">
        <button class="btn" onclick={backup}>💾 Wykonaj backup</button>
        <button class="btn btn-secondary" onclick={restore}>📥 Przywróć</button>
      </div>
    </section>

    <section class="card">
      <h2>📅 Cel dzienny</h2>
      <p class="card-desc">Ile minut dziennie chcesz poświęcać na naukę?</p>
      <div class="slider-row">
        <input type="range" min="5" max="120" step="5" bind:value={dailyGoal} class="slider" />
        <span class="slider-val">{dailyGoal} min</span>
      </div>
      <button class="btn" onclick={saveGoal}>Zapisz cel</button>
    </section>

    <section class="card">
      <h2>⏱️ Sesja nauki</h2>
      <p class="card-desc">Zmierz czas poświęcony na naukę.</p>
      {#if sessionRunning}
        <p class="timer">{sessionTimer} min</p>
        <button class="btn btn-danger" onclick={stopSession}>Zatrzymaj sesję</button>
      {:else}
        <button class="btn" onclick={startSession}>▶ Rozpocznij sesję</button>
      {/if}
    </section>

    <section class="card">
      <h2>🔔 Powiadomienia</h2>
      <p class="card-desc">Przetestuj powiadomienia desktopowe.</p>
      <button class="btn" onclick={testNotify}>📬 Wyślij testowe powiadomienie</button>
    </section>

    <section class="card">
      <h2>📄 Eksport Markdown</h2>
      <p class="card-desc">Wygeneruj podsumowanie postępów w formacie Markdown.</p>
      <button class="btn" onclick={exportMd}>📝 Generuj</button>
      {#if markdown}
        <pre class="md-output">{markdown}</pre>
        <button class="btn btn-secondary" onclick={copyMd}>📋 Kopiuj do schowka</button>
      {/if}
    </section>
  </div>

  {#if status}
    <div class="toast">{status}</div>
  {/if}
</div>

<style>
  .settings-page { max-width: 900px; }
  .page-header h1 { font-size: 28px; font-weight: 700; color: #f1f5f9; margin-bottom: 4px; }
  .page-header p { color: #64748b; margin-bottom: 24px; }
  .settings-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
  .card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; }
  .card h2 { font-size: 16px; font-weight: 600; color: #f1f5f9; margin-bottom: 8px; }
  .card-desc { font-size: 13px; color: #94a3b8; margin-bottom: 12px; line-height: 1.5; }
  .input { width: 100%; padding: 10px 12px; background: #0f172a; border: 1px solid #475569; border-radius: 8px; color: #e2e8f0; font-size: 14px; margin-bottom: 12px; }
  .input:focus { outline: none; border-color: #0ea5e9; }
  .btn { padding: 10px 18px; background: #0ea5e9; color: #fff; border: none; border-radius: 8px; cursor: pointer; font-size: 13px; font-weight: 600; transition: background 0.15s; }
  .btn:hover { background: #0284c7; }
  .btn-secondary { background: #475569; }
  .btn-secondary:hover { background: #64748b; }
  .btn-danger { background: #ef4444; }
  .btn-danger:hover { background: #dc2626; }
  .btn-row { display: flex; gap: 8px; }
  .gist-id { font-size: 12px; color: #38bdf8; word-break: break-all; margin-bottom: 8px; }
  .slider-row { display: flex; align-items: center; gap: 12px; margin-bottom: 12px; }
  .slider { flex: 1; accent-color: #0ea5e9; }
  .slider-val { font-size: 18px; font-weight: 700; color: #38bdf8; min-width: 60px; }
  .timer { font-size: 36px; font-weight: 800; color: #38bdf8; text-align: center; margin: 12px 0; }
  .md-output { background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px; font-size: 12px; max-height: 200px; overflow-y: auto; margin-top: 12px; white-space: pre-wrap; word-break: break-all; }
  .toast { position: fixed; bottom: 24px; right: 24px; background: #1e293b; border: 1px solid #0ea5e9; color: #e2e8f0; padding: 12px 20px; border-radius: 8px; font-size: 13px; animation: fadeInOut 3s forwards; }
  @keyframes fadeInOut { 0% { opacity: 0; transform: translateY(10px); } 10% { opacity: 1; transform: translateY(0); } 80% { opacity: 1; } 100% { opacity: 0; } }
</style>
