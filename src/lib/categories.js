export const categoryInfo = {
  linux: { name: 'Linux', color: '#3b82f6' },
  system: { name: 'System', color: '#10b981' },
  network: { name: 'Sieć', color: '#8b5cf6' },
  security: { name: 'Bezpieczeństwo', color: '#ef4444' },
  shell: { name: 'Shell', color: '#f59e0b' },
  devops: { name: 'DevOps', color: '#06b6d4' },
};

export function getCategoryInfo(cat) {
  return categoryInfo[cat] || { name: cat, color: '#38bdf8' };
}

export function getCategoryStats(challenges) {
  const cats = {};
  for (const c of challenges) {
    const info = getCategoryInfo(c.category);
    if (!cats[c.category]) {
      cats[c.category] = { name: info.name, color: info.color, total: 0, done: 0 };
    }
    cats[c.category].total++;
    if (c.completed) cats[c.category].done++;
  }
  return Object.values(cats).map(c => ({ ...c, percent: c.total > 0 ? Math.round((c.done / c.total) * 100) : 0 }));
}
