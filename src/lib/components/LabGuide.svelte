<script>
  let { project, oncomplete, onsaverepo } = $props();

  let revealedStep = $state(0);
  let checkedSteps = $state({});
  let showFull = $state(false);
  let editingRepo = $state(false);
  let repoUrl = $state(project.github_repo || '');

  let parsed = $derived(parseGuide(project.guide || ''));
  let totalSteps = $derived(parsed.steps.length);
  let doneSteps = $derived(Object.values(checkedSteps).filter(Boolean).length);

  $effect(() => {
    const saved = localStorage.getItem(`lab-${project.id}`);
    if (saved) {
      const data = JSON.parse(saved);
      checkedSteps = data.checkedSteps || {};
      revealedStep = data.revealedStep || 0;
    }
  });

  $effect(() => {
    localStorage.setItem(`lab-${project.id}`, JSON.stringify({ checkedSteps, revealedStep }));
  });

  function revealNext() {
    if (revealedStep < totalSteps) revealedStep++;
  }

  function toggleStep(idx) {
    checkedSteps[idx] = !checkedSteps[idx];
    checkedSteps = { ...checkedSteps };
  }

  function resetLab() {
    checkedSteps = {};
    revealedStep = 0;
    showFull = false;
  }

  function saveRepo() {
    if (repoUrl.trim()) {
      onsaverepo?.(project.id, repoUrl.trim());
    }
    editingRepo = false;
  }

  function parseGuide(text) {
    const lines = text.split('\n');
    let title = '';
    let goal = '';
    const steps = [];
    let currentStep = null;
    let i = 0;

    // Parse title
    if (lines[0]?.startsWith('LAB:')) {
      title = lines[0].slice(4).trim();
      i = 2; // skip title line and ==== separator
    }
    if (lines[i]?.startsWith('CEL:')) {
      goal = lines[i].slice(4).trim();
      i++;
    }

    for (; i < lines.length; i++) {
      const line = lines[i];
      const stepMatch = line.match(/^KROK (\d+)\s*[–-]\s*(.+)$/);
      const verifyMatch = line.match(/^WERYFIKACJA:/);

      if (stepMatch) {
        if (currentStep) steps.push(currentStep);
        currentStep = { num: parseInt(stepMatch[1]), title: stepMatch[2].trim(), lines: [] };
        i++; // skip --- separator
        continue;
      }

      if (verifyMatch || line.match(/^==+/)) {
        if (currentStep) {
          currentStep.lines.push(line);
          steps.push(currentStep);
          currentStep = null;
        }
        continue;
      }

      if (currentStep) {
        currentStep.lines.push(line);
      }
    }
    if (currentStep) steps.push(currentStep);

    // Gather remaining lines after steps as notes
    const allStepLines = new Set();
    for (const s of steps) {
      for (const l of s.lines) allStepLines.add(l);
    }

    return { title, goal, steps };
  }

  function formatLines(lines) {
    let html = '';
    let inCode = false;
    let codeBuf = [];

    function flushCode() {
      if (codeBuf.length > 0) {
        html += `<pre class="lg-code"><code>${escapeHtml(codeBuf.join('\n'))}</code></pre>`;
        codeBuf = [];
      }
    }

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed) { flushCode(); html += '<br>'; continue; }

      if (trimmed.startsWith('```')) {
        if (inCode) { flushCode(); inCode = false; }
        else { flushCode(); inCode = true; }
        continue;
      }

      if (inCode) { codeBuf.push(line); continue; }

      flushCode();

      if (trimmed.startsWith('#')) {
        html += `<p class="lg-comment">${escapeHtml(trimmed)}</p>`;
      } else if (trimmed.startsWith('$ ')) {
        html += `<pre class="lg-cmd"><code>${escapeHtml(trimmed.slice(2))}</code></pre>`;
      } else if (line.startsWith('  ') || line.startsWith('\t')) {
        codeBuf.push(line);
      } else {
        html += `<p class="lg-line">${escapeHtml(trimmed)}</p>`;
      }
    }
    flushCode();
    return html;
  }

  function escapeHtml(str) {
    return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }
</script>

<div class="lab-guide">
  <div class="lab-header">
    <div class="lab-title-row">
      <span class="lab-emoji">🔬</span>
      <div>
        <h3 class="lab-title">{parsed.title}</h3>
        <p class="lab-goal">{parsed.goal}</p>
      </div>
    </div>
    <div class="lab-meta">
      <span class="lab-badge">{totalSteps} kroków</span>
      {#if doneSteps > 0}
        <span class="lab-badge lab-badge-done">{doneSteps}/{totalSteps} ✓</span>
      {/if}
    </div>
  </div>

  <div class="lab-progress">
    <div class="lab-progress-bar">
      <div class="lab-progress-fill" style="width: {totalSteps > 0 ? (doneSteps / totalSteps) * 100 : 0}%"></div>
    </div>
    <span class="lab-progress-text">{doneSteps}/{totalSteps}</span>
  </div>

  <!-- Action bar -->
  <div class="lab-actions">
    {#if revealedStep < totalSteps && !showFull}
      <button class="lab-btn lab-btn-primary" onclick={revealNext}>
        {revealedStep === 0 ? '🚀 Rozpocznij lab' : '📖 Pokaż krok ' + (revealedStep + 1)}
      </button>
    {/if}
    {#if revealedStep > 0 && !showFull}
      <button class="lab-btn lab-btn-ghost" onclick={() => showFull = true}>
        👁️ Pokaż wszystko
      </button>
    {/if}
    {#if showFull}
      <button class="lab-btn lab-btn-ghost" onclick={resetLab}>
        🔄 Resetuj postęp
      </button>
    {/if}
  </div>

  <!-- Steps -->
  <div class="lab-steps">
    {#each parsed.steps as step, idx}
      {@const isRevealed = showFull || idx < revealedStep}
      <div class="lab-step" class:revealed={isRevealed} class:done={checkedSteps[idx]}>
        {#if isRevealed}
          <div class="step-header">
            <button class="step-check" onclick={() => toggleStep(idx)}>
              {checkedSteps[idx] ? '✅' : '⬜'}
            </button>
            <span class="step-num">Krok {step.num}</span>
            <span class="step-title">{step.title}</span>
          </div>
          <div class="step-body">
            {@html formatLines(step.lines)}
          </div>
        {:else}
          <div class="step-locked">
            <span class="step-lock-icon">🔒</span>
            <span>Krok {step.num} – {step.title}</span>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <!-- Repo + Complete -->
  <div class="lab-footer">
    {#if !project.github_repo}
      {#if editingRepo}
        <div class="repo-edit">
          <input type="url" bind:value={repoUrl} placeholder="https://github.com/twoj-repo" class="repo-input" />
          <button class="lab-btn lab-btn-primary" onclick={saveRepo}>💾 Zapisz</button>
          <button class="lab-btn lab-btn-ghost" onclick={() => editingRepo = false}>Anuluj</button>
        </div>
      {:else}
        <button class="lab-btn lab-btn-secondary" onclick={() => editingRepo = true}>
          🔗 Dodaj link do GitHub
        </button>
      {/if}
    {:else}
      <a href={project.github_repo} target="_blank" class="repo-link">🔗 {project.github_repo}</a>
    {/if}

    {#if !project.completed}
      <button class="lab-btn lab-btn-success" onclick={() => oncomplete?.(project.id)}>
        ✓ Oznacz jako ukończone
      </button>
    {/if}
  </div>
</div>

<style>
  .lab-guide { margin-bottom: 8px; }
  .lab-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 12px; flex-wrap: wrap; gap: 8px; }
  .lab-title-row { display: flex; gap: 10px; align-items: flex-start; }
  .lab-emoji { font-size: 24px; }
  .lab-title { font-size: 15px; font-weight: 600; color: #f1f5f9; margin-bottom: 2px; }
  .lab-goal { font-size: 12px; color: #64748b; }
  .lab-meta { display: flex; gap: 6px; }
  .lab-badge { font-size: 10px; font-weight: 600; padding: 3px 8px; border-radius: 4px; background: #334155; color: #94a3b8; }
  .lab-badge-done { background: #16653430; color: #22c55e; }

  .lab-progress { display: flex; align-items: center; gap: 8px; margin-bottom: 12px; }
  .lab-progress-bar { flex: 1; height: 6px; background: #1e293b; border: 1px solid #334155; border-radius: 3px; overflow: hidden; }
  .lab-progress-fill { height: 100%; background: linear-gradient(90deg, #0ea5e9, #38bdf8); border-radius: 3px; transition: width 0.3s; }
  .lab-progress-text { font-size: 11px; color: #64748b; min-width: 40px; text-align: right; }

  .lab-actions { display: flex; gap: 8px; margin-bottom: 12px; flex-wrap: wrap; }
  .lab-btn { padding: 8px 16px; border: none; border-radius: 8px; font-size: 12px; font-weight: 600; cursor: pointer; transition: all 0.15s; white-space: nowrap; }
  .lab-btn-primary { background: #0ea5e9; color: #fff; }
  .lab-btn-primary:hover { background: #0284c7; }
  .lab-btn-secondary { background: #334155; color: #94a3b8; }
  .lab-btn-secondary:hover { background: #475569; color: #e2e8f0; }
  .lab-btn-ghost { background: transparent; color: #64748b; border: 1px solid #334155; }
  .lab-btn-ghost:hover { color: #e2e8f0; border-color: #475569; }
  .lab-btn-success { background: #22c55e; color: #fff; }
  .lab-btn-success:hover { background: #16a34a; }

  .lab-steps { display: flex; flex-direction: column; gap: 8px; margin-bottom: 12px; }
  .lab-step { border: 1px solid #334155; border-radius: 10px; overflow: hidden; transition: all 0.2s; }
  .lab-step.revealed { background: #1e293b; }
  .lab-step.done { border-color: #166534; opacity: 0.85; }
  .lab-step:not(.revealed) { background: #0f172a; opacity: 0.5; }

  .step-locked { display: flex; align-items: center; gap: 8px; padding: 12px 14px; color: #64748b; font-size: 13px; }
  .step-lock-icon { font-size: 14px; }

  .step-header { display: flex; align-items: center; gap: 8px; padding: 10px 14px; background: #0f172a; border-bottom: 1px solid #334155; cursor: pointer; }
  .step-check { background: none; border: none; font-size: 16px; cursor: pointer; padding: 0; line-height: 1; }
  .step-num { font-size: 11px; font-weight: 700; color: #0ea5e9; text-transform: uppercase; }
  .step-title { font-size: 13px; font-weight: 600; color: #e2e8f0; }

  .step-body { padding: 12px 14px; }
  .step-body :global(.lg-comment) { font-size: 13px; color: #64748b; margin-bottom: 4px; }
  .step-body :global(.lg-line) { font-size: 13px; color: #cbd5e1; line-height: 1.6; margin-bottom: 4px; }
  .step-body :global(.lg-cmd) { margin: 6px 0; padding: 8px 12px; background: #0f172a; border: 1px solid #334155; border-left: 3px solid #0ea5e9; border-radius: 6px; overflow-x: auto; }
  .step-body :global(.lg-cmd code) { font-family: 'JetBrains Mono', monospace; font-size: 13px; color: #22c55e; white-space: pre; }
  .step-body :global(.lg-code) { margin: 6px 0; padding: 10px 12px; background: #0f172a; border: 1px solid #334155; border-radius: 6px; overflow-x: auto; }
  .step-body :global(.lg-code code) { font-family: 'JetBrains Mono', monospace; font-size: 13px; color: #e2e8f0; white-space: pre; }

  .lab-footer { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 8px; padding-top: 8px; }
  .repo-edit { display: flex; gap: 6px; align-items: center; flex-wrap: wrap; }
  .repo-input { flex: 1; min-width: 200px; padding: 8px 12px; background: #0f172a; border: 1px solid #334155; border-radius: 6px; color: #e2e8f0; font-size: 13px; }
  .repo-input:focus { outline: none; border-color: #0ea5e9; }
  .repo-link { color: #38bdf8; font-size: 13px; text-decoration: none; }
  .repo-link:hover { text-decoration: underline; }
</style>
