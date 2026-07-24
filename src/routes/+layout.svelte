<script>
  import '$lib/styles/shared.css';
  import { page } from '$app/stores';
  let { children } = $props();
  let sidebarOpen = $state(true);

  const navItems = [
    { href: '/', label: 'Dashboard', icon: '📊' },
    { href: '/challenges', label: 'Wyzwania', icon: '🎯' },
    { href: '/milestones', label: 'Kamienie milowe', icon: '🏆' },
    { href: '/projects', label: 'Projekty', icon: '💻' },
    { href: '/quiz', label: 'Quiz', icon: '🧠' },
    { href: '/exam', label: 'Egzamin', icon: '📝' },
    { href: '/speed', label: 'Speed', icon: '⚡' },
    { href: '/review', label: 'Powtórka', icon: '🔄' },
    { href: '/progress', label: 'Postępy', icon: '📈' },
    { href: '/resources', label: 'Zasoby', icon: '📚' },
    { href: '/certification', label: 'Certyfikacje', icon: '🎓' },
    { href: '/settings', label: 'Ustawienia', icon: '⚙️' },
  ];
</script>

<div class="app-shell">
  <aside class="sidebar" class:collapsed={!sidebarOpen}>
    <div class="sidebar-header">
      <h1 class="logo" class:small={!sidebarOpen}>CP</h1>
      <span class="sidebar-title" class:hidden={!sidebarOpen}>RootForge</span>
      <button class="toggle-btn" onclick={() => sidebarOpen = !sidebarOpen}>
        {sidebarOpen ? '◀' : '▶'}
      </button>
    </div>
    <nav class="nav">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-item"
          class:active={$page.url.pathname === item.href}
        >
          <span class="nav-icon">{item.icon}</span>
          <span class="nav-label" class:hidden={!sidebarOpen}>{item.label}</span>
        </a>
      {/each}
    </nav>
  </aside>
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: 'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif;
    background: #0f172a;
    color: #e2e8f0;
    overflow: hidden;
  }

  .app-shell {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .sidebar {
    width: 260px;
    background: #1e293b;
    border-right: 1px solid #334155;
    display: flex;
    flex-direction: column;
    transition: width 0.2s ease;
    flex-shrink: 0;
  }

  .sidebar.collapsed {
    width: 72px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    padding: 20px 16px;
    gap: 10px;
    border-bottom: 1px solid #334155;
  }

  .logo {
    font-size: 24px;
    font-weight: 800;
    color: #38bdf8;
    min-width: 32px;
    text-align: center;
  }

  .logo.small {
    font-size: 18px;
  }

  .sidebar-title {
    font-size: 18px;
    font-weight: 700;
    color: #f1f5f9;
    white-space: nowrap;
  }

  .toggle-btn {
    margin-left: auto;
    background: none;
    border: none;
    color: #64748b;
    cursor: pointer;
    font-size: 14px;
    padding: 4px;
  }

  .toggle-btn:hover {
    color: #94a3b8;
  }

  .nav {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 12px 8px;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 12px;
    border-radius: 8px;
    text-decoration: none;
    color: #94a3b8;
    transition: all 0.15s;
    white-space: nowrap;
  }

  .nav-item:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .nav-item.active {
    background: #0ea5e9;
    color: #fff;
    font-weight: 600;
  }

  .nav-icon {
    font-size: 18px;
    min-width: 24px;
    text-align: center;
  }

  .nav-label {
    font-size: 14px;
  }

  .hidden {
    display: none;
  }

  .content {
    flex: 1;
    padding: 32px;
    overflow-y: auto;
    background: #0f172a;
  }
</style>
