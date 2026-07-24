<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import LabGuide from '$lib/components/LabGuide.svelte';

  let projects = $state([]);
  let loading = $state(true);

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

  async function completeProject(id) {
    const [success, level] = await invoke('complete_project', { id });
    if (success) {
      const p = projects.find(x => x.id === id);
      if (p) p.completed = true;
    }
  }

  async function saveRepo(id, url) {
    await invoke('update_project_repo', { id, repo: url });
    const p = projects.find(x => x.id === id);
    if (p) p.github_repo = url;
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
            <div class="pc-title-row">
              <span class="pc-icon">{p.completed ? '✅' : '💻'}</span>
              <div>
                <h3>{p.title}</h3>
                <p class="pc-desc">{p.description}</p>
              </div>
            </div>
            {#if p.completed}
              <span class="completed-badge">✓ Ukończono</span>
            {/if}
          </div>

          <LabGuide
            project={p}
            oncomplete={completeProject}
            onsaverepo={saveRepo}
          />
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
    align-items: flex-start;
    margin-bottom: 12px;
  }

  .pc-title-row { display: flex; gap: 10px; align-items: flex-start; }
  .pc-icon { font-size: 20px; }

  .pc-header h3 { font-size: 18px; font-weight: 600; color: #f1f5f9; margin-bottom: 4px; }

  .completed-badge { color: #22c55e; font-weight: 600; font-size: 13px; white-space: nowrap; }

  .pc-desc { font-size: 13px; color: #94a3b8; line-height: 1.5; }
</style>
