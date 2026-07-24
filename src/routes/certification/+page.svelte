<script>
  import { certificationRoadmaps, getRoadmapByName } from '$lib/certification';
  import { getCategoryInfo } from '$lib/categories';

  let activeRoadmap = $state(certificationRoadmaps[0].name);

  let roadmap = $derived(getRoadmapByName(activeRoadmap));
</script>

<div class="cert-page">
  <header class="page-header">
    <h1>🎓 Certyfikacje</h1>
    <p>Mapa drogi do LPIC i Red Hat Certification</p>
  </header>

  <div class="cert-selector">
    {#each certificationRoadmaps as road}
      <button
        class="cert-btn"
        class:active={activeRoadmap === road.name}
        onclick={() => activeRoadmap = road.name}
      >
        {road.name}
      </button>
    {/each}
  </div>

  {#if roadmap}
    <div class="roadmap">
      <div class="road-header">
        <h2>{roadmap.name} – {roadmap.fullName}</h2>
        <p class="road-desc">{roadmap.description}</p>
        <a href={roadmap.url} target="_blank" rel="noopener noreferrer" class="road-link">
          Oficjalna strona →
        </a>
      </div>

      {#if roadmap.exams}
        <div class="exams">
          <h3>Egzaminy</h3>
          <div class="exam-grid">
            {#each roadmap.exams as exam}
              <div class="exam-card">
                <span class="exam-code">{exam.code}</span>
                <span class="exam-name">{exam.name}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="topics">
        <h3>Zagadnienia</h3>
        <div class="topic-grid">
          {#each roadmap.topics as topic}
            {@const info = getCategoryInfo(topic.category)}
            <div class="topic-card">
              <div class="topic-header">
                <span class="topic-badge" style="background: {info.color}20; color: {info.color}">
                  {info.name}
                </span>
                <h4>{topic.name}</h4>
              </div>
              <ul class="topic-tasks">
                {#each topic.tasks as task}
                  <li>{task}</li>
                {/each}
              </ul>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .cert-page { max-width: 1000px; }
  .cert-selector { display: flex; gap: 8px; margin-bottom: 24px; }
  .cert-btn { padding: 10px 20px; border: 1px solid #334155; background: #1e293b; color: #94a3b8; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 600; transition: all 0.15s; }
  .cert-btn:hover { background: #334155; color: #e2e8f0; }
  .cert-btn.active { background: #0ea5e9; color: #fff; border-color: #0ea5e9; }

  .road-header { margin-bottom: 24px; }
  .road-header h2 { font-size: 20px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px; }
  .road-desc { color: #94a3b8; font-size: 14px; line-height: 1.6; margin-bottom: 8px; }
  .road-link { color: #38bdf8; font-size: 13px; text-decoration: none; }
  .road-link:hover { text-decoration: underline; }

  .exams { margin-bottom: 24px; }
  .exams h3, .topics h3 { font-size: 16px; font-weight: 600; color: #f1f5f9; margin-bottom: 12px; }
  .exam-grid { display: flex; gap: 12px; }
  .exam-card { background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 16px; flex: 1; }
  .exam-code { display: block; font-size: 20px; font-weight: 800; color: #38bdf8; margin-bottom: 4px; }
  .exam-name { font-size: 13px; color: #94a3b8; }

  .topic-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px; }
  .topic-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; }
  .topic-header { margin-bottom: 12px; }
  .topic-badge { font-size: 10px; font-weight: 600; padding: 3px 8px; border-radius: 4px; text-transform: uppercase; display: inline-block; margin-bottom: 8px; }
  .topic-header h4 { font-size: 15px; font-weight: 600; color: #f1f5f9; }
  .topic-tasks { list-style: none; padding: 0; }
  .topic-tasks li { font-size: 13px; color: #94a3b8; padding: 4px 0; padding-left: 16px; position: relative; }
  .topic-tasks li::before { content: '›'; position: absolute; left: 0; color: #0ea5e9; font-weight: 700; }
</style>
