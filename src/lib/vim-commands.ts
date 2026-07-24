export interface VimChallenge {
  id: number;
  task: string;
  hint?: string;
  answer: string;
  alternatives?: string[];
  level: number;
  category: string;
}

export const vimChallenges: VimChallenge[] = [
  // LEVEL 1: Nawigacja
  { id: 1, task: 'Przesuń kursor o jedną linię w dół', hint: 'użyj klawisza pod paluchem', answer: 'j', level: 1, category: 'navigation' },
  { id: 2, task: 'Przesuń kursor o jedną linię w górę', hint: 'w górnym rzędzie', answer: 'k', level: 1, category: 'navigation' },
  { id: 3, task: 'Przesuń kursor w lewo', hint: 'lewą ręką', answer: 'h', level: 1, category: 'navigation' },
  { id: 4, task: 'Przesuń kursor w prawo', hint: 'prawą ręką', answer: 'l', level: 1, category: 'navigation' },
  { id: 5, task: 'Przeskocz o jedno słowo do przodu', hint: 'word', answer: 'w', level: 1, category: 'navigation' },
  { id: 6, task: 'Przeskocz o jedno słowo do tyłu', hint: 'backward', answer: 'b', level: 1, category: 'navigation' },
  { id: 7, task: 'Przeskocz na koniec słowa', hint: 'end of word', answer: 'e', level: 1, category: 'navigation' },
  { id: 8, task: 'Przeskocz na początek linii', answer: '0', alternatives: ['^'], level: 1, category: 'navigation' },
  { id: 9, task: 'Przeskocz na koniec linii', answer: '$', level: 1, category: 'navigation' },
  { id: 10, task: 'Przeskocz na pierwszy znak nie-biały w linii', answer: '^', level: 1, category: 'navigation' },
  { id: 11, task: 'Przeskocz na pierwszą linię pliku', answer: 'gg', level: 1, category: 'navigation' },
  { id: 12, task: 'Przeskocz na ostatnią linię pliku', answer: 'G', level: 1, category: 'navigation' },
  { id: 13, task: 'Przeskocz na linię z numerem N', answer: ':42', alternatives: ['42G', '42gg'], level: 1, category: 'navigation' },
  { id: 14, task: 'Przewiń pół strony w dół', hint: 'control + d', answer: '<C-d>', level: 1, category: 'navigation' },
  { id: 15, task: 'Przewiń pół strony w górę', hint: 'control + u', answer: '<C-u>', level: 1, category: 'navigation' },

  // LEVEL 2: Tryby i podstawowa edycja
  { id: 16, task: 'Przejdź w tryb INSERT przed kursorem', answer: 'i', level: 2, category: 'editing' },
  { id: 17, task: 'Przejdź w tryb INSERT za kursorem', hint: 'append', answer: 'a', level: 2, category: 'editing' },
  { id: 18, task: 'Przejdź w tryb INSERT na końcu linii', answer: 'A', level: 2, category: 'editing' },
  { id: 19, task: 'Dodaj pustą linię poniżej i wejdź w INSERT', hint: 'open new line', answer: 'o', level: 2, category: 'editing' },
  { id: 20, task: 'Dodaj pustą linię powyżej i wejdź w INSERT', answer: 'O', level: 2, category: 'editing' },
  { id: 21, task: 'Usuń znak pod kursorem', answer: 'x', level: 2, category: 'editing' },
  { id: 22, task: 'Usuń znak przed kursorem (backspace w Vimie)', answer: 'X', level: 2, category: 'editing' },
  { id: 23, task: 'Usuń całą linię', answer: 'dd', level: 2, category: 'editing' },
  { id: 24, task: 'Usuń 5 linii od kursora', answer: '5dd', level: 2, category: 'editing' },
  { id: 25, task: 'Usuń słowo od kursora', answer: 'dw', level: 2, category: 'editing' },
  { id: 26, task: 'Usuń do końca linii', answer: 'D', alternatives: ['d$'], level: 2, category: 'editing' },
  { id: 27, task: 'Skopiuj (yank) całą linię', answer: 'yy', alternatives: ['Y'], level: 2, category: 'editing' },
  { id: 28, task: 'Skopiuj 3 linie', answer: '3yy', alternatives: ['3Y', 'y3y'], level: 2, category: 'editing' },
  { id: 29, task: 'Wklej poniżej kursora', answer: 'p', level: 2, category: 'editing' },
  { id: 30, task: 'Wklej powyżej kursora', answer: 'P', level: 2, category: 'editing' },
  { id: 31, task: 'Cofnij ostatnią zmianę', answer: 'u', level: 2, category: 'editing' },
  { id: 32, task: 'Ponów cofnięcie (redo)', answer: '<C-r>', level: 2, category: 'editing' },

  // LEVEL 3: Zaawansowana edycja
  { id: 33, task: 'Zamień znak pod kursorem na inny', answer: 'r', level: 3, category: 'advanced' },
  { id: 34, task: 'Zamień (replace) tryb – nadpisuje znaki', answer: 'R', level: 3, category: 'advanced' },
  { id: 35, task: 'Usuń wnętrze cudzysłowu (")", będąc wewnątrz', hint: 'change inside quotes', answer: 'ci"', alternatives: ['ci"'], level: 3, category: 'advanced' },
  { id: 36, task: 'Zmień słowo od kursora (usuń + insert)', hint: 'change word', answer: 'cw', alternatives: ['ce'], level: 3, category: 'advanced' },
  { id: 37, task: 'Zmień całą linię (usuń + insert)', answer: 'cc', alternatives: ['S'], level: 3, category: 'advanced' },
  { id: 38, task: 'Usuń do nawiasu (będąc przed nawiasem)', hint: 'delete to parenthesis', answer: 'dt)', level: 3, category: 'advanced' },
  { id: 39, task: 'Usuń między nawiasami (włącznie)', hint: 'delete inside brackets', answer: 'di(', alternatives: ['di)'], level: 3, category: 'advanced' },
  { id: 40, task: 'Połącz dolną linię z bieżącą', answer: 'J', level: 3, category: 'advanced' },
  { id: 41, task: 'Tryb wizualny (zaznaczanie znakowe)', answer: 'v', level: 3, category: 'advanced' },
  { id: 42, task: 'Tryb wizualny linowy', answer: 'V', level: 3, category: 'advanced' },
  { id: 43, task: 'Tryb wizualny blokowy', answer: '<C-v>', level: 3, category: 'advanced' },
  { id: 44, task: 'Wcięcie zaznaczonego bloku w prawo', answer: '>', level: 3, category: 'advanced' },
  { id: 45, task: 'Wcięcie zaznaczonego bloku w lewo', answer: '<', level: 3, category: 'advanced' },
  { id: 46, task: 'Zamień wielkość liter znaku pod kursorem', answer: '~', level: 3, category: 'advanced' },
  { id: 47, task: 'Nagraj makro w rejestrze a', answer: 'qa', level: 3, category: 'advanced' },
  { id: 48, task: 'Zakończ nagrywanie makra', answer: 'q', level: 3, category: 'advanced' },
  { id: 49, task: 'Odtwórz makro z rejestru a', answer: '@a', level: 3, category: 'advanced' },
  { id: 50, task: 'Ponów ostatnie makro', answer: '@@', level: 3, category: 'advanced' },

  // LEVEL 4: Ex commands
  { id: 51, task: 'Zapisz plik', answer: ':w', level: 4, category: 'ex' },
  { id: 52, task: 'Zapisz i wyjdź', answer: ':wq', alternatives: ['ZZ', ':x'], level: 4, category: 'ex' },
  { id: 53, task: 'Wyjdź bez zapisu', answer: ':q!', alternatives: ['ZQ'], level: 4, category: 'ex' },
  { id: 54, task: 'Zapisz jako "nowy.txt"', answer: ':w nowy.txt', alternatives: [':saveas nowy.txt'], level: 4, category: 'ex' },
  { id: 55, task: 'Zamień wszystkie "foo" na "bar" w pliku', answer: ':%s/foo/bar/g', level: 4, category: 'ex' },
  { id: 56, task: 'Zamień "foo" na "bar" tylko w linii 5-10', answer: ':5,10s/foo/bar/g', level: 4, category: 'ex' },
  { id: 57, task: 'Zamień z potwierdzeniem', answer: ':%s/foo/bar/gc', level: 4, category: 'ex' },
  { id: 58, task: 'Usuń wszystkie linie zawierające "error"', answer: ':g/error/d', level: 4, category: 'ex' },
  { id: 59, task: 'Usuń wszystkie LINIE NIE zawierające "config"', answer: ':v/config/d', alternatives: [':g!/config/d'], level: 4, category: 'ex' },
  { id: 60, task: 'Wyświetl numery linii', answer: ':set nu', alternatives: [':set number'], level: 4, category: 'ex' },
  { id: 61, task: 'Wyłącz numery linii', answer: ':set nonu', alternatives: [':set nonumber'], level: 4, category: 'ex' },
  { id: 62, task: 'Włącz podświetlanie wyszukiwania', answer: ':set hls', alternatives: [':set hlsearch'], level: 4, category: 'ex' },
  { id: 63, task: 'Włącz tryb myszki', answer: ':set mouse=a', level: 4, category: 'ex' },
  { id: 64, task: 'Otwórz plik do edycji', answer: ':e plik.txt', alternatives: [':edit plik.txt'], level: 4, category: 'ex' },
  { id: 65, task: 'Podziel okno poziomo', answer: ':sp', alternatives: [':split'], level: 4, category: 'ex' },
  { id: 66, task: 'Podziel okno pionowo', answer: ':vsp', alternatives: [':vsplit'], level: 4, category: 'ex' },
  { id: 67, task: 'Przełącz między oknami', answer: '<C-w>w', alternatives: ['<C-w><C-w>'], level: 4, category: 'ex' },
  { id: 68, task: 'Uruchom polecenie shella z Vima', answer: ':!ls', alternatives: [':!ls -la'], level: 4, category: 'ex' },
  { id: 69, task: 'Wczytaj output polecenia do pliku', answer: ':r !date', alternatives: [':read !date'], level: 4, category: 'ex' },
  { id: 70, task: 'Otwórz pomoc dla słowa pod kursorem', answer: 'K', level: 4, category: 'ex' },

  // LEVEL 5: Mistrz Vima
  { id: 71, task: 'Zamień słowo pod kursorem (usuń + insert)', hint: 'change inner word', answer: 'ciw', level: 5, category: 'master' },
  { id: 72, task: 'Usuń do końca słowa', answer: 'd$', alternatives: ['D'], level: 5, category: 'master' },
  { id: 73, task: 'Zaznacz cały plik i skopiuj', answer: 'ggVGy', alternatives: ['gg"+yG', ':%y'], level: 5, category: 'master' },
  { id: 74, task: 'Wklej z rejestru systemowego (schowek)', answer: '"+p', alternatives: ['"*p'], level: 5, category: 'master' },
  { id: 75, task: 'Przeskocz do definicji (ctags)', answer: '<C-]>', level: 5, category: 'master' },
  { id: 76, task: 'Wróć ze skoku', answer: '<C-t>', level: 5, category: 'master' },
  { id: 77, task: 'Przełącz się do ostatniego pliku', answer: '<C-^>', alternatives: ['<C-6>'], level: 5, category: 'master' },
  { id: 78, task: 'Otwórz listę buforów', answer: ':ls', alternatives: [':buffers'], level: 5, category: 'master' },
  { id: 79, task: 'Przełącz do bufora o numerze N', answer: ':b3', alternatives: [':buffer 3'], level: 5, category: 'master' },
  { id: 80, task: 'Podziel okno i otwórz plik', answer: ':sp plik.txt', alternatives: [':split plik.txt'], level: 5, category: 'master' },
  { id: 81, task: 'Nagraj makro, które doda średnik na końcu linii', hint: 'qa + A; + <Esc> + j + q', answer: 'qaA;^[jq', level: 5, category: 'master' },
  { id: 82, task: 'Edytuj plik .vimrc', answer: ':e ~/.vimrc', alternatives: [':edit ~/.vimrc', ':e $MYVIMRC'], level: 5, category: 'master' },
  { id: 83, task: 'Zainstaluj plugin z vim-plug', answer: ':PlugInstall', level: 5, category: 'master' },
  { id: 84, task: 'Uruchom diff dwóch plików', answer: ':diffsplit plik2.txt', alternatives: [':vert diffsplit plik2.txt', 'vimdiff'], level: 5, category: 'master' },
  { id: 85, task: 'Złóż (fold) wszystkie linie pasujące do wzorca', answer: ':g/foo/ fold', alternatives: [':g/pattern/ foldclose'], level: 5, category: 'master' },
];

export function searchVim(query: string): VimChallenge[] {
  const q = query.toLowerCase();
  return vimChallenges.filter(c =>
    c.task.toLowerCase().includes(q) ||
    c.answer.toLowerCase().includes(q) ||
    c.category.toLowerCase().includes(q)
  );
}

export function getVimByLevel(level: number): VimChallenge[] {
  return vimChallenges.filter(c => c.level === level);
}

export const vimLevelNames: Record<number, string> = {
  1: 'Nawigacja',
  2: 'Edycja',
  3: 'Zaawansowana',
  4: 'Ex commands',
  5: 'Mistrz',
};
