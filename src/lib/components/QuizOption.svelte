<script>
  let { text, index, correctIndex, selected, showResult, disabled = false, onselect } = $props();

  let letter = $derived(String.fromCharCode(65 + index));
  let isSelected = $derived(selected === index);
  let isCorrect = $derived(showResult && index === correctIndex);
  let isWrong = $derived(showResult && isSelected && index !== correctIndex);
</script>

<button
  class="quiz-option"
  class:selected={isSelected}
  class:correct={isCorrect}
  class:wrong={isWrong}
  onclick={() => { if (!showResult) onselect(index); }}
  disabled={disabled || showResult}
>
  <span class="opt-letter">{letter}</span>
  <span class="opt-text">{text}</span>
  {#if isCorrect}
    <span class="opt-icon">✓</span>
  {:else if isWrong}
    <span class="opt-icon">✗</span>
  {/if}
</button>

<style>
  .quiz-option {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 10px;
    color: #e2e8f0;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s;
    font-size: 14px;
  }
  .quiz-option:hover:not(:disabled) {
    border-color: #475569;
    background: #1a2332;
  }
  .quiz-option.selected {
    border-color: #0ea5e9;
    background: #0ea5e920;
  }
  .quiz-option.correct {
    border-color: #22c55e;
    background: #22c55e20;
  }
  .quiz-option.wrong {
    border-color: #ef4444;
    background: #ef444420;
  }
  .quiz-option:disabled {
    cursor: default;
  }
  .opt-letter {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: #334155;
    font-weight: 700;
    font-size: 13px;
    flex-shrink: 0;
  }
  .quiz-option.correct .opt-letter {
    background: #22c55e;
    color: #fff;
  }
  .quiz-option.wrong .opt-letter {
    background: #ef4444;
    color: #fff;
  }
  .opt-text {
    flex: 1;
  }
  .opt-icon {
    font-weight: 700;
    font-size: 16px;
  }
</style>
