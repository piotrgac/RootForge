<script>
  let { percent = 0, height = 8, color = 'linear-gradient(90deg, #0ea5e9, #38bdf8)', label = undefined } = $props();
</script>

<div class="progress-bar-bg" style="height: {height}px">
  <div class="progress-bar-fill" style="width: {Math.min(percent, 100)}%; background: {color}"></div>
</div>
{#if label}
  <span class="progress-label">{label}</span>
{/if}
