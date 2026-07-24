<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories';
  import { commands, searchCommands } from '$lib/cheatsheet';
  import { speedCommands } from '$lib/speed-commands';
  import { vimChallenges, vimLevelNames } from '$lib/vim-commands';

  let query = $state('');
  let activeTab = $state('all');
  let challenges = $state([]);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      challenges = data.challenges || [];
    } catch (_) {}
  });

  let results = $derived.by(() => {
    if (!query.trim()) return [];
    const q = query.toLowerCase();
    const all = [];

    if (activeTab === 'all' || activeTab === 'challenges') {
      for (const c of challenges) {
        if (c.title.toLowerCase().includes(q) || c.description.toLowerCase().includes(q)) {
          all.push({ type: 'challenge', title: c.title, desc: c.description, link: '/challenges', icon: '📖', id: c.id });
        }
      }
    }
    if (activeTab === 'all' || activeTab === 'cheatsheet') {
      for (const cmd of searchCommands(query)) {
        all.push({ type: 'cheatsheet', title: cmd.cmd, desc: cmd.desc, link: '/resources', icon: '💻', extra: cmd.example });
      }
    }
    if (activeTab === 'all' || activeTab === 'speed') {
      for (const sp of speedCommands) {
        if (sp.description.toLowerCase().includes(q) || sp.answers.some(a => a.includes(q))) {
          all.push({ type: 'speed', title: sp.answers[0], desc: sp.description, link: '/speed', icon: '⚡' });
        }
      }
    }
    if (activeTab === 'all' || activeTab === 'vim') {
      for (const vm of vimChallenges) {
        if (vm.task.toLowerCase().includes(q) || vm.answer.toLowerCase().includes(q)) {
          all.push({ type: 'vim', title: vm.answer, desc: vm.task, link: '/vim', icon: '🔤', extra: `Lvl ${vm.level}: ${vimLevelNames[vm.level]}` });
        }
      }
    }

    return all.slice(0, 50);
  });

  let tabs = [
    { id: 'all', label: 'Wszystko' },
    { id: 'challenges', label: 'Wyzwania' },
    { id: 'cheatsheet', label: 'Ściągawka' },
    { id: 'speed', label: 'Speed' },
    { id: 'vim', label: 'Vim' },
  ];
</script>

<div class="search-page">
  <header class="page-header">
    <h1>🔍 Szukaj</h1>
    <p>Przeszukaj wyzwania, ściągawkę, speed challenge i Vim Master</p>
  </header>

  <div class="search-bar">
    <input type="search" bind:value={query} placeholder="Szukaj komendy, tematu, sekwencji..." autofocus class="search-input" />
  </div>

  <div class="tabs">
    {#each tabs as tab}
      <button class="tab" class:active={activeTab === tab.id} onclick={() => activeTab = tab.id}>{tab.label}</button>
    {/each}
  </div>

  {#if results.length > 0}
    <div class="results">
      {#each results as r}
        <a href={r.link} class="result-item">
          <span class="ri-icon">{r.icon}</span>
          <div class="ri-info">
            <span class="ri-title">{r.title}</span>
            <span class="ri-desc">{r.desc}</span>
            {#if r.extra}
              <span class="ri-extra">{r.extra}</span>
            {/if}
          </div>
        </a>
      {/each}
    </div>
  {:else if query.trim()}
    <div class="empty">
      <p>Brak wyników dla "<strong>{query}</strong>"</p>
    </div>
  {:else}
    <div class="empty">
      <p>Wpisz szukaną frazę – przeszukamy wszystkie materiały</p>
    </div>
  {/if}
</div>

<style>
  .search-page { max-width: 700px; }
  .search-bar { margin-bottom: 16px; }
  .search-input { width: 100%; padding: 14px 16px; background: #1e293b; border: 2px solid #334155; border-radius: 12px; color: #e2e8f0; font-size: 16px; outline: none; transition: border-color 0.15s; }
  .search-input:focus { border-color: #0ea5e9; }

  .tabs { display: flex; gap: 4px; margin-bottom: 16px; background: #1e293b; border-radius: 10px; padding: 4px; }
  .tab { flex: 1; padding: 8px; border: none; background: transparent; color: #94a3b8; cursor: pointer; border-radius: 8px; font-size: 13px; transition: all 0.15s; }
  .tab.active { background: #0ea5e9; color: #fff; }
  .tab:hover:not(.active) { background: #334155; }

  .results { display: flex; flex-direction: column; gap: 6px; }
  .result-item { display: flex; align-items: center; gap: 12px; padding: 12px 16px; background: #1e293b; border: 1px solid #334155; border-radius: 10px; text-decoration: none; transition: all 0.15s; }
  .result-item:hover { border-color: #475569; background: #263548; }
  .ri-icon { font-size: 20px; min-width: 28px; text-align: center; }
  .ri-info { flex: 1; }
  .ri-title { display: block; font-size: 14px; font-weight: 600; color: #f1f5f9; }
  .ri-desc { display: block; font-size: 12px; color: #94a3b8; }
  .ri-extra { display: block; font-size: 11px; color: #64748b; margin-top: 2px; }

  .empty { text-align: center; padding: 40px; color: #64748b; font-size: 14px; }
</style>
