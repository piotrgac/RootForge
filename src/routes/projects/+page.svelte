<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let projects = $state([]);
  let loading = $state(true);
  let editingRepo = $state(null);
  let repoUrl = $state('');

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      projects = data.projects || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function saveRepo(id) {
    if (repoUrl.trim()) {
      await invoke('update_project_repo', { id, repo: repoUrl.trim() });
      const p = projects.find(x => x.id === id);
      if (p) p.github_repo = repoUrl.trim();
    }
    editingRepo = null;
    repoUrl = '';
  }

  async function completeProject(id) {
    const [success, level] = await invoke('complete_project', { id });
    if (success) {
      const p = projects.find(x => x.id === id);
      if (p) p.completed = true;
    }
  }
</script>

<div class="projects-page">
  <header class="page-header">
    <h1>💻 Projekty</h1>
    <p>Mini-projekty do portfolio – wrzuć na GitHub i pokaż przyszłemu pracodawcy</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="project-list">
      {#each projects as p (p.id)}
        <div class="project-card" class:completed={p.completed}>
          <div class="pc-header">
            <h3>{p.title}</h3>
            {#if p.completed}
              <span class="completed-badge">✓ Ukończono</span>
            {/if}
          </div>
          <p class="pc-desc">{p.description}</p>

          <details class="pc-guide">
            <summary>📖 Zobacz guide</summary>
            <pre class="guide-content">{p.guide}</pre>
          </details>

          <div class="pc-footer">
            {#if !p.github_repo && !p.completed}
              {#if editingRepo === p.id}
                <div class="repo-input">
                  <input
                    type="url"
                    placeholder="https://github.com/twoj-repo"
                    bind:value={repoUrl}
                  />
                  <button class="save-repo-btn" onclick={() => saveRepo(p.id)}>Zapisz</button>
                </div>
              {:else}
                <button class="add-repo-btn" onclick={() => { editingRepo = p.id; repoUrl = ''; }}>
                  + Dodaj link do GitHub
                </button>
              {/if}
            {:else if p.github_repo}
              <a href={p.github_repo} target="_blank" class="repo-link">🔗 {p.github_repo}</a>
            {/if}
            {#if !p.completed}
              <button class="complete-btn" onclick={() => completeProject(p.id)}>
                Oznacz jako ukończone
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .projects-page { max-width: 800px; }

  .project-list { display: flex; flex-direction: column; gap: 16px; }

  .project-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 12px;
    padding: 20px;
    transition: all 0.2s;
  }

  .project-card.completed { border-color: #166534; opacity: 0.8; }

  .pc-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .pc-header h3 { font-size: 18px; font-weight: 600; color: #f1f5f9; }

  .completed-badge { color: #22c55e; font-weight: 600; font-size: 13px; }

  .pc-desc { font-size: 13px; color: #94a3b8; margin-bottom: 12px; line-height: 1.5; }

  .pc-guide { margin-bottom: 12px; }

  .pc-guide summary {
    cursor: pointer;
    color: #38bdf8;
    font-size: 13px;
    font-weight: 600;
    padding: 4px 0;
  }

  .guide-content {
    background: #0f172a;
    padding: 12px;
    border-radius: 8px;
    font-size: 12px;
    color: #94a3b8;
    white-space: pre-wrap;
    margin-top: 8px;
    font-family: 'JetBrains Mono', monospace;
  }

  .pc-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 8px;
  }

  .repo-input {
    display: flex;
    gap: 8px;
    flex: 1;
    max-width: 400px;
  }

  .repo-input input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #334155;
    border-radius: 6px;
    background: #0f172a;
    color: #e2e8f0;
    font-size: 13px;
  }

  .repo-input input::placeholder { color: #475569; }

  .add-repo-btn {
    padding: 8px 14px;
    background: transparent;
    color: #38bdf8;
    border: 1px dashed #334155;
    border-radius: 8px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
    transition: all 0.15s;
  }

  .add-repo-btn:hover { border-color: #38bdf8; background: #38bdf810; }

  .save-repo-btn, .complete-btn {
    padding: 8px 14px;
    background: #0ea5e9;
    color: #fff;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
    transition: background 0.15s;
    white-space: nowrap;
  }

  .save-repo-btn:hover, .complete-btn:hover { background: #0284c7; }

  .repo-link {
    color: #38bdf8;
    font-size: 13px;
    text-decoration: none;
  }

  .repo-link:hover { text-decoration: underline; }
</style>
