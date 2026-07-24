<script>
  let { items, active, onchange } = $props();
</script>

<div class="filter-bar">
  {#each items as item}
    <button
      class="filter-btn"
      class:active={active === item.value}
      onclick={() => onchange(item.value)}
    >
      {item.label}
    </button>
  {/each}
</div>
