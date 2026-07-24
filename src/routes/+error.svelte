<script>
  import { page } from '$app/stores';
</script>

<div class="error-page">
  <div class="error-card">
    <div class="error-icon">
      {#if $page.status === 404}
        🔍
      {:else}
        ⚠️
      {/if}
    </div>
    <h1>Błąd {$page.status || 'nieznany'}</h1>
    <p>{$page.error?.message || 'Coś poszło nie tak.'}</p>
    <a href="/" class="btn-primary">🏠 Wróć do dashboardu</a>
  </div>
</div>

<style>
  .error-page {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: #0f172a;
  }

  .error-card {
    text-align: center;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 16px;
    padding: 48px;
    max-width: 420px;
  }

  .error-icon {
    font-size: 64px;
    margin-bottom: 16px;
  }

  h1 {
    font-size: 28px;
    font-weight: 700;
    color: #f1f5f9;
    margin-bottom: 8px;
  }

  p {
    color: #64748b;
    margin-bottom: 24px;
    font-size: 14px;
  }
</style>
