<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

  let data = $state(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      data = await invoke('get_dashboard_stats');
    } catch (_) {} finally { loading = false; }
  });

  let completedCount = $derived(data?.challenges?.filter(c => c.completed).length || 0);
  let totalCount = $derived(data?.challenges?.length || 0);
  let rhcsaCompleted = $derived(data?.challenges?.filter(c => c.completed && c.exam_tag === 'rhcsa').length || 0);
  let rhcsaTotal = $derived(data?.challenges?.filter(c => c.exam_tag === 'rhcsa').length || 0);
  let devopsCompleted = $derived(data?.challenges?.filter(c => c.completed && c.exam_tag === 'devops').length || 0);
  let devopsTotal = $derived(data?.challenges?.filter(c => c.exam_tag === 'devops').length || 0);

  let rhcsaExamReady = $derived(rhcsaCompleted / Math.max(rhcsaTotal, 1) >= 0.8);

  const levels = [
    {
      id: 'beginner',
      title: 'Początkujący',
      subtitle: 'Nigdy nie pracowałeś z Linuxem',
      icon: '🌱',
      color: '#10b981',
      skills: ['Podstawy terminala', 'System plików', 'Edytory Vim/Nano', 'Uprawnienia plików', 'Procesy'],
      challenges: [1,2,3,4,7,21,22],
      salary: '6-9 tys. PLN',
      next: 'Junior Linux Admin',
    },
    {
      id: 'junior',
      title: 'Junior Linux Admin',
      subtitle: 'Potrafisz obsługiwać serwer',
      icon: '💻',
      color: '#3b82f6',
      skills: ['Sieci (ip, nmcli)', 'SSH i klucze', 'Użytkownicy i grupy', 'Pakiety RPM/DNF', 'Systemd'],
      challenges: [5,9,14,23,24,25,28,42,55],
      salary: '9-14 tys. PLN',
      next: 'RHCSA',
    },
    {
      id: 'rhcsa',
      title: 'RHCSA Certified',
      subtitle: 'Możesz administrować RHEL',
      icon: '🏅',
      color: '#7c3aed',
      skills: ['LVM i storage', 'SELinux', 'Firewalld', 'Kontenery Podman', 'Logowanie i audyt'],
      challenges: [8,10,11,13,17,29,41,46,47,52,53,59,61,66,68,71,72,73,74,75,76,77],
      salary: '14-20 tys. PLN',
      cert: 'EX200',
      next: 'RHCE',
    },
    {
      id: 'rhce',
      title: 'RHCE / Senior Admin',
      subtitle: 'Automatyzujesz i projektujesz',
      icon: '🚀',
      color: '#f59e0b',
      skills: ['Ansible', 'Advanced scripting', 'Monitoring', 'Performance tuning', 'Backup/DR'],
      challenges: [7,16,19,36,37,38,45,54,60,70],
      salary: '18-28 tys. PLN',
      cert: 'EX294',
      next: 'DevOps / SRE',
    },
    {
      id: 'devops',
      title: 'DevOps / SRE',
      subtitle: 'Tworzysz infrastrukturę jako kod',
      icon: '🔧',
      color: '#ef4444',
      skills: ['CI/CD pipelines', 'Docker/K8s', 'Terraform', 'Cloud (AWS)', 'Monitoring stack'],
      challenges: [98,99,100,113,114,115,116,117,118,119,143,144,145,150,153,154,155,159,160,161,168,169,170,171,172,173],
      salary: '22-40 tys. PLN',
      next: 'Architect / Lead',
    },
  ];

  function getLevelProgress(level) {
    const chs = data?.challenges?.filter(c => level.challenges.includes(c.id)) || [];
    const done = chs.filter(c => c.completed).length;
    const total = level.challenges.length;
    return { done, total, pct: total > 0 ? Math.round(done / total * 100) : 0 };
  }

  let currentLevelIndex = $derived.by(() => {
    for (let i = levels.length - 1; i >= 0; i--) {
      const lvl = levels[i];
      const prog = getLevelProgress(lvl);
      if (prog.done > 0) return i;
    }
    return 0;
  });
</script>

<div class="career-page">
  <header class="page-header">
    <h1>🚀 Ścieżka Linux Administrator</h1>
    <p>Od zera do DevOps – sprawdzona ścieżka kariery krok po kroku</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <!-- Overall progress -->
    <div class="career-summary card">
      <div class="cs-left">
        <span class="cs-icon">📊</span>
        <div>
          <h2>Twój postęp</h2>
          <p>{completedCount}/{totalCount} wyzwań ukończonych</p>
        </div>
      </div>
      <div class="cs-right">
        <span class="cs-badge rhcsa">RHCSA: {rhcsaCompleted}/{rhcsaTotal}</span>
        <span class="cs-badge devops">DevOps: {devopsCompleted}/{devopsTotal}</span>
      </div>
    </div>

    <!-- Roadmap -->
    <div class="roadmap">
      {#each levels as level, idx}
        {@const prog = getLevelProgress(level)}
        {@const isCurrent = idx === currentLevelIndex}
        {@const isUnlocked = idx <= currentLevelIndex}
        {@const isNext = idx === currentLevelIndex + 1}

        <div class="road-node" class:current={isCurrent} class:unlocked={isUnlocked} class:locked={!isUnlocked} class:next={isNext}>
          <!-- Connector line (except first) -->
          {#if idx > 0}
            <div class="connector" class:active={isUnlocked}></div>
          {/if}

          <div class="node-card card" style="border-color: {isUnlocked ? level.color : '#334155'}">
            <div class="node-header">
              <span class="node-icon" style="background: {level.color}20">{level.icon}</span>
              <div class="node-title-area">
                <h3>{level.title}</h3>
                <p class="node-subtitle">{level.subtitle}</p>
              </div>
              {#if isUnlocked}
                <div class="node-progress-pct">{prog.pct}%</div>
              {:else}
                <div class="node-locked-icon">🔒</div>
              {/if}
            </div>

            {#if isUnlocked}
              <div class="node-progress">
                <ProgressBar percent={prog.pct} height={6} color={prog.pct >= 100 ? '#22c55e' : level.color} />
                <span class="node-count">{prog.done}/{prog.total}</span>
              </div>

              <div class="node-skills">
                {#each level.skills as skill}
                  <span class="skill-tag">{skill}</span>
                {/each}
              </div>

              {#if level.cert}
                <div class="node-cert">
                  <span class="cert-badge">📜 {level.cert}</span>
                </div>
              {/if}

              <div class="node-footer">
                <span class="node-salary">💰 {level.salary}</span>
                {#if prog.done < prog.total}
                  <a href="/challenges" class="node-action">📖 Kontynuuj</a>
                {:else if level.next}
                  <span class="node-next">➡️ {level.next}</span>
                {:else}
                  <span class="node-done">✅ Ukończono</span>
                {/if}
              </div>
            {:else if isNext}
              <div class="node-locked-msg">
                <p>🔒 Ukończ poprzedni poziom, aby odblokować</p>
                <p class="node-requirements">Wymagane: {level.skills.slice(0, 3).join(', ')} i więcej</p>
              </div>
            {:else}
              <div class="node-locked-msg">
                <p>🔒 Odblokuj wcześniejsze poziomy</p>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>

    <!-- RHCSA Readiness -->
    <div class="exam-readiness card">
      <h2>📝 Gotowość do RHCSA EX200</h2>
      <div class="exam-grid">
        <div class="exam-stat">
          <span class="exam-val">{rhcsaCompleted}/{rhcsaTotal}</span>
          <span class="exam-label">Wymagane wyzwania</span>
        </div>
        <div class="exam-stat">
          <span class="exam-val">{Math.round(rhcsaCompleted / Math.max(rhcsaTotal, 1) * 100)}%</span>
          <span class="exam-label">Postęp RHCSA</span>
        </div>
        <div class="exam-stat" class:ready={rhcsaExamReady}>
          <span class="exam-val">{rhcsaExamReady ? '✅ Gotowy' : '📚 Ucz się dalej'}</span>
          <span class="exam-label">Status egzaminu</span>
        </div>
      </div>
      {#if rhcsaExamReady}
        <div class="exam-cta">
          <p>🎉 Jesteś gotowy na RHCSA! Wykonaj egzamin próbny, aby potwierdzić.</p>
          <a href="/exam" class="cta-btn">📝 Rozpocznij mock exam</a>
          <a href="/study-plan" class="cta-btn secondary">📅 Plan nauki</a>
        </div>
      {:else}
        <div class="exam-cta">
          <p>Wykonaj wszystkie wyzwania oznaczone <span class="rhcsa-tag">RHCSA</span>, aby być gotowym do egzaminu.</p>
          <a href="/challenges" class="cta-btn">📖 Idź do wyzwań</a>
          <a href="/study-plan" class="cta-btn secondary">📅 Plan nauki</a>
        </div>
      {/if}
    </div>

    <!-- Quick stats -->
    <div class="quick-stats">
      <div class="qs-card card">
        <span class="qs-icon">📈</span>
        <span class="qs-val">{completedCount}</span>
        <span class="qs-label">Ukończone wyzwania</span>
      </div>
      <div class="qs-card card">
        <span class="qs-icon">🧠</span>
        <span class="qs-val">{data?.quiz_results?.length || 0}</span>
        <span class="qs-label">Rozwiązane quizy</span>
      </div>
      <div class="qs-card card">
        <span class="qs-icon">⚡</span>
        <span class="qs-val">{data?.speed_records?.filter(r => r.correct).length || 0}</span>
        <span class="qs-label">Speed challenge</span>
      </div>
      <div class="qs-card card">
        <span class="qs-icon">🏆</span>
        <span class="qs-val">{data?.achievements?.filter(a => a.unlocked).length || 0}/{data?.achievements?.length || 0}</span>
        <span class="qs-label">Osiągnięcia</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .career-page { max-width: 800px; }

  .career-summary { display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; margin-bottom: 20px; flex-wrap: wrap; gap: 12px; }
  .cs-left { display: flex; align-items: center; gap: 12px; }
  .cs-icon { font-size: 28px; }
  .cs-left h2 { font-size: 16px; font-weight: 600; color: #f1f5f9; }
  .cs-left p { font-size: 13px; color: #64748b; }
  .cs-right { display: flex; gap: 6px; }
  .cs-badge { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; }
  .cs-badge.rhcsa { background: #0ea5e920; color: #38bdf8; }
  .cs-badge.devops { background: #7c3aed20; color: #a78bfa; }

  .roadmap { position: relative; margin-bottom: 24px; }
  .road-node { position: relative; }
  .connector { width: 3px; height: 24px; background: #334155; margin-left: 32px; transition: background 0.3s; }
  .connector.active { background: linear-gradient(180deg, #0ea5e9, #38bdf8); }

  .node-card { padding: 16px; margin-bottom: 2px; border-left-width: 3px; transition: all 0.3s; }
  .road-node.current .node-card { border-left-color: #0ea5e9 !important; background: #1a2332; }
  .road-node.next .node-card { animation: pulse-border 2s infinite; }
  @keyframes pulse-border { 0%, 100% { border-left-color: #334155; } 50% { border-left-color: #0ea5e9; } }

  .node-header { display: flex; align-items: center; gap: 12px; margin-bottom: 10px; }
  .node-icon { font-size: 28px; padding: 6px; border-radius: 10px; line-height: 1; }
  .node-title-area { flex: 1; }
  .node-title-area h3 { font-size: 16px; font-weight: 600; color: #f1f5f9; }
  .node-subtitle { font-size: 12px; color: #64748b; }
  .node-progress-pct { font-size: 18px; font-weight: 700; color: #38bdf8; }
  .node-locked-icon { font-size: 20px; }

  .node-progress { display: flex; align-items: center; gap: 8px; margin-bottom: 10px; }
  .node-progress :global(.progress-bar-bg) { flex: 1; }
  .node-count { font-size: 11px; color: #64748b; white-space: nowrap; }

  .node-skills { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 10px; }
  .skill-tag { font-size: 10px; padding: 3px 8px; background: #334155; border-radius: 4px; color: #94a3b8; }

  .node-cert { margin-bottom: 8px; }
  .cert-badge { font-size: 11px; font-weight: 600; padding: 3px 10px; background: #f59e0b20; color: #fbbf24; border-radius: 4px; }

  .node-footer { display: flex; justify-content: space-between; align-items: center; }
  .node-salary { font-size: 12px; color: #22c55e; font-weight: 600; }
  .node-action { padding: 6px 14px; background: #0ea5e9; color: #fff; border-radius: 6px; text-decoration: none; font-size: 12px; font-weight: 600; }
  .node-action:hover { background: #0284c7; }
  .node-next { font-size: 12px; color: #38bdf8; }
  .node-done { font-size: 12px; color: #22c55e; }

  .node-locked-msg { padding: 8px 0; }
  .node-locked-msg p { font-size: 13px; color: #64748b; }
  .node-requirements { font-size: 11px !important; margin-top: 4px; font-style: italic; }

  .exam-readiness { padding: 20px; margin-bottom: 20px; border: 1px solid #0ea5e9; }
  .exam-readiness h2 { font-size: 18px; font-weight: 700; color: #f1f5f9; margin-bottom: 12px; }
  .exam-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; margin-bottom: 16px; }
  .exam-stat { text-align: center; padding: 12px; background: #0f172a; border-radius: 8px; }
  .exam-stat.ready { border: 1px solid #22c55e; }
  .exam-val { display: block; font-size: 22px; font-weight: 700; color: #38bdf8; }
  .exam-stat.ready .exam-val { color: #22c55e; }
  .exam-label { font-size: 11px; color: #64748b; }

  .exam-cta { text-align: center; }
  .exam-cta p { font-size: 13px; color: #94a3b8; margin-bottom: 12px; }
  .rhcsa-tag { background: #0ea5e920; color: #38bdf8; padding: 2px 6px; border-radius: 4px; font-weight: 600; }
  .cta-btn { display: inline-block; margin: 4px; padding: 10px 22px; background: #0ea5e9; color: #fff; border-radius: 8px; text-decoration: none; font-size: 13px; font-weight: 600; }
  .cta-btn:hover { background: #0284c7; }
  .cta-btn.secondary { background: #1e293b; border: 1px solid #334155; color: #94a3b8; }
  .cta-btn.secondary:hover { background: #334155; color: #e2e8f0; }

  .quick-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
  .qs-card { text-align: center; padding: 14px; }
  .qs-icon { display: block; font-size: 24px; margin-bottom: 4px; }
  .qs-val { display: block; font-size: 20px; font-weight: 700; color: #f1f5f9; }
  .qs-label { font-size: 11px; color: #64748b; }
</style>
