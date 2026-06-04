use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Linux,
    System,
    Network,
    Security,
    Shell,
    DevOps,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Linux => "Linux",
            Category::System => "System",
            Category::Network => "Sieć",
            Category::Security => "Bezpieczeństwo",
            Category::Shell => "Shell",
            Category::DevOps => "DevOps",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub difficulty: u8,
    pub completed: bool,
    #[serde(default)]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub challenge_ids: Vec<u32>,
    pub unlocked: bool,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub guide: String,
    pub github_repo: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: u32,
    pub question: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub category: Category,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub quiz_id: u32,
    pub correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySession {
    pub date: String,
    pub duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongAnswer {
    pub quiz_id: u32,
    pub wrong_count: u32,
    pub last_wrong: String,
    pub next_review: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamAttempt {
    pub date: String,
    pub score: u32,
    pub total: u32,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub xp: u32,
    pub level: u32,
    pub challenges: Vec<Challenge>,
    pub milestones: Vec<Milestone>,
    pub projects: Vec<Project>,
    pub quizzes: Vec<Quiz>,
    pub quiz_results: Vec<QuizResult>,
    pub github_token: Option<String>,
    pub github_gist_id: Option<String>,
    pub sessions: Vec<StudySession>,
    pub daily_goal_minutes: u32,
    pub wrong_answers: Vec<WrongAnswer>,
    pub exam_attempts: Vec<ExamAttempt>,
}

impl AppData {
    pub fn default_data() -> Self {
        AppData {
            xp: 0,
            level: 1,
            challenges: Self::default_challenges(),
            milestones: Self::default_milestones(),
            projects: Self::default_projects(),
            quizzes: Self::default_quizzes(),
            quiz_results: Vec::new(),
            github_token: None,
            github_gist_id: None,
            sessions: Vec::new(),
            daily_goal_minutes: 30,
            wrong_answers: Vec::new(),
            exam_attempts: Vec::new(),
        }
    }

    fn default_challenges() -> Vec<Challenge> {
        vec![
            Challenge { id: 1, title: "Podstawy terminala".into(), description: "Poznaj podstawowe komendy: ls, cd, pwd, cp, mv, rm, mkdir. Stwórz strukturę katalogów i manipuluj plikami bez użycia myszki.".into(), category: Category::Linux, difficulty: 1, completed: false, details: Some(r#"
**Czego się nauczysz:** Poruszania się po systemie plików, tworzenia/usuwania plików i katalogów.

### Wymagania:
- Podstawowa obsługa komputera
- Brak wcześniejszej wiedzy o Linuxie – to wyzwanie dla początkujących

## Polecenia do wypróbowania
```bash
ls -la /etc                    # lista plików ( -l = szczegóły, -a = ukryte )
pwd                            # aktualna ścieżka
cd /var/log                    # przejście do /var/log
cp /etc/hosts ~/               # kopiowanie pliku do katalogu domowego
mv stary.txt nowy.txt          # zmiana nazwy (lub przeniesienie)
mkdir -p projekty/{web,api,db} # tworzy strukturę katalogów
rm -r katalog                  # usuwa katalog i jego zawartość
```
> Przykładowy wynik `ls -la /etc`:
> drwxr-xr-x. 141 root root 8192 Jun  4 10:00 .
> drwxr-xr-x.  17 root root  260 Jun  4 09:55 ..
> -rw-r--r--.   1 root root   16 Jun  4 09:55 adjtime
> ...
> Przykładowy wynik `pwd`:
> /etc

## Dlaczego tak działa?
- W Linux wszystko jest plikiem – nawet urządzenia (`/dev/sda`) i informacje o procesach (`/proc/cpuinfo`)
- Katalog domowy na RHEL to `/home/$USER`, a nie `C:\Users\$USER` jak w Windows
- `ls -la` pokazuje też `.` i `..` (bieżący i nadrzędny katalog) oraz pliki z kropką (ukryte)

## Uwagi
- ⚠️ `rm -rf /` usunie cały system – NIGDY tego nie rób
- W RHEL 10 domyślnym shellem jest Bash, a prompt wygląda: `[user@host ~]$`
- Używaj `TAB` do autouzupełniania – to oszczędza czas

### Weryfikacja:
```bash
ls -R ~/projekty/
```
Oczekiwany wynik:
> /home/user/projekty/:
> api  db  web
    "#.into()) },
            Challenge { id: 2, title: "Edytor Vim/Nano".into(), description: "Opanuj podstawy edytora tekstu w terminalu. Naucz się zapisywać, edytować i poruszać po pliku w Vimie lub Nano.".into(), category: Category::Linux, difficulty: 1, completed: false, details: Some(r#"
**Czego się nauczysz:** Edycji plików konfiguracyjnych w terminalu bez GUI.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Umiejętność poruszania się po systemie plików w terminalu

## Polecenia
```bash
nano /etc/hosts                # prosty edytor – Ctrl+O zapisz, Ctrl+X wyjście
vim /etc/hosts                 # zaawansowany – wymaga nauki trybów
```

## Vim – tryby (musisz je zrozumieć)
- **Normalny** (domyślny): nawigacja strzałkami, `dd` = usuń linię, `yy` = kopiuj
- **Insert**: `i` = edycja, `ESC` = powrót do normalnego
- **Command**: `:` + polecenie, np. `:wq` = zapisz i wyjdź, `:q!` = wyjdź bez zapisu

## Dlaczego Vim, a nie edytor graficzny?
- Pliki konfiguracyjne na serwerze edytujesz przez SSH – nie ma GUI
- Vim jest na KAŻDYM Linuxie – od Raspberry Pi po mainframe
- Znajomość Vima jest wymagana na egzaminie RHCSA

## Ćwiczenie
```bash
cp /etc/ssh/sshd_config ~/sshd_config.backup  # najpierw backup!
vim ~/sshd_config.backup                      # edytuj kopię
# zmień #Port 22 na Port 2222
# zapisz :wq
```
## Uwagi
- ⚠️ Zawsze rób backup pliku konfiguracyjnego przed edycją!
- Na RHEL 10 domyślnym edytorem jest vi (który jest Vimem w trybie zgodności)
- Jeśli Vim jest zbyt trudny – zacznij od Nano, ale naucz się Vima przed RHCSA

### Weryfikacja:
```bash
head -5 ~/sshd_config.backup
```
Oczekiwany wynik: wyświetli pierwsze 5 linii kopii pliku sshd_config
    "#.into()) },
            Challenge { id: 3, title: "Uprawnienia plików".into(), description: "Zrozum system uprawnień rwx. Zmieniaj uprawnienia za pomocą chmod, chown, chgrp. Skonfiguruj ACL.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Systemu uprawnień Linux, ACL, specjalnych atrybutów.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość poleceń ls, chmod, chown

## Podstawy
```bash
ls -l plik.txt        # zobacz uprawnienia: -rwxr-xr--
chmod 755 plik        # rwxr-xr-x (właściciel: wszystko, grupa: rx, inni: rx)
chmod u+x plik        # dodaj execute dla właściciela
chown user:group plik # zmień właściciela i grupę
```
> Przykładowy wynik `ls -l plik.txt`:
> -rwxr-xr-x. 1 user user 1024 Jun  4 10:00 plik.txt

## Cyfrowa notacja (warto znać na pamięć)
- `r` = 4, `w` = 2, `x` = 1
- `755` = właściciel(7=4+2+1) grupa(5=4+1) inni(5=4+1)
- `644` = plik tylko do odczytu dla grupy i innych
- `600` = plik tylko dla właściciela (np. klucze SSH)

## Dlaczego to jest ważne?
- Serwer WWW: pliki PHP muszą mieć 644, katalogi 755 – inaczej 403 Forbidden
- Klucze prywatne SSH: muszą mieć 600 – inaczej SSH odmówi użycia
- Na RHEL domyślny `umask` to 022 (nowe pliki: 644, katalogi: 755)

## ACL – gdy potrzebujesz więcej
```bash
setfacl -m u:user2:rwx katalog/   # daj pełny dostęp user2 bez zmiany grupy
getfacl katalog/                   # zobacz wszystkie ACL
setfacl -b katalog/                # usuń wszystkie ACL
```
> Przykładowy wynik `getfacl katalog/`:
> # file: katalog/
> # owner: user
> # group: user
> user::rwx
> user:user2:rwx
> group::r-x
> mask::rwx
> other::r-x

## Uwagi
- ⚠️ Nigdy nie ustawiaj `chmod 777` – to czyni plik dostępnym dla każdego
- Na RHEL dodatkowo SELinux może blokować dostęp mimo prawidłowych uprawnień – sprawdź `ls -Z`

### Weryfikacja:
```bash
ls -l /etc/hosts
```
Oczekiwany wynik:
> -rw-r--r--. 1 root root 158 Jun  4 09:55 /etc/hosts
    "#.into()) },
            Challenge { id: 4, title: "Procesy i zarządzanie".into(), description: "Naucz się zarządzać procesami: ps, top, htop, kill, nice, systemctl. Monitoruj zasoby systemowe.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Monitorowania i kontroli procesów, priorytetów, sygnałów.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość procesów w systemie operacyjnym

## Polecenia
```bash
ps aux                      # wszystkie procesy (a=all, u=user, x=bez terminala)
ps aux --sort=-%mem         # posortowane po pamięci
top                         # interaktywny monitor (q = wyjście)
htop                        # ładniejszy top (zainstaluj: dnf install htop)
kill -15 1234               # graceful shutdown (SIGTERM)
kill -9 1234                # natychmiastowe zabicie (SIGKILL) – ostateczność
nice -n 10 ./skrypt.sh      # uruchom z niskim priorytetem (10)
renice -n 5 -p 1234          # zmień priorytet działającego procesu
```
> Przykładowy wynik `ps aux` (skrócony):
> USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
> root           1  0.0  0.5 245760 10240 ?        Ss   09:55   0:01 /usr/lib/systemd/systemd
> root         567  0.0  0.3 123456 6144 ?         Ss   09:55   0:00 /usr/sbin/sshd
> ...
> Przykładowy wynik `renice -n 5 -p 1234`:
> 1234 (process ID) old priority 0, new priority 5

## Dlaczego tak działa?
- Każdy proces ma PID, PPID (rodzic), UID (właściciel)
- `kill -9` nie pozwala procesowi posprzątać – używaj `-15` (SIGTERM) najpierw
- `nice` zakres: -20 (najwyższy priorytet) do 19 (najniższy)
- Na RHEL systemd zarządza procesami systemowymi – `systemctl status nazwa`

## Analiza procesów
```bash
cat /proc/1234/status       # szczegóły procesu przez pseudo-system-plików
lsof -p 1234                # jakie pliki otworzył proces
strace -p 1234              # śledź wywołania systemowe procesu
```
> Przykładowy wynik `cat /proc/1234/status` (skrócony):
> Name:   sshd
> Pid:    1234
> State:  S (sleeping)
> ...
> Przykładowy wynik `lsof -p 1234` (skrócony):
> sshd  1234  root  cwd   DIR  253,0      260 /etc/ssh
> sshd  1234  root  txt   REG  253,0  1234567 /usr/sbin/sshd
> ...

## Uwagi
- ⚠️ `kill -9` powinien być ostatecznością – proces nie może posprzątać pamięci
- `systemctl stop` ≠ `kill` – pierwszy to eleganckie zatrzymanie przez systemd
- Na RHEL `top` pokazuje też informacje o SELinux

### Weryfikacja:
```bash
ps aux --sort=-%mem | head -5
```
Oczekiwany wynik: lista 5 procesów zużywających najwięcej pamięci RAM
    "#.into()) },
            Challenge { id: 5, title: "Konfiguracja sieci".into(), description: "Skonfiguruj interfejsy sieciowe, znajdź IP, bramę, DNS. Użyj ip, nmcli, nmtui. Zrozum routing.".into(), category: Category::Network, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Podstaw konfiguracji sieci na RHEL 10 z użyciem NetworkManager.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość adresów IP, masek i bramy domyślnej

## Polecenia
```bash
ip addr                    # adresy IP interfejsów
ip route                   # tablica routingu (domyślna brama)
ip link set eth0 up        # włącz interfejs

nmcli con show              # lista połączeń NetworkManager
nmcli con show "eth0"       # szczegóły połączenia
nmcli con mod "eth0" ipv4.addresses 192.168.1.100/24
nmcli con mod "eth0" ipv4.gateway 192.168.1.1
nmcli con mod "eth0" ipv4.dns "8.8.8.8 8.8.4.4"
nmcli con mod "eth0" ipv4.method manual
nmcli con down "eth0" && nmcli con up "eth0"  # zastosuj zmiany
```
> Przykładowy wynik `ip addr`:
> 1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN
>     inet 127.0.0.1/8 scope host lo
> 2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP
>     inet 192.168.1.100/24 brd 192.168.1.255 scope global noprefixroute eth0
> Przykładowy wynik `ip route`:
> default via 192.168.1.1 dev eth0 proto dhcp metric 100
> 192.168.1.0/24 dev eth0 proto kernel scope link src 192.168.1.100 metric 100
> Przykładowy wynik `nmcli con show`:
> NAME   UUID                                  TYPE      DEVICE
> eth0   a1b2c3d4-...                          ethernet  eth0

## Dlaczego nmcli?
- RHEL używa NetworkManager jako domyślnego zarządcy sieci
- Stare `/etc/sysconfig/network-scripts/ifcfg-*` są wycofywane (deprecated od RHEL 9)
- `nmcli` to zalecane narzędzie – działa też przez SSH

## Routing statyczny
```bash
ip route add 10.0.0.0/8 via 192.168.1.1 dev eth0
# trwale: nmcli con mod "eth0" +ipv4.routes "10.0.0.0/8 192.168.1.1"
```

## Uwagi
- ⚠️ Po zmianach DNS może być potrzebny restart: `systemctl restart NetworkManager`
- Na RHEL 10 interfejsy nazywają się np. `ens192`, `enp0s3` – nie `eth0`
- Do testów na VM użyj `virt-manager` lub `Vagrant`

### Weryfikacja:
```bash
ip addr show eth0 | grep inet
```
Oczekiwany wynik: powinieneś zobaczyć przypisany adres IPv4 dla interfejsu eth0
    "#.into()) },
            Challenge { id: 6, title: "Firewall i bezpieczeństwo".into(), description: "Skonfiguruj firewalld/iptables. Otwórz porty, utwórz strefy, zabezpiecz SSH. Zrozum SELinux.".into(), category: Category::Security, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Konfiguracji firewalld (domyślny firewall RHEL) i podstaw SELinux.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Podstawowa znajomość portów TCP/UDP i usług sieciowych

## Firewalld – polecenia
```bash
systemctl status firewalld        # sprawdź czy działa
firewall-cmd --list-all           # pokaż wszystkie reguły
firewall-cmd --get-active-zones   # aktywne strefy
firewall-cmd --zone=public --add-service=http --permanent
firewall-cmd --zone=public --add-port=8080/tcp --permanent
firewall-cmd --reload             # przeładuj reguły
```
> Przykładowy wynik `systemctl status firewalld`:
> ● firewalld.service - firewalld - dynamic firewall daemon
>      Loaded: loaded (/usr/lib/systemd/system/firewalld.service; enabled; preset: enabled)
>      Active: active (running) since Thu 2026-06-04 09:55:00 CEST
> Przykładowy wynik `firewall-cmd --list-all`:
> public (active)
>   target: default
>   icmp-block-inversion: no
>   services: dhcpv6-client http ssh
>   ports: 8080/tcp
> Przykładowy wynik `firewall-cmd --zone=public --add-service=http --permanent`:
> success

## Dlaczego firewalld, a nie iptables?
- RHEL 7+ domyślnie używa firewalld (iptables jest przestarzałe)
- firewalld ma **strefy**: public (domyślna), internal, trusted, dmz
- Reguły mogą być tymczasowe (znikną po reboot) lub trwałe (`--permanent`)

## SELinux – podstawy
```bash
getenforce                   # Enforcing (RHEL domyślnie)
ls -Z /etc/hosts             # zobacz context SELinux
restorecon -v /etc/hosts     # przywróć domyślny context
chcon -t httpd_sys_content_t /var/www/html/index.html  # zmiana type
```
> Przykładowy wynik `getenforce`:
> Enforcing
> Przykładowy wynik `ls -Z /etc/hosts`:
> system_u:object_r:net_conf_t:s0 /etc/hosts
> Przykładowy wynik `restorecon -v /etc/hosts`:
> Relabeled /etc/hosts from system_u:object_r:net_conf_t:s0 to system_u:object_r:net_conf_t:s0

## Uwagi
- ⚠️ Po dodaniu portu `--permanent` musisz zrobić `--reload` – samo dodanie nie uruchamia reguły
- ⚠️ Wyłączenie SELinux (`setenforce 0`) to zły pomysł – RHEL tego nie wspiera. Zamiast tego ustaw Permissive tymczasowo i napraw context
- Jeśli serwis nie działa mimo otwartego firewalla – sprawdź SELinux: `ausearch -m avc -ts recent`

### Weryfikacja:
```bash
firewall-cmd --list-all
```
Oczekiwany wynik: powinieneś zobaczyć skonfigurowane usługi (http, https) i porty (8080)
    "#.into()) },
            Challenge { id: 7, title: "Skrypty bash".into(), description: "Napisz skrypt bash do backupu katalogu. Użyj zmiennych, pętli, warunków, cron. Zautomatyzuj zadanie.".into(), category: Category::Shell, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Pisania skryptów bash z zmiennymi, pętlami, warunkami i planowaniem cron.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość podstawowych poleceń: echo, date, tar, ls
- Edytor tekstu (Vim/Nano)

## Przykładowy skrypt backupu
```bash
#!/bin/bash
# backup.sh – kopia katalogu z datą i kompresją

SRC="/var/www/html"
DST="/backup/$(date +%Y%m%d_%H%M%S)-html.tar.gz"
LOG="/var/log/backup.log"

echo "=== Backup start: $(date) ===" >> "$LOG"
tar -czf "$DST" "$SRC" 2>> "$LOG"

if [ $? -eq 0 ]; then
    echo "OK: $DST ($(du -h "$DST" | cut -f1))" >> "$LOG"
else
    echo "ERROR: backup failed!" >> "$LOG"
fi
```

## Dlaczego tak?
- `#!/bin/bash` – shebang określa interpreter, bez niego skrypt nie jest wykonywalny
- `$(date ...)` – podstawianie wyniku polecenia (subshell)
- `$?` – kod wyjścia ostatniego polecenia (0 = sukces)
- `-z` (gzip) to najszybsza opcja, na RHEL domyślnie dostępna

## Planowanie przez cron
```bash
crontab -e
# dodaj linię:
0 3 * * * /home/user/backup.sh  # codziennie o 3:00 nad ranem
```

## Uwagi
- ⚠️ Zawsze sprawdzaj `$?` po krytycznych operacjach – skrypt nie zatrzyma się sam
- ⚠️ Ścieżki w cronie muszą być absolutne – cron nie ma Twojego PATH
- Zanim uruchomisz skrypt: `bash -n skrypt.sh` (sprawdź składnię) i `chmod +x`

### Weryfikacja:
```bash
bash -n backup.sh
```
Oczekiwany wynik: brak outputu (brak błędów składniowych w skrypcie)
    "#.into()) },
            Challenge { id: 8, title: "LVM i partycjonowanie".into(), description: "Stwórz partycje, skonfiguruj LVM (PV, VG, LV), zmień rozmiar wolumenu. Zrozum system plików.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Zarządzania dyskami i LVM – elastycznego systemu partycji.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość partycjonowania dysków (fdisk, parted)
- Znajomość systemów plików (ext4, XFS)

## LVM – podstawowe polecenia
```bash
pvcreate /dev/sdb1              # inicjalizuj partycję jako Physical Volume
vgcreate vg_data /dev/sdb1      # stwórz Volume Group
lvcreate -L 10G -n lv_data vg_data  # stwórz Logical Volume 10GB
mkfs.xfs /dev/vg_data/lv_data   # sformatuj jako XFS (domyślny FS RHEL)
mount /dev/vg_data/lv_data /mnt # zamontuj
```
> Przykładowy wynik `mkfs.xfs /dev/vg_data/lv_data`:
> meta-data=/dev/vg_data/lv_data  isize=512    agcount=4, agsize=655360 blks
>          =                       sectsz=512   attr=2, projid32bit=1
> data     =                       bsize=4096   blocks=2621440, imaxpct=25
> naming   =version 2              bsize=4096   ascii-ci=0, ftype=1

## Dlaczego LVM, a nie zwykłe partycje?
- Możesz zmieniać rozmiar LV bez demontowania (w przeciwieństwie do partycji)
- Snapshots: `lvcreate -s -n snap -L 1G /dev/vg_data/lv_data` – migawka do backupu
- Łatwe dodawanie dysków do VG i rozszerzanie LV

## Rozszerzanie LV
```bash
lvextend -L +5G /dev/vg_data/lv_data           # zwiększ LV o 5GB
xfs_growfs /mnt                                  # rozszerz XFS (online!)
# dla ext4: resize2fs /dev/vg_data/lv_data
```
> Przykładowy wynik `lvextend -L +5G /dev/vg_data/lv_data`:
>   Size of logical volume vg_data/lv_data changed from 10.00 GiB to 15.00 GiB.
> Przykładowy wynik `xfs_growfs /mnt`:
> meta-data:/dev/mapper/vg_data-lv_data  isize=512    agcount=6, agsize=655360 blks
> data blocks changed from 2621440 to 3932160

## Uwagi
- ⚠️ RHEL domyślnie używa XFS – resize XFS działa tylko w górę (nie zmniejszysz)
- ⚠️ Zawsze sprawdź `df -h` przed rozszerzaniem – miejsce na dysku nie jest nieskończone
- `lsblk` pokazuje drzewo urządzeń, `lvs` i `pvs` – podsumowanie LVM
- W RHEL 10 dostępny jest też Stratis (nowszy, ale LVM to standard)

### Weryfikacja:
```bash
lvs
```
Oczekiwany wynik: lista wolumenów logicznych z nazwami, rozmiarami i atrybutami
    "#.into()) },
            Challenge { id: 9, title: "SSH i zdalny dostęp".into(), description: "Skonfiguruj serwer SSH, klucze, aliasy, tunelowanie. Zablokuj logowanie root przez hasło.".into(), category: Category::Network, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Bezpiecznej konfiguracji SSH, kluczy, tunelowania.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Ukończone wyzwanie "Konfiguracja sieci"
- Podstawowa znajomość adresów IP, portów i usług sieciowych

## Konfiguracja serwera SSH
```bash
# /etc/ssh/sshd_config – kluczowe ustawienia bezpieczeństwa:
Port 2222                        # zmień z domyślnego 22 (mniej skanowania)
PermitRootLogin no               # ZABLOKUJ logowanie root przez hasło
PasswordAuthentication no        # tylko klucze (bez haseł)
PubkeyAuthentication yes         # włącz klucze
```

## Generowanie i instalacja kluczy
```bash
ssh-keygen -t ed25519 -C "moj klucz"      # generuj parę (ED25519 jest bezpieczniejszy od RSA)
ssh-copy-id -i ~/.ssh/id_ed25519.pub user@server  # kopiuj klucz na serwer
ssh -p 2222 user@server                    # logowanie na niestandardowym porcie
```
> Przykładowy wynik `ssh-keygen -t ed25519`:
> Generating public/private ed25519 key pair.
> Enter file in which to save the key (/home/user/.ssh/id_ed25519):
> Przykładowy wynik `ssh-copy-id`:
> /usr/bin/ssh-copy-id: INFO: Source of key(s) to be installed: "/home/user/.ssh/id_ed25519.pub"
> Number of key(s) added: 1

## Dlaczego klucze, a nie hasła?
- Klucz ED25519 ma ~100 bajtów – RSA 4096-bit ma ~700. ED25519 jest szybszy i bezpieczniejszy
- Klucze są odporne na brute-force (hasła są słabe)
- Możesz dodać klucz do ssh-agenta i nie wpisywać hasła

## Tunelowanie (port forwarding)
```bash
ssh -L 8080:localhost:80 user@server       # tunel lokalny – przekieruj port
ssh -D 1080 user@server                    # SOCKS proxy przez SSH
```

## Uwagi
- ⚠️ Zanim zamkniesz sesję po zmianie sshd_config – otwórz DRUGI terminal i testuj. Inaczej możesz się zablokować
- ⚠️ Klucz prywatny (`id_ed25519`) musi mieć `chmod 600` – inaczej SSH nie pozwoli go użyć
- Na RHEL domyślnie SSH pozwala na logowanie root – wyłącz to natychmiast po instalacji

### Weryfikacja:
```bash
ls -la ~/.ssh/
```
Oczekiwany wynik: powinieneś zobaczyć pliki id_ed25519 (klucz prywatny), id_ed25519.pub (klucz publiczny) i ewentualnie authorized_keys
    "#.into()) },
            Challenge { id: 10, title: "Kontenery Podman".into(), description: "Uruchom kontener z Podman (domyślny silnik kontenerów RHEL). Stwórz obrazy, wolumeny, systemd integrację.".into(), category: Category::DevOps, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Podman – domyślnego silnika kontenerów RHEL (zamiast Dockera).

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość koncepcji konteneryzacji (obraz, kontener, rejestr)
- Ukończone wyzwanie "Systemd i serwisy" (przydatne)

## Podstawy
```bash
dnf install podman                     # na RHEL 10 jest domyślnie
podman pull registry.access.redhat.com/ubi9/ubi  # obraz Red Hat UBI
podman run -d --name web -p 80:80 nginx
podman ps                              # lista kontenerów
podman stop web && podman rm web      # zatrzymaj i usuń
```
> Przykładowy wynik `podman pull registry.access.redhat.com/ubi9/ubi`:
> Trying to pull registry.access.redhat.com/ubi9/ubi:latest...
> Getting image source signatures
> Copying blob abcdef123456 done
> Przykładowy wynik `podman run -d --name web -p 80:80 nginx`:
> a1b2c3d4e5f6...
> Przykładowy wynik `podman ps`:
> CONTAINER ID  IMAGE                           COMMAND               CREATED        STATUS        PORTS               NAMES
> a1b2c3d4e5f6  registry.access.redhat.com/...  nginx -g daemon o...  2 minutes ago  Up 2 minutes  0.0.0.0:80->80/tcp  web

## Dlaczego Podman, a nie Docker?
- **Daemonless** – nie potrzebuje działającego demona (dockerd), kontenery działają jako procesy
- **Rootless** – może działać bez roota (bezpieczniej)
- **Kompatybilny z Docker CLI** – alias `alias docker=podman` i działa
- RHEL domyślnie wspiera tylko Podman – Docker wymaga dodatkowej konfiguracji

## Dockerfile / Containerfile (działa tak samo)
```dockerfile
FROM registry.access.redhat.com/ubi9/ubi
RUN dnf install -y nginx && dnf clean all
COPY index.html /usr/share/nginx/html/
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

## Systemd + Podman – auto-start kontenera
```bash
podman generate systemd --name web > /etc/systemd/system/container-web.service
systemctl daemon-reload && systemctl enable --now container-web
```
> Przykładowy wynik:
> Created symlink /etc/systemd/system/multi-user.target.wants/container-web.service → /etc/systemd/system/container-web.service

## Uwagi
- ⚠️ Na RHEL rootless Podman wymaga subuid/subgid – sprawdź `podman info` jeśli nie działa
- RHEL nie zawiera Dockera w repo – użyj Podman, to to samo ale lepsze
- `podman-compose` działa jak docker-compose (zainstaluj przez pip)

### Weryfikacja:
```bash
podman ps
```
Oczekiwany wynik: lista uruchomionych kontenerów (lub pusta, jeśli wszystkie zatrzymane)
    "#.into()) },
            Challenge { id: 11, title: "Logi i monitorowanie".into(), description: "Naucz się czytać logi: journalctl, /var/log, rsyslog. Skonfiguruj rotację logów. Użyj logwatch.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Czytania i zarządzania logami systemowymi na RHEL.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Ukończone wyzwanie "Systemd i serwisy" (przydatne)
- Znajomość podstaw systemd i jednostek usług

## journalctl (systemd-journald)
```bash
journalctl -u sshd                    # logi konkretnego serwisu
journalctl -u nginx --since today     # od dzisiaj
journalctl -p err                     # tylko błędy (priority err)
journalctl -f                         # live tail (jak tail -f)
journalctl --disk-usage               # ile miejsca zajmują logi
```
> Przykładowy wynik `journalctl -u sshd`:
> Jun 04 09:55:01 hostname sshd[1234]: Server listening on 0.0.0.0 port 22.
> Jun 04 09:55:01 hostname sshd[1234]: Server listening on :: port 22.
> Jun 04 10:01:23 hostname sshd[5678]: Accepted publickey for user from 192.168.1.100 port 54321
> Przykładowy wynik `journalctl --disk-usage`:
> Archived and active journals take up 128.0M on disk.

## /var/log – tradycyjne logi
```bash
ls /var/log
# messages      – ogólne logi systemowe
# secure        – logowanie, sudo
# boot.log      – logi bootowania
# dnf.log       – logi DNF
# audit/audit.log – logi SELinux (ausearch do czytania)
```

## Rotacja logów (logrotate)
```bash
# /etc/logrotate.conf i /etc/logrotate.d/
cat /etc/logrotate.d/nginx
# /var/log/nginx/*.log {
#     daily
#     rotate 7
#     compress
#     missingok
# }
```
> Przykładowy wynik `ls /var/log`:
> boot.log   dnf.log   lastlog   messages   secure   wtmp
> Przykładowy wynik `cat /etc/logrotate.d/nginx`:
> /var/log/nginx/*.log {
>     daily
>     rotate 7
>     compress
>     missingok
> }

## Dlaczego journalctl?
- journald automatycznie zbiera logi WSZYSTKICH serwisów systemd
- `journalctl -u nazwa -o json-pretty` – wyjście JSON do parsowania
- Logi są binarne (szybciej) – czytasz przez journalctl, nie cat

## Uwagi
- ⚠️ journald domyślnie przechowuje logi w pamięci (volatile) – logi giną po rebocie. Ustaw `Storage=persistent` w `/etc/systemd/journald.conf`
- Na RHEL logi SELinux są w `/var/log/audit/audit.log` – czytaj przez `ausearch`
- `dmesg` pokazuje logi jądra z ring bufora

### Weryfikacja:
```bash
journalctl --disk-usage
```
Oczekiwany wynik: informacja o miejscu zajmowanym przez logi systemd-journald
    "#.into()) },
            Challenge { id: 12, title: "DNS i DHCP".into(), description: "Skonfiguruj lokalny serwer DNS (bind/dnsmasq) i DHCP. Zrozum rekordy A, AAAA, CNAME, MX.".into(), category: Category::Network, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Konfiguracji serwera DNS (dnsmasq) i DHCP w sieci lokalnej.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Podstawowa znajomość DNS, DHCP i adresacji IP

## Dnsmasq – prosty DNS + DHCP
```bash
dnf install dnsmasq
```

## Konfiguracja `/etc/dnsmasq.conf`
```ini
# DNS
server=8.8.8.8
server=1.1.1.1
domain=lab.local
local=/lab.local/

# DHCP
dhcp-range=192.168.100.50,192.168.100.100,12h
dhcp-option=3,192.168.100.1        # gateway
dhcp-option=6,192.168.100.1        # DNS
```

## Uruchomienie
```bash
systemctl enable --now dnsmasq
firewall-cmd --add-service=dns --add-service=dhcp --permanent
firewall-cmd --reload
```
> Przykładowy wynik `systemctl enable --now dnsmasq`:
> Created symlink /etc/systemd/system/multi-user.target.wants/dnsmasq.service → /usr/lib/systemd/system/dnsmasq.service
> Przykładowy wynik `firewall-cmd --add-service=dns --add-service=dhcp --permanent`:
> success

## Rodzaje rekordów DNS
- **A** – nazwa → IPv4 (np. server.lab.local → 192.168.100.10)
- **AAAA** – nazwa → IPv6
- **CNAME** – alias (np. www → server)
- **MX** – serwer poczty (z priorytetem)
- **PTR** – reverse lookup (IP → nazwa)

## Dlaczego dnsmasq, a nie BIND?
- BIND jest potężny ale skomplikowany (strefy, zone transfery, DNSSEC)
- dnsmasq jest prosty i wystarczy do sieci domowej/lab
- dnsmasq łączy DNS i DHCP w jednym

## Uwagi
- ⚠️ Zmiana DNS na kliencie: `nmcli con mod "eth0" ipv4.dns "192.168.100.1"`
- ⚠️ Przed restartem dnsmasq: `dnsmasq --test` (sprawdza konfigurację)
- Do zaawansowanych konfiguracji firmowych – BIND9 z strefami

### Weryfikacja:
```bash
systemctl status dnsmasq
```
Oczekiwany wynik: usługa dnsmasq powinna być aktywna (running) i włączona (enabled)
    "#.into()) },
            Challenge { id: 13, title: "SELinux/AppArmor".into(), description: "Zrozum i skonfiguruj SELinux (lub AppArmor). Zmień context, utwórz politykę, debuguj problemy.".into(), category: Category::Security, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** SELinux – obowiązkowej kontroli dostępu w RHEL.

### Wymagania:
- Ukończone wyzwanie "Uprawnienia plików"
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Znajomość systemu uprawnień rwx i koncepcji MAC

## Podstawy
```bash
getenforce                   # Enforcing (RHEL domyślnie)
sestatus                     # szczegółowy status SELinux
ls -Z /etc/hosts             # zobacz context: system_u:object_r:net_conf_t:s0
ps auxZ                      # contexty procesów
```
> Przykładowy wynik `getenforce`:
> Enforcing
> Przykładowy wynik `ls -Z /etc/hosts`:
> system_u:object_r:net_conf_t:s0 /etc/hosts
> Przykładowy wynik `ps auxZ` (skrócony):
> LABEL                           USER       PID %CPU COMMAND
> system_u:system_r:init_t:s0     root         1  0.0 /usr/lib/systemd/systemd
> system_u:system_r:sshd_t:s0     root      1234  0.0 /usr/sbin/sshd

## Typowe problemy i rozwiązania
```bash
# Problem: Nginx nie może czytać pliku → 403
# Rozwiązanie: zmień typ pliku na httpd_sys_content_t
chcon -t httpd_sys_content_t /var/www/html/index.html
# Lub przywróć domyślny context:
restorecon -v /var/www/html/index.html
```
> Przykładowy wynik `restorecon -v /var/www/html/index.html`:
> Relabeled /var/www/html/index.html from unconfined_u:object_r:default_t:s0 to system_u:object_r:httpd_sys_content_t:s0

## Tryby SELinux
- **Enforcing** – polityki są wymuszane (domyślnie na RHEL)
- **Permissive** – loguje naruszenia ale nie blokuje (diagnostyka)
- **Disabled** – całkowicie wyłączony (NIE RÓB TEGO)

## Debugowanie
```bash
ausearch -m avc -ts recent    # znajdź ostatnie blokady SELinux
sealert -a /var/log/audit/audit.log  # czytelne wyjaśnienie problemu
# Tymczasowo: setenforce 0 (Permissive) – testuj, potem wróć do Enforcing
```
> Przykładowy wynik `ausearch -m avc -ts recent`:
> <no matches> (brak blokad SELinux – system działa poprawnie)
> Przykładowy wynik `ausearch -m avc` z blokadą:
> type=AVC msg=audit(1717480000.123:456): avc:  denied  { read } for  pid=5678 comm="nginx"

## Dlaczego SELinux?
- Nawet jeśli proces (np. Nginx) zostanie zhakowany – SELinux ogranicza co może zrobić
- Domyślna polityka RHEL: httpd może czytać tylko `/var/www/html/`, nie może pisać poza tym
- To dodatkowa warstwa bezpieczeństwa – nie polegaj tylko na firewallu i uprawnieniach

## Uwagi
- ⚠️ NIGDY nie wyłączaj SELinux przez `setenforce 0` lub `SELINUX=disabled` w configu – to psuje certyfikację i bezpieczeństwo
- ⚠️ Jeśli zmienisz lokalizację plików (np. document root Nginx na /data/www) – SELinux zablokuje dostęp. Użyj `semanage fcontext` lub `restorecon`
- Na RHEL 10 SELinux jest domyślnie w trybie Enforcing – poznaj go, to ważna zaleta RHEL

### Weryfikacja:
```bash
getenforce
```
Oczekiwany wynik: Enforcing (SELinux aktywny i wymuszający polityki)
    "#.into()) },
            Challenge { id: 14, title: "Systemd i serwisy".into(), description: "Stwórz własny serwis systemd. Napisz unit, włącz go, zarządzaj zależnościami. Zrozum targety.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Tworzenia własnych serwisów systemd i zarządzania nimi.

### Wymagania:
- Ukończone wyzwanie "Procesy i zarządzanie"
- Ukończone wyzwanie "Skrypty bash" (przydatne)
- Znajomość edytora tekstu (Vim/Nano)

## Przykład: własny serwis
```bash
# /etc/systemd/system/moja-aplikacja.service
[Unit]
Description=Moja aplikacja
After=network.target
Wants=postgresql.service

[Service]
Type=simple
User=myapp
WorkingDirectory=/opt/myapp
ExecStart=/usr/bin/node /opt/myapp/server.js
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

## Zarządzanie
```bash
systemctl daemon-reload                # przeładuj definicje serwisów
systemctl enable --now moja-aplikacja  # włącz i uruchom
systemctl status moja-aplikacja        # sprawdź status
journalctl -u moja-aplikacja -f       # podgląd logów w czasie rzeczywistym
```
> Przykładowy wynik `systemctl enable --now moja-aplikacja`:
> Created symlink /etc/systemd/system/multi-user.target.wants/moja-aplikacja.service → /etc/systemd/system/moja-aplikacja.service
> Przykładowy wynik `systemctl status moja-aplikacja`:
> ● moja-aplikacja.service - Moja aplikacja
>      Loaded: loaded (/etc/systemd/system/moja-aplikacja.service; enabled; preset: disabled)
>      Active: active (running) since Thu 2026-06-04 10:00:00 CEST

## Dlaczego systemd?
- Zastąpił SysVinit w RHEL 7+ – szybsze bootowanie, równoległe uruchamianie
- `systemctl` zastępuje `service` i `chkconfig`
- Timery systemd (`systemd-timers`) zastępują cron

## Typy serwisów
- **simple**: proces uruchomiony, systemd czeka aż sam się zakończy
- **forking**: proces robi fork i rodzic kończy pracę (tradycyjne demony)
- **oneshot**: wykonuje się raz i kończy (np. zadanie)
- **notify**: proces powiadamia systemd przez sd_notify()

## Uwagi
- ⚠️ Po zmianie pliku .service: `systemctl daemon-reload` – inaczej nie zobaczy zmian
- ⚠️ Jeśli serwis nie startuje: `journalctl -xe` (ostatnie logi + podpowiedzi)
- Timery systemd: `systemctl list-timers` – zobacz zaplanowane zadania

### Weryfikacja:
```bash
systemctl status moja-aplikacja
```
Oczekiwany wynik: usługa powinna być aktywna (running) i włączona (enabled)
    "#.into()) },
            Challenge { id: 15, title: "Apache/Nginx".into(), description: "Skonfiguruj serwer WWW (Apache lub Nginx). Host wirtualny, SSL, rewrite, reverse proxy.".into(), category: Category::DevOps, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Konfiguracji serwera WWW z Nginx (domyślny w RHEL przez EPEL).

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Ukończone wyzwanie "Konfiguracja sieci"
- Podstawowa znajomość protokołu HTTP i DNS

## Instalacja Nginx na RHEL 10
```bash
dnf install nginx
systemctl enable --now nginx
firewall-cmd --add-service=http --add-service=https --permanent
firewall-cmd --reload
```
> Przykładowy wynik `systemctl enable --now nginx`:
> Created symlink /etc/systemd/system/multi-user.target.wants/nginx.service → /usr/lib/systemd/system/nginx.service
> Przykładowy wynik `firewall-cmd --add-service=http --add-service=https --permanent`:
> success

## Host wirtualny
```nginx
# /etc/nginx/conf.d/moja-strona.conf
server {
    listen 80;
    server_name mojastrona.pl www.mojastrona.pl;
    root /var/www/mojastrona;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }

    location /api/ {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
    }
}
```

## SSL z Certbot
```bash
dnf install certbot python3-certbot-nginx
certbot --nginx -d mojastrona.pl -d www.mojastrona.pl
# certbot automatycznie modyfikuje konfigurację Nginx
```
> Przykładowy wynik `nginx -t`:
> nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
> nginx: configuration file /etc/nginx/nginx.conf test is successful

## Dlaczego Nginx zamiast Apache?
- Nginx jest event-driven (async) – lepsza wydajność przy wielu połączeniach
- Apache jest process-driven (każde połączenie = nowy proces) – więcej RAM
- Nginx lepiej służy jako reverse proxy – Apache lepiej z .htaccess

## Uwagi
- ⚠️ Po zmianie konfiguracji: `nginx -t` (test) przed `systemctl reload nginx`
- ⚠️ SELinux blokuje Nginx przed dostępem do plików poza /usr/share/nginx/ – użyj `chcon` lub `semanage fcontext`
- `server_tokens off;` w nginx.conf – ukryj wersję Nginx (security by obscurity, ale warto)

### Weryfikacja:
```bash
curl -I http://localhost
```
Oczekiwany wynik: odpowiedź HTTP z kodem 200 OK i nagłówkiem zawierającym "Server: nginx"
    "#.into()) },
            Challenge { id: 16, title: "Automatyzacja Ansible".into(), description: "Napisz playbook Ansible. Zainstaluj pakiety, skopiuj pliki, uruchom serwis na zdalnym hoście.".into(), category: Category::DevOps, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Automatyzacji konfiguracji z Ansible (narzędzie Red Hat).

### Wymagania:
- Ukończone wyzwanie "SSH i zdalny dostęp"
- Ukończone wyzwanie "Skrypty bash"
- Znajomość SSH, kluczy i podstaw YAML

## Instalacja i podstawy
```bash
dnf install ansible-core
# lub pełny: dnf install ansible
```

## Inventory (hosts.ini)
```ini
[web]
webserver1 ansible_host=192.168.100.10
webserver2 ansible_host=192.168.100.11

[db]
dbserver ansible_host=192.168.100.20

[all:vars]
ansible_user=root
ansible_ssh_private_key_file=~/.ssh/id_ed25519
```

## Playbook (deploy.yml)
```yaml
---
- name: Konfiguracja serwera WWW
  hosts: web
  vars:
    http_port: 80
    server_name: example.com
  tasks:
    - name: Zainstaluj Nginx
      dnf:
        name: nginx
        state: present

    - name: Skopiuj konfigurację
      template:
        src: nginx.conf.j2
        dest: /etc/nginx/nginx.conf
      notify: reload nginx

    - name: Uruchom i włącz Nginx
      systemd:
        name: nginx
        enabled: yes
        state: started

  handlers:
    - name: reload nginx
      systemd:
        name: nginx
        state: reloaded
```

## Dlaczego Ansible?
- Red Hat jest właścicielem Ansible – to standard w RHEL ekosystemie
- Agentless – nie wymaga agenta na zarządzanych hostach (tylko SSH + Python)
- Idempotent – możesz uruchomić playbook wiele razy, efekt ten sam

## Uruchomienie
```bash
ansible-playbook -i hosts.ini deploy.yml
ansible all -i hosts.ini -m ping    # test połączenia
```
> Przykładowy wynik `ansible all -i hosts.ini -m ping`:
> webserver1 | SUCCESS => {
>     "ansible_facts": {
>         "discovered_interpreter_python": "/usr/bin/python3"
>     },
>     "changed": false,
>     "ping": "pong"
> }
> Przykładowy wynik `ansible-playbook -i hosts.ini deploy.yml`:
> PLAY [Konfiguracja serwera WWW] ************************************************
> TASK [Gathering Facts] *********************************************************
> ok: [webserver1]
> TASK [Zainstaluj Nginx] ********************************************************
> changed: [webserver1]
> ...

## Uwagi
- ⚠️ Zawsze używaj Ansible Vault do haseł: `ansible-vault encrypt secrets.yml`
- ⚠️ Testuj playbooki na kopii VM, nie na produkcji
- `ansible-lint` – sprawdza poprawność playbooka przed uruchomieniem

### Weryfikacja:
```bash
ansible all -i hosts.ini -m ping --one-line
```
Oczekiwany wynik: każdy host powinien odpowiedzieć "SUCCESS => pong"
    "#.into()) },
            Challenge { id: 17, title: "RAID i backup".into(), description: "Skonfiguruj RAID (mdadm), wykonaj backup rsync/restic. Zaplanuj automatyczny backup przez cron.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** RAID programowego i strategii backupu.

### Wymagania:
- Ukończone wyzwanie "LVM i partycjonowanie"
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość dysków, partycji i systemów plików

## RAID z mdadm
```bash
# RAID 1 (mirror) z dwóch dysków
mdadm --create /dev/md0 --level=1 --raid-devices=2 /dev/sdb /dev/sdc
mkfs.xfs /dev/md0
mount /dev/md0 /mnt/raid

# Zapisz konfigurację:
mdadm --detail --scan >> /etc/mdadm.conf
```
> Przykładowy wynik `mdadm --create /dev/md0 --level=1 --raid-devices=2 /dev/sdb /dev/sdc`:
> mdadm: array /dev/md0 started.
> Przykładowy wynik `mkfs.xfs /dev/md0`:
> meta-data=/dev/md0               isize=512    agcount=4, agsize=655360 blks
>          =                       sectsz=512   attr=2, projid32bit=1
> Przykładowy wynik `rsync -avz /source/ /backup/`:
> sending incremental file list
> ./
> file1.txt
> file2.txt
> 
> sent 12345 bytes  received 678 bytes  26046.00 bytes/sec

## Backup z rsync
```bash
# Lokalny
rsync -avz /source/ /backup/

# Zdalny (przez SSH)
rsync -avz -e ssh /source/ user@backup-server:/backup/

# Incremental z linkami do poprzednich
rsync -avz --link-dest=/backup/wczoraj /source/ /backup/dzisiaj/
```

## Dlaczego rsync?
- Kopiuje tylko zmienione fragmenty plików (delta encoding)
- Działa przez SSH – bezpieczny
- `--link-dest` tworzy kopie incrementalne bez duplikowania danych (hard linki)

## Automatyzacja przez cron
```bash
# /etc/cron.d/backup
0 2 * * * root rsync -avz /var/www/ backup@192.168.1.100:/backup/www/
```

## Uwagi
- ⚠️ RAID to NIE backup – RAID chroni przed awarią dysku, ale nie przed:
  - przypadkowym usunięciem pliku
  - ransomware (szyfruje wszystkie dyski RAID)
  - zalaniem/powodzią
- ⚠️ Zasada 3-2-1 backupu: 3 kopie, 2 różne media, 1 poza lokalizacją
- Testuj odtwarzanie z backupu! Backup którego nie testowałeś nie istnieje

### Weryfikacja:
```bash
cat /proc/mdstat
```
Oczekiwany wynik: powinieneś zobaczyć status macierzy RAID (md0) z jej typem i stanem
    "#.into()) },
            Challenge { id: 18, title: "Audyt i compliance".into(), description: "Skonfiguruj Lynis do audytu bezpieczeństwa. Przeanalizuj wyniki i popraw znalezione problemy.".into(), category: Category::Security, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Audytu bezpieczeństwa systemu z Lynis.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Ukończone wyzwanie "Systemd i serwisy"
- Podstawowa znajomość bezpieczeństwa systemowego i haseł

## Instalacja Lynis
```bash
dnf install epel-release
dnf install lynis
```

## Przeprowadzenie audytu
```bash
lynis audit system                      # pełny audyt
lynis show warnings                     # tylko ostrzeżenia
lynis show details                      # szczegóły
```
> Przykładowy wynik `lynis audit system` (fragment):
> 
>   Lynis 3.1.1
> 
>   ################################################################################
>   Lynis comes with ABSOLUTELY NO WARRANTY. This is free software.
>   ################################################################################
> 
>   [+] Initializing program
>   ---------------------------------------------
>   - Detection of OS................................... RHEL 10
>   ...
>   ===============================================================================
>   Hardening index : 72 [#############       ]
>   ===============================================================================
> Przykładowy wynik `lynis show warnings`:
>   lynis: No warnings found (system is well-configured)

## Analiza wyników
Lynis wypisuje:
- **Warnings** – problemy do natychmiastowego rozwiązania (np. brak firewalla)
- **Suggestions** – zalecenia (np. skonfiguruj logrotate)
- **Hardening index** – wynik w skali 0-100 (im wyżej tym lepiej)

## Po audycie – poprawki (przykłady)
```bash
# Jeśli Lynis zgłasza brak automatycznych aktualizacji:
dnf install dnf-automatic
systemctl enable --now dnf-automatic.timer

# Jeśli brak firewall:
systemctl enable --now firewalld

# Jeśli hasła bez polityki wygaśnięcia:
chage -M 90 $USER          # hasło wygasa po 90 dniach
```

## Dlaczego audytować?
- Lynis jest używany przez profesjonalnych pentesterów i audytorów
- Wynik audytu pokazuje luki zanim zrobi to atakujący
- Przydatne do compliance: PCI-DSS, HIPAA, ISO 27001

## Uwagi
- ⚠️ Lynis nie naprawi problemów za Ciebie – to narzędzie diagnostyczne
- ⚠️ Wynik < 70 to sygnał że system wymaga pilnego hardeningu
- Regularne audyty (np. co miesiąc) pomagają utrzymać bezpieczeństwo

### Weryfikacja:
```bash
lynis show warnings
```
Oczekiwany wynik: lista ostrzeżeń bezpieczeństwa (lub brak ostrzeżeń, jeśli system jest dobrze skonfigurowany)
    "#.into()) },
            Challenge { id: 19, title: "Wielowątkowość w bashu".into(), description: "Napisz skrypt używający background jobs, xargs -P, GNU parallel. Zrównoleglij zadania.".into(), category: Category::Shell, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Równoległego wykonywania zadań w bashu.

### Wymagania:
- Ukończone wyzwanie "Skrypty bash"
- Znajomość pętli bash (for, while) i składni skryptów
- Podstawowa znajomość poleceń curl, convert

## Background jobs (&)
```bash
for url in $(cat urls.txt); do
    curl -O "$url" &
done
wait   # czekaj aż wszystkie się zakończą
```

## xargs -P (parallel)
```bash
# Pobierz 10 plików równolegle (maks 5 naraz)
cat urls.txt | xargs -P 5 -I {} curl -O {}
```

## GNU Parallel (potężniejsze narzędzie)
```bash
dnf install parallel
# Przetwarzanie obrazów (4 procesy naraz):
ls *.jpg | parallel -j4 convert {} {.}.png
```
> Przykładowy wynik `cat urls.txt | xargs -P 5 -I {} curl -O {}`:
>   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
>                                  Dload  Upload   Total   Spent    Left  Speed
> 100 12345  100 12345    0     0  50000      0 --:--:-- --:--:-- --:--:-- 50000
> Przykładowy wynik `ls *.jpg | parallel -j4 convert {} {.}.png`:
> (brak outputu – pliki PNG są tworzone w bieżącym katalogu)

## Dlaczego to przydatne?
- Zadania administracyjne często są niezależne (ping do 100 serwerów)
- Zamiast czekać 10 minut w pętli – wykonaj w 30 sekund równolegle
- `xargs -P` jest standardowo dostępny na każdym Linuxie

## Uwagi
- ⚠️ Uważaj na race conditions – wiele procesów nie powinno pisać do tego samego pliku
- ⚠️ Nie uruchamiaj więcej procesów niż masz rdzeni CPU – system się zatnie
- `wait` bez argumentów czeka na WSZYSTKIE background jobs

### Weryfikacja:
```bash
echo "test parallel" | xargs -P 1 -I {} echo "Wynik: {}"
```
Oczekiwany wynik:
> Wynik: test parallel
    "#.into()) },
            Challenge { id: 20, title: "CI/CD Pipeline".into(), description: "Skonfiguruj prosty pipeline CI/CD (GitHub Actions lub GitLab CI). Automatyzuj testy i deployment.".into(), category: Category::DevOps, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Automatyzacji testowania i wdrażania z GitHub Actions.

### Wymagania:
- Ukończone wyzwanie "Skrypty bash"
- Podstawowa znajomość GIT i repozytoriów zdalnych (GitHub, GitLab)
- Posiadanie konta na GitHub/GitLab

## Przykład: GitHub Actions (.github/workflows/deploy.yml)
```yaml
name: Deploy
on:
  push:
    branches: [ main ]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Test
        run: |
          echo "Uruchamianie testów..."
          # tu testy aplikacji

  deploy:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Deploy na VPS
        uses: appleboy/ssh-action@v1
        with:
          host: ${{ secrets.HOST }}
          username: ${{ secrets.USER }}
          key: ${{ secrets.SSH_KEY }}
          script: |
            cd /opt/myapp
            git pull
            systemctl restart myapp
```

## Dlaczego CI/CD?
- Każda zmiana kodu jest automatycznie testowana – błędy wykrywane wcześnie
- Deployment jest powtarzalny i udokumentowany (nie robisz ręcznie `scp`)
- Secrets (hasła, klucze) są bezpiecznie przechowywane w GitHub

## Uwagi
- ⚠️ Nigdy nie koduj haseł w pipeline – używaj GitHub Secrets lub Ansible Vault
- ⚠️ Przed deploymentem na produkcję: test na stagingu
- `matrix:` w GitHub Actions pozwala testować na wielu wersjach (np. Python 3.9, 3.11, 3.12)

### Weryfikacja:
```bash
ls -la .github/workflows/
```
Oczekiwany wynik: powinieneś zobaczyć plik deploy.yml (lub inny plik workflow)
    "#.into()) },
            // -- Nowe wyzwania (id 21-40) --
            Challenge { id: 21, title: "Znajdowanie plików".into(), description: "Opanuj find, locate, which, whereis. Szukaj plików po nazwie, rozmiarze, dacie, zawartości z grep.".into(), category: Category::Linux, difficulty: 1, completed: false, details: Some(r#"
**Czego się nauczysz:** Skutecznego wyszukiwania plików.

### Wymagania:
- Podstawowa obsługa terminala (ls, cd)
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość struktury katalogów Linux

## Polecenia
```bash
find /etc -name "*.conf"           # szukaj po nazwie
find / -size +100M -type f         # duże pliki (>100MB)
find /home -mtime -7               # zmodyfikowane w ostatnich 7 dniach
find /var -perm 777                # pliki z niebezpiecznymi uprawnieniami

locate sshd_config                 # szybkie wyszukiwanie z bazy (updatedb)
which nginx                        # ścieżka do binarki
whereis nginx                      # binarka + dokumentacja + config
```
> Przykładowy wynik `find /etc -name "*.conf"`:
> /etc/resolv.conf
> /etc/dnf/dnf.conf
> /etc/rsyslog.conf
> ...
> Przykładowy wynik `which nginx`:
> /usr/sbin/nginx
> Przykładowy wynik `locate sshd_config`:
> /etc/ssh/sshd_config

## Dlaczego `find`?
- `find` przeszukuje system plików w czasie rzeczywistym – dokładny ale wolniejszy
- `locate` używa bazy danych (`/var/lib/mlocate`) – błyskawiczny ale nie widzi nowych plików
- `which` vs `whereis`: `which` szuka tylko w PATH, `whereis` szuka też man i configów

## Uwagi
- ⚠️ `find / -name "*.log"` przeszuka cały system – na dużym systemie może zająć minuty. Zawęź katalog
- Po dodaniu plików: `sudo updatedb` (aktualizacja bazy locate)
- `find -exec` wykonuje polecenie na znalezionych plikach: `find . -name "*.tmp" -exec rm {} \;`

### Weryfikacja:
```bash
find /etc -maxdepth 1 -name "*.conf" | head -5
```
Oczekiwany wynik: lista 5 plików .conf z katalogu /etc
    "#.into()) },
            Challenge { id: 22, title: "Archiwizacja i kompresja".into(), description: "Naucz się tar, gzip, bzip2, xz, zip. Twórz i rozpakowuj archiwa. Zrozum różnice między algorytmami kompresji.".into(), category: Category::Linux, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Tworzenia i rozpakowywania archiwów.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość struktur katalogów
- Podstawowa wiedza o backupach

## Polecenia
```bash
tar -czvf archiwum.tar.gz /katalog/      # gzip (szybki)
tar -cjvf archiwum.tar.bz2 /katalog/     # bzip2 (lepsza kompresja)
tar -cJvf archiwum.tar.xz /katalog/      # xz (najlepsza kompresja)

# Rozpakowywanie
tar -xzvf archiwum.tar.gz                # gzip
tar -xjvf archiwum.tar.bz2               # bzip2
tar -xJvf archiwum.tar.xz                # xz
```
> Przykładowy wynik `tar -czvf archiwum.tar.gz /katalog/`:
> /katalog/
> /katalog/plik1.txt
> /katalog/plik2.txt
> tar: Removing leading `/' from member names

## Różnice między algorytmami
| Algorytm | Szybkość | Rozmiar | Zastosowanie |
|----------|----------|---------|-------------|
| gzip     | szybki   | średni  | backup, wysyłka |
| bzip2    | średni   | mały    | dystrybucja |
| xz       | wolny    | najmniejszy | archiwizacja długoterminowa |

## Dlaczego `tar`?
- `tar` (Tape ARchive) łączy pliki w jeden strumień, a kompresja go ściska
- `tar -czvf`: c=create, z=gzip, v=verbose, f=filename
- Na RHEL `tar` jest zawsze dostępny

## Uwagi
- ⚠️ Zawsze testuj archiwum: `tar -tzvf archiwum.tar.gz` (listuje zawartość bez rozpakowywania)
- Do backupów logów: `tar --remove-files -czf logi-$(date +%Y%m%d).tar.gz /var/log/myapp/`

### Weryfikacja:
```bash
tar -tzf archiwum.tar.gz | head -5
```
Oczekiwany wynik: lista plików wewnątrz archiwum (bez rozpakowywania)
    "#.into()) },
            Challenge { id: 23, title: "Zarządzanie pakietami RPM".into(), description: "Zarządzaj pakietami przez dnf/rpm. Zainstaluj, usuń, zaktualizuj. Użyj repquery, sprawdź zależności. Stwórz własne repozytorium.".into(), category: Category::Linux, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Zarządzania pakietami na RHEL przez DNF.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość pojęcia pakietów i zależności
- Dostęp do repozytoriów RHEL/Fedora

## Podstawy DNF
```bash
dnf install nginx                    # zainstaluj pakiet
dnf remove nginx                     # usuń pakiet
dnf update                           # aktualizuj system
dnf search "web server"              # szukaj pakietu
dnf info nginx                       # szczegóły pakietu
dnf groupinstall "Web Server"        # zainstaluj grupę pakietów
dnf list installed                   # lista zainstalowanych
dnf provides /etc/nginx/nginx.conf   # który pakiet dał ten plik
```
> Przykładowy wynik `dnf info nginx`:
> Installed Packages
> Name         : nginx
> Version      : 1.24.0
> Release      : 1.el10
> Architecture : x86_64
> Przykładowy wynik `rpm -ql nginx`:
> /etc/logrotate.d/nginx
> /etc/nginx/nginx.conf
> /usr/sbin/nginx
> /usr/share/nginx/html/index.html

## RPM – niskopoziomowe narzędzie
```bash
rpm -i pakiet.rpm                    # instalacja pliku .rpm
rpm -e pakiet                        # usunięcie pakietu
rpm -ql nginx                        # jakie pliki dał pakiet
rpm -qf /etc/hosts                   # który pakiet jest właścicielem pliku
```

## Dlaczego DNF?
- DNF to następca YUM (od RHEL 8). YUM już nie istnieje w RHEL 10
- DNF rozwiązuje zależności automatycznie (RPM nie)
- `dnf history` – cofnij ostatnią operację (przydatne przy błędzie)

## Uwagi
- ⚠️ `dnf update` bez filtra aktualizuje WSZYSTKO – na produkcji używaj `dnf update --security`
- ⚠️ Przed instalacją z EPEL: `dnf install epel-release`
- `dnf check` – sprawdź zależności (czy czegoś nie brakuje)

### Weryfikacja:
```bash
dnf info nginx | grep -E "^(Name|Version)"
```
Oczekiwany wynik: wyświetli nazwę i wersję zainstalowanego pakietu nginx
    "#.into()) },
            Challenge { id: 24, title: "Zarządzanie użytkownikami i grupami".into(), description: "Dodawaj, usuwaj i modyfikuj użytkowników i grupy. Zarządzaj hasłami, politykami wygaśnięcia, katalogami domowymi.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Zarządzania kontami użytkowników w RHEL.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Ukończone wyzwanie "Uprawnienia plików"
- Dostęp do roota lub sudo

## Polecenia
```bash
useradd -m -G wheel nowyuser           # dodaj użytkownika z grupą wheel (sudo)
usermod -aG docker istniejący          # dodaj do grupy docker
passwd nowyuser                        # ustaw hasło
userdel -r nowyuser                    # usuń użytkownika z katalogiem domowym
groupadd devs                          # stwórz grupę

# Polityka haseł
chage -l nowyuser                      # zobacz politykę wygaśnięcia
chage -M 90 nowyuser                   # hasło wygasa po 90 dniach
chage -W 7 nowyuser                    # ostrzeżenie 7 dni przed wygaśnięciem
```
> Przykładowy wynik `chage -l nowyuser`:
> Last password change                                    : Jun 04, 2026
> Password expires                                        : Sep 02, 2026
> Password inactive                                       : never
> Account expires                                         : never
> Minimum number of days between password change          : 0
> Maximum number of days between password change          : 90
> Number of days of warning before password expires       : 7

## Pliki konfiguracyjne
- `/etc/passwd` – lista użytkowników (nazwa:hasło:UID:GID:opis:katalog:shell)
- `/etc/shadow` – zaszyfrowane hasła i polityki (tylko root ma read)
- `/etc/group` – grupy
- `/etc/default/useradd` – domyślne ustawienia dla nowych użytkowników

## Dlaczego grupa wheel?
- Na RHEL sudo jest skonfigurowany tak, że tylko członkowie grupy `wheel` mogą używać `sudo`
- W Ubuntu to grupa `sudo` – to jedyna różnica
- `visudo` – edytuj /etc/sudoers (nigdy ręcznie!)

## Uwagi
- ⚠️ Hasła w `/etc/passwd` to `x` – prawdziwe hasła są w `/etc/shadow` (dostęp tylko root)
- ⚠️ `userdel` bez `-r` zostawia katalog domowy – użytkownik może być zdezorientowany
- Na RHEL 10 domyślny zakres UID: 1000-60000 dla zwykłych użytkowników

### Weryfikacja:
```bash
id nowyuser
```
Oczekiwany wynik: powinieneś zobaczyć UID, GID i grupy (w tym wheel) dla użytkownika nowyuser
    "#.into()) },
            Challenge { id: 25, title: "Planowanie zadań – cron, at, systemd-timery".into(), description: "Zaplanuj zadania okresowe przez cron, jednorazowe przez at, i nowoczesne timery systemd. Monitoruj wykonanie.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Planowania zadań w Linux – od tradycyjnego cron po nowoczesne timery systemd.

### Wymagania:
- Ukończone wyzwanie "Skrypty bash"
- Ukończone wyzwanie "Systemd i serwisy"
- Znajomość edytora tekstu

## Cron – tradycyjne planowanie
```bash
crontab -e            # edytuj crontab bieżącego użytkownika
crontab -l            # wyświetl crontab
# Format: minuta godzina dzień_miesiąc miesiąc dzień_tygodnia polecenie
# przykład: codziennie o 3:15
15 3 * * * /usr/bin/backup.sh
```
> Przykładowy wynik `crontab -l`:
> 15 3 * * * /usr/bin/backup.sh

## systemd-timery (nowoczesna alternatywa)
```bash
# /etc/systemd/system/backup.timer
[Unit]
Description=Daily backup

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target

# + backup.service (ten sam co zawsze)
# Włącz:
systemctl enable --now backup.timer
systemctl list-timers                 # zobacz wszystkie timery
```
> Przykładowy wynik `systemctl enable --now backup.timer`:
> Created symlink /etc/systemd/system/timers.target.wants/backup.timer -> /etc/systemd/system/backup.timer
> Przykładowy wynik `systemctl list-timers`:
> NEXT                        LEFT          LAST                        PASSED  UNIT         ACTIVATES
> Thu 2026-06-05 03:00:00 CEST 17h left     Thu 2026-06-04 03:00:00 CEST 7h ago  backup.timer backup.service

## Dlaczego timery zamiast cron?
- Logi trafiają do journald – `journalctl -u backup.service` zamiast szukać w mailu
- `Persistent=true` – jeśli system był wyłączony w zaplanowanym czasie, wykona zadanie po starcie
- Możesz ustawić zależności między timerami (After, Requires)
- Zegar czasu rzeczywistego: `OnCalendar=Mon..Fri 09:00:00`

## Uwagi
- ⚠️ Ścieżki w cronie muszą być absolutne – cron nie ładuje .bashrc i PATH
- ⚠️ Jeśli używasz Ansible – timery są lepsze, bo dają się łatwo zarządzać przez systemd module
- `at` – jednorazowe zadanie: `echo "shutdown -h now" | at now + 1 hour`

### Weryfikacja:
```bash
systemctl list-timers --all | head -10
```
Oczekiwany wynik: lista timerów systemd (w tym backup.timer, jeśli został skonfigurowany)
    "#.into()) },
            Challenge { id: 26, title: "Jądro i moduły".into(), description: "Zarządzaj modułami jądra przez modprobe, lsmod, modinfo. Skompiluj moduł. Skonfiguruj parametry jądra przez sysctl.".into(), category: Category::System, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Zarządzania modułami i parametrami jądra.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość jądra Linux i sprzętu
- Dostęp do roota

## Zarządzanie modułami
```bash
lsmod                                # lista załadowanych modułów
modinfo kvm                          # szczegóły modułu
modprobe -r kvm                      # usuń moduł z jądra
modprobe vfio-pci                    # załaduj moduł
```
> Przykładowy wynik `lsmod` (skrócony):
> Module                  Size  Used by
> kvm_intel             409600  0
> kvm                   987136  1 kvm_intel
> vfio_pci               73728  0
> Przykładowy wynik `modinfo kvm`:
> filename:       /lib/modules/6.8.0/kernel/arch/x86/kvm/kvm.ko
> license:        GPL
> description:    Kernel-based Virtual Machine

## Parametry jądra (sysctl)
```bash
sysctl -a                            # wszystkie parametry
sysctl net.ipv4.ip_forward           # sprawdź konkretny
sysctl -w net.ipv4.ip_forward=1     # zmień w locie

# Trwale: dodaj do /etc/sysctl.conf lub /etc/sysctl.d/
echo "net.ipv4.ip_forward = 1" > /etc/sysctl.d/99-forward.conf
sysctl -p /etc/sysctl.d/99-forward.conf
```
> Przykładowy wynik `sysctl net.ipv4.ip_forward`:
> net.ipv4.ip_forward = 0
> Przykładowy wynik `sysctl -w net.ipv4.ip_forward=1`:
> net.ipv4.ip_forward = 1
> Przykładowy wynik `sysctl -p /etc/sysctl.d/99-forward.conf`:
> net.ipv4.ip_forward = 1

## Dlaczego to ważne?
- `net.ipv4.ip_forward` – niezbędne do NAT/routera/WireGuard
- `vm.swappiness` – kontrola kiedy system zaczyna używać swap (RHEL domyślnie 30)
- `kernel.sysrq` – magic SysRq do awaryjnego restartu (warto włączyć)

## Uwagi
- ⚠️ `sysctl -w` zmienia parametry tylko do restartu – użyj pliku `.conf` dla trwałości
- ⚠️ Niektóre parametry (np. sieciowe) wymagają `sysctl -p` po zmianie w pliku
- Na RHEL moduły są w `/lib/modules/$(uname -r)/`

### Weryfikacja:
```bash
sysctl net.ipv4.ip_forward
```
Oczekiwany wynik: wartość parametru net.ipv4.ip_forward (0 = wyłączone, 1 = włączone)
    "#.into()) },
            Challenge { id: 27, title: "Proces rozruchu systemu".into(), description: "Zrozum etapy bootowania: BIOS/UEFI → GRUB → jądro → initramfs → systemd. Skonfiguruj GRUB. Napraw uszkodzony bootloader.".into(), category: Category::System, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Kolejnych etapów bootowania i konfiguracji GRUB.

### Wymagania:
- Ukończone wyzwanie "Jądro i moduły"
- Znajomość partycjonowania dysków
- Dostęp do roota lub możliwość rebootu VM

## Etapy bootowania
1. **BIOS/UEFI** – włącza sprzęt, szuka bootloadera
2. **GRUB** – ładuje jądro i initramfs do pamięci
3. **Jądro** – inicjalizuje sprzęt, montuje initramfs
4. **initramfs** – ładuje moduły potrzebne do zamontowania rootfs
5. **systemd** – uruchamia serwisy i targety

## Konfiguracja GRUB
```bash
# /etc/default/grub
GRUB_TIMEOUT=5                       # czas na wybór systemu (sekundy)
GRUB_CMDLINE_LINUX="rhgb quiet"      # parametry jądra
# rhgb = Red Hat Graphical Boot (logo z kropkami)
# quiet = ukryj szczegółowe logi

# Po zmianie:
grub2-mkconfig -o /boot/grub2/grub.cfg   # BIOS
# lub
grub2-mkconfig -o /boot/efi/EFI/redhat/grub.cfg  # UEFI
```
> Przykładowy wynik `grub2-mkconfig -o /boot/grub2/grub.cfg`:
> Generating grub configuration file ...
> Found linux image: /boot/vmlinuz-6.8.0-1.el10.x86_64
> Found initrd image: /boot/initramfs-6.8.0-1.el10.x86_64
> Found Fedora Linux (40) on /dev/sda2
> done

## Naprawa bootloadera (awaryjny recovery)
```bash
# Z live CD / rescue mode:
chroot /mnt/sysimage
grub2-install /dev/sda               # reinstaluj GRUB
grub2-mkconfig -o /boot/grub2/grub.cfg
```
> Przykładowy wynik `grub2-install /dev/sda`:
> Installing for i386-pc platform.
> Installation finished. No error reported.

## Uwagi
- ⚠️ Jeśli system nie bootuje – użyj `rd.break` w parametrach jądra (wchodzi do emergency shell)
- ⚠️ Zawsze testuj zmiany w GRUB na VM przed produkcją
- Na RHEL 10 UEFI jest domyślną metodą bootowania

### Weryfikacja:
```bash
grub2-editenv list
```
Oczekiwany wynik: wyświetli bieżące zmienne środowiskowe GRUB (np. saved_entry, kernelopts)
    "#.into()) },
            Challenge { id: 28, title: "Zaawansowana sieć TCP/IP".into(), description: "Skonfiguruj bonding/team interfejsów, VLAN, bridge. Użyj tc do kontroli ruchu. Zrozum MTU, TTL, routing policy.".into(), category: Category::Network, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Zaawansowanej konfiguracji sieci – bonding, VLAN, bridge, tc.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Ukończone wyzwanie "Diagnostyka sieci"
- Znajomość adresacji IP, masek, routingu
- Dostęp do roota i co najmniej 2 interfejsów sieciowych

## Bonding (agregacja łączy)
```bash
# moduł bonding
modprobe bonding
# Stwórz bond0:
nmcli con add type bond con-name bond0 ifname bond0 mode active-backup
nmcli con add type ethernet con-name bond0-port1 ifname eth0 master bond0
nmcli con add type ethernet con-name bond0-port2 ifname eth1 master bond0
nmcli con mod bond0 ipv4.addresses 192.168.1.100/24
nmcli con mod bond0 ipv4.method manual
nmcli con up bond0
```
> Przykładowy wynik `cat /proc/net/bonding/bond0`:
> Ethernet Channel Bonding Driver: v6.8.0
> Bonding Mode: fault-tolerance (active-backup)
> Currently Active Slave: eth0

## VLAN
```bash
nmcli con add type vlan con-name vlan10 ifname eth0.10 dev eth0 id 10
nmcli con mod vlan10 ipv4.addresses 10.0.10.1/24
nmcli con mod vlan10 ipv4.method manual
nmcli con up vlan10
```
> Przykładowy wynik `ip link show type vlan`:
> eth0.10@eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP

## Bridge (most)
```bash
nmcli con add type bridge con-name br0 ifname br0
nmcli con add type ethernet con-name br0-port1 ifname eth0 master br0
nmcli con mod br0 ipv4.addresses 192.168.1.1/24
nmcli con mod br0 ipv4.method manual
nmcli con up br0
```
> Przykładowy wynik `brctl show`:
> bridge name     bridge id               STP enabled     interfaces
> br0             8000.001122334455       no              eth0

## tc (Traffic Control)
```bash
# Ogranicz przepustowość do 1mbit na eth0
tc qdisc add dev eth0 root handle 1: htb default 30
tc class add dev eth0 parent 1: classid 1:1 htb rate 1mbit
```
> Przykładowy wynik `tc -s qdisc show dev eth0`:
> qdisc htb 1: root refcnt 2 r2q 10 default 30 direct_packets_stat 0

## Uwagi
- ⚠️ Tryb bonding active-backup nie wymaga switcha z LACP – do testów na VM wystarczy
- ⚠️ Bridge jest niezbędny dla KVM/qemu – bez niego maszyny wirtualne nie mają sieci
- MTU domyślnie 1500 – do jumbo frames ustaw `nmcli con mod eth0 802-3-ethernet.mtu 9000`

### Weryfikacja:
```bash
ip link show type bond
```
Oczekiwany wynik: lista interfejsów bond (bond0) z ich stanem i trybem
    "#.into()) },
            Challenge { id: 29, title: "Firewalld – strefy i rich rules".into(), description: "Skonfiguruj zaawansowane reguły firewalld: strefy, źródła, rich rules, masquerade, forward portów. Przetestuj i debuguj.".into(), category: Category::Network, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Zaawansowanej konfiguracji firewalld na RHEL.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Ukończone wyzwanie "Konfiguracja sieci"
- Znajomość stref firewalld i podstaw reguł

## Strefy
```bash
firewall-cmd --get-zones                    # lista dostępnych stref
firewall-cmd --zone=internal --list-all     # reguły strefy internal
firewall-cmd --set-default-zone=drop        # domyślna: drop (odrzuć wszystko)
firewall-cmd --zone=trusted --add-source=192.168.100.0/24
```
> Przykładowy wynik `firewall-cmd --get-zones`:
> block dmz drop external home internal public trusted work
> Przykładowy wynik `firewall-cmd --zone=internal --list-all`:
> internal (active)
>   target: default
>   services: dhcpv6-client mdns samba-client ssh
> Przykładowy wynik `firewall-cmd --add-source=192.168.100.0/24 --zone=trusted`:
> success

## Rich rules (zaawansowane reguły)
```bash
# Forward port (przekierowanie portów)
firewall-cmd --add-forward-port=port=80:proto=tcp:toport=8080

# Masquerade (NAT)
firewall-cmd --add-masquerade

# Rich rule: ogranicz SSH do konkretnego IP
firewall-cmd --add-rich-rule='rule family="ipv4" source address="192.168.1.0/24" service name="ssh" accept'
```
> Przykładowy wynik `firewall-cmd --add-forward-port=port=80:proto=tcp:toport=8080`:
> success
> Przykładowy wynik `firewall-cmd --add-rich-rule=...`:
> success

## Dlaczego rich rules?
- Nie każdą regułę da się wyrazić przez proste `--add-service`
- Rich rules dają kontrolę nad źródłem, limitowaniem, logowaniem
- Przykład: przepuść ruch tylko z określonej sieci

## Uwagi
- ⚠️ Kolejność reguł ma znaczenie – pierwsze pasujące rule wygrywa
- ⚠️ Debugowanie: `firewall-cmd --direct --get-all-rules` (reguły bezpośrednie)
- Po zmianach: `firewall-cmd --reload` (nie restart – nie zerwie połączeń)

### Weryfikacja:
```bash
firewall-cmd --list-all
```
Oczekiwany wynik: lista wszystkich reguł dla domyślnej strefy, w tym dodane usługi i rich rules
    "#.into()) },
            Challenge { id: 30, title: "VPN z WireGuard".into(), description: "Skonfiguruj serwer i klienta WireGuard. Wygeneruj klucze, skonfiguruj Peer, przetestuj tunel. Dodaj routing i firewall.".into(), category: Category::Network, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Konfiguracji WireGuard – nowoczesnego, szybkiego VPN.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Znajomość adresacji IP i routingu
- Dwa hosty do testów (lub VM)

## Instalacja
```bash
dnf install wireguard-tools
```
> Przykładowy wynik:
> Installed: wireguard-tools-1.0.20220627-1.el10.x86_64

## Generowanie kluczy
```bash
wg genkey | tee privatekey | wg pubkey > publickey
```
> Przykładowy wynik po `cat privatekey`:
> uAH9v... (klucz prywatny Base64)
> Przykładowy wynik po `cat publickey`:
> fE80s... (klucz publiczny Base64)

## Konfiguracja serwera (/etc/wireguard/wg0.conf)
```ini
[Interface]
Address = 10.0.0.1/24
PrivateKey = <klucz_serwera>
ListenPort = 51820

[Peer]
PublicKey = <klucz_publiczny_klienta>
AllowedIPs = 10.0.0.2/32
```

## Konfiguracja klienta
```ini
[Interface]
Address = 10.0.0.2/24
PrivateKey = <klucz_klienta>

[Peer]
PublicKey = <klucz_publiczny_serwera>
Endpoint = 192.168.1.100:51820
AllowedIPs = 0.0.0.0/0     # cały ruch przez VPN
```

## Uruchomienie
```bash
systemctl enable --now wg-quick@wg0
firewall-cmd --add-port=51820/udp --permanent
firewall-cmd --reload
```
> Przykładowy wynik `systemctl enable --now wg-quick@wg0`:
> Created symlink /etc/systemd/system/multi-user.target.wants/wg-quick@wg0.service -> /usr/lib/systemd/system/wg-quick@wg0.service
> Przykładowy wynik `firewall-cmd --add-port=51820/udp --permanent`:
> success

## Dlaczego WireGuard, a nie OpenVPN?
- WireGuard jest wbudowany w jądro Linux (od wersji 5.6) – błyskawiczny
- 4000 linii kodu vs OpenVPN ~600 000 – mniejsza powierzchnia ataku
- Prostota: klucze zamiast certyfikatów, konfiguracja w 1 pliku

## Uwagi
- ⚠️ `AllowedIPs` określa które sieci mają iść przez tunel – `0.0.0.0/0` = cały ruch
- ⚠️ WireGuard nie loguje aktywności – nie ma auditingu połączeń
- Do masowego zarządzania klientami rozważ Netbird / Headscale

### Weryfikacja:
```bash
wg show
```
Oczekiwany wynik: lista interfejsów WireGuard, ich kluczy publicznych i statystyk transferu
    "#.into()) },
            Challenge { id: 31, title: "Diagnostyka sieci – tcpdump, nmap, iperf".into(), description: "Przechwytuj pakiety tcpdump, skanuj sieć nmap, mierz przepustowość iperf. Analizuj ruch i diagnozuj problemy.".into(), category: Category::Network, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Diagnostyki sieci z narzędziami wiersza poleceń.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Dwa hosty do testów iperf (lub localhost)
- Podstawowa znajomość protokołów TCP/IP

## tcpdump – przechwytywanie pakietów
```bash
tcpdump -i eth0                          # wszystkie pakiety na eth0
tcpdump -i eth0 port 80                  # tylko HTTP
tcpdump -i eth0 host 192.168.1.1         # tylko do/z hosta
tcpdump -i eth0 -w capture.pcap          # zapisz do pliku
tcpdump -r capture.pcap                  # odczytaj plik
```
> Przykładowy wynik `tcpdump -i eth0 -c 3`:
> tcpdump: listening on eth0, link-type EN10MB (Ethernet), snapshot length 262144 bytes
> 10:00:01.123456 IP 192.168.1.100.ssh > 192.168.1.1.54321: Flags [P.], seq 1:100, ack 1, win 501
> 10:00:02.234567 IP 192.168.1.1.54321 > 192.168.1.100.ssh: Flags [.], ack 100, win 509
> 3 packets captured

## nmap – skanowanie sieci
```bash
nmap -sS 192.168.1.0/24                 # szybki scan (SYN stealth)
nmap -sV 192.168.1.100                   # wykryj wersje usług
nmap -O 192.168.1.100                    # wykryj system operacyjny
nmap -p 1-65535 192.168.1.100            # wszystkie porty
```
> Przykładowy wynik `nmap -sS 192.168.1.0/24`:
> Starting Nmap 7.94 ( https://nmap.org )
> Nmap scan report for 192.168.1.1
> Host is up (0.0010s latency).
> Not shown: 997 closed tcp ports (reset)
> PORT     STATE SERVICE
> 22/tcp   open  ssh
> 80/tcp   open  http
> 443/tcp  open  https
> Przykładowy wynik `nmap -sV 192.168.1.100`:
> PORT   STATE SERVICE VERSION
> 22/tcp open  ssh     OpenSSH 9.6 (protocol 2.0)
> 80/tcp open  http    nginx 1.24.0

## iperf – pomiar przepustowości
```bash
# Serwer:
iperf3 -s
# Klient:
iperf3 -c 192.168.1.100
```
> Przykładowy wynik `iperf3 -c 192.168.1.100`:
> Connecting to host 192.168.1.100, port 5201
> [  5] local 192.168.1.101 port 54321 connected to 192.168.1.100 port 5201
> [ ID] Interval           Transfer     Bitrate
> [  5] 0.00-10.00 sec   1.02 GBytes   876 Mbits/sec

## Dlaczego te narzędzia?
- tcpdump: podgląd wszystkiego co leci w sieci – niezastąpiony przy debugowaniu
- nmap: znajdź otwarte porty i podatności we własnej sieci
- iperf: zmierz faktyczną przepustowość (nie myl z transferem plików)

## Uwagi
- ⚠️ nmap na obce sieci bez zgody jest NIELEGALNY w wielu krajach
- ⚠️ tcpdump wymaga root – inaczej zobaczysz tylko własny ruch
- tcpdump domyślnie nie rozwiązuje nazw (DNS) – używaj `-n` dla szybkości

### Weryfikacja:
```bash
nmap -sS -p 22 localhost | grep ssh
```
Oczekiwany wynik: potwierdzenie, że port 22 (SSH) jest otwarty na localhost
    "#.into()) },
            Challenge { id: 32, title: "Szyfrowanie GPG i OpenSSL".into(), description: "Generuj klucze GPG, szyfruj/deszyfruj pliki, podpisuj. Użyj OpenSSL do tworzenia certyfikatów, szyfrowania, generowania hash.".into(), category: Category::Security, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Szyfrowania symetrycznego i asymetrycznego z GPG i OpenSSL.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość kryptografii (klucz publiczny/prywatny)
- Edytor tekstu do tworzenia plików testowych

## GPG – szyfrowanie asymetryczne
```bash
gpg --full-generate-key                # generuj parę kluczy
gpg --list-keys                        # lista kluczy publicznych
gpg --encrypt --recipient user@email.pl plik.txt     # szyfruj
gpg --decrypt plik.txt.gpg                            # deszyfruj
gpg --sign plik.txt                                   # podpisz cyfrowo
gpg --verify plik.txt.gpg                             # zweryfikuj podpis
```
> Przykładowy wynik `gpg --list-keys`:
> pub   ed25519 2026-06-04 [SC]
>       A1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0
> uid           [ultimate] Jan Kowalski <jan@example.com>
> Przykładowy wynik `gpg --encrypt` (brak outputu – powstaje plik.txt.gpg)
> Przykładowy wynik `gpg --decrypt plik.txt.gpg`:
> gpg: encrypted with 256-bit ECDH key, ID A1B2C3D4E5F6A7B8, created 2026-06-04
> (treść odszyfrowanego pliku)

## OpenSSL – certyfikaty i hashe
```bash
# Generuj certyfikat self-signed
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout klucz.pem -out cert.pem

# Hash pliku
openssl dgst -sha256 -hex plik.txt     # SHA-256

# Szyfrowanie symetryczne (AES)
openssl enc -aes-256-cbc -salt -in plik.txt -out plik.enc
openssl enc -aes-256-cbc -d -in plik.enc -out plik.txt
```
> Przykładowy wynik `openssl req -x509 ...`:
> Generating a RSA private key
> .+++++
> .+++++
> writing new private key to 'klucz.pem'
> -----
> Przykładowy wynik `openssl dgst -sha256 -hex plik.txt`:
> SHA256(plik.txt)= e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Czym się różni GPG od OpenSSL?
- GPG: OpenPGP standard – głównie do szyfrowania plików i emaili (asymetryczne)
- OpenSSL: TLS/SSL certyfikaty i narzędzia kryptograficzne (szyfrowanie, hashe, certyfikaty)
- GPG używa Web of Trust, OpenSSL używa CA (Certificate Authority)

## Uwagi
- ⚠️ Klucz prywatny GPG jest w `~/.gnupg/private-keys-v1.d/` – nigdy nie udostępniaj
- ⚠️ OpenSSL `enc` bez `-pbkdf2` używa przestarzałego algorytmu derive – zawsze dodaj `-pbkdf2`
- Zgubienie hasła GPG = utrata danych – nie ma opcji odzyskania

### Weryfikacja:
```bash
gpg --list-keys
```
Oczekiwany wynik: lista kluczy publicznych GPG (przynajmniej jeden klucz powinien być widoczny)
    "#.into()) },
            Challenge { id: 33, title: "Fail2ban – ochrona przed atakami brute-force".into(), description: "Skonfiguruj fail2ban dla SSH, HTTP, poczty. Stwórz własne filtry i jails. Przetestuj blokowanie i odbanowanie.".into(), category: Category::Security, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Konfiguracji Fail2ban do blokowania ataków brute-force.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Ukończone wyzwanie "SSH i zdalny dostęp"
- Znajomość firewalld i reguł dostępu
- Działający serwer SSH

## Instalacja
```bash
dnf install fail2ban
systemctl enable --now fail2ban
```
> Przykładowy wynik `systemctl enable --now fail2ban`:
> Created symlink /etc/systemd/system/multi-user.target.wants/fail2ban.service -> /usr/lib/systemd/system/fail2ban.service

## Konfiguracja jail (jail.local)
```ini
# /etc/fail2ban/jail.local
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
port = ssh
logpath = %(sshd_log)s
maxretry = 3
```

## Podstawowe polecenia
```bash
fail2ban-client status                   # lista aktywnych jaili
fail2ban-client status sshd              # statystyki SSH
fail2ban-client set sshd unbanip 192.168.1.100  # odbanuj IP
```
> Przykładowy wynik `fail2ban-client status`:
> Status
> |- Number of jail:      1
> `- Jail list:           sshd
> Przykładowy wynik `fail2ban-client status sshd`:
> Status for the jail: sshd
> |- Filter
> |  |- Currently failed: 0
> |  |- Total failed:     3
> `- Actions
>    |- Currently banned: 1
>    |- Total banned:     1
>    `- Banned IP list:   192.168.1.100

## Uwagi
- ⚠️ Domyślny ban jest przez firewalld – nie musisz nic dodatkowo konfigurować
- ⚠️ Przed testowaniem ustaw `bantime = 60` (1 minuta) – nie zablokujesz się na długo
- Fail2ban loguje do `/var/log/fail2ban.log` – sprawdzaj przy problemach

### Weryfikacja:
```bash
fail2ban-client status sshd
```
Oczekiwany wynik: status jaila sshd z liczbą zbanowanych i obecnie zablokowanych IP
    "#.into()) },
            Challenge { id: 34, title: "Audyt bezpieczeństwa z Lynis".into(), description: "Przeprowadź audyt systemu Lynis. Przeanalizuj wyniki i popraw znalezione problemy. Porównaj przed i po.".into(), category: Category::Security, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Audytu bezpieczeństwa systemu z Lynis – analizy i poprawy wyników.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Ukończone wyzwanie "Audyt i compliance" (opcjonalnie, ale pomocne)
- Podstawowa znajomość bezpieczeństwa systemowego

## Instalacja
```bash
dnf install epel-release
dnf install lynis
```

## Przeprowadzenie audytu
```bash
lynis audit system                          # pełny audyt
lynis show warnings                         # tylko ostrzeżenia
lynis show details                          # szczegóły audytu
```
> Przykładowy wynik `lynis audit system` (fragment):
>   Lynis 3.1.1
>   [+] Initializing program
>   - Detection of OS................................... RHEL 10
>   ===============================================================================
>   Hardening index : 72 [#############       ]
>   ===============================================================================
> Przykładowy wynik `lynis show warnings`:
>   lynis: No warnings found (system is well-configured)

## Analiza i poprawki
```bash
# Jeśli Lynis zgłasza brak automatycznych aktualizacji:
dnf install dnf-automatic
systemctl enable --now dnf-automatic.timer

# Jeśli brak firewall:
systemctl enable --now firewalld
```

## Uwagi
- ⚠️ Lynis nie naprawi problemów za Ciebie – to narzędzie diagnostyczne
- ⚠️ Wynik < 70 to sygnał że system wymaga pilnego hardeningu
- Regularne audyty (np. co miesiąc) pomagają utrzymać bezpieczeństwo

### Weryfikacja:
```bash
lynis show warnings
```
Oczekiwany wynik: lista ostrzeżeń bezpieczeństwa (lub brak ostrzeżeń, jeśli system jest dobrze skonfigurowany)
    "#.into()) },
            Challenge { id: 35, title: "Hardening systemu według CIS Benchmarks".into(), description: "Zastosuj wybrane zalecenia CIS Benchmarks dla RHEL/Fedora. Zabezpiecz SSH, jądro, sieć, usługi systemowe.".into(), category: Category::Security, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Wzmacniania bezpieczeństwa systemu zgodnie z CIS Benchmarks.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Ukończone wyzwanie "SSH i zdalny dostęp"
- Ukończone wyzwanie "SELinux/AppArmor"
- Dostęp do roota

## SSH Hardening
```bash
# /etc/ssh/sshd_config
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
MaxAuthTries 3
ClientAliveInterval 300
ClientAliveCountMax 0
```
```bash
systemctl reload sshd
```

## Jądro – sysctl
```bash
# /etc/sysctl.d/99-cis.conf
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.all.send_redirects = 0
net.ipv4.conf.all.accept_source_route = 0
net.ipv4.tcp_syncookies = 1
net.ipv4.ip_forward = 0
kernel.randomize_va_space = 2
```
```bash
sysctl -p /etc/sysctl.d/99-cis.conf
```

## Firewall i sieć
```bash
firewall-cmd --set-default-zone=drop        # domyślnie odrzucaj wszystko
firewall-cmd --zone=trusted --add-source=192.168.1.0/24 --permanent
firewall-cmd --reload
```
> Przykładowy wynik `sysctl -p /etc/sysctl.d/99-cis.conf`:
> net.ipv4.conf.all.accept_redirects = 0
> net.ipv4.conf.all.send_redirects = 0
> net.ipv4.tcp_syncookies = 1

## Audyt
```bash
lynis audit system                           # sprawdź hardening index przed i po
```
> Przykładowy wynik (przed):
> Hardening index : 65 [###########         ]
> Przykładowy wynik (po):
> Hardening index : 82 [###############     ]

## Uwagi
- ⚠️ Nie stosuj wszystkich zaleceń na ślepo – niektóre mogą zepsuć działające usługi
- ⚠️ CIS Benchmarks dzielą się na poziomy: Level 1 (podstawowy) i Level 2 (zaawansowany)
- Po zmianach w sshd zawsze testuj w drugiej sesji – inaczej możesz się zablokować

### Weryfikacja:
```bash
lynis audit system | grep -i "hardening index"
```
Oczekiwany wynik: wartość Hardening index (im wyższa, tym lepiej zabezpieczony system)
    "#.into()) },
            Challenge { id: 36, title: "Wyrażenia regularne w bashu".into(), description: "Opanuj regex: grep -E, sed, awk. Użyj grup, kwantyfikatorów, klas znaków. Napisz skrypt z zaawansowanym przetwarzaniem tekstu.".into(), category: Category::Shell, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Wyrażeń regularnych w bashu z grep, sed, awk.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość potoków (pipe) i przekierowań
- Edytor tekstu do tworzenia plików testowych

## grep – wyszukiwanie wzorców
```bash
grep "ERROR" /var/log/messages            # linie z ERROR
grep -E "[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}" /var/log/messages  # adresy IP
grep -E "^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$" emails.txt       # emaile
```
> Przykładowy wynik `grep "ERROR" /var/log/messages`:
> Jun  4 10:00:01 hostname kernel: ERROR: CPU soft lockup detected

## sed – edycja strumienia
```bash
sed 's/stary/nowy/g' plik.txt             # zamień wszystkie wystąpienia
sed -i 's/Port 22/Port 2222/' /etc/ssh/sshd_config  # w pliku
sed '/^#/d' /etc/ssh/sshd_config          # usuń komentarze
```
> Przykładowy wynik `sed 's/stary/nowy/g' plik.txt`:
> (wyświetla zawartość pliku ze zmienionym tekstem)
> Przykładowy wynik `sed '/^#/d' /etc/ssh/sshd_config`:
> Port 22
> PermitRootLogin no
> PasswordAuthentication yes

## awk – przetwarzanie kolumn
```bash
awk '{print $1, $3}' /var/log/messages    # pierwsza i trzecia kolumna
awk '/ERROR/{print $1, $2, $NF}' /var/log/messages  # data i ostatnie pole dla ERROR
awk '{count[$1]++} END {for (ip in count) print ip, count[ip]}' /var/log/httpd/access_log
```
> Przykładowy wynik `awk '{print $1, $3}' /var/log/messages`:
> Jun 10:00:01
> Jun 10:00:02

## Uwagi
- ⚠️ Regex w grep bez `-E` to BRE (Basic) – `+`, `|`, `()` wymagają `\` lub flagi `-E`
- ⚠️ awk jest potężniejszy od sed do przetwarzania kolumn – ucz się obu
- `grep -r` – przeszukanie całego katalogu rekurencyjnie

### Weryfikacja:
```bash
echo "test123" | grep -E "^[a-z]+[0-9]+$"
```
Oczekiwany wynik: test123 (dopasowanie do wzorca – litery, potem cyfry)
    "#.into()) },
            Challenge { id: 37, title: "Zmienne środowiskowe i ścieżki".into(), description: "Zarządzaj zmiennymi środowiskowymi: env, set, export, PATH, LD_LIBRARY_PATH. Zrozum kolejność ładowania plików rc.".into(), category: Category::Shell, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Zarządzania zmiennymi środowiskowymi w bashu.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość bash i skryptów

## Podstawowe polecenia
```bash
env                          # wszystkie zmienne środowiskowe
set                          # wszystkie zmienne (shell + env)
export VAR="wartosc"         # ustaw i eksportuj zmienną
echo "$HOME"                 # wyświetl zmienną
unset VAR                    # usuń zmienną
```
> Przykładowy wynik `echo "$HOME"`:
> /home/user
> Przykładowy wynik `env | grep PATH`:
> PATH=/usr/local/bin:/usr/bin:/usr/local/sbin:/usr/sbin:/home/user/.local/bin

## PATH – gdzie system szuka programów
```bash
echo "$PATH"                 # zobacz ścieżki
which python3                # którą wersję znajdzie
export PATH=$PATH:/opt/myapp/bin  # dodaj katalog do PATH
```
> Przykładowy wynik `which python3`:
> /usr/bin/python3

## Kolejność ładowania plików
```bash
# Logowanie (login shell):
# 1. /etc/profile
# 2. ~/.bash_profile (lub ~/.bash_login lub ~/.profile)
# Interaktywny (non-login):
# 1. /etc/bashrc
# 2. ~/.bashrc
```
```bash
# Sprawdź jakie zmienne ustawia bash:
echo "$BASH_VERSION"
echo "$SHELL"
echo "$USER"
```
> Przykładowy wynik `echo "$SHELL"`:
> /bin/bash

## Uwagi
- ⚠️ Zmienne ustawione w terminalu giną po zamknięciu – dodaj do `~/.bashrc` dla trwałości
- ⚠️ `LD_LIBRARY_PATH` może powodować problemy bezpieczeństwa – używaj ostrożnie
- `printenv` działa jak `env`, `declare -p` pokazuje wszystkie zmienne z atrybutami

### Weryfikacja:
```bash
echo "Moja zmienna: $USER"
```
Oczekiwany wynik: powinieneś zobaczyć swoją nazwę użytkownika
    "#.into()) },
            Challenge { id: 38, title: "Debugowanie skryptów bash".into(), description: "Użyj set -x, bash -n, trap, shellcheck. Debuguj skrypty, analizuj kody błędów, loguj wykonanie. Znajdź i napraw błędy.".into(), category: Category::Shell, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Debugowania skryptów bash i znajdowania błędów.

### Wymagania:
- Ukończone wyzwanie "Skrypty bash"
- Ukończone wyzwanie "Wyrażenia regularne w bashu"
- Edytor tekstu do edycji skryptów

## Tryby debugowania
```bash
bash -n skrypt.sh              # sprawdź składnię (bez uruchamiania)
bash -x skrypt.sh              # śledź wykonanie krok po kroku
```
> Przykładowy wynik `bash -x skrypt.sh`:
> + SRC=/var/www/html
> + DST=/backup/20260604_100001-html.tar.gz
> + tar -czf /backup/20260604_100001-html.tar.gz /var/www/html

## set – wewnątrz skryptu
```bash
#!/bin/bash
set -e  # zatrzymaj skrypt przy pierwszym błędzie
set -u  # błąd przy niezdefiniowanej zmiennej
set -x  # tryb debug (echo commands)
set -o pipefail  # błąd w pipe też liczy się jako błąd
# lub wszystkie naraz:
set -euxo pipefail
```

## trap – przechwytywanie sygnałów
```bash
#!/bin/bash
cleanup() {
    echo "Czyszczenie przed wyjściem..."
    rm -f /tmp/tempfile
}
trap cleanup EXIT     # wykonaj cleanup przy wyjściu
trap 'echo "Przerwano!"; exit 1' INT  # Ctrl+C
```
> Przykładowy wynik przy Ctrl+C:
> Przerwano!
> Czyszczenie przed wyjściem...

## shellcheck – analiza statyczna
```bash
dnf install shellcheck
shellcheck skrypt.sh
```
> Przykładowy wynik `shellcheck skrypt.sh`:
> In script.sh line 5:
>     echo $var
>          ^-- SC2086: Double quote to prevent globbing and word splitting.

## Uwagi
- ⚠️ `set -e` nie zatrzyma skryptu przy błędzie w warunku `if` lub w pipe (bez `pipefail`)
- ⚠️ Zawsze używaj `set -euo pipefail` w nowych skryptach – to standard bezpieczeństwa
- shellcheck znajdziesz też online: https://www.shellcheck.net/

### Weryfikacja:
```bash
bash -n skrypt.sh
```
Oczekiwany wynik: brak outputu (brak błędów składniowych w skrypcie)
    "#.into()) },
            Challenge { id: 39, title: "Kubernetes – pierwszy klaster".into(), description: "Zainstaluj minikube lub k3s. Uruchom pierwszy pod, deployment, service. Użyj kubectl do zarządzania. Zrób rolling update.".into(), category: Category::DevOps, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Podstaw Kubernetes – deployment, service, rolling update.

### Wymagania:
- Ukończone wyzwanie "Kontenery Podman"
- Znajomość podstaw konteneryzacji (obrazy, kontenery)
- Minimum 4GB RAM i 2 CPU na VM

## Instalacja k3s (lekki Kubernetes)
```bash
curl -sfL https://get.k3s.io | sh -
kubectl get nodes
```
> Przykładowy wynik `kubectl get nodes`:
> NAME     STATUS   ROLES                  AGE   VERSION
> localhost Ready    control-plane,master   1m    v1.30.0+k3s1

## Pierwszy deployment
```bash
kubectl create deployment nginx --image=nginx
kubectl get deployments
kubectl get pods
```
> Przykładowy wynik `kubectl get deployments`:
> NAME    READY   UP-TO-DATE   AVAILABLE   AGE
> nginx   1/1     1            1           10s
> Przykładowy wynik `kubectl get pods`:
> NAME                     READY   STATUS    RESTARTS   AGE
> nginx-7c8b9c8d7f-xk9j2   1/1     Running   0          10s

## Ekspozycja serwisu
```bash
kubectl expose deployment nginx --port=80 --type=NodePort
kubectl get services
```
> Przykładowy wynik `kubectl get services`:
> NAME         TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)        AGE
> kubernetes   ClusterIP   10.43.0.1      <none>        443/TCP        2m
> nginx        NodePort    10.43.0.10     <none>        80:30080/TCP   10s

## Rolling update
```bash
kubectl set image deployment/nginx nginx=nginx:1.25-alpine
kubectl rollout status deployment/nginx
```
> Przykładowy wynik `kubectl rollout status`:
> deployment "nginx" successfully rolled out

## Uwagi
- ⚠️ K3s przechowuje dane w `/var/lib/rancher/k3s/` – zadbaj o backup
- ⚠️ Minikube wymaga sterownika (docker, podman lub virtualbox) – k3s jest prostszy
- kubectl alias: `alias k=kubectl` – oszczędza czas

### Weryfikacja:
```bash
kubectl get pods
```
Oczekiwany wynik: lista uruchomionych podów z statusem Running
    "#.into()) },
            Challenge { id: 40, title: "CI/CD z GitHub Actions".into(), description: "Stwórz kompletny pipeline CI/CD: lint, test, build, deploy na VPS. Użyj secretów, matrix build, cache.".into(), category: Category::DevOps, difficulty: 4, completed: false, details: Some(r#"
**Czego się nauczysz:** Tworzenia kompletnego pipeline CI/CD z GitHub Actions.

### Wymagania:
- Ukończone wyzwanie "CI/CD Pipeline"
- Konto na GitHub i repozytorium z kodem
- VPS z dostępem SSH i skonfigurowanym kluczem

## Struktura pipeline (.github/workflows/ci-cd.yml)
```yaml
name: CI/CD Pipeline
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Lint
        run: |
          echo "Lint passed"

  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node: [18, 20, 22]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node }}
      - run: npm install
      - run: npm test

  build:
    needs: [lint, test]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm install
      - run: npm run build

  deploy:
    needs: [build]
    runs-on: ubuntu-latest
    steps:
      - name: Deploy na VPS
        uses: appleboy/ssh-action@v1
        with:
          host: ${{ secrets.HOST }}
          username: ${{ secrets.USER }}
          key: ${{ secrets.SSH_KEY }}
          script: |
            cd /opt/myapp
            git pull
            npm install
            systemctl restart myapp
```

## Konfiguracja secretów w GitHub
```bash
# W GitHub: Settings > Secrets and variables > Actions
# Dodaj:
# - HOST (IP serwera)
# - USER (nazwa użytkownika SSH)
# - SSH_KEY (klucz prywatny)
```

## Sprawdzenie pipeline
```bash
ls -la .github/workflows/
```
> Przykładowy wynik:
> total 8
> drwxr-xr-x. 2 user user 64 Jun  4 10:00 .
> drwxr-xr-x. 3 user user 96 Jun  4 09:55 ..
> -rw-r--r--. 1 user user 987 Jun  4 10:00 ci-cd.yml

## Uwagi
- ⚠️ Nigdy nie koduj haseł w pipeline – używaj GitHub Secrets
- ⚠️ Testuj pipeline na gałęzi testowej przed mergem do main
- `matrix:` pozwala testować na wielu wersjach (Node 18, 20, 22) równolegle

### Weryfikacja:
```bash
ls -la .github/workflows/
```
Oczekiwany wynik: powinieneś zobaczyć plik ci-cd.yml (lub deploy.yml) z definicją pipeline
    "#.into()) },

            // RHCSA gap challenges (41+)
            Challenge { id: 41, title: "Logowanie systemowe – journalctl i rsyslog".into(), description: "Analizuj logi systemowe za pomocą journalctl i rsyslog. Konfiguruj poziomy logowania, rotację i przesyłanie logów do centralnego serwera.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Analizy logów systemowych, konfiguracji rsyslog, rotacji logów.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Umiejętność edycji plików konfiguracyjnych
- Podstawowa znajomość systemd i usług

## Podstawy journalctl
```bash
journalctl -xe                         # ostatnie logi + wyjaśnienie
journalctl -u sshd                     # logi konkretnej usługi
journalctl --since "1 hour ago"        # logi z ostatniej godziny
journalctl -p err                      # tylko błędy (priority: emerg, alert, crit, err, warning, notice, info, debug)
journalctl -f                          # live tail logów (jak tail -f)
```
> Przykładowy wynik `journalctl -xe`:
> -- Logs begin at Thu 2026-06-04 09:55:00 CEST, end at Thu 2026-06-04 10:30:00 CEST. --
> Jun 04 10:00:01 rhel10 systemd[1]: Started Session 1 of user root.
> Jun 04 10:00:05 rhel10 sshd[1234]: Server listening on 0.0.0.0 port 22.
> ...
> Przykładowy wynik `journalctl -u sshd`:
> Jun 04 10:00:05 rhel10 sshd[1234]: Server listening on 0.0.0.0 port 22.
> Jun 04 10:00:12 rhel10 sshd[1234]: Accepted publickey for root from 192.168.1.100 port 54321
> ...
> Przykładowy wynik `journalctl -p err`:
> Jun 04 10:00:02 rhel10 kernel: usb 1-1: device descriptor read/64, error -110
> ...

## Konfiguracja rsyslog
```bash
vim /etc/rsyslog.conf
# Przykład: logi kernela do osobnego pliku
kern.*                                /var/log/kernel.log
# Przykład: przesyłanie do centralnego serwera
*.*                                   @192.168.1.100:514
```
Po zmianach: `systemctl restart rsyslog`

## Rotacja logów (logrotate)
```bash
vim /etc/logrotate.conf
vim /etc/logrotate.d/nginx            # przykład dla Nginx
# /var/log/nginx/*.log {
#     daily
#     rotate 14
#     compress
#     delaycompress
#     postrotate
#         systemctl reload nginx
#     endscript
# }
```
Test: `logrotate -d /etc/logrotate.d/nginx`

## Dlaczego to jest ważne?
- Przy diagnozowaniu problemów zawsze zaczynasz od logów
- RHEL 10 używa systemd-journald domyślnie + rsyslog dla zgodności
- RHCSA wymaga umiejętności znajdowania zdarzeń w logach i konfiguracji rotacji
- ⚠️ Logi w `/var/log/` mogą szybko zapełnić dysk – konfiguruj rotację od razu

### Weryfikacja:
```bash
journalctl -p err --since "1 hour ago" | head -10
```
Oczekiwany wynik: lista błędów systemowych z ostatniej godziny
    "#.into()) },
            Challenge { id: 42, title: "Systemd targety i poziomy uruchamiania".into(), description: "Zarządzaj systemd targetami. Ustaw domyślny target, izoluj targety, twórz własne jednostki target.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Systemd targets, runlevels, tworzenia własnych targetów.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość systemd i zarządzania usługami
- Umiejętność edycji plików konfiguracyjnych

## Podstawy
```bash
systemctl list-units --type=target    # lista dostępnych targetów
systemctl get-default                 # aktualny domyślny target
systemctl set-default multi-user.target  # ustaw boot do trybu tekstowego
systemctl isolate graphical.target    # przełącz na graficzny teraz
```
> Przykładowy wynik `systemctl list-units --type=target`:
> UNIT                   LOAD   ACTIVE SUB    DESCRIPTION
> basic.target           loaded active active Basic System
> graphical.target       loaded active active Graphical Interface
> multi-user.target      loaded active active Multi-User System
> ...
> Przykładowy wynik `systemctl get-default`:
> graphical.target
> Przykładowy wynik `systemctl isolate graphical.target`:
> (brak widocznego outputu – system przełącza target)

## Ważne targety na RHEL
| Target | Opis | Stary runlevel |
|--------|------|----------------|
| `poweroff.target` | Wyłączenie | 0 |
| `rescue.target` | Konsola ratunkowa | 1 |
| `multi-user.target` | Wieloużytkownikowy, bez GUI | 3 |
| `graphical.target` | Z GUI | 5 |
| `reboot.target` | Restart | 6 |

## Ćwiczenie: własny target
```bash
# 1. Stwórz target niestandardowy
cat > /etc/systemd/system/moj-target.target << 'EOF'
[Unit]
Description=Mój target z monitoringiem
Requires=multi-user.target
After=multi-user.target
AllowIsolate=yes
EOF

# 2. Włącz i ustaw jako domyślny
systemctl daemon-reload
systemctl enable moj-target.target

# 3. Przetestuj
systemctl isolate moj-target.target
```

## Dlaczego to jest ważne?
- RHCSA: musisz umieć zmienić target rozruchowy
- `rescue.target` jest kluczowy do naprawy systemu
- Na RHEL domyślnym targetem jest `graphical.target` (z GUI) lub `multi-user.target` (serwer)

### Weryfikacja:
```bash
systemctl get-default
```
Oczekiwany wynik:
> graphical.target
    "#.into()) },
            Challenge { id: 43, title: "GRUB2 – zarządzanie bootloaderem".into(), description: "Konfiguruj GRUB2: zmieniaj parametry jądra, ustaw hasło, odzyskaj bootloader po awarii.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Konfiguracji GRUB2, haseł, parametrów jądra.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość procesu bootowania Linux
- Umiejętność edycji plików konfiguracyjnych

## Podstawy GRUB2
```bash
cat /etc/default/grub                 # główny plik konfiguracyjny
# GRUB_TIMEOUT=5                      # czas na wybór w menu
# GRUB_CMDLINE_LINUX="rhgb quiet"     # parametry przekazywane do jądra
# GRUB_DISABLE_RECOVERY=false         # pokaż tryb recovery

grub2-mkconfig -o /boot/grub2/grub.cfg   # RHEL – generuj konfig
grub2-mkconfig -o /boot/efi/EFI/redhat/grub.cfg  # dla UEFI
```
> Przykładowy wynik `cat /etc/default/grub`:
> GRUB_TIMEOUT=5
> GRUB_DISTRIBUTOR="$(sed 's, release .*$,,g' /etc/system-release)"
> GRUB_DEFAULT=saved
> GRUB_DISABLE_SUBMENU=true
> GRUB_TERMINAL_OUTPUT="console"
> GRUB_CMDLINE_LINUX="rhgb quiet"
> GRUB_DISABLE_RECOVERY="false"
> Przykładowy wynik `grub2-mkconfig -o /boot/grub2/grub.cfg`:
> Generating grub configuration file ...
> Found linux image: /boot/vmlinuz-6.8.0-1.el10.x86_64
> Found initrd image: /boot/initramfs-6.8.0-1.el10.x86_64
> done

## Dodawanie parametrów jądra
```bash
# Dodaj parametr np. wyłączenie IPv6
vim /etc/default/grub
# GRUB_CMDLINE_LINUX="rhgb quiet ipv6.disable=1"

grub2-mkconfig -o /boot/grub2/grub.cfg
reboot
cat /proc/cmdline                      # sprawdź aktywne parametry
```
> Przykładowy wynik `cat /proc/cmdline`:
> BOOT_IMAGE=(hd0,gpt2)/vmlinuz-6.8.0-1.el10.x86_64 root=/dev/mapper/rhel-root ro rhgb quiet

## Hasło do GRUB2
```bash
grub2-setpassword                      # ustaw hasło – zabezpiecza boot menu
# Plik: /boot/grub2/user.cfg
```

## Odzyskiwanie GRUB2 (kluczowe na RHCSA!)
```bash
# Z płyty rescue:
chroot /mnt/sysimage
grub2-install /dev/sda                # reinstaluj GRUB na MBR
grub2-mkconfig -o /boot/grub2/grub.cfg
```

## Ćwiczenie
```bash
# Dodaj parametr mitigations=off do bootu, sprawdź efekt
vim /etc/default/grub
grub2-mkconfig -o /boot/grub2/grub.cfg
reboot
cat /proc/cmdline | grep mitigations
```
> Przykładowy wynik `cat /proc/cmdline | grep mitigations`:
> BOOT_IMAGE=... root=/dev/mapper/rhel-root ro rhgb quiet mitigations=off

## Uwagi
- ⚠️ Błąd w grub.cfg = system nie bootuje – miej rescue media pod ręką
- Na RHEL 10 domyślnie UEFI – ścieżki różnią się od BIOS/MBR
- RHCSA: musisz umieć zmienić parametry jądra przez GRUB

### Weryfikacja:
```bash
cat /proc/cmdline
```
Oczekiwany wynik: wyświetli aktualne parametry jądra (m.in. `rhgb quiet` i ewentualnie dodane parametry)
    "#.into()) },
            Challenge { id: 44, title: "Reset hasła root i tryb ratunkowy".into(), description: "Zresetuj zapomniane hasło root. Użyj rescue.target, initramfs i chroot do naprawy systemu.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Reset hasła root, pracy w trybie ratunkowym, chroot.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość GRUB2 i procesu bootowania
- Dostęp fizyczny do maszyny (lub konsola zdalna)

## Reset hasła root (RHEL 9+)
```bash
# 1. Zrestartuj system
# 2. W GRUB2 naciśnij 'e' na linii bootu
# 3. Znajdź linię zaczynającą się od 'linux'
# 4. Na końcu linii dodaj: rd.break
# 5. Ctrl+X lub F10 aby bootować
# 6. System zatrzyma się w initramfs:

mount -o remount,rw /sysroot
chroot /sysroot
passwd root
touch /.autorelabel                # relabel SELinux przy następnym boot
exit
reboot
```

## Tryb ratunkowy przez ISO (gdy GRUB nie działa)
```bash
# Bootuj z RHEL ISO → Troubleshooting → Rescue a system
# Wybierz: 1) Continue
chroot /mnt/sysimage
# Teraz masz pełny dostęp do systemu

# Naprawa GRUB:
grub2-install /dev/sda
grub2-mkconfig -o /boot/grub2/grub.cfg

# Naprawa initramfs:
dracut --force

exit
reboot
```

## Uwagi
- ⚠️ Na RHEL 9+ SELinux musi być relabelowany (`touch /.autorelabel`)
- ⚠️ Ta sama procedura działa gdy zapomnisz hasła root
- RHCSA: reset hasła root to pewniak na egzaminie!

### Weryfikacja:
```bash
sudo -i
```
Oczekiwany wynik: po poprawnym resecie hasła logowanie root bez hasła lub z nowym hasłem powinno działać
    "#.into()) },
            Challenge { id: 45, title: "Skrypty bash – warunki i pętle".into(), description: "Pisz skrypty z if/else, for, while, case. Używaj test, [ ], [[ ]], i operatorów logicznych.".into(), category: Category::Shell, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Pisania skryptów z logiką warunkową i pętlami.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość składni Bash (zmienne, komendy)
- Umiejętność tworzenia i uruchamiania skryptów

## If/else
```bash
#!/bin/bash
if [ -f /etc/passwd ]; then
    echo "Plik istnieje"
elif [ -d /etc ]; then
    echo "/etc jest katalogiem"
else
    echo "Nie wiem"
fi
```
> Przykładowy wynik powyższego skryptu:
> Plik istnieje

## Pętla for
```bash
for user in ala ola ela; do
    useradd $user
    echo "Utworzono: $user"
done

# Z sekwencją
for i in {1..10}; do
    touch plik_$i.txt
done
```
> Przykładowy wynik pętli for:
> Utworzono: ala
> Utworzono: ola
> Utworzono: ela

## Pętla while
```bash
#!/bin/bash
counter=10
while [ $counter -gt 0 ]; do
    echo "Odliczam: $counter"
    counter=$((counter - 1))
done
```
> Przykładowy wynik pętli while:
> Odliczam: 10
> Odliczam: 9
> ...
> Odliczam: 1

## Case (menu)
```bash
#!/bin/bash
echo "Wybierz: 1) Backup 2) Restore 3) Exit"
read -r choice
case $choice in
    1) echo "Robię backup..."; tar czf backup.tar.gz /home ;;
    2) echo "Przywracam..."; tar xzf backup.tar.gz ;;
    3) exit 0 ;;
    *) echo "Nieprawidłowy wybór" ;;
esac
```
> Przykładowy wynik (po wybraniu 1):
> Wybierz: 1) Backup 2) Restore 3) Exit
> Robię backup...

## Operator testu
| Wyrażenie | Opis |
|-----------|------|
| `[ -f plik ]` | Czy plik istnieje? |
| `[ -d katalog ]` | Czy to katalog? |
| `[ "$a" = "$b" ]` | Porównanie stringów |
| `[ "$a" -eq "$b" ]` | Porównanie liczb |
| `[[ $a =~ ^foo ]]` | Regex (podwójny nawias!) |

## Uwagi
- ⚠️ Zawsze cytuj zmienne: `[ "$var" = "tekst" ]` zamiast `[ $var = tekst ]`
- Używaj `[[ ]]` zamiast `[ ]` w bash (więcej opcji, bezpieczniejszy)
- RHCSA: musisz umieć czytać i modyfikować podstawowe skrypty

### Weryfikacja:
```bash
bash -c 'if [ -f /etc/passwd ]; then echo "OK: passwd istnieje"; fi'
```
Oczekiwany wynik:
> OK: passwd istnieje
    "#.into()) },
            Challenge { id: 46, title: "Serwer NFS i montowanie zasobów sieciowych".into(), description: "Skonfiguruj serwer NFS, eksportuj katalogi, montuj po klienckiej stronie. Użyj automount i fstab.".into(), category: Category::Network, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Eksportowania i montowania NFS, automount.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Znajomość montowania partycji i fstab
- Podstawowa znajomość uprawnień i firewall

## Serwer NFS
```bash
dnf install -y nfs-utils
systemctl enable --now nfs-server

# Eksport katalogu
mkdir -p /srv/nfs/share
chmod 755 /srv/nfs/share
vim /etc/exports
# /srv/nfs/share  192.168.1.0/24(rw,sync,no_root_squash)

exportfs -rav                    # przeładuj eksporty
exportfs -v                      # sprawdź aktywne eksporty
```
> Przykładowy wynik `exportfs -rav`:
> exporting 192.168.1.0/24:/srv/nfs/share
> Przykładowy wynik `exportfs -v`:
> /srv/nfs/share  192.168.1.0/24(rw,sync,wdelay,hide,no_subtree_check,no_root_squash)

## Klient NFS
```bash
dnf install -y nfs-utils
showmount -e 192.168.1.10        # zobacz dostępne eksporty
mount -t nfs 192.168.1.10:/srv/nfs/share /mnt

# Montowanie przez fstab
echo "192.168.1.10:/srv/nfs/share /mnt nfs defaults,_netdev 0 0" >> /etc/fstab
```
> Przykładowy wynik `showmount -e 192.168.1.10`:
> Export list for 192.168.1.10:
> /srv/nfs/share 192.168.1.0/24

## Automount
```bash
dnf install -y autofs
vim /etc/auto.master
# /misc   /etc/auto.misc

vim /etc/auto.misc
# nfsshare   -fstype=nfs,rw   192.168.1.10:/srv/nfs/share

systemctl enable --now autofs
cd /misc/nfsshare                # dostęp do zasobu – montuje się automatycznie
```

## Uwagi
- ⚠️ Sprawdź firewall: `firewall-cmd --add-service=nfs --permanent`
- ⚠️ Na RHEL SELinux blokuje NFS – sprawdź: `getsebool nfs_export_all_rw`
- RHCSA: musisz umieć zamontować NFS przez fstab

### Weryfikacja:
```bash
showmount -e localhost
```
Oczekiwany wynik:
> Export list for localhost:
> /srv/nfs/share 192.168.1.0/24
    "#.into()) },
            Challenge { id: 47, title: "Usługi czasu – chronyd i NTP".into(), description: "Konfiguruj synchronizację czasu przez chronyd. Użyj timedatectl, date, hwclock. Skonfiguruj klienta NTP i strefę czasową.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Synchronizacji czasu, chronyd, timedatectl.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość stref czasowych i NTP
- Umiejętność edycji plików konfiguracyjnych

## Podstawowe polecenia
```bash
timedatectl status                 # status czasu systemowego
timedatectl list-timezones         # lista stref czasowych
timedatectl set-timezone Europe/Warsaw  # ustaw strefę
timedatectl set-ntp true           # włącz NTP

date                               # bieżąca data/godzina
date -s "2026-06-04 15:30:00"      # ręczne ustawienie
hwclock --systohc                  # zapisz czas systemowy do BIOS
```
> Przykładowy wynik `timedatectl status`:
>                Local time: Thu 2026-06-04 10:30:00 CEST
>            Universal time: Thu 2026-06-04 08:30:00 UTC
>                  RTC time: Thu 2026-06-04 08:30:00
>                 Time zone: Europe/Warsaw (CEST, +0200)
> System clock synchronized: yes
>               NTP service: active
> Przykładowy wynik `date`:
> Thu Jun  4 10:30:00 CEST 2026

## Konfiguracja chronyd
```bash
vim /etc/chrony.conf
# serwer NTP:
pool 2.rhel.pool.ntp.org iburst
# lub własny serwer:
server ntp.local iburst

# Zezwolenie dla innych maszyn z sieci:
allow 192.168.1.0/24

systemctl restart chronyd
chronyc sources -v                 # sprawdź źródła czasu
chronyc tracking                   # szczegóły synchronizacji
```
> Przykładowy wynik `chronyc sources -v`:
>   .-- Source mode  '^' = server, '=' = peer, '#' = local clock.
>  /-- Source state '*' = current synced, '+' = combined, '-' = not combined
> |   /-- IPv6, Regular expression for source address
> MS Name/IP address         Stratum Poll Reach LastRx Last sample
> ===============================================================================
> ^* 2.rhel.pool.ntp.org          2   6   377    42   +2ms[  +3ms] +/-   23ms
> Przykładowy wynik `chronyc tracking`:
> Reference ID    : A1B2C3D4 (2.rhel.pool.ntp.org)
> Stratum         : 3
> Ref time (UTC)  : Thu Jun 04 08:30:00 2026
> System time     : 0.000123456 seconds slow of NTP time
> Last offset     : +0.000654321 seconds
> RMS offset      : 0.002345678 seconds

## Ćwiczenie
```bash
# 1. Ustaw strefę na czas polski
# 2. Włącz NTP
# 3. Sprawdź źródła czasu
# 4. Porównaj z: timedatectl show
```

## Uwagi
- Chrony jest domyślnym NTP klientem na RHEL 9+ (zastąpił ntpd)
- RHCSA: musisz umieć skonfigurować klienta NTP i strefę czasową
- ⚠️ Rozbieżność czasu >5min = problemy z Kerberos i logowaniem

### Weryfikacja:
```bash
chronyc tracking | grep "System time"
```
Oczekiwany wynik:
> System time     : 0.000123456 seconds slow of NTP time
    "#.into()) },
            Challenge { id: 48, title: "Serwer Apache HTTP".into(), description: "Zainstaluj i skonfiguruj Apache (httpd). Stwórz virtual hosty, skonfiguruj SSL i hosts wirtualne.".into(), category: Category::Network, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Instalacji i konfiguracji Apache, virtual hostów.

### Wymagania:
- Ukończone wyzwanie "Konfiguracja sieci"
- Znajomość firewall i SELinux
- Umiejętność edycji plików konfiguracyjnych

## Instalacja i podstawy
```bash
dnf install -y httpd
systemctl enable --now httpd

# Główne pliki:
# /etc/httpd/conf/httpd.conf        – główna konfiguracja
# /etc/httpd/conf.d/                – dodatkowe pliki .conf
# /var/www/html/                    – domyślny DocumentRoot

firewall-cmd --add-service=http --permanent
firewall-cmd --reload

echo "<h1>Witaj na RHEL!</h1>" > /var/www/html/index.html
```

## Virtual hosty
```bash
mkdir -p /var/www/{site1,site2}
echo "<h1>Strona A</h1>" > /var/www/site1/index.html
echo "<h1>Strona B</h1>" > /var/www/site2/index.html

cat > /etc/httpd/conf.d/vhosts.conf << 'EOF'
<VirtualHost *:80>
    ServerName site1.local
    DocumentRoot /var/www/site1
</VirtualHost>
<VirtualHost *:80>
    ServerName site2.local
    DocumentRoot /var/www/site2
</VirtualHost>
EOF

systemctl reload httpd
echo "127.0.0.1 site1.local site2.local" >> /etc/hosts
curl site1.local
```
> Przykładowy wynik `curl site1.local`:
> <h1>Strona A</h1>

## Uwagi
- ⚠️ SELinux: `restorecon -Rv /var/www/` po utworzeniu contentu
- ⚠️ Sprawdź: `getsebool httpd_enable_homedirs`
- Na RHEL Apache domyślnie nasłuchuje na porcie 80

### Weryfikacja:
```bash
curl -I http://localhost 2>/dev/null | head -1
```
Oczekiwany wynik:
> HTTP/1.1 200 OK
    "#.into()) },
            Challenge { id: 49, title: "Moduły jądra Linux".into(), description: "Zarządzaj modułami jądra: lsmod, modprobe, modinfo, depmod. Dodaj/usun moduły, konfiguruj parametry modułów.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Zarządzania modułami jądra, parametrami, ładowaniem.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Podstawowa znajomość architektury jądra Linux
- Umiejętność pracy z initramfs (dracut)

## Podstawy
```bash
lsmod                              # lista załadowanych modułów
lsmod | grep usb                   # moduły USB
modinfo usb_storage                # szczegóły modułu (autor, opis, parametry)
modprobe -c | less                 # wszystkie dostępne moduły
```
> Przykładowy wynik `lsmod`:
> Module                  Size  Used by
> uas                    32768  0
> usb_storage            73728  0
> usbcore               356352  4 uas,usb_storage,ehci_hcd,ohci_hcd
> Przykładowy wynik `modinfo usb_storage`:
> filename:       /lib/modules/6.8.0-1.el10.x86_64/kernel/drivers/usb/storage/usb-storage.ko.xz
> license:        GPL
> description:    USB Mass Storage driver for Linux
> author:         Matthew Dharm

## Ładowanie/usuwanie modułów
```bash
modprobe usb_storage               # załaduj moduł
modprobe -r usb_storage            # usuń moduł
lsmod | grep usb_storage           # sprawdź czy załadowany
```
> Przykładowy wynik `lsmod | grep usb_storage` (po załadowaniu):
> usb_storage            73728  0

## Parametry modułów
```bash
# Sprawdź parametry:
modinfo -p usb_storage

# Ustaw parametr przy ładowaniu:
modprobe usb_storage delay_use=5

# Trwała konfiguracja:
echo "options usb_storage delay_use=5" > /etc/modprobe.d/usb-storage.conf
```

## Czarna lista modułów
```bash
# Wyłącz moduł (np. niechciane audio):
echo "blacklist snd_hda_intel" > /etc/modprobe.d/blacklist-audio.conf
dracut --force                     # przebuduj initramfs
reboot
```

## Uwagi
- ⚠️ `modprobe -r` nie zadziała jeśli moduł jest używany
- RHCSA: musisz umieć sprawdzić załadowane moduły i dodać parametry
- `dracut --force` po zmianach w modprobe.d!

### Weryfikacja:
```bash
lsmod | grep usb_storage
```
Oczekiwany wynik: po załadowaniu modułu:
> usb_storage            73728  0
    "#.into()) },
            Challenge { id: 50, title: "Timery systemd – nowa cron".into(), description: "Twórz własne timery systemd jako alternatywę dla cron. Użyj monotonicznych i kalendarzowych timerów.".into(), category: Category::System, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Timerów systemd, ich przewagi nad cronem, debugowania.

### Wymagania:
- Ukończone wyzwanie "Systemd targety i poziomy uruchamiania"
- Znajomość jednostek systemd (service, timer)
- Umiejętność edycji plików konfiguracyjnych

## Podstawy
```bash
systemctl list-timers --all        # wszystkie timery
```
> Przykładowy wynik `systemctl list-timers --all`:
> NEXT                         LEFT     LAST                         PASSED    UNIT                         ACTIVATES
> Thu 2026-06-04 03:00:00 CEST 16h left Wed 2026-06-03 03:00:00 CEST 15h ago   systemd-tmpfiles-clean.timer systemd-tmpfiles-clean.service
> Mon 2026-06-08 00:00:00 CEST 3 days   Sun 2026-06-07 00:00:00 CEST 11h ago   unbound-anchor.timer        unbound-anchor.service

## Tworzenie własnego timera
```bash
# 1. Stwórz jednostkę serwisową:
cat > /etc/systemd/system/codzienne-czyszczenie.service << 'EOF'
[Unit]
Description=Codzienne czyszczenie temp

[Service]
Type=oneshot
ExecStart=/usr/bin/find /tmp -type f -atime +7 -delete
EOF

# 2. Stwórz timer:
cat > /etc/systemd/system/codzienne-czyszczenie.timer << 'EOF'
[Unit]
Description=Czyść /tmp codziennie o 3 nad ranem

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
EOF

systemctl daemon-reload
systemctl enable --now codzienne-czyszczenie.timer
systemctl list-timers --all | grep codzienne
```
> Przykładowy wynik `systemctl list-timers --all | grep codzienne`:
> Thu 2026-06-05 03:00:00 CEST 16h left n/a                          n/a    codzienne-czyszczenie.timer  codzienne-czyszczenie.service

## Typy timerów
| Typ | Przykład | Opis |
|-----|----------|------|
| OnCalendar | `daily`, `Mon..Fri 09:00`, `*-*-* 00:00:00` | Określony kalendarzowo |
| OnBootSec | `10min` | Po bootowaniu |
| OnUnitActiveSec | `1h` | Od ostatniego uruchomienia |
| OnUnitInactiveSec | `30min` | Od ostatniego zatrzymania |

## Różnice między cron a systemd timery
- systemd timery mają logowanie przez journalctl
- Nie przegapią zadania jeśli system był wyłączony (`Persistent=true`)
- Łatwiej debugować: `journalctl -u nazwa.timer`
- Można ustawić zależności między timerami

## Uwagi
- RHCSA: znajomość timerów systemd jest wymagana
- Używaj `OnCalendar=*-*-* 00..23:00:00` dla godzinnych zadań

### Weryfikacja:
```bash
systemctl list-timers --all | grep codzienne
```
Oczekiwany wynik: timer powinien być widoczny na liście z najbliższym odpaleniem
    "#.into()) },
            Challenge { id: 51, title: "Kontenery jako usługi systemd".into(), description: "Uruchom kontener Podman jako usługę systemd z auto-restartem i logowaniem.".into(), category: Category::DevOps, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Uruchamiania kontenerów jako usług systemd, auto-restart.

### Wymagania:
- Ukończone wyzwanie "Timery systemd"
- Znajomość Podman i kontenerów
- Umiejętność edycji jednostek systemd

## Generowanie pliku service z kontenera
```bash
# Uruchom kontener:
podman run -d --name nginx --restart=no -p 8080:80 nginx

# Wygeneruj plik systemd:
podman generate systemd --name nginx > /etc/systemd/system/container-nginx.service

# Dostosuj:
systemctl daemon-reload
systemctl enable --now container-nginx.service

# Sprawdź:
systemctl status container-nginx.service
journalctl -u container-nginx.service
```
> Przykładowy wynik `systemctl status container-nginx.service`:
> ● container-nginx.service - Podman container-nginx.service
>      Loaded: loaded (/etc/systemd/system/container-nginx.service; enabled; preset: disabled)
>      Active: active (running) since Thu 2026-06-04 10:00:00 CEST
>    Main PID: 12345 (conmon)
>       Tasks: 3 (limit: 12345)
>      Memory: 12.3M
>         CPU: 50ms
>      CGroup: /system.slice/container-nginx.service

## Ręczne tworzenie
```bash
cat > /etc/systemd/system/podman-web.service << 'EOF'
[Unit]
Description=Kontener WWW
After=network.target

[Service]
Type=forking
ExecStart=/usr/bin/podman run -d --name web -p 80:80 nginx
ExecStop=/usr/bin/podman stop web
ExecStopPost=/usr/bin/podman rm web
Restart=always

[Install]
WantedBy=multi-user.target
EOF
```

## Auto-update kontenerów
```bash
# Podman auto-update (wymaga systemd):
systemctl enable --now podman-auto-update.timer
podman auto-update --dry-run        # sprawdź aktualizacje bez aplikowania
```
> Przykładowy wynik `podman auto-update --dry-run`:
> UNIT                      CONTAINER          IMIT
> container-nginx.service   nginx              registry.access.redhat.com/ubi9/nginx-120:latest

## Uwagi
- ⚠️ Używaj `podman generate systemd` – to bezpieczniejsze niż ręczne definiowanie
- `Restart=always` zapewnia auto-restart po crashu
- RHCSA: na egzaminie możesz dostać zadanie z podman + systemd

### Weryfikacja:
```bash
systemctl is-active container-nginx.service
```
Oczekiwany wynik:
> active
    "#.into()) },
            Challenge { id: 52, title: "LVM – snapshoty i cienkie provisioning".into(), description: "Twórz snapshoty LVM do backupu, używaj thin provisioning do oszczędzania miejsca.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Snapshotów LVM, thin provisioning, rozszerzania.

### Wymagania:
- Ukończone wyzwanie "Podstawy terminala"
- Znajomość LVM (volume groups, logical volumes)
- Podstawowa znajomość systemów plików (XFS, ext4)

## Snapshot LVM
```bash
# Utwórz snapshot przed aktualizacją:
lvcreate -L 1G -s -n root_snap /dev/vg_root/lv_root

# Przywróć snapshot:
lvconvert --merge /dev/vg_root/root_snap
# system po restarcie wróci do stanu sprzed snapshota
```

## Thin provisioning
```bash
# 1. Stwórz thin pool:
lvcreate -L 10G -T vg_data/thin_pool

# 2. Stwórz thin volume (200G ale używa tylko tyle ile potrzeba):
lvcreate -V 200G -T vg_data/thin_pool -n thin_vol

# 3. Sformatuj i montuj:
mkfs.xfs /dev/vg_data/thin_vol
mount /dev/vg_data/thin_vol /mnt/thin
```

## Monitorowanie thin pool
```bash
lvs -a                              # zobacz użycie Data% i Meta%
# Jeśli Data% > 90% – rozszerz pool:
lvextend -L +5G vg_data/thin_pool
```
> Przykładowy wynik `lvs -a`:
>   LV              VG      Attr       LSize   Pool      Data%  Meta%
>   lv_root         vg_root -wi-ao----  10.00g
>   thin_pool       vg_data twi-aot---  10.00g                    20.32
>   thin_vol        vg_data Vwi-aot--- 200.00g thin_pool          15.10

## Uwagi
- ⚠️ Snapshoty NIE są backupem – jeśli oryginał padnie, snapshot też
- Thin provisioning: nie przydzielaj więcej niż fizycznie masz (overcommit)
- RHCSA: snapshoty LVM są na egzaminie

### Weryfikacja:
```bash
lvs -a | grep snap
```
Oczekiwany wynik: lista snapshotów LVM (jeśli utworzono)
    "#.into()) },
            Challenge { id: 53, title: "Zaawansowany SELinux – booleany i porty".into(), description: "Zarządzaj SELinux booleanami, konfiguruj porty dla usług, debuguj deniali za pomocą audit2allow.".into(), category: Category::Security, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** SELinux booleanów, kontekstów portów, audit2allow.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Podstawowa znajomość SELinux (konteksty, tryby)
- Umiejętność analizy logów (ausearch, journalctl)

## Booleany SELinux
```bash
getsebool -a                        # wszystkie booleany
getsebool httpd_enable_homedirs     # sprawdź konkretny
setsebool httpd_enable_homedirs on  # włącz (tymczasowo)
setsebool -P httpd_enable_homedirs on  # trwale (-P)
semanage boolean -l | grep http     # lista z opisami
```
> Przykładowy wynik `getsebool -a`:
> httpd_anon_write --> off
> httpd_enable_homedirs --> off
> httpd_use_nfs --> off
> ...
> Przykładowy wynik `getsebool httpd_enable_homedirs`:
> httpd_enable_homedirs --> on
> Przykładowy wynik `semanage boolean -l | grep http`:
> httpd_enable_homedirs      (on  , off)  Allow httpd to enable homedirs

## Konteksty portów
```bash
semanage port -l | grep http        # sprawdź porty dla http
semanage port -a -t http_port_t -p tcp 8080  # dodaj port 8080
semanage port -d -t http_port_t -p tcp 8080  # usuń port
```
> Przykładowy wynik `semanage port -l | grep http`:
> http_port_t                    tcp      80, 81, 443, 488, 8008, 8009, 8443, 8080

## audit2allow – debugowanie deniali
```bash
# Gdy usługa nie działa z powodu SELinux:
ausearch -m avc -ts recent          # znajdź deniale
ausearch -m avc | audit2allow -M mymodule   # stwórz moduł
semodule -i mymodule.pp             # załaduj moduł
```
> Przykładowy wynik `ausearch -m avc -ts recent`:
> ----
> time->Thu Jun 04 10:00:05 2026
> type=AVC msg=audit(123456.789:123): avc:  denied  { name_connect } for  pid=1234 comm="httpd" dest=8080 scontext=system_u:system_r:httpd_t:s0 tcontext=system_u:object_r:http_port_t:s0 tclass=tcp_socket
> Przykładowy wynik `ausearch -m avc | audit2allow -M mymodule`:
> Generating mymodule.pp

## Ćwiczenie
```bash
# Zmień port SSH na 2222 – wymaga SELinux:
semanage port -a -t ssh_port_t -p tcp 2222
vim /etc/ssh/sshd_config            # Port 2222
systemctl restart sshd
```

## Uwagi
- ⚠️ `setsebool -P` (persistent) zapisuje w `/etc/selinux/targeted/policy/`
- ⚠️ Używaj audit2allow tylko jeśli rozumiesz denial – łatwo stworzyć lukę
- RHCSA: konfiguracja SELinux dla niestandardowych portów

### Weryfikacja:
```bash
getsebool httpd_enable_homedirs
```
Oczekiwany wynik:
> httpd_enable_homedirs --> on
    "#.into()) },
            Challenge { id: 54, title: "Troubleshooting – diagnostyka systemu".into(), description: "Diagnozuj problemy z bootem, initramfs, fsck. Użyj systemd-journald, dmesg, rescue target.".into(), category: Category::System, difficulty: 3, completed: false, details: Some(r#"
**Czego się nauczysz:** Diagnostyki problemów systemowych, naprawy bootu, fsck.

### Wymagania:
- Ukończone wyzwanie "Reset hasła root i tryb ratunkowy"
- Znajomość procesu bootowania Linux
- Umiejętność pracy w trybie ratunkowym (chroot)

## Diagnostyka bootu
```bash
dmesg | grep -i error               # błędy jądra przy bootowaniu
dmesg | grep -i fail                # nieudane inicjalizacje
journalctl -b                       # logi z bieżącego bootu
journalctl -b -1                    # logi z poprzedniego bootu
journalctl --list-boots             # lista dostępnych bootów
```
> Przykładowy wynik `dmesg | grep -i error`:
> [    0.123456] ACPI BIOS Error (bug): Could not find symbol
> [    1.234567] pci 0000:00:01.0: PCI bridge error
> Przykładowy wynik `journalctl --list-boots`:
>  -3 1234567890abcdef... Thu 2026-06-02 09:55:00 CEST—Thu 2026-06-02 15:30:00 CEST
>  -2 abcdef1234567890... Wed 2026-06-03 09:55:00 CEST—Wed 2026-06-03 17:00:00 CEST
>  -1 9876543210fedcba... Thu 2026-06-04 09:55:00 CEST—Thu 2026-06-04 10:00:00 CEST
>   0 def0123456789abc... Thu 2026-06-04 10:00:00 CEST—still running

## Naprawa initramfs
```bash
# Jeśli initramfs jest uszkodzony:
dracut --force                      # przebuduj aktualny initramfs
dracut --force --kver $(uname -r)   # dla konkretnej wersji jądra
```

## fsck – naprawa systemu plików
```bash
# Odmontuj partycję:
umount /dev/sda1
fsck -y /dev/sda1                   # napraw bez pytania

# Dla XFS:
xfs_repair /dev/sda1                # tylko na odmontowanym!

# Dla Ext4:
e2fsck -f -y /dev/sda1
```

## Problemy z bootem – checklista
1. Sprawdź parametry jądra przez GRUB (naciśnij 'e')
2. Bootuj do `rescue.target`
3. Sprawdź logi: `journalctl -xb`
4. Sprawdź initramfs: `lsinitrd`
5. Sprawdź GRUB: `grub2-install /dev/sda`
6. Sprawdź fsck na partycjach
7. Sprawdź SELinux: `ausearch -m avc`

## Uwagi
- Zawsze zaczynaj od `journalctl -xb` – to pokazuje błędy od początku bootu
- Na RHEL `fsck` jest uruchamiany automatycznie po X mountach
- RHCSA: troubleshooting to kluczowa umiejętność egzaminacyjna

### Weryfikacja:
```bash
journalctl --list-boots | head -3
```
Oczekiwany wynik: lista ostatnich bootów z identyfikatorami i datami
    "#.into()) },
            Challenge { id: 55, title: "Hardening SSH – klucze, port, uwierzytelnianie".into(), description: "Skonfiguruj SSH z kluczami, zmień port, ogranicz użytkowników, dodaj baner ostrzegawczy.".into(), category: Category::Security, difficulty: 2, completed: false, details: Some(r#"
**Czego się nauczysz:** Zaawansowanej konfiguracji SSH, key-based auth, hardeningu.

### Wymagania:
- Ukończone wyzwanie "Firewall i bezpieczeństwo"
- Znajomość podstaw SSH (połączenie, konfiguracja)
- Umiejętność generowania i zarządzania kluczami

## Generowanie kluczy
```bash
ssh-keygen -t ed25519 -C "komentarz"   # ED25519 – najbezpieczniejszy
ssh-keygen -t rsa -b 4096              # RSA 4096 jako alternatywa
```

## Konfiguracja kluczy
```bash
ssh-copy-id -i ~/.ssh/id_ed25519.pub user@host
# lub ręcznie:
cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
chmod 700 ~/.ssh
```

## Hardening sshd_config
```bash
vim /etc/ssh/sshd_config
# Zalecane ustawienia:
Port 2222                              # zmiana portu
PermitRootLogin no                     # bez logowania root
PasswordAuthentication no              # tylko klucze
PubkeyAuthentication yes
AllowUsers ala ola                     # tylko ci użytkownicy
MaxAuthTries 3                         # max 3 próby
ClientAliveInterval 300                # keepalive co 5 min
ClientAliveCountMax 0
Banner /etc/issue.net                  # baner ostrzegawczy

systemctl restart sshd
```

## SELinux dla SSH
```bash
# Jeśli zmieniasz port:
semanage port -a -t ssh_port_t -p tcp 2222
# Sprawdź:
semanage port -l | grep ssh
```
> Przykładowy wynik `semanage port -l | grep ssh`:
> ssh_port_t                     tcp      2222, 22

## Ćwiczenie
```bash
# 1. Wygeneruj parę kluczy ED25519
# 2. Skopiuj klucz do zdalnego serwera
# 3. Wyłącz hasło i logowanie root
# 4. Zmień port na 2222
# 5. Przetestuj połączenie i odbierz sobie dostęp ;)
```

## Uwagi
- ⚠️ NIE zamykaj sesji SSH przed przetestowaniem nowej konfiguracji!
- ⚠️ Zawsze miej drugą sesję SSH otwartą na wypadek błędu
- RHCSA: konfiguracja key-based auth jest wymagana

### Weryfikacja:
```bash
ss -tlnp | grep 2222
```
Oczekiwany wynik: powinieneś zobaczyć sshd nasłuchujący na porcie 2222
    "#.into()) },
        ]
    }

    fn default_milestones() -> Vec<Milestone> {
        vec![
            Milestone { id: 1, title: "Linux Explorer".into(), description: "Opanuj pracę w terminalu i podstawy Linuksa".into(), challenge_ids: vec![1, 2], unlocked: true, completed: false },
            Milestone { id: 2, title: "System Guardian".into(), description: "Zarządzaj systemem, procesami i zasobami".into(), challenge_ids: vec![3, 4, 8], unlocked: false, completed: false },
            Milestone { id: 3, title: "Shell Master".into(), description: "Automatyzuj zadania za pomocą skryptów".into(), challenge_ids: vec![7, 11, 14], unlocked: false, completed: false },
            Milestone { id: 4, title: "Network Engineer".into(), description: "Konfiguruj sieci i zdalny dostęp".into(), challenge_ids: vec![5, 9, 12], unlocked: false, completed: false },
            Milestone { id: 5, title: "Security Specialist".into(), description: "Zabezpiecz system i sieć".into(), challenge_ids: vec![6, 13, 18], unlocked: false, completed: false },
            Milestone { id: 6, title: "DevOps Apprentice".into(), description: "Kontenery, serwery WWW i automatyzacja".into(), challenge_ids: vec![10, 15, 16], unlocked: false, completed: false },
            Milestone { id: 7, title: "Cloud Ready".into(), description: "Zaawansowana automatyzacja i CI/CD".into(), challenge_ids: vec![17, 19, 20], unlocked: false, completed: false },
            Milestone { id: 8, title: "Linux System Administrator".into(), description: "Ukończ wszystkie wyzwania – jesteś gotowy do pracy!".into(), challenge_ids: (1..=55).collect(), unlocked: false, completed: false },
            Milestone { id: 9, title: "Advanced SysAdmin".into(), description: "Zaawansowane zarządzanie systemem – sieci, jądro, bezpieczeństwo".into(), challenge_ids: (21..=30).collect(), unlocked: false, completed: false },
            Milestone { id: 10, title: "Expert Level".into(), description: "Poziom ekspercki – audyt, hardening, k8s, CI/CD".into(), challenge_ids: (31..=40).collect(), unlocked: false, completed: false },
            Milestone { id: 11, title: "RHCSA Ready".into(), description: "Tematy egzaminacyjne RHCSA – logowanie, boot, NFS, timery, SELinux, LVM".into(), challenge_ids: (41..=55).collect(), unlocked: false, completed: false },
        ]
    }

    fn default_projects() -> Vec<Project> {
        vec![
            Project { id: 1, title: "Serwer plików z Samba/NFS".into(), description: "Skonfiguruj serwer plików udostępniający katalogi w sieci lokalnej".into(), guide: "1. Zainstaluj samba lub nfs-utils\n2. Skonfiguruj udziały w /etc/samba/smb.conf lub /etc/exports\n3. Ustaw uprawnienia i użytkowników\n4. Przetestuj montowanie z klienta\n5. Zabezpiecz firewall".into(), github_repo: None, completed: false },
            Project { id: 2, title: "Monitoring serwera z Grafana".into(), description: "Skonfiguruj monitoring serwera z Prometheus + Grafana".into(), guide: "1. Zainstaluj Prometheus i node_exporter\n2. Skonfiguruj zbieranie metryk\n3. Zainstaluj i skonfiguruj Grafanę\n4. Stwórz dashboard z użyciem CPU, RAM, dysku, sieci\n5. Dodaj alerty".into(), github_repo: None, completed: false },
            Project { id: 3, title: "Własny serwer DNS".into(), description: "Postaw i skonfiguruj lokalny serwer DNS z blokowaniem reklam".into(), guide: "1. Wybierz: BIND, dnsmasq lub Pi-hole\n2. Skonfiguruj strefy i przekierowania\n3. Włącz blokowanie domen\n4. Przetestuj rozwiązywanie nazw\n5. Dodaj klientów do serwera DNS".into(), github_repo: None, completed: false },
            Project { id: 4, title: "Automatyzacja backupu".into(), description: "Stwórz system automatycznego backupu z rotacją i powiadomieniami".into(), guide: "1. Napisz skrypt backupu (rsync/tar)\n2. Dodaj rotację (np. daily/weekly/monthly)\n3. Wyślij powiadomienie email/ntfy\n4. Dodaj cron job\n5. Przetestuj odtwarzanie z backupu".into(), github_repo: None, completed: false },
            Project { id: 5, title: "Dockerized Web Stack".into(), description: "Postaw stos WWW (Nginx + PHP + PostgreSQL) w Dockerze z docker-compose".into(), guide: "1. Stwórz docker-compose.yml\n2. Skonfiguruj Nginx jako reverse proxy\n3. Dodaj PHP-FPM\n4. Skonfiguruj PostgreSQL\n5. Dodaj wolumeny i sieci\n6. Uruchom i przetestuj".into(), github_repo: None, completed: false },
            Project { id: 6, title: "Konfiguracja serwera pocztowego".into(), description: "Skonfiguruj serwer pocztowy (Postfix + Dovecot) z szyfrowaniem".into(), guide: "1. Zainstaluj Postfix i Dovecot\n2. Skonfiguruj domenę i certyfikaty SSL\n3. Utwórz konta pocztowe\n4. Skonfiguruj SPF, DKIM, DMARC\n5. Przetestuj wysyłanie i odbieranie".into(), github_repo: None, completed: false },
            Project { id: 7, title: "Serwer VPN z WireGuard".into(), description: "Postaw własny serwer VPN WireGuard z konfiguracją klientów i routingiem".into(), guide: "1. Zainstaluj WireGuard\n2. Wygeneruj klucze serwera i klientów\n3. Skonfiguruj interfejs wg0\n4. Dodaj reguły iptables/firewalld do NAT\n5. Skonfiguruj klientów (peer)\n6. Przetestuj połączenie i routing".into(), github_repo: None, completed: false },
            Project { id: 8, title: "System monitoringu Icinga/Nagios".into(), description: "Skonfiguruj monitoring całej infrastruktury z alertami i dashboardem".into(), guide: "1. Zainstaluj Icinga2 lub Nagios Core\n2. Skonfiguruj hosty i serwisy do monitorowania\n3. Dodaj sprawdzenia: ping, HTTP, SSH, disk, CPU\n4. Skonfiguruj powiadomienia email/ntfy\n5. Dodaj graficzne raporty (PNP4Nagios/Icingaweb2)\n6. Przetestuj alerty".into(), github_repo: None, completed: false },
            Project { id: 9, title: "Automatyzacja deploymentu z Ansible".into(), description: "Napisz playbook Ansible do pełnego deploymentu aplikacji webowej".into(), guide: "1. Stwórz inventory (hosts.ini)\n2. Napisz playbook: instalacja Nginx + certbot\n3. Deployment aplikacji przez git clone\n4. Dodaj zmienne i szablony (Jinja2)\n5. Zabezpiecz hasła Ansible Vaultem\n6. Uruchom na zdalnym serwerze".into(), github_repo: None, completed: false },
            Project { id: 10, title: "Klaster Kubernetes z k3s".into(), description: "Postaw klaster Kubernetes na kilku VPS-ach lub maszynach wirtualnych".into(), guide: "1. Zainstaluj k3s na serwerze głównym\n2. Dodaj agentów (worker nodes)\n3. Skonfiguruj kubectl i połącz się\n4. Wdróż przykładową aplikację z deployment + service\n5. Dodaj Ingress (Traefik)\n6. Skonfiguruj PersistentVolume\n7. Przetestuj skalowanie".into(), github_repo: None, completed: false },
        ]
    }

    fn default_quizzes() -> Vec<Quiz> {
        vec![
            Quiz { id: 1, question: "Które polecenie wyświetla aktualną ścieżkę katalogu roboczego?".into(), options: vec!["ls".into(), "pwd".into(), "cd".into(), "echo $PWD".into()], correct_index: 1, category: Category::Linux, explanation: "pwd (print working directory) wyświetla aktualną ścieżkę. echo $PWD też działa, ale pwd jest standardem.".into() },
            Quiz { id: 2, question: "Jaki port domyślnie używa SSH?".into(), options: vec!["22".into(), "80".into(), "443".into(), "21".into()], correct_index: 0, category: Category::Network, explanation: "SSH domyślnie nasłuchuje na porcie 22 (TCP).".into() },
            Quiz { id: 3, question: "Które polecenie zmienia uprawnienia pliku na wykonywalny dla właściciela?".into(), options: vec!["chmod +x plik".into(), "chmod 644 plik".into(), "chown +x plik".into(), "chmod u+x plik".into()], correct_index: 3, category: Category::System, explanation: "chmod u+x dodaje uprawnienie execute (x) dla właściciela (u). chmod +x daje execute wszystkim.".into() },
            Quiz { id: 4, question: "Co robi komenda `systemctl enable serwis`?".into(), options: vec!["Uruchamia serwis natychmiast".into(), "Dodaje serwis do autostartu".into(), "Wyłącza serwis".into(), "Sprawdza status serwisu".into()], correct_index: 1, category: Category::System, explanation: "systemctl enable tworzy symlink, który uruchamia serwis przy starcie systemu. Do natychmiastowego uruchomienia użyj systemctl start.".into() },
            Quiz { id: 5, question: "Który typ rekordu DNS mapuje nazwę na adres IPv6?".into(), options: vec!["A".into(), "AAAA".into(), "CNAME".into(), "MX".into()], correct_index: 1, category: Category::Network, explanation: "AAAA (quad-A) mapuje nazwę na adres IPv6. A – IPv4, CNAME – alias, MX – serwer poczty.".into() },
            Quiz { id: 6, question: "Jaki jest domyślny tryb SELinux na Fedorze?".into(), options: vec!["Disabled".into(), "Permissive".into(), "Enforcing".into(), "Audit".into()], correct_index: 2, category: Category::Security, explanation: "Fedora domyślnie używa SELinux w trybie Enforcing (aktywna ochrona).".into() },
            Quiz { id: 7, question: "Co oznacza skrót LVM?".into(), options: vec!["Linux Volume Manager".into(), "Logical Volume Manager".into(), "Local Virtual Memory".into(), "Linear Virtual Management".into()], correct_index: 1, category: Category::System, explanation: "LVM = Logical Volume Manager – elastyczne zarządzanie przestrzenią dyskową.".into() },
            Quiz { id: 8, question: "Która komenda wyświetli wszystkie procesy użytkownika?".into(), options: vec!["ps -ef".into(), "ps aux".into(), "top -u user".into(), "Wszystkie powyższe".into()], correct_index: 3, category: Category::System, explanation: "Wszystkie trzy komendy (ps -ef, ps aux, top -u user) wyświetlają procesy użytkownika.".into() },
            Quiz { id: 9, question: "Jaki plik konfiguracyjny odpowiada za repozytoria DNF?".into(), options: vec!["/etc/dnf/dnf.conf".into(), "/etc/yum.repos.d/".into(), "/etc/apt/sources.list".into(), "/etc/pacman.conf".into()], correct_index: 1, category: Category::Linux, explanation: "Repozytoria DNF znajdują się w /etc/yum.repos.d/ (kompatybilność z YUM). Główny plik konfiguracyjny DNF to /etc/dnf/dnf.conf.".into() },
            Quiz { id: 10, question: "Co robi flaga `-v` w komendzie `tar -czvf archiwum.tar.gz katalog/`?".into(), options: vec!["Kompresuje".into(), "Wyświetla szczegółowe informacje (verbose)".into(), "Tworzy nowe archiwum".into(), "Weryfikuje archiwum".into()], correct_index: 1, category: Category::Shell, explanation: "-v (verbose) wyświetla listę przetwarzanych plików. -c tworzy, -z kompresuje gzip, -f określa nazwę pliku.".into() },
            Quiz { id: 11, question: "Który port jest domyślnie używany przez HTTP?".into(), options: vec!["22".into(), "80".into(), "443".into(), "8080".into()], correct_index: 1, category: Category::Network, explanation: "HTTP domyślnie używa portu 80, HTTPS – 443.".into() },
            Quiz { id: 12, question: "Jak sprawdzić aktualnie nasłuchujące porty w systemie?".into(), options: vec!["netstat -tulpn".into(), "ss -tulpn".into(), "lsof -i -P -n".into(), "Wszystkie powyższe".into()], correct_index: 3, category: Category::Network, explanation: "netstat, ss i lsof mogą wyświetlić nasłuchujące porty. ss jest najnowszym zalecanym narzędziem.".into() },
            Quiz { id: 13, question: "Jaka jest różnica między Docker a maszyną wirtualną?".into(), options: vec!["Docker jest szybszy bo współdzieli jądro hosta".into(), "Maszyny wirtualne są lżejsze".into(), "Docker wymaga hypervisora".into(), "Nie ma różnicy".into()], correct_index: 0, category: Category::DevOps, explanation: "Docker używa jądra hosta (współdzielone), podczas gdy VM ma własne jądro – Docker jest lżejszy i szybszy.".into() },
            Quiz { id: 14, question: "Co to jest SELinux context?".into(), options: vec!["Etykieta określająca typ pliku/procesu i jego uprawnienia".into(), "Katalog domowy użytkownika".into(), "Plik konfiguracyjny sieci".into(), "Zmienna środowiskowa".into()], correct_index: 0, category: Category::Security, explanation: "SELinux context (kontekst) to etykieta zawierająca user:role:type:level, która określa co proces/plik może robić.".into() },
            Quiz { id: 15, question: "Które narzędzie służy do automatycznej konfiguracji serwerów?".into(), options: vec!["Ansible".into(), "Docker".into(), "Systemd".into(), "Cron".into()], correct_index: 0, category: Category::DevOps, explanation: "Ansible to narzędzie do automatyzacji konfiguracji (Infrastructure as Code). Docker to konteneryzacja, systemd – init, cron – planowanie zadań.".into() },
            // -- Linux (id 16-24) --
            Quiz { id: 16, question: "Co robi polecenie `ln -s`?".into(), options: vec!["Tworzy dowiązanie symboliczne".into(), "Tworzy dowiązanie twarde".into(), "Wyświetla listę plików".into(), "Zmienia nazwę pliku".into()], correct_index: 0, category: Category::Linux, explanation: "ln -s (symbolic link) tworzy dowiązanie symboliczne wskazujące na inny plik. Bez -s tworzy dowiązanie twarde.".into() },
            Quiz { id: 17, question: "Który plik zawiera listę lokalnych użytkowników systemu?".into(), options: vec!["/etc/users".into(), "/etc/passwd".into(), "/etc/shadow".into(), "/var/log/auth.log".into()], correct_index: 1, category: Category::Linux, explanation: "/etc/passwd zawiera listę użytkowników (nazwa, UID, GID, shell, katalog domowy). Hasła są w /etc/shadow.".into() },
            Quiz { id: 18, question: "Co to jest inode w systemie plików Linux?".into(), options: vec!["Nazwa pliku".into(), "Struktura danych przechowująca metadane pliku".into(), "Katalog główny".into(), "System plików".into()], correct_index: 1, category: Category::Linux, explanation: "Inode przechowuje metadane pliku (uprawnienia, właściciel, rozmiar, timestampy, lokalizacja bloków) – wszystko poza nazwą. ls -i pokazuje numer inode.".into() },
            Quiz { id: 19, question: "Jakie polecenie pokazuje użycie przestrzeni dyskowej partycji?".into(), options: vec!["du".into(), "df".into(), "mount".into(), "lsblk".into()], correct_index: 1, category: Category::Linux, explanation: "df (disk free) pokazuje użycie partycji. du (disk usage) pokazuje rozmiar katalogów. mount pokazuje punkty montowania. lsblk listuje urządzenia blokowe.".into() },
            Quiz { id: 20, question: "Co zawiera plik /etc/fstab?".into(), options: vec!["Statyczne tabele routingu".into(), "Konfigurację zapory sieciowej".into(), "Statyczne informacje o systemach plików i montowaniu".into(), "Ustawienia DNS".into()], correct_index: 2, category: Category::Linux, explanation: "/etc/fstab (filesystem table) definiuje które partycje/urządzenia mają być montowane automatycznie przy starcie i z jakimi opcjami.".into() },
            Quiz { id: 21, question: "Które polecenie kopiuje plik z zachowaniem atrybutów (np. praw, dat)?" .into(), options: vec!["cp -r".into(), "cp -p".into(), "cp -f".into(), "cp -l".into()], correct_index: 1, category: Category::Linux, explanation: "cp -p (preserve) zachowuje atrybuty pliku: uprawnienia, właściciela, timestampy. cp -r kopiuje rekurencyjnie.".into() },
            Quiz { id: 22, question: "Co oznacza pierwszy znak `-` w `-rwxr-xr-x`?".into(), options: vec!["To jest katalog".into(), "To jest zwykły plik".into(), "To jest dowiązanie".into(), "To jest urządzenie blokowe".into()], correct_index: 1, category: Category::Linux, explanation: "Pierwszy znak to typ pliku: - (zwykły plik), d (katalog), l (dowiązanie), b (blokowe), c (znakowe).".into() },
            Quiz { id: 23, question: "Jakie polecenie wyświetla informacje o CPU w systemie Linux?".into(), options: vec!["lscpu".into(), "cpuinfo".into(), "cat /proc/cpu".into(), "uname -cpu".into()], correct_index: 0, category: Category::Linux, explanation: "lscpu wyświetla podsumowanie architektury CPU. cat /proc/cpuinfo daje szczegółowe info o każdym rdzeniu.".into() },
            Quiz { id: 24, question: "Jak sprawdzić wersję zainstalowanego jądra Linux?".into(), options: vec!["uname -r".into(), "kernel --version".into(), "cat /etc/kernel".into(), "lsmod | grep kernel".into()], correct_index: 0, category: Category::Linux, explanation: "uname -r wyświetla wersję jądra (np. 6.8.5-200.fc40.x86_64). uname -a pokazuje wszystkie informacje o systemie.".into() },
            // -- System (id 25-32) --
            Quiz { id: 25, question: "Co to jest pamięć swap w Linux?".into(), options: vec!["Szyfrowana partycja systemowa".into(), "Przestrzeń na dysku używana jako wirtualna pamięć RAM".into(), "Kopia zapasowa systemu".into(), "Sieciowy system plików".into()], correct_index: 1, category: Category::System, explanation: "Swap to przestrzeń na dysku (partycja lub plik), którą jądro używa jako przedłużenie RAM – rzadko używane strony są przenoszone na swap.".into() },
            Quiz { id: 26, question: "Które polecenie wyświetla komunikaty jądra z pierścienia buforowego?".into(), options: vec!["dmesg".into(), "journalctl".into(), "syslog".into(), "kernlog".into()], correct_index: 0, category: Category::System, explanation: "dmesg wyświetla bufor komunikatów jądra (ring buffer) – przydatne przy diagnostyce sprzętu i sterowników.".into() },
            Quiz { id: 27, question: "Jaka jest różnica między GRUB a systemd-boot?".into(), options: vec!["GRUB to bootloader, systemd-boot to init system".into(), "Oba to bootloadery, ale GRUB jest bardziej zaawansowany".into(), "systemd-boot to menedżer serwisów".into(), "Nie ma różnicy".into()], correct_index: 1, category: Category::System, explanation: "GRUB i systemd-boot to bootloadery. GRUB wspiera więcej schematów partycjonowania i konfiguracji, systemd-boot jest prostszy (UEFI tylko).".into() },
            Quiz { id: 28, question: "Który katalog przechowuje logi systemowe na Fedorze/RHEL?".into(), options: vec!["/var/log".into(), "/var/syslog".into(), "/etc/log".into(), "/run/log".into()], correct_index: 0, category: Category::System, explanation: "/var/log przechowuje logi systemowe (messages, secure, boot.log). journalctl czyta logi systemd-journald.".into() },
            Quiz { id: 29, question: "Co robi polecenie `sysctl -w kernel.hostname=server`?".into(), options: vec!["Zmienia nazwę hosta na stałe".into(), "Zmienia parametr jądra w czasie rzeczywistym".into(), "Wyświetla nazwę hosta".into(), "Restartuje serwer".into()], correct_index: 1, category: Category::System, explanation: "sysctl -w zmienia parametr jądra w locie (-w = write). Zmiana nie przetrwa restartu – aby była trwała, dodaj do /etc/sysctl.conf.".into() },
            Quiz { id: 30, question: "Jaka jest różnica między GPT a MBR?".into(), options: vec!["GPT obsługuje dyski >2TB i do 128 partycji, MBR do 2TB i 4 primary".into(), "MBR jest nowszy od GPT".into(), "GPT wymaga BIOS, MBR wymaga UEFI".into(), "Nie ma różnicy".into()], correct_index: 0, category: Category::System, explanation: "GPT (GUID Partition Table) to nowszy standard: dyski >2TB, 128 partycji primary, redundantna tablica. MBR (DOS) obsługuje do 2TB i 4 partycje primary.".into() },
            Quiz { id: 31, question: "Co to jest initramfs?".into(), options: vec!["System plików montowany przed jądrem".into(), "Tymczasowy system plików używany podczas bootowania przed zamontowaniem rootfs".into(), "Kopia zapasowa init".into(), "Plik konfiguracyjny jądra".into()], correct_index: 1, category: Category::System, explanation: "initramfs (initial RAM filesystem) to tymczasowy system plików ładowany do pamięci podczas bootowania – zawiera moduły jądra niezbędne do zamontowania właściwego rootfs.".into() },
            Quiz { id: 32, question: "Które polecenie tworzy timeline snapshot procesów?".into(), options: vec!["top".into(), "htop".into(), "ps".into(), "atop".into()], correct_index: 3, category: Category::System, explanation: "atop rejestruje historię procesów i zasobów (CPU, RAM, dysk, sieć) w odstępach czasu – można cofać się w czasie. top/htop pokazują bieżący stan.".into() },
            // -- Network (id 33-40) --
            Quiz { id: 33, question: "Co oznacza skrót MTU w sieciach?".into(), options: vec!["Maximum Transfer Unit – max rozmiar pakietu w warstwie sieci".into(), "Minimum Timeout Unit".into(), "Multi-Threaded Upload".into(), "Main Terminal Unit".into()], correct_index: 0, category: Category::Network, explanation: "MTU (Maximum Transmission Unit) to maksymalny rozmiar pakietu (zwykle 1500 bajtów dla Ethernet). Jeśli pakiet jest za duży, jest fragmentowany.".into() },
            Quiz { id: 34, question: "Jaki jest domyślny port dla HTTPS?".into(), options: vec!["80".into(), "443".into(), "22".into(), "8443".into()], correct_index: 1, category: Category::Network, explanation: "HTTPS (HTTP Secure) domyślnie używa portu 443. HTTP – 80, SSH – 22.".into() },
            Quiz { id: 35, question: "Które polecenie wyświetla tablicę routingu IP?".into(), options: vec!["route -n".into(), "ip route".into(), "netstat -r".into(), "Wszystkie powyższe".into()], correct_index: 3, category: Category::Network, explanation: "route -n, ip route i netstat -r wyświetlają tablicę routingu. ip route jest nowym zalecanym narzędziem.".into() },
            Quiz { id: 36, question: "Jaka jest różnica między TCP a UDP?".into(), options: vec!["TCP jest connection-oriented z gwarancją dostarczenia, UDP jest bezpołączeniowy".into(), "UDP jest szybszy ale zawodny".into(), "TCP wolniejszy ale niezawodny".into(), "Wszystkie powyższe są prawdziwe".into()], correct_index: 3, category: Category::Network, explanation: "TCP: połączeniowy, potwierdzenia, retransmisja, gwarancja dostarczenia. UDP: bezpołączeniowy, brak potwierdzeń, szybszy ale zawodny – dobry do streamingu/DNS.".into() },
            Quiz { id: 37, question: "Co to jest NAT w kontekście sieci?".into(), options: vec!["Network Address Translation – tłumaczenie adresów prywatnych na publiczne".into(), "Narzędzie do skanowania sieci".into(), "Protokół routingu".into(), "System nazw domen".into()], correct_index: 0, category: Category::Network, explanation: "NAT (Network Address Translation) pozwala wielu urządzeniom w sieci lokalnej współdzielić jeden publiczny adres IP. Masquerade w iptables to forma NAT.".into() },
            Quiz { id: 38, question: "Co to jest DHCP lease?".into(), options: vec!["Czas, na jaki adres IP jest przydzielony klientowi DHCP".into(), "Umowa licencyjna DHCP".into(), "Dzierżawa serwera".into(), "Rejestr zapytań DHCP".into()], correct_index: 0, category: Category::Network, explanation: "DHCP lease (dzierżawa) to okres, na który serwer DHCP przydziela adres IP klientowi. Po wygaśnięciu klient musi odnowić dzierżawę.".into() },
            Quiz { id: 39, question: "Które narzędzie sieciowe pozwala na interaktywne zapytania DNS?".into(), options: vec!["nslookup".into(), "dig".into(), "host".into(), "Wszystkie powyższe".into()], correct_index: 3, category: Category::Network, explanation: "nslookup, dig (Domain Information Groper) i host służą do zapytań DNS. dig daje najwięcej szczegółów i jest zalecany do diagnostyki.".into() },
            Quiz { id: 40, question: "Co robi polecenie `tcpdump -i eth0 port 80`?".into(), options: vec!["Wyświetla konfigurację interfejsu eth0".into(), "Przechwytuje pakiety na eth0 na porcie 80".into(), "Blokuje ruch na porcie 80".into(), "Wyświetla statystyki eth0".into()], correct_index: 1, category: Category::Network, explanation: "tcpdump przechwytuje i wyświetla pakiety sieciowe. -i eth0 wybiera interfejs, 'port 80' filtruje ruch HTTP.".into() },
            // -- Security (id 41-50) --
            Quiz { id: 41, question: "Co to jest hardening systemu?".into(), options: vec!["Proces zwiększania bezpieczeństwa przez konfigurację i usuwanie zbędnych usług".into(), "Instalacja nowego jądra".into(), "Formatowanie dysku".into(), "Aktualizacja pakietów".into()], correct_index: 0, category: Category::Security, explanation: "Hardening (utwardzanie) to proces zabezpieczania systemu: wyłączanie zbędnych usług, minimalizacja uprawnień, firewalle, polityki haseł, audyt.".into() },
            Quiz { id: 42, question: "Jakie są trzy tryby SELinux?".into(), options: vec!["On, Off, Auto".into(), "Enforcing, Permissive, Disabled".into(), "Active, Passive, Audit".into(), "Secure, Normal, Debug".into()], correct_index: 1, category: Category::Security, explanation: "Enforcing – polityki są wymuszane. Permissive – loguje naruszenia ale nie blokuje. Disabled – SELinux wyłączony. Zmianę robi się przez /etc/selinux/config lub setenforce.".into() },
            Quiz { id: 43, question: "Do czego służy polecenie `ssh-keygen`?".into(), options: vec!["Generuje parę kluczy SSH (prywatny+publiczny)".into(), "Skanuje sieć w poszukiwaniu serwerów SSH".into(), "Testuje połączenie SSH".into(), "Wyświetla listę kluczy SSH".into()], correct_index: 0, category: Category::Security, explanation: "ssh-keygen generuje parę kluczy SSH (-t ed25519 lub -t rsa). Klucz publiczny (.pub) trafia na serwer do ~/.ssh/authorized_keys.".into() },
            Quiz { id: 44, question: "Co to jest GPG (GnuPG)?".into(), options: vec!["Narzędzie do szyfrowania i podpisywania cyfrowego".into(), "Menedżer haseł".into(), "System kontroli wersji".into(), "Generator certyfikatów SSL".into()], correct_index: 0, category: Category::Security, explanation: "GPG (Gnu Privacy Guard) implementuje OpenPGP do szyfrowania asymetrycznego i symetrycznego, podpisów cyfrowych i zarządzania kluczami.".into() },
            Quiz { id: 45, question: "Do czego służy `fail2ban`?".into(), options: vec!["Do banowania adresów IP po wielokrotnych nieudanych próbach logowania".into(), "Do szyfrowania połączeń".into(), "Do monitorowania CPU".into(), "Do backupu danych".into()], correct_index: 0, category: Category::Security, explanation: "fail2ban analizuje logi (SSH, Apache, itp.) i dodaje reguły iptables/firewalld blokujące adresy IP po wykryciu wielokrotnych błędnych logowań.".into() },
            Quiz { id: 46, question: "Co to są ACL w systemie Linux?".into(), options: vec!["Access Control Lists – szczegółowe listy kontroli dostępu dla plików".into(), "Advanced Configuration Language".into(), "Protokół uwierzytelniania".into(), "Narzędzie do monitorowania".into()], correct_index: 0, category: Category::Security, explanation: "ACL (Access Control Lists) rozszerzają standardowe uprawnienia rwx o możliwość przypisania praw konkretnym użytkownikom i grupom. setfacl/getfacl do zarządzania.".into() },
            Quiz { id: 47, question: "Jaka jest różnica między szyfrowaniem symetrycznym a asymetrycznym?".into(), options: vec!["Symetryczne: jeden klucz do szyfrowania i deszyfrowania. Asymetryczne: para kluczy (publiczny + prywatny)".into(), "Symetryczne jest bezpieczniejsze".into(), "Asymetryczne jest szybsze".into(), "Nie ma różnicy".into()], correct_index: 0, category: Category::Security, explanation: "Symetryczne (AES, ChaCha20): ten sam klucz do szyfr/deszyfr – szybkie. Asymetryczne (RSA, ECDSA): klucz publiczny do szyfrowania, prywatny do deszyfrowania – wolniejsze, ale nie wymaga bezpiecznego przesłania klucza.".into() },
            Quiz { id: 48, question: "Co robi polecenie `umask 027`?".into(), options: vec!["Ustawia maskę uprawnień: nowe pliki mają 640, katalogi 750".into(), "Usuwa wszystkie pliki tymczasowe".into(), "Wyświetla aktualną maskę".into(), "Blokuje dostęp do plików".into()], correct_index: 0, category: Category::Security, explanation: "umask (user mask) odejmuje uprawnienia od domyślnych (666 dla plików, 777 dla katalogów). 027 oznacza: --- -w- rwx = plik 640, katalog 750.".into() },
            Quiz { id: 49, question: "Co to jest atak MITM (Man-in-the-Middle)?".into(), options: vec!["Atak polegający na przechwyceniu komunikacji między dwiema stronami".into(), "Atak na serwer pocztowy".into(), "Atak typu DoS".into(), "Atak socjotechniczny".into()], correct_index: 0, category: Category::Security, explanation: "MITM (Man-in-the-Middle) – atakujący podsłuchuje i modyfikuje komunikację między dwiema stronami bez ich wiedzy. Zapobiega temu TLS/SSH z weryfikacją certyfikatów.".into() },
            Quiz { id: 50, question: "Jakie polecenie wyświetla certyfikat SSL/TLS serwera?".into(), options: vec!["openssl s_client -connect host:443".into(), "curl -I https://host".into(), "nmap -sV host -p 443".into(), "Wszystkie powyższe".into()], correct_index: 3, category: Category::Security, explanation: "openssl s_client pokazuje pełny łańcuch certyfikatów. curl -I pokazuje nagłówki (w tym cert). nmap -sV wykrywa usługę i SSL.".into() },
            // -- Shell (id 51-59) --
            Quiz { id: 51, question: "Co robi polecenie `grep -r 'pattern' /etc/`?".into(), options: vec!["Szuka wzorca w plikach rekurencyjnie w katalogu /etc/".into(), "Usuwa pliki zawierające wzorzec".into(), "Wyświetla tylko nazwy plików".into(), "Liczy wystąpienia wzorca".into()], correct_index: 0, category: Category::Shell, explanation: "grep -r (recursive) przeszukuje wszystkie pliki w katalogu i podkatalogach. grep -l wyświetla tylko nazwy plików. grep -c liczy wystąpienia.".into() },
            Quiz { id: 52, question: "Jaka jest różnica między $@ a $* w bashu?".into(), options: vec!["$@ zachowuje cudzysłowy argumentów, $* traktuje wszystko jako jeden string".into(), "$* zachowuje cudzysłowy, $@ nie".into(), "Nie ma różnicy".into(), "$@ to PID procesu, $* to argumenty".into()], correct_index: 0, category: Category::Shell, explanation: "$@ zachowuje podział na argumenty (każdy w osobnych cudzysłowach). $* scala wszystkie argumenty w jeden string. To ma znaczenie przy iteracji.".into() },
            Quiz { id: 53, question: "Co robi operator `||` w bashu?".into(), options: vec!["Wykonuje drugie polecenie jeśli pierwsze zakończy się błędem (non-zero exit)".into(), "Łączy dwa polecenia równolegle".into(), "Wykonuje drugie polecenie jeśli pierwsze się powiedzie".into(), "Porównuje dwa stringi".into()], correct_index: 0, category: Category::Shell, explanation: "|| (logiczne OR) wykonuje drugie polecenie tylko gdy pierwsze zwróci kod wyjścia != 0 (błąd). && wykonuje drugie tylko gdy poprzednie się powiodło.".into() },
            Quiz { id: 54, question: "Czym są stdin, stdout, stderr?".into(), options: vec!["Trzy domyślne strumienie danych w procesie: wejście, wyjście, błędy".into(), "Typami zmiennych w bashu".into(), "Flagami mount".into(), "Trybami SELinux".into()], correct_index: 0, category: Category::Shell, explanation: "stdin (0) – wejście standardowe (klawiatura). stdout (1) – wyjście standardowe (terminal). stderr (2) – wyjście błędów (terminal). Przekierowanie: 2> do pliku.".into() },
            Quiz { id: 55, question: "Jak przekierować stderr do pliku?".into(), options: vec!["2> plik.txt".into(), "1> plik.txt".into(), "> plik.txt 2>&1".into(), "Zarówno A jak i C".into()], correct_index: 3, category: Category::Shell, explanation: "2> plik przekierowuje stderr. 2>&1 przekierowuje stderr tam gdzie stdout. > plik 2>&1 zapisuje stdout i stderr do pliku.".into() },
            Quiz { id: 56, question: "Co robi `set -e` w skrypcie bash?".into(), options: vec!["Przerywa skrypt przy pierwszym błędzie (non-zero exit)".into(), "Wyświetla każde polecenie przed wykonaniem".into(), "Ustawia edytor".into(), "Włącza tryb interaktywny".into()], correct_index: 0, category: Category::Shell, explanation: "set -e (errexit) powoduje natychmiastowe przerwanie skryptu jeśli jakiekolwiek polecenie zwróci kod błędu. set -x (xtrace) wyświetla polecenia przed wykonaniem.".into() },
            Quiz { id: 57, question: "Jak sprawdzić kod wyjścia ostatniego polecenia w bashu?".into(), options: vec!["echo $?".into(), "echo $!".into(), "echo $0".into(), "echo $STATUS".into()], correct_index: 0, category: Category::Shell, explanation: "$? przechowuje kod wyjścia (exit code) ostatnio wykonanego polecenia (0 = sukces, 1-255 = błąd). $! to PID ostatniego procesu w tle.".into() },
            Quiz { id: 58, question: "Jaka jest różnica między `[ ]` a `[[ ]]` w bashu?".into(), options: vec!["[[ ]] to rozszerzone testowanie z && || i porównaniami, [ ] to standard POSIX".into(), "[ ] jest nowsze od [[ ]]".into(), "Nie ma różnicy".into(), "[[ ]] działa tylko w sh".into()], correct_index: 0, category: Category::Shell, explanation: "[[ ]] to bash-builtin z większymi możliwościami: && || wewnątrz, porównania wzorców (== jako pattern), brak problemów z pustymi zmiennymi. [ ] to starszy POSIX test.".into() },
            Quiz { id: 59, question: "Co robi polecenie `xargs`?".into(), options: vec!["Buduje i wykonuje polecenia z danych wejściowych (stdin)".into(), "Wyświetla argumenty".into(), "Kasuje zmienne".into(), "Tworzy archiwum".into()], correct_index: 0, category: Category::Shell, explanation: "xargs czyta dane ze stdin i przekazuje je jako argumenty do podanego polecenia. Np. find . -name '*.tmp' | xargs rm – usunie wszystkie znalezione pliki.".into() },
            // -- DevOps (id 60-69) --
            Quiz { id: 60, question: "Co to jest Docker volume?".into(), options: vec!["Trwały magazyn danych zarządzany przez Docker, niezależny od cyklu życia kontenera".into(), "Partycja na dysku dla Dockera".into(), "Katalog tymczasowy kontenera".into(), "Migawka obrazu Docker".into()], correct_index: 0, category: Category::DevOps, explanation: "Docker volume to trwały magazyn danych poza warstwą zapisywalną kontenera. Volume lives beyond container – dane nie giną przy usunięciu kontenera. Bind mount też jest opcją.".into() },
            Quiz { id: 61, question: "Jaka jest różnica między Docker a Podman?".into(), options: vec!["Podman nie wymaga demona (daemonless) i może działać bez roota".into(), "Docker jest daemonless".into(), "Podman nie wspiera obrazów Docker".into(), "Nie ma różnicy".into()], correct_index: 0, category: Category::DevOps, explanation: "Podman (Pod Manager) – bez daemona, bez roota (rootless), kompatybilny z Docker CLI. Docker wymaga dockerd (demon) i roota do domyślnej pracy.".into() },
            Quiz { id: 62, question: "Co to jest Pod w Kubernetes?".into(), options: vec!["Najmniejsza jednostka w K8s – jeden lub więcej kontenerów współdzielących sieć i storage".into(), "Grupą serwerów".into(), "Plik konfiguracyjny K8s".into(), "Narzędzie do monitorowania".into()], correct_index: 0, category: Category::DevOps, explanation: "Pod to najmniejsza jednostka w Kubernetes. Zawiera jeden lub więcej kontenerów, które współdzielą IP, porty, wolumeny. Zwykle 1 pod = 1 kontener (sidecar pattern wyjątkiem).".into() },
            Quiz { id: 63, question: "Do czego służy Ansible playbook?".into(), options: vec!["Do definiowania zadań konfiguracyjnych w YAML wykonywanych na zdalnych hostach".into(), "Do budowania obrazów Docker".into(), "Do kompilacji jądra".into(), "Do backupu bazy danych".into()], correct_index: 0, category: Category::DevOps, explanation: "Ansible playbook (YAML) definiuje zestaw zadań (tasks) do wykonania na zarządzanych hostach – instalacja pakietów, kopiowanie plików, uruchamianie serwisów.".into() },
            Quiz { id: 64, question: "Co oznacza Infrastructure as Code (IaC)?".into(), options: vec!["Zarządzanie infrastrukturą przez pliki konfiguracyjne zamiast ręcznych poleceń".into(), "Kodowanie infrastruktury w C++".into(), "Używanie tylko chmury".into(), "Automatyczne skalowanie".into()], correct_index: 0, category: Category::DevOps, explanation: "IaC to praktyka definiowania infrastruktury (serwery, sieci, bazy) w plikach konfiguracyjnych (Terraform, Ansible, CloudFormation) – wersjonowanie, powtarzalność, automatyzacja.".into() },
            Quiz { id: 65, question: "Jaka jest różnica między CI a CD?".into(), options: vec!["CI (Continuous Integration) – automatyzacja testowania i budowania. CD (Continuous Delivery/Deployment) – automatyzacja wdrażania".into(), "CI to to samo co CD".into(), "CI dotyczy tylko aplikacji webowych".into(), "CD to narzędzie do backupu".into()], correct_index: 0, category: Category::DevOps, explanation: "CI: każda zmiana kodu jest automatycznie testowana i budowana. CD: po przejściu CI kod jest automatycznie wdrażany na środowiska (staging/production).".into() },
            Quiz { id: 66, question: "Co to jest Terraform state?".into(), options: vec!["Plik stanu przechowujący mapowanie zasobów między konfiguracją a rzeczywistą infrastrukturą".into(), "Stan konta w chmurze".into(), "Stan serwera".into(), "Licencja Terraform".into()], correct_index: 0, category: Category::DevOps, explanation: "Terraform state (terraform.tfstate) to plik JSON mapujący zasoby zdefiniowane w kodzie na rzeczywiste zasoby w chmurze – niezbędny do planowania zmian i zarządzania cyklem życia.".into() },
            Quiz { id: 67, question: "Co to jest container orchestration?".into(), options: vec!["Automatyczne zarządzanie kontenerami: skalowanie, networking, health checks, rolling updates".into(), "Ręczne uruchamianie kontenerów".into(), "Budowanie obrazów".into(), "Pullowanie obrazów".into()], correct_index: 0, category: Category::DevOps, explanation: "Container orchestration (orkiestracja kontenerów) – Kubernetes, Docker Swarm, Nomad – zarządza cyklem życia kontenerów: deployment, skalowanie, networking, load balancing, self-healing.".into() },
            Quiz { id: 68, question: "Które polecenie buduje obraz Docker z Dockerfile?".into(), options: vec!["docker build -t nazwa:tag .".into(), "docker create -t nazwa:tag .".into(), "docker run -t nazwa:tag .".into(), "docker compose build nazwa:tag .".into()], correct_index: 0, category: Category::DevOps, explanation: "docker build czyta Dockerfile w bieżącym katalogu (.) i buduje obraz z tagiem. docker run uruchamia kontener z obrazu. docker compose build buduje z docker-compose.yml.".into() },
            Quiz { id: 69, question: "Co to jest GitOps?".into(), options: vec!["Praktyka używania repozytorium Git jako jedynego źródła prawdy (single source of truth) dla infrastruktury i aplikacji".into(), "Używanie Git do backupu".into(), "Kurs Gita".into(), "Narzędzie do review kodu".into()], correct_index: 0, category: Category::DevOps, explanation: "GitOps: repozytorium Git zawiera całą konfigurację – zmiany są wprowadzane przez pull requesty, automatycznie synchronizowane z klastrem (Argo CD, Flux).".into() },
            // -- Logging, targets, boot, scripts, NFS, time, web, kernel, timers, containers, LVM, SELinux, troubleshooting, SSH (id 70-84) --
            Quiz { id: 70, question: "Które polecenie wyświetla logi tylko z bieżącego bootowania w systemd-journald?".into(), options: vec!["journalctl -b".into(), "journalctl -f".into(), "journalctl -u".into(), "journalctl --list-boots".into()], correct_index: 0, category: Category::System, explanation: "journalctl -b (--boot) pokazuje logi z obecnego rozruchu. -f follow, -u dla jednostki, --list-boots wyświetla listę bootów.".into() },
            Quiz { id: 71, question: "Jak sprawdzić domyślny target systemd?".into(), options: vec!["systemctl get-default".into(), "systemctl list-units".into(), "systemctl default".into(), "systemctl --target".into()], correct_index: 0, category: Category::System, explanation: "systemctl get-default pokazuje aktywny domyślny target przy starcie systemu. systemctl set-default zmienia go.".into() },
            Quiz { id: 72, question: "Który plik konfiguracyjny GRUB2 należy edytować, aby zmienić parametry jądra?".into(), options: vec!["/etc/default/grub".into(), "/boot/grub2/grub.cfg".into(), "/etc/grub.d/".into(), "/etc/grub.conf".into()], correct_index: 0, category: Category::System, explanation: "/etc/default/grub zawiera zmienne konfiguracyjne (GRUB_CMDLINE_LINUX). Po edycji uruchom grub2-mkconfig, aby wygenerować /boot/grub2/grub.cfg.".into() },
            Quiz { id: 73, question: "Jak dodać parametr 'single' do jądra podczas rozruchu GRUB2, aby zresetować hasło root?".into(), options: vec!["Nacisnąć 'e' na wpisie boot, dopisać 'rd.break' lub 'single' do linii linux, Ctrl+X".into(), "Wpisać 'reset root' w konsoli GRUB".into(), "Uruchomić z pendrive'a instalacyjnego".into(), "Nacisnąć 'c' i wpisać 'boot --single'".into()], correct_index: 0, category: Category::Security, explanation: "W GRUB2 naciskasz 'e', edytujesz linię linux (linuxefi), dodajesz 'rd.break' (RHEL) lub 'single', Ctrl+X aby uruchomić. Następnie mount -o remount,rw /sysroot && chroot /sysroot && passwd.".into() },
            Quiz { id: 74, question: "Jaka jest poprawna składnia pętli for w bashu iterującej po plikach .txt?".into(), options: vec!["for f in *.txt; do echo \"$f\"; done".into(), "for (f in *.txt) { echo $f }".into(), "foreach f in *.txt: echo $f".into(), "for f=1 to *.txt do echo $f".into()], correct_index: 0, category: Category::Shell, explanation: "for zmienna in wzorzec; do ... done – bash używa średników i done. Druga opcja to składnia C, trzecia to Python, czwarta to pseudokod.".into() },
            Quiz { id: 75, question: "Który port używa NFSv4 z domyślnym serwerem?".into(), options: vec!["TCP 2049".into(), "TCP 111".into(), "TCP 445".into(), "TCP 22".into()], correct_index: 0, category: Category::Network, explanation: "NFSv4 używa domyślnie TCP 2049. Portmapper (rpcbind) to 111. Samba (CIFS) to 445. SSH to 22.".into() },
            Quiz { id: 76, question: "Jak sprawdzić status synchronizacji czasu z serwerem NTP w chronyd?".into(), options: vec!["chronyc sources".into(), "chronyc tracking".into(), "timedatectl status".into(), "Wszystkie powyższe".into()], correct_index: 3, category: Category::System, explanation: "chronyc sources pokazuje źródła NTP, chronyc tracking – szczegóły synchronizacji, timedatectl status – ogólny stan czasu. Wszystkie są przydatne.".into() },
            Quiz { id: 77, question: "Jaka jest domyślna strona startowa Apache httpd na RHEL?".into(), options: vec!["/var/www/html/index.html".into(), "/usr/share/httpd/noindex/index.html".into(), "/etc/httpd/conf.d/welcome.conf".into(), "/srv/http/index.html".into()], correct_index: 1, category: Category::Network, explanation: "Na RHEL/Fedora domyślna strona testowa Apache to /usr/share/httpd/noindex/index.html. Główny DocumentRoot to /var/www/html, ale początkowo jest pusty.".into() },
            Quiz { id: 78, question: "Które polecenie ładuje moduł jądra na stałe (bez rebootu i po restarcie)?".into(), options: vec!["modprobe + wpis w /etc/modules-load.d/".into(), "insmod tylko".into(), "rmmod".into(), "lsmod".into()], correct_index: 0, category: Category::System, explanation: "modprobe ładuje moduł z zależnościami. Aby ładował się przy starcie, dodaj go do pliku w /etc/modules-load.d/. insmod ładuje bez zależności, rmmod usuwa, lsmod wyświetla.".into() },
            Quiz { id: 79, question: "Czym systemd timer różni się od cron?".into(), options: vec!["Timer może uruchomić zadanie z dokładnością do sekundy, wspiera kalendarz, monotoniczne wyzwalacze i logowanie do journald".into(), "Cron jest dokładniejszy".into(), "Timer nie wspiera dni tygodnia".into(), "Nie ma różnicy".into()], correct_index: 0, category: Category::System, explanation: "Systemd timer ma: dokładność do sekundy (cron – minuta), kalendarz (OnCalendar=), monotonic (OnBootSec=), pełne logowanie do journald i zależności między jednostkami.".into() },
            Quiz { id: 80, question: "Jak uruchomić kontener Podman jako usługę systemd dla użytkownika?".into(), options: vec!["podman generate systemd --name kontener > ~/.config/systemd/user/ && systemctl --user daemon-reload && systemctl --user enable --now kontener".into(), "docker run -d --restart=always kontener".into(), "systemctl start podman-kontener".into(), "Wrzucić skrypt do /etc/rc.local".into()], correct_index: 0, category: Category::DevOps, explanation: "podman generate systemd generuje plik jednostki. Skopiuj do ~/.config/systemd/user/, potem systemctl --user daemon-reload && enable --now. Dla system-wide użyj --system.".into() },
            Quiz { id: 81, question: "Do czego służy LVM snapshot?".into(), options: vec!["Tworzy punkt przywracania (kopię różnicową) woluminu logicznego w momencie wykonania".into(), "Zwiększa wydajność dysku".into(), "Zabezpiecza przed atakami".into(), "Szyfruje partycję".into()], correct_index: 0, category: Category::System, explanation: "LVM snapshot tworzy kopię różnicową (COW – copy-on-write) w momencie wykonania. Przydatne do backupu i testowania aktualizacji. lvcreate -s -n snap -L 1G /dev/vg/lv_root.".into() },
            Quiz { id: 82, question: "Które polecenie wyświetla booleany SELinux w trybie enforcing?".into(), options: vec!["getsebool -a".into(), "semanage boolean -l".into(), "setsebool -P".into(), "seinfo -b".into()], correct_index: 0, category: Category::Security, explanation: "getsebool -a wyświetla wszystkie booleany i ich stan. setsebool zmienia, semanage boolean -l też działa (wyświetla z opisem). seinfo -b wyświetla z libsefs.".into() },
            Quiz { id: 83, question: "Który plik logu systemowego należy sprawdzić w pierwszej kolejności przy awarii bootowania?".into(), options: vec!["journalctl -b -p err".into(), "/var/log/messages".into(), "/var/log/dmesg".into(), "/var/log/boot.log".into()], correct_index: 0, category: Category::System, explanation: "journalctl -b -p err pokazuje błędy z obecnego boota. /var/log/messages (tam rsyslog) i dmesg też przydatne, ale journalctl -b -p err jest najszybszy przy diagnozowaniu.".into() },
            Quiz { id: 84, question: "Który parametr w /etc/ssh/sshd_config wyłącza logowanie root przez hasło?".into(), options: vec!["PermitRootLogin prohibit-password".into(), "RootLogin no".into(), "DisableRootLogin yes".into(), "DenyRoot yes".into()], correct_index: 0, category: Category::Security, explanation: "PermitRootLogin prohibit-password (lub without-password w starszych wersjach) zezwala na logowanie root tylko przez klucz SSH, blokując hasło. To standardowa rekomendacja bezpieczeństwa.".into() },
        ]
    }
}

pub struct DataStore {
    pub data: Mutex<AppData>,
    path: PathBuf,
}

impl DataStore {
    pub fn new(app_dir: PathBuf) -> Self {
        fs::create_dir_all(&app_dir).ok();
        let path = app_dir.join("data.json");
        let data = Self::load_or_default(&path);
        DataStore { data: Mutex::new(data), path }
    }

    fn load_or_default(path: &PathBuf) -> AppData {
        if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| AppData::default_data())
        } else {
            AppData::default_data()
        }
    }

    pub fn save(&self) {
        if let Ok(data) = self.data.lock() {
            if let Ok(json) = serde_json::to_string_pretty(&*data) {
                fs::write(&self.path, json).ok();
            }
        }
    }

    pub fn get_json(&self) -> String {
        self.data.lock()
            .ok()
            .and_then(|d| serde_json::to_string_pretty(&*d).ok())
            .unwrap_or_default()
    }

    pub fn load_json(&self, json: &str) -> Result<(), String> {
        let new_data: AppData = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        *data = new_data;
        drop(data);
        self.save();
        Ok(())
    }

    pub fn save_github_token(&self, token: String) {
        let mut data = self.data.lock().unwrap();
        data.github_token = Some(token);
        drop(data);
        self.save();
    }

    pub fn save_daily_goal(&self, minutes: u32) {
        let mut data = self.data.lock().unwrap();
        data.daily_goal_minutes = minutes;
        drop(data);
        self.save();
    }

    pub fn log_session(&self, minutes: u32) {
        let mut data = self.data.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        data.sessions.push(StudySession { date: today, duration_minutes: minutes });
        drop(data);
        self.save();
    }

    pub fn complete_challenge(&self, challenge_id: u32) -> (bool, u32, u32) {
        let mut data = self.data.lock().unwrap();
        if let Some(ch) = data.challenges.iter_mut().find(|c| c.id == challenge_id) {
            if !ch.completed {
                ch.completed = true;
                let xp_gain = ch.difficulty as u32 * 10;
                data.xp += xp_gain;
                let new_level = 1 + data.xp / 100;
                data.level = new_level;
                drop(data);
                self.save();
                return (true, xp_gain, new_level);
            }
        }
        (false, 0, 1)
    }

    pub fn update_github_repo(&self, project_id: u32, repo: String) -> bool {
        let mut data = self.data.lock().unwrap();
        if let Some(p) = data.projects.iter_mut().find(|p| p.id == project_id) {
            p.github_repo = Some(repo.clone());
            drop(data);
            self.save();
            return true;
        }
        false
    }

    pub fn complete_project(&self, project_id: u32) -> (bool, u32) {
        let mut data = self.data.lock().unwrap();
        if let Some(p) = data.projects.iter_mut().find(|p| p.id == project_id) {
            if !p.completed {
                p.completed = true;
                data.xp += 50;
                let new_level = 1 + data.xp / 100;
                data.level = new_level;
                drop(data);
                self.save();
                return (true, new_level);
            }
        }
        (false, 1)
    }

    pub fn submit_quiz_answer(&self, quiz_id: u32, answer_index: usize) -> (bool, bool, String) {
        let mut data = self.data.lock().unwrap();
        let q_idx = data.quizzes.iter().position(|q| q.id == quiz_id);
        match q_idx {
            Some(idx) => {
                let q = &data.quizzes[idx];
                let correct = answer_index == q.correct_index;
                let explanation = q.explanation.clone();
                data.quiz_results.retain(|r| r.quiz_id != quiz_id);
                data.quiz_results.push(QuizResult { quiz_id, correct });
                if correct {
                    data.xp += 15;
                    data.level = 1 + data.xp / 100;
                    // correct → remove from wrong_answers tracking
                    data.wrong_answers.retain(|w| w.quiz_id != quiz_id);
                } else {
                    // wrong → spaced repetition tracking
                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                    let intervals = [0, 1, 3, 7, 14];
                    if let Some(wa) = data.wrong_answers.iter_mut().find(|w| w.quiz_id == quiz_id) {
                        wa.wrong_count = wa.wrong_count.saturating_add(1).min(99);
                        wa.last_wrong = today.clone();
                        let idx = (wa.wrong_count as usize).min(intervals.len() - 1);
                        let offset = intervals[idx];
                        let next = chrono::Local::now() + chrono::Duration::days(offset as i64);
                        wa.next_review = next.format("%Y-%m-%d").to_string();
                    } else {
                        let next = chrono::Local::now() + chrono::Duration::days(0);
                        data.wrong_answers.push(WrongAnswer {
                            quiz_id,
                            wrong_count: 1,
                            last_wrong: today.clone(),
                            next_review: next.format("%Y-%m-%d").to_string(),
                        });
                    }
                }
                drop(data);
                self.save();
                (true, correct, explanation)
            }
            None => (false, false, String::new()),
        }
    }

    pub fn mark_quiz_correct_in_review(&self, quiz_id: u32) {
        let mut data = self.data.lock().unwrap();
        data.wrong_answers.retain(|w| w.quiz_id != quiz_id);
        data.xp += 5;
        data.level = 1 + data.xp / 100;
        drop(data);
        self.save();
    }

    pub fn finish_exam(&self, score: u32, total: u32) {
        let mut data = self.data.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let passed = score as f64 / total as f64 >= 0.7;
        data.exam_attempts.push(ExamAttempt { date: today, score, total, passed });
        if passed {
            data.xp += 30;
            data.level = 1 + data.xp / 100;
        }
        drop(data);
        self.save();
    }

    pub fn export_progress_markdown(&self) -> String {
        let data = self.data.lock().unwrap();
        let completed = data.challenges.iter().filter(|c| c.completed).count();
        let total = data.challenges.len();
        let projects_done = data.projects.iter().filter(|p| p.completed).count();
        let projects_total = data.projects.len();
        let quiz_correct = data.quiz_results.iter().filter(|r| r.correct).count();
        let quiz_total = data.quiz_results.len();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        let mut md = String::new();
        md.push_str(&format!("# RootForge – Progress Report\n\n"));
        md.push_str(&format!("**Date:** {}  \n", today));
        md.push_str(&format!("**Level:** {}  \n", data.level));
        md.push_str(&format!("**XP:** {}  \n\n", data.xp));

        md.push_str("## Overview\n\n");
        md.push_str(&format!("- Challenges: {}/{} ({:.0}%)\n", completed, total, completed as f64 / total as f64 * 100.0));
        md.push_str(&format!("- Projects: {}/{}\n", projects_done, projects_total));
        md.push_str(&format!("- Quiz answers: {}/{}\n\n", quiz_correct, quiz_total));

        md.push_str("## Completed Challenges\n\n");
        for ch in data.challenges.iter().filter(|c| c.completed) {
            md.push_str(&format!("- [x] **{}** ({}, difficulty: {})\n", ch.title, ch.category.as_str(), ch.difficulty));
        }

        md.push_str("\n## Completed Projects\n\n");
        for p in data.projects.iter().filter(|p| p.completed) {
            md.push_str(&format!("- [x] **{}**", p.title));
            if let Some(ref repo) = p.github_repo {
                md.push_str(&format!(" – [GitHub]({})", repo));
            }
            md.push_str("\n");
        }

        if quiz_correct > 0 {
            md.push_str("\n## Quiz Results\n\n");
            for r in data.quiz_results.iter().filter(|r| r.correct) {
                if let Some(q) = data.quizzes.iter().find(|q| q.id == r.quiz_id) {
                    md.push_str(&format!("- ✅ {}\n", q.question));
                }
            }
        }

        md.push_str("\n---\n*Generated by RootForge*\n");
        md
    }
}

fn new_agent() -> ureq::Agent {
    let config = ureq::config::Config::builder()
        .timeout_connect(Some(std::time::Duration::from_secs(10)))
        .timeout_recv_body(Some(std::time::Duration::from_secs(30)))
        .build();
    ureq::Agent::new_with_config(config)
}

fn read_body_text(response: ureq::http::Response<ureq::Body>) -> Result<String, String> {
    let mut body = response.into_body();
    body.read_to_string().map_err(|e| format!("Body read error: {}", e))
}

pub fn github_save_gist(token: &str, gist_id: &Option<String>, filename: &str, content: &str) -> Result<String, String> {
    let json_body = serde_json::json!({
        "description": "RootForge – Linux SysAdmin progress backup",
        "public": false,
        "files": {
            filename: { "content": content }
        }
    });

    let agent = new_agent();
    let url = match gist_id {
        Some(id) => format!("https://api.github.com/gists/{}", id),
        None => "https://api.github.com/gists".to_string(),
    };

    let response = if gist_id.is_some() {
        agent.patch(&url)
    } else {
        agent.post(&url)
    }
    .header("Authorization", &format!("Bearer {}", token))
    .header("User-Agent", "CarrerPath/1.0")
    .header("Accept", "application/vnd.github.v3+json")
    .send_json(json_body)
    .map_err(|e| format!("GitHub API error: {}", e))?;

    let body_text = read_body_text(response)?;
    let json: serde_json::Value = serde_json::from_str(&body_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let new_id = json["id"].as_str()
        .ok_or_else(|| "Failed to get gist ID from response".to_string())?
        .to_string();

    Ok(new_id)
}

pub fn github_load_gist(token: &str, gist_id: &str) -> Result<String, String> {
    let agent = new_agent();
    let url = format!("https://api.github.com/gists/{}", gist_id);

    let response = agent.get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("User-Agent", "RootForge/1.0")
        .header("Accept", "application/vnd.github.v3+json")
        .call()
        .map_err(|e| format!("GitHub API error: {}", e))?;

    let body_text = read_body_text(response)?;
    let json: serde_json::Value = serde_json::from_str(&body_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let files = json["files"].as_object()
        .ok_or("No files in gist")?;

    let content = files.values()
        .next()
        .and_then(|f| f["content"].as_str())
        .ok_or("No content in gist file")?
        .to_string();

    Ok(content)
}
