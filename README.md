# RootForge – Linux SysAdmin Career Path

Interaktywna aplikacja desktopowa do nauki administracji Linux (RHCSA).  
Built with **Tauri (Rust) + Svelte 5 + TypeScript**.

## 🚀 Instalacja

```bash
# Zainstaluj przez RPM (RHEL/Fedora)
sudo rpm -ivh RootForge-0.1.0-1.x86_64.rpm

# Lub przez DEB (Debian/Ubuntu)
sudo dpkg -i RootForge_0.1.0_amd64.deb

# Lub uruchom binary wprost
./rootforge
```

## 🧠 Tryby nauki

| Tryb | Opis | Ilość |
|------|------|-------|
| **📖 Wyzwania** | Lekcje + zadania krok po kroku z Linux admina. Oznaczone RHCSA/EXTRA. 77 challenge'ów z zależnościami i progresją. | 77 |
| **🧠 Quiz** | Pytania wielokrotnego wyboru z **confidence rating** (1-5) i kalibracją wiedzy. Spaced repetition dla błędnych odpowiedzi. | 151 |
| **⚡ Speed Challenge** | Wpisz komendę Linux z pamięci. Timer + scoring (do 20 XP). Mądre podpowiedzi gdy popełnisz błąd. | 101 |
| **🔤 Vim Master** | 5 poziomów nauki Vima: od `h j k l` po makra, diff i `:g//d`. Wpisz sekwencję klawiszy z pamięci. | 85 |
| **🔧 Troubleshooting** | Scenariusze diagnostyczne: "Apache nie startuje", "Dysk się zapełnia". Progresywne hinty + rozwiązanie. | 25 |
| **📝 Mock Exam** | 15 losowych pytań, 30 minut, próg 70%. Symulacja RHCSA EX200. | ∞ |
| **🎲 Tryb mieszany** | Interleaving – quiz + speed + vim wymieszane w jednej sesji. 2x skuteczniejsza nauka. | ∞ |
| **💻 Projekty** | Lab walkthrough krok po kroku. 10 projektów z GitHub repo linking. | 10 |
| **🎯 Misje** | Scenariusze łączące wiele wyzwań: "Wystartuj serwer WWW", "Odzyskiwanie systemu", "RHCSA symulacja". | 10 |
| **🔄 Powtórki** | Spaced repetition dla quizów (interwały 0/1/3/7/14 dni) + challenge'y (7/14/30 dni). | ∞ |
| **📅 Daily** | Codzienne wyzwanie + streak (do 45 XP dziennie). | 1/dzień |

## 🎮 Gamifikacja

| System | Opis |
|--------|------|
| **XP** | Każda aktywność daje XP. 100 XP = 1 poziom. |
| **Poziomy** | ∞ poziomów. Każdy poziom to 100 XP. |
| **Streak** | Codzienny bonus rosnący z passą (20 + 5 × streak). |
| **Osiągnięcia** | 18 achievementów od "Pierwsze kroki" po "Vim Master". |
| **Kamienie milowe** | 11 milestone'ów odblokowywanych sekwencyjnie. |
| **Kalibracja** | Dashboard pokazuje Twoją pewność vs poprawność. |
| **Słabe obszary** | Analiza kategorii z accuracy < 60% + rekomendacje. |

## 📊 Postępy

- **Dashboard** – statystyki, daily goal ring, postęp kategorii/etapów, osiągnięcia, kalibracja, słabe obszary
- **Postępy** – Wykresy Chart.js: doughnut kategorii, bar postępu, stage progress, 14-dniowy czas nauki
- **Plan nauki** – Automatycznie generowany 4-tygodniowy plan RHCSA dostosowany do postępów
- **Export Markdown** – Podsumowanie postępów w formacie Markdown
- **Backup Gist** – Backup na GitHub Gist + przywracanie

## 📚 Zasoby

- **Ściągawka** – 114 komend Linux z przykładami, sortowane po kategorii
- **Linki** – 29 starannie wybranych linków do dokumentacji
- **Certyfikacje** – Mapa drogowa LPIC-1, LPIC-2, RHCSA, RHCE

## 🔍 Szukaj

Unified search przez wyzwania + ściągawkę + speed + vim w jednym miejscu.

## 🛠 Backend

- **Rust + Tauri v2** – 21 komend IPC
- **Persistence** – JSON file w katalogu danych aplikacji
- **Struktury** – Challenge, Quiz, Project, Mission, Troubleshoot, Achievement i więcej

## 🏗 Development

```bash
# Instalacja zależności
npm install

# Uruchomienie w trybie deweloperskim
npm run tauri dev

# Budowanie
npm run tauri build

# Testy
cargo test --lib          # Rust unit tests
npm run check             # Svelte/TypeScript check

# Struktura projektu
src/
  lib/                    # TypeScript: kategorie, certyfikacje, resources, komendy
  lib/components/         # Svelte komponenty (QuizOption, LabGuide, ChallengeModal itp.)
  routes/                 # 19 stron (dashboard, challenges, quiz, exam, vim, itp.)
  lib/styles/             # Wspólny CSS z design tokenami
src-tauri/
  src/data.rs             # Rust: struktury + DataStore + logika gry
  src/lib.rs              # Rust: komendy Tauri (21)
  data/                   # JSON: challenges, quiz, projects, milestones, missions, achievements, troubleshoot
```

## 📦 Data files

| Plik | Items | Opis |
|------|-------|------|
| `challenges.json` | 77 | Wyzwania z lekcjami, exam_tag, depends_on |
| `quiz.json` | 151 | Pytania quizowe z hintami i stage |
| `projects.json` | 10 | Projekty z lab walkthrough |
| `milestones.json` | 11 | Kamienie milowe |
| `missions.json` | 10 | Misje łączące challenge'e |
| `troubleshoot.json` | 25 | Scenariusze diagnostyczne |
| `achievements.json` | 18 | Osiągnięcia |

## 🔑 Obsługa tokena GitHub

Aby użyć backup/restore przez Gist:
1. Utwórz token na GitHub → Settings → Developer settings → Personal access tokens
2. Daj scope `gist`
3. Wklej token w Settings → GitHub Token w aplikacji

## 📋 Wymagania systemowe

- **OS:** Linux (testowane na RHEL 9/Fedora)
- **Runtime:** Node.js 20+ (tylko do developmentu)
- **Zależności build:** `webkit2gtk`, `cargo`, `rustc`
