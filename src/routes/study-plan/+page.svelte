<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

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

  let plan = $derived.by(() => {
    if (!data) return [];
    const challenges = data.challenges || [];
    const completed = challenges.filter(c => c.completed).length;
    const total = challenges.length;
    const pct = total > 0 ? completed / total : 0;

    const weeks = [];
    const stages = [1, 2, 3, 4, 5];
    const stageChallenges = s => challenges.filter(c => c.stage === s && c.exam_tag === 'rhcsa');
    const doneInStage = s => stageChallenges(s).filter(c => c.completed).length;
    const totalInStage = s => stageChallenges(s).length;

    if (pct < 0.2) {
      weeks.push({ week: 1, title: 'Podstawy Linuksa', focus: 'Terminal, Vim, uprawnienia, procesy', challenges: [1,2,3,4,7,21,22], done: 0, total: 7 });
      weeks.push({ week: 2, title: 'Sieci i użytkownicy', focus: 'Sieć, SSH, użytkownicy, pakiety', challenges: [5,9,23,24,28,55], done: 0, total: 6 });
      weeks.push({ week: 3, title: 'Storage i LVM', focus: 'Dyski, LVM, RAID, fstab', challenges: [8,17,46,52,59,66,71], done: 0, total: 7 });
      weeks.push({ week: 4, title: 'Systemd i bootowanie', focus: 'Systemd, GRUB, jądro, logi', challenges: [11,14,25,26,27,41,42,43], done: 0, total: 8 });
    } else if (pct < 0.5) {
      weeks.push({ week: 1, title: 'SELinux i bezpieczeństwo', focus: 'SELinux, firewall, sudo, hardening', challenges: [6,13,24,29,35,53,55,72], done: 0, total: 8 });
      weeks.push({ week: 2, title: 'Kontenery i DevOps', focus: 'Podman, Skopeo, Buildah, systemd kontener', challenges: [10,51,56,62,63,75], done: 0, total: 6 });
      weeks.push({ week: 3, title: 'Zaawansowany storage', focus: 'Stratis, VDO, iSCSI, LVM snapshoty', challenges: [52,59,61,66], done: 0, total: 4 });
      weeks.push({ week: 4, title: 'Monitoring i wydajność', focus: 'tuned, logging, auditd, NTP', challenges: [47,68,76,77], done: 0, total: 4 });
    } else {
      weeks.push({ week: 1, title: 'Powtórka RHCSA', focus: 'Wszystkie kluczowe tematy od początku', challenges: [1,5,8,9,10,13,14,23,24,25], done: 0, total: 10 });
      weeks.push({ week: 2, title: 'Mock exam', focus: 'Rozwiąż egzamin próbny, powtórz słabe obszary', challenges: [44,47,55,68,71,72,73], done: 0, total: 7 });
      weeks.push({ week: 3, title: 'Projekty praktyczne', focus: 'Zrób projekty labowe', challenges: [], done: 0, total: 0 });
      weeks.push({ week: 4, title: 'Speed + Vim + Troubleshooting', focus: 'Speed challenge, Vim master, troubleshooting', challenges: [], done: 0, total: 0 });
    }

    for (const w of weeks) {
      const chs = challenges.filter(c => w.challenges.includes(c.id));
      w.done = chs.filter(c => c.completed).length;
    }

    return weeks;
  });

  let totalPlanDone = $derived(plan.reduce((a, w) => a + w.done, 0));
  let totalPlanTotal = $derived(plan.reduce((a, w) => a + w.total, 0));
</script>

<div class="plan-page">
  <header class="page-header">
    <h1>📅 Plan nauki RHCSA</h1>
    <p>Spersonalizowany plan przygotowania do egzaminu – generowany na podstawie Twoich postępów</p>
  </header>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="plan-summary card">
      <h2>Twój plan</h2>
      <p class="plan-advice">
        {#if totalPlanTotal > 0}
          Postęp: <strong>{totalPlanDone}/{totalPlanTotal}</strong> ({Math.round(totalPlanDone / totalPlanTotal * 100)}%)
        {:else}
          Ukończ wyzwania z planu aby śledzić postęp
        {/if}
      </p>
      <div class="plan-global-bar">
        <ProgressBar percent={totalPlanTotal > 0 ? totalPlanDone / totalPlanTotal * 100 : 0} height={8} />
      </div>
    </div>

    <div class="plan-weeks">
      {#each plan as week, idx}
        <div class="plan-week card">
          <div class="plan-week-header">
            <span class="plan-week-num">Tydzień {week.week}</span>
            <span class="plan-week-pct">{week.total > 0 ? Math.round(week.done / week.total * 100) : 0}%</span>
          </div>
          <h3>{week.title}</h3>
          <p class="plan-focus">{week.focus}</p>

          {#if week.challenges.length > 0}
            <div class="plan-week-progress">
              <ProgressBar percent={week.total > 0 ? week.done / week.total * 100 : 0} height={4} color={week.done === week.total ? '#22c55e' : undefined} />
              <span class="plan-week-count">{week.done}/{week.total}</span>
            </div>
            <div class="plan-week-chs">
              {#each week.challenges as cid}
                {@const ch = data.challenges?.find(c => c.id === cid)}
                {#if ch}
                  <span class="plan-ch" class:done={ch.completed}>
                    {ch.completed ? '✅' : '⬜'} {ch.title}
                  </span>
                {/if}
              {/each}
            </div>
          {:else}
            <p class="plan-week-note">⚡ Ćwicz praktyczne umiejętności w projektach i wyzwaniach</p>
          {/if}
        </div>
      {/each}
    </div>

    <div class="plan-cta card">
      <h2>🏁 Gotowy do egzaminu?</h2>
      <p>Rozwiąż egzamin próbny, aby sprawdzić swoją gotowość.</p>
      <a href="/exam" class="cta-btn">📝 Rozpocznij mock exam</a>
    </div>
  {/if}
</div>

<style>
  .plan-page { max-width: 700px; }

  .plan-summary { padding: 20px; margin-bottom: 16px; }
  .plan-summary h2 { font-size: 18px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .plan-advice { font-size: 14px; color: #94a3b8; margin-bottom: 12px; }
  .plan-global-bar :global(.progress-bar-bg) { flex: 1; }

  .plan-weeks { display: flex; flex-direction: column; gap: 12px; margin-bottom: 16px; }
  .plan-week { padding: 16px; }
  .plan-week-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px; }
  .plan-week-num { font-size: 11px; font-weight: 700; color: #0ea5e9; text-transform: uppercase; }
  .plan-week-pct { font-size: 16px; font-weight: 700; color: #38bdf8; }
  .plan-week h3 { font-size: 16px; font-weight: 600; color: #f1f5f9; margin-bottom: 4px; }
  .plan-focus { font-size: 12px; color: #64748b; margin-bottom: 10px; }
  .plan-week-progress { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
  .plan-week-progress :global(.progress-bar-bg) { flex: 1; }
  .plan-week-count { font-size: 11px; color: #64748b; white-space: nowrap; }
  .plan-week-chs { display: flex; flex-wrap: wrap; gap: 4px; }
  .plan-ch { font-size: 11px; padding: 3px 8px; background: #334155; border-radius: 4px; color: #94a3b8; }
  .plan-ch.done { background: #16653430; color: #22c55e; }
  .plan-week-note { font-size: 12px; color: #64748b; font-style: italic; }

  .plan-cta { text-align: center; padding: 24px; }
  .plan-cta h2 { font-size: 18px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .plan-cta p { font-size: 14px; color: #94a3b8; margin-bottom: 16px; }
  .cta-btn { display: inline-block; padding: 12px 28px; background: #0ea5e9; color: #fff; border-radius: 10px; text-decoration: none; font-size: 15px; font-weight: 600; }
  .cta-btn:hover { background: #0284c7; }
</style>
