<script>
  let { correct, explanation, hint = undefined, children = undefined } = $props();
</script>

<div class="result-box" class:correct class:wrong={!correct}>
  <strong>{correct ? '✅ Poprawna odpowiedź!' : '❌ Niepoprawna'}</strong>
  <p>{explanation}</p>
  {#if !correct && hint && children}
    {@render children()}
  {/if}
</div>

<style>
  .result-box {
    padding: 16px;
    border-radius: 10px;
    margin-bottom: 12px;
    font-size: 14px;
    line-height: 1.6;
  }
  .result-box.correct {
    background: #22c55e20;
    border: 1px solid #22c55e;
    color: #bbf7d0;
  }
  .result-box.wrong {
    background: #ef444420;
    border: 1px solid #ef4444;
    color: #fecaca;
  }
  .result-box strong {
    display: block;
    margin-bottom: 8px;
    font-size: 16px;
  }
</style>
