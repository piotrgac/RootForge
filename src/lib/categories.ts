export interface Challenge {
  id: number;
  title: string;
  description: string;
  category: string;
  difficulty: number;
  completed: boolean;
  details?: string | null;
  stage: number;
  exam_tag?: string | null;
}

export interface CategoryInfo {
  name: string;
  color: string;
}

export interface CategoryStat {
  name: string;
  color: string;
  total: number;
  done: number;
  percent: number;
}

type CategoryMap = Record<string, CategoryInfo>;

export const categoryInfo: CategoryMap = {
  linux: { name: 'Linux', color: '#3b82f6' },
  system: { name: 'System', color: '#10b981' },
  network: { name: 'Sieć', color: '#8b5cf6' },
  security: { name: 'Bezpieczeństwo', color: '#ef4444' },
  shell: { name: 'Shell', color: '#f59e0b' },
  devops: { name: 'DevOps', color: '#06b6d4' },
};

export function getCategoryInfo(cat: string): CategoryInfo {
  return categoryInfo[cat] || { name: cat, color: '#38bdf8' };
}

export function getCategoryStats(challenges: Challenge[]): CategoryStat[] {
  const cats: Record<string, CategoryStat> = {};
  for (const c of challenges) {
    const info = getCategoryInfo(c.category);
    if (!cats[c.category]) {
      cats[c.category] = { name: info.name, color: info.color, total: 0, done: 0, percent: 0 };
    }
    const stat = cats[c.category]!;
    stat.total++;
    if (c.completed) stat.done++;
  }
  return Object.values(cats).map(c => ({ ...c, percent: c.total > 0 ? Math.round((c.done / c.total) * 100) : 0 }));
}
