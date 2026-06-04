<script>
  import { getCategoryInfo } from '$lib/categories.js';
  import { commands, searchCommands, getCommandsByCategory, categories } from '$lib/cheatsheet.js';
  import { resources, getCategoryResources } from '$lib/resources.js';

  let tab = $state('cheatsheet');
  let query = $state('');
  let activeCat = $state('all');

  let results = $derived(query
    ? searchCommands(query)
    : activeCat === 'all'
      ? commands
      : getCommandsByCategory(activeCat));

  let resourceCats = $derived(Object.keys(resources));

  function setCat(cat) {
    activeCat = cat;
    query = '';
  }
</script>

<div class="resources-page">
  <header class="page-header">
    <h1>📚 Zasoby</h1>
    <p>Ściągawka Linux i linki do dokumentacji</p>
  </header>

  <div class="tabs">
    <button class="tab" class:active={tab === 'cheatsheet'} onclick={() => tab = 'cheatsheet'}>💻 Ściągawka</button>
    <button class="tab" class:active={tab === 'links'} onclick={() => tab = 'links'}>🔗 Linki</button>
  </div>

  {#if tab === 'cheatsheet'}
    <div class="cheatsheet">
      <div class="cs-top">
        <input type="search" class="search" bind:value={query} placeholder="Szukaj polecenia..." />
        <div class="cat-filters">
          <button class="cat-btn" class:active={activeCat === 'all'} onclick={() => setCat('all')}>Wszystkie</button>
          {#each categories as cat}
            {@const info = getCategoryInfo(cat)}
            <button class="cat-btn" style="--cat-color: {info.color}" class:active={activeCat === cat} onclick={() => setCat(cat)}>
              {info.name}
            </button>
          {/each}
        </div>
      </div>

      <div class="cs-count">{results.length} poleceń</div>

      <div class="cs-grid">
        {#each results as cmd (cmd.cmd + cmd.category)}
          {@const info = getCategoryInfo(cmd.category)}
          <div class="cs-card">
            <div class="cs-card-top">
              <code class="cs-cmd">{cmd.cmd}</code>
              <span class="cs-cat-badge" style="background: {info.color}20; color: {info.color}">{info.name}</span>
            </div>
            <p class="cs-desc">{cmd.desc}</p>
            <pre class="cs-example">$ {cmd.example}</pre>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="links">
      {#each resourceCats as cat}
        {@const info = getCategoryInfo(cat)}
        {@const res = getCategoryResources(cat)}
        <section class="link-section">
          <h2 style="color: {info.color}">{res.icon} {res.name}</h2>
          <div class="link-grid">
            {#each res.links as link}
              <a href={link.url} target="_blank" rel="noopener noreferrer" class="link-card">
                <h3>{link.title}</h3>
                <p>{link.desc}</p>
              </a>
            {/each}
          </div>
        </section>
      {/each}
    </div>
  {/if}
</div>

<style>
  .resources-page { max-width: 1000px; }
  .page-header h1 { font-size: 28px; font-weight: 700; color: #f1f5f9; margin-bottom: 4px; }
  .page-header p { color: #64748b; margin-bottom: 20px; }

  .tabs { display: flex; gap: 4px; margin-bottom: 20px; background: #1e293b; border-radius: 10px; padding: 4px; }
  .tab { flex: 1; padding: 10px; border: none; background: transparent; color: #94a3b8; cursor: pointer; border-radius: 8px; font-size: 14px; font-weight: 500; transition: all 0.15s; }
  .tab.active { background: #0ea5e9; color: #fff; }
  .tab:hover:not(.active) { background: #334155; }

  .cs-top { margin-bottom: 16px; }
  .search { width: 100%; padding: 10px 14px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #e2e8f0; font-size: 14px; margin-bottom: 12px; }
  .search:focus { outline: none; border-color: #0ea5e9; }
  .cat-filters { display: flex; flex-wrap: wrap; gap: 6px; }
  .cat-btn { padding: 6px 14px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 6px; cursor: pointer; font-size: 12px; font-weight: 500; transition: all 0.15s; }
  .cat-btn:hover { background: #334155; color: #e2e8f0; }
  .cat-btn.active { background: var(--cat-color); color: #fff; border-color: var(--cat-color); }

  .cs-count { font-size: 12px; color: #64748b; margin-bottom: 12px; }
  .cs-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 12px; }
  .cs-card { background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 16px; }
  .cs-card-top { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
  .cs-cmd { font-size: 16px; font-weight: 700; color: #38bdf8; background: #0f172a; padding: 4px 10px; border-radius: 6px; }
  .cs-cat-badge { font-size: 10px; font-weight: 600; padding: 3px 8px; border-radius: 4px; text-transform: uppercase; }
  .cs-desc { font-size: 13px; color: #94a3b8; margin-bottom: 8px; }
  .cs-example { font-size: 12px; color: #22c55e; background: #0f172a; padding: 8px 10px; border-radius: 6px; overflow-x: auto; }

  .link-section { margin-bottom: 28px; }
  .link-section h2 { font-size: 18px; font-weight: 600; margin-bottom: 12px; }
  .link-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 12px; }
  .link-card { background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 16px; text-decoration: none; transition: all 0.15s; }
  .link-card:hover { border-color: #0ea5e9; }
  .link-card h3 { font-size: 14px; font-weight: 600; color: #f1f5f9; margin-bottom: 4px; }
  .link-card p { font-size: 12px; color: #64748b; }
</style>
