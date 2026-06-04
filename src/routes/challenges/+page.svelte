<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCategoryInfo } from '$lib/categories.js';

  let challenges = $state([]);
  let loading = $state(true);
  let filter = $state('all');
  let detailChallenge = $state(null);

  onMount(async () => {
    try {
      const data = await invoke('get_dashboard_stats');
      challenges = data.challenges || [];
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  let filtered = $derived(filter === 'all' ? challenges
    : filter === 'completed' ? challenges.filter(c => c.completed)
    : challenges.filter(c => !c.completed));

  async function toggleComplete(ch) {
    const [success, xp, level] = await invoke('complete_challenge', { id: ch.id });
    if (success) {
      ch.completed = true;
    }
  }

  function difficultyStars(n) {
    return '★'.repeat(n) + '☆'.repeat(5 - n);
  }

  function formatDetails(text) {
    if (!text) return '';
    let html = '';
    let inCodeBlock = false;
    let codeLang = '';
    let codeLines = [];
    let inSpecial = null; // 'req' or 'verify'

    for (const line of text.split('\n')) {
      if (line.startsWith('```')) {
        if (inCodeBlock) {
          const code = codeLines.join('\n');
          html += `<div class="det-code-block"><button class="copy-btn" data-copy-btn data-code="${escapeAttr(code)}">📋</button><pre class="det-code"><code>${code}</code></pre></div>`;
          codeLines = [];
          inCodeBlock = false;
        } else {
          inCodeBlock = true;
          codeLang = line.slice(3).trim();
        }
        continue;
      }

      if (inCodeBlock) {
        codeLines.push(escapeHtml(line));
        continue;
      }

      const trimmed = line.trim();
      if (!trimmed) { html += '<br>'; continue; }

      if (trimmed.startsWith('## ')) {
        if (inSpecial) { html += '</div>'; inSpecial = null; }
        html += `<h3 class="det-h3">${escapeHtml(trimmed.slice(3))}</h3>`;
      } else if (trimmed.startsWith('### ')) {
        if (inSpecial) { html += '</div>'; inSpecial = null; }
        const title = trimmed.slice(4);
        const lower = title.toLowerCase();
        if (lower.includes('wymagania')) {
          html += '<div class="det-req-box">';
          html += `<div class="det-req-head">📋 ${escapeHtml(title)}</div>`;
          inSpecial = 'req';
        } else if (lower.includes('weryfikacja')) {
          html += '<div class="det-verify-box">';
          html += `<div class="det-verify-head">✅ ${escapeHtml(title)}</div>`;
          inSpecial = 'verify';
        } else {
          html += `<h4 class="det-h4">${escapeHtml(title)}</h4>`;
        }
      } else if (trimmed.startsWith('> ')) {
        html += `<div class="det-output">${formatInline(trimmed.slice(2))}</div>`;
      } else if (trimmed.startsWith('**') && trimmed.endsWith('**')) {
        html += `<p class="det-highlight"><strong>${escapeHtml(trimmed.slice(2, -2))}</strong></p>`;
      } else if (trimmed.startsWith('- ')) {
        html += `<li class="det-li">${formatInline(trimmed.slice(2))}</li>`;
      } else if (trimmed.startsWith('|')) {
        // table row - skip for simplicity
      } else {
        html += `<p class="det-p">${formatInline(trimmed)}</p>`;
      }
    }
    if (inCodeBlock) {
      const code = codeLines.join('\n');
      html += `<div class="det-code-block"><button class="copy-btn" data-copy-btn data-code="${escapeAttr(code)}">📋</button><pre class="det-code"><code>${code}</code></pre></div>`;
    }
    if (inSpecial) html += '</div>';
    return html;
  }

  function formatInline(text) {
    const escaped = escapeHtml(text);
    return escaped
      .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
      .replace(/`(.+?)`/g, '<code class="det-inline-code">$1</code>');
  }

  function escapeHtml(str) {
    return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }

  function escapeAttr(str) {
    return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#39;');
  }

  function handleCopyClick(e) {
    const btn = e.target.closest('[data-copy-btn]');
    if (!btn) return;
    btn.textContent = '✓';
    btn.classList.add('copied');
    navigator.clipboard.writeText(btn.dataset.code).catch(() => {});
    setTimeout(() => {
      btn.textContent = '📋';
      btn.classList.remove('copied');
    }, 1500);
  }

  // Step-by-step checklist
  let stepMode = $state(false);
  let steps = $state([]);
  let checkedSteps = $state({});

  $effect(() => {
    if (detailChallenge && stepMode) {
      const saved = localStorage.getItem(`cs-${detailChallenge.id}`);
      checkedSteps = saved ? JSON.parse(saved) : {};
    }
  });

  function toggleStepMode() {
    stepMode = !stepMode;
    if (stepMode && detailChallenge) {
      steps = extractCodeBlocks(detailChallenge.details);
      const saved = localStorage.getItem(`cs-${detailChallenge.id}`);
      checkedSteps = saved ? JSON.parse(saved) : {};
    }
  }

  function extractCodeBlocks(text) {
    if (!text) return [];
    const blocks = [];
    let inCode = false;
    let lines = [];
    for (const line of text.split('\n')) {
      if (line.startsWith('```')) {
        if (inCode) {
          const code = lines.join('\n').trim();
          if (code) blocks.push(code);
          lines = [];
          inCode = false;
        } else {
          inCode = true;
        }
      } else if (inCode) {
        lines.push(line);
      }
    }
    return blocks;
  }

  function toggleStep(idx) {
    checkedSteps[idx] = !checkedSteps[idx];
    checkedSteps = { ...checkedSteps };
    localStorage.setItem(`cs-${detailChallenge.id}`, JSON.stringify(checkedSteps));
  }

  let stepsDone = $derived(Object.values(checkedSteps).filter(Boolean).length);
  let stepsTotal = $derived(steps.length);
</script>

<div class="challenges-page">
  <header class="page-header">
    <h1>🎯 Wyzwania</h1>
    <p>Wykonuj wyzwania, zdobywaj XP i odblokowuj kamienie milowe</p>
  </header>

  <div class="filters">
    <button class="filter-btn" class:active={filter === 'all'} onclick={() => filter = 'all'}>Wszystkie</button>
    <button class="filter-btn" class:active={filter === 'active'} onclick={() => filter = 'active'}>Aktywne</button>
    <button class="filter-btn" class:active={filter === 'completed'} onclick={() => filter = 'completed'}>Ukończone</button>
  </div>

  {#if loading}
    <div class="loading">Ładowanie...</div>
  {:else}
    <div class="challenge-grid">
      {#each filtered as ch (ch.id)}
        {@const cat = getCategoryInfo(ch.category)}
        <div class="challenge-card" class:completed={ch.completed}>
          <div class="ch-top">
            <span class="ch-category" style="background: {cat.color}20; color: {cat.color}">
              {cat.name}
            </span>
            <span class="ch-difficulty">{difficultyStars(ch.difficulty)}</span>
          </div>
          <h3 class="ch-title">{ch.title}</h3>
          <p class="ch-desc">{ch.description}</p>
          <div class="ch-footer">
            <span class="ch-xp">+{ch.difficulty * 10} XP</span>
            <div class="ch-actions">
              {#if ch.details}
                <button class="details-btn" onclick={() => detailChallenge = ch}>📖</button>
              {/if}
              {#if !ch.completed}
                <button class="complete-btn" onclick={() => toggleComplete(ch)}>
                  Oznacz jako ukończone
                </button>
              {:else}
                <span class="completed-badge">✓ Ukończono</span>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<svelte:window onclick={handleCopyClick} />

<!-- Modal szczegółów -->
{#if detailChallenge}
  {@const cat = getCategoryInfo(detailChallenge.category)}
    <div class="modal-backdrop" role="presentation" onclick={() => detailChallenge = null} onkeydown={(e) => e.key === 'Escape' && (detailChallenge = null)}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div>
          <span class="modal-category" style="background: {cat.color}20; color: {cat.color}">{cat.name}</span>
          <span class="modal-diff">{difficultyStars(detailChallenge.difficulty)}</span>
        </div>
        <button class="modal-close" onclick={() => detailChallenge = null}>✕</button>
      </div>
      <h2 class="modal-title">{detailChallenge.title}</h2>
      <p class="modal-desc">{detailChallenge.description}</p>
      <div class="modal-body">
        <div class="modal-toolbar">
          {#if detailChallenge.details}
            <button class="step-toggle" class:active={stepMode} onclick={toggleStepMode}>
              {stepMode ? '◉ ' : '○ '} Krok po kroku
              {#if stepMode && stepsTotal > 0}
                <span class="step-progress">{stepsDone}/{stepsTotal}</span>
              {/if}
            </button>
          {/if}
        </div>
        {#if detailChallenge.details}
          {@html formatDetails(detailChallenge.details)}
          {#if stepMode && steps.length > 0}
            <div class="step-list">
              <h3 class="det-h3">Lista kroków</h3>
              {#each steps as code, idx}
                <label class="step-item" class:checked={checkedSteps[idx]}>
                  <input type="checkbox" checked={checkedSteps[idx] ?? false} onchange={() => toggleStep(idx)} />
                  <code class="step-code">{code.split('\n')[0]}{code.includes('\n') ? ' …' : ''}</code>
                </label>
              {/each}
            </div>
          {/if}
        {:else}
          <p class="det-p">Brak szczegółowej lekcji dla tego wyzwania.</p>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .challenges-page { max-width: 1000px; }
  .page-header h1 { font-size: 28px; font-weight: 700; color: #f1f5f9; margin-bottom: 4px; }
  .page-header p { color: #64748b; margin-bottom: 20px; }

  .filters { display: flex; gap: 8px; margin-bottom: 20px; }
  .filter-btn { padding: 8px 16px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 8px; cursor: pointer; font-size: 13px; transition: all 0.15s; }
  .filter-btn:hover { background: #334155; color: #e2e8f0; }
  .filter-btn.active { background: #0ea5e9; color: #fff; border-color: #0ea5e9; }

  .loading { color: #64748b; }

  .challenge-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(360px, 1fr)); gap: 16px; }

  .challenge-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; transition: all 0.2s; }
  .challenge-card:hover { border-color: #475569; }
  .challenge-card.completed { opacity: 0.7; border-color: #166534; }

  .ch-top { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
  .ch-category { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; letter-spacing: 0.5px; }
  .ch-difficulty { color: #f59e0b; font-size: 13px; letter-spacing: 1px; }
  .ch-title { font-size: 16px; font-weight: 600; color: #f1f5f9; margin-bottom: 8px; }
  .ch-desc { font-size: 13px; color: #94a3b8; line-height: 1.5; margin-bottom: 16px; }

  .ch-footer { display: flex; justify-content: space-between; align-items: center; }
  .ch-xp { font-size: 13px; font-weight: 600; color: #38bdf8; }
  .ch-actions { display: flex; align-items: center; gap: 6px; }
  .details-btn { background: #334155; border: none; color: #94a3b8; cursor: pointer; padding: 6px 10px; border-radius: 6px; font-size: 14px; transition: all 0.15s; }
  .details-btn:hover { background: #475569; color: #e2e8f0; }
  .complete-btn { padding: 8px 14px; background: #0ea5e9; color: #fff; border: none; border-radius: 8px; cursor: pointer; font-size: 12px; font-weight: 600; transition: background 0.15s; }
  .complete-btn:hover { background: #0284c7; }
  .completed-badge { color: #22c55e; font-weight: 600; font-size: 13px; }

  /* Modal */
  .modal-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000; padding: 40px; }
  .modal { background: #1e293b; border: 1px solid #334155; border-radius: 16px; max-width: 800px; max-height: 80vh; width: 100%; overflow-y: auto; padding: 28px; }
  .modal-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px; }
  .modal-header > div { display: flex; align-items: center; gap: 10px; }
  .modal-category { font-size: 11px; font-weight: 600; padding: 4px 10px; border-radius: 6px; text-transform: uppercase; }
  .modal-diff { color: #f59e0b; font-size: 13px; letter-spacing: 1px; }
  .modal-close { background: none; border: none; color: #64748b; font-size: 20px; cursor: pointer; padding: 4px; }
  .modal-close:hover { color: #e2e8f0; }
  .modal-title { font-size: 22px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .modal-desc { font-size: 14px; color: #94a3b8; margin-bottom: 20px; line-height: 1.5; }

  /* svelte-ignore css_unused_selector */
  .det-h3 { font-size: 16px; font-weight: 600; color: #38bdf8; margin-top: 20px; margin-bottom: 8px; }
  /* svelte-ignore css_unused_selector */
  .det-h4 { font-size: 14px; font-weight: 600; color: #94a3b8; margin-top: 16px; margin-bottom: 6px; text-transform: uppercase; letter-spacing: 0.3px; }
  /* svelte-ignore css_unused_selector */
  .det-p { font-size: 14px; color: #cbd5e1; line-height: 1.6; margin-bottom: 4px; }
  /* svelte-ignore css_unused_selector */
  .det-highlight { font-size: 14px; color: #f1f5f9; background: #0f172a; padding: 8px 12px; border-radius: 6px; border-left: 3px solid #0ea5e9; }
  /* svelte-ignore css_unused_selector */
  .det-req-box { background: #1e1b0e; border: 1px solid #854d0e; border-radius: 10px; padding: 12px 16px; margin: 12px 0; }
  /* svelte-ignore css_unused_selector */
  .det-req-head { font-size: 13px; font-weight: 700; color: #fbbf24; margin-bottom: 6px; }
  /* svelte-ignore css_unused_selector */
  .det-verify-box { background: #0f1f0f; border: 1px solid #166534; border-radius: 10px; padding: 12px 16px; margin: 12px 0; }
  /* svelte-ignore css_unused_selector */
  .det-verify-head { font-size: 13px; font-weight: 700; color: #22c55e; margin-bottom: 6px; }
  /* svelte-ignore css_unused_selector */
  .det-output { font-size: 13px; color: #a3e635; background: #0f172a; padding: 8px 14px; border-radius: 6px; border-left: 3px solid #65a30d; margin: 4px 0 8px 0; font-family: 'JetBrains Mono', monospace; white-space: pre-wrap; }
  .det-code-block { position: relative; margin: 8px 0; }
  .copy-btn { position: absolute; top: 6px; right: 6px; background: #1e293b; border: 1px solid #475569; color: #64748b; cursor: pointer; padding: 4px 8px; border-radius: 6px; font-size: 14px; line-height: 1; transition: all 0.15s; z-index: 1; }
  .copy-btn:hover { background: #334155; color: #e2e8f0; border-color: #64748b; }
  .copy-btn.copied { background: #166534; border-color: #22c55e; color: #22c55e; }
  /* svelte-ignore css_unused_selector */
  .det-code { background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 14px; overflow-x: auto; margin: 0; }
  .det-code code { font-family: 'JetBrains Mono', 'Fira Code', monospace; font-size: 13px; color: #22c55e; white-space: pre; }
  /* svelte-ignore css_unused_selector */
  .det-inline-code { font-family: 'JetBrains Mono', monospace; font-size: 13px; background: #334155; padding: 2px 6px; border-radius: 4px; color: #38bdf8; }
  /* svelte-ignore css_unused_selector */
  .det-li { font-size: 14px; color: #cbd5e1; line-height: 1.6; margin-left: 20px; }

  .modal-toolbar { display: flex; gap: 8px; margin-bottom: 12px; }
  .step-toggle { display: inline-flex; align-items: center; gap: 6px; padding: 6px 12px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 8px; cursor: pointer; font-size: 12px; transition: all 0.15s; }
  .step-toggle:hover { background: #334155; color: #e2e8f0; }
  .step-toggle.active { background: #0ea5e9; color: #fff; border-color: #0ea5e9; }
  .step-progress { background: rgba(255,255,255,0.15); padding: 1px 6px; border-radius: 4px; font-size: 11px; font-weight: 600; }
  .step-list { margin-top: 20px; padding: 16px; background: #0f172a; border: 1px solid #334155; border-radius: 12px; }
  .step-item { display: flex; align-items: center; gap: 10px; padding: 10px 12px; border-radius: 8px; cursor: pointer; transition: background 0.15s; }
  .step-item:hover { background: #1e293b; }
  .step-item.checked { opacity: 0.6; }
  .step-item.checked .step-code { text-decoration: line-through; color: #64748b; }
  .step-item input[type="checkbox"] { accent-color: #22c55e; width: 16px; height: 16px; cursor: pointer; flex-shrink: 0; }
  .step-code { font-family: 'JetBrains Mono', monospace; font-size: 12px; color: #38bdf8; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

  @media (max-width: 768px) {
    .modal-backdrop { padding: 16px; }
    .modal { padding: 20px; }
  }
</style>
