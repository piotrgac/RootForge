export const commands = [
  // Linux
  { category: 'linux', cmd: 'ls', desc: 'Listuje pliki i katalogi', example: 'ls -la /home' },
  { category: 'linux', cmd: 'cd', desc: 'Zmienia bieżący katalog', example: 'cd /var/log' },
  { category: 'linux', cmd: 'pwd', desc: 'Wyświetla ścieżkę bieżącego katalogu', example: 'pwd' },
  { category: 'linux', cmd: 'cp', desc: 'Kopiuje pliki i katalogi', example: 'cp -r src/ backup/' },
  { category: 'linux', cmd: 'mv', desc: 'Przenosi lub zmienia nazwę pliku', example: 'mv stary.txt nowy.txt' },
  { category: 'linux', cmd: 'rm', desc: 'Usuwa pliki', example: 'rm -rf /tmp/old/' },
  { category: 'linux', cmd: 'mkdir', desc: 'Tworzy katalog', example: 'mkdir -p a/b/c' },
  { category: 'linux', cmd: 'touch', desc: 'Tworzy pusty plik lub aktualizuje timestamp', example: 'touch nowy.txt' },
  { category: 'linux', cmd: 'find', desc: 'Szuka plików według kryteriów', example: 'find / -name "*.conf"' },
  { category: 'linux', cmd: 'locate', desc: 'Szybkie wyszukiwanie plików z bazy', example: 'locate .bashrc' },
  { category: 'linux', cmd: 'tree', desc: 'Wyświetla drzewo katalogów', example: 'tree -L 2 /etc' },
  { category: 'linux', cmd: 'du', desc: 'Pokazuje rozmiar plików/katalogów', example: 'du -sh /home/*' },
  { category: 'linux', cmd: 'df', desc: 'Pokazuje użycie partycji', example: 'df -h' },
  { category: 'linux', cmd: 'ln', desc: 'Tworzy dowiązania (symlink/hardlink)', example: 'ln -s /target link' },
  { category: 'linux', cmd: 'stat', desc: 'Szczegółowe informacje o pliku', example: 'stat /etc/passwd' },
  { category: 'linux', cmd: 'file', desc: 'Określa typ pliku', example: 'file /bin/bash' },

  // System
  { category: 'system', cmd: 'systemctl', desc: 'Zarządza usługami systemd', example: 'systemctl status sshd' },
  { category: 'system', cmd: 'journalctl', desc: 'Przegląda logi systemd', example: 'journalctl -u nginx --since today' },
  { category: 'system', cmd: 'ps', desc: 'Listuje procesy', example: 'ps aux --sort=-%mem' },
  { category: 'system', cmd: 'top', desc: 'Monitoruje procesy w czasie rzeczywistym', example: 'top -o %MEM' },
  { category: 'system', cmd: 'htop', desc: 'Interaktywny monitor procesów', example: 'htop' },
  { category: 'system', cmd: 'kill', desc: 'Wysyła sygnał do procesu', example: 'kill -9 1234' },
  { category: 'system', cmd: 'nice', desc: 'Uruchamia proces z priorytetem', example: 'nice -n 10 ./script.sh' },
  { category: 'system', cmd: 'crontab', desc: 'Zarządza zadaniami cron', example: 'crontab -e' },
  { category: 'system', cmd: 'at', desc: 'Planuje jednorazowe zadanie', example: 'at now + 1 hour' },
  { category: 'system', cmd: 'uname', desc: 'Wyświetla informacje o systemie', example: 'uname -a' },
  { category: 'system', cmd: 'dmesg', desc: 'Pokazuje bufor komunikatów jądra', example: 'dmesg | tail -20' },
  { category: 'system', cmd: 'lscpu', desc: 'Informacje o CPU', example: 'lscpu' },
  { category: 'system', cmd: 'lsblk', desc: 'Listuje urządzenia blokowe', example: 'lsblk -f' },
  { category: 'system', cmd: 'free', desc: 'Pokazuje użycie pamięci', example: 'free -h' },
  { category: 'system', cmd: 'uptime', desc: 'Czas działania systemu', example: 'uptime' },
  { category: 'system', cmd: 'reboot', desc: 'Restartuje system', example: 'sudo reboot' },
  { category: 'system', cmd: 'shutdown', desc: 'Wyłącza system', example: 'sudo shutdown -h now' },
  { category: 'system', cmd: 'modprobe', desc: 'Ładuje moduły jądra', example: 'sudo modprobe vfio-pci' },
  { category: 'system', cmd: 'lsmod', desc: 'Listuje załadowane moduły', example: 'lsmod' },
  { category: 'system', cmd: 'udevadm', desc: 'Zarządza urządzeniami udev', example: 'udevadm info /dev/sda' },

  // Sieć
  { category: 'network', cmd: 'ip', desc: 'Konfiguruje interfejsy sieciowe', example: 'ip addr show' },
  { category: 'network', cmd: 'ss', desc: 'Bada gniazda sieciowe', example: 'ss -tuln' },
  { category: 'network', cmd: 'ping', desc: 'Testuje łączność z hostem', example: 'ping -c 4 8.8.8.8' },
  { category: 'network', cmd: 'traceroute', desc: 'Śledzi trasę pakietów', example: 'traceroute google.com' },
  { category: 'network', cmd: 'nslookup', desc: 'Zapytanie DNS', example: 'nslookup example.com' },
  { category: 'network', cmd: 'dig', desc: 'Szczegółowe zapytanie DNS', example: 'dig mx gmail.com' },
  { category: 'network', cmd: 'curl', desc: 'Klient URL / HTTP', example: 'curl -I https://example.com' },
  { category: 'network', cmd: 'wget', desc: 'Pobiera pliki z sieci', example: 'wget -c https://example.com/file.iso' },
  { category: 'network', cmd: 'netstat', desc: 'Statystyki sieci (starsze)', example: 'netstat -tulpn' },
  { category: 'network', cmd: 'nmap', desc: 'Skaner portów', example: 'nmap -sS 192.168.1.0/24' },
  { category: 'network', cmd: 'tcpdump', desc: 'Przechwytuje pakiety sieciowe', example: 'tcpdump -i eth0 port 80' },
  { category: 'network', cmd: 'iptables', desc: 'Firewall / NAT', example: 'iptables -L -n -v' },
  { category: 'network', cmd: 'firewall-cmd', desc: 'FirewallD (firewalld)', example: 'firewall-cmd --add-service=http' },
  { category: 'network', cmd: 'hostnamectl', desc: 'Ustawia nazwę hosta', example: 'hostnamectl set-hostname server1' },
  { category: 'network', cmd: 'nmcli', desc: 'NetworkManager CLI', example: 'nmcli con show' },
  { category: 'network', cmd: 'ethtool', desc: 'Konfiguruje karty sieciowe', example: 'ethtool eth0' },

  // Bezpieczeństwo
  { category: 'security', cmd: 'chmod', desc: 'Zmienia prawa dostępu', example: 'chmod 755 script.sh' },
  { category: 'security', cmd: 'chown', desc: 'Zmienia właściciela pliku', example: 'chown user:group file.txt' },
  { category: 'security', cmd: 'umask', desc: 'Ustawia domyślne maski uprawnień', example: 'umask 027' },
  { category: 'security', cmd: 'passwd', desc: 'Zmienia hasło użytkownika', example: 'passwd username' },
  { category: 'security', cmd: 'useradd', desc: 'Dodaje użytkownika', example: 'sudo useradd -m -s /bin/bash nowy' },
  { category: 'security', cmd: 'usermod', desc: 'Modyfikuje konto użytkownika', example: 'usermod -aG sudo username' },
  { category: 'security', cmd: 'groupadd', desc: 'Dodaje grupę', example: 'sudo groupadd devs' },
  { category: 'security', cmd: 'sudo', desc: 'Wykonuje polecenie jako root', example: 'sudo visudo' },
  { category: 'security', cmd: 'su', desc: 'Zmienia użytkownika', example: 'su - username' },
  { category: 'security', cmd: 'ssh-keygen', desc: 'Generuje klucze SSH', example: 'ssh-keygen -t ed25519' },
  { category: 'security', cmd: 'ssh-copy-id', desc: 'Kopiuje klucz publiczny na host', example: 'ssh-copy-id user@server' },
  { category: 'security', cmd: 'openssl', desc: 'Narzędzie kryptograficzne', example: 'openssl req -new -x509 ...' },
  { category: 'security', cmd: 'gpg', desc: 'Szyfrowanie GPG', example: 'gpg -c file.txt' },
  { category: 'security', cmd: 'getfacl', desc: 'Wyświetla ACL pliku', example: 'getfacl /etc/shadow' },
  { category: 'security', cmd: 'setfacl', desc: 'Ustawia ACL pliku', example: 'setfacl -m u:user:rwx file' },
  { category: 'security', cmd: 'apparmor_status', desc: 'Sprawdza status AppArmor', example: 'sudo aa-status' },
  { category: 'security', cmd: 'selinuxenabled', desc: 'Sprawdza czy SELinux jest włączony', example: 'getenforce' },

  // Shell
  { category: 'shell', cmd: 'grep', desc: 'Wyszukuje wzorzec w plikach', example: 'grep -r "error" /var/log/' },
  { category: 'shell', cmd: 'awk', desc: 'Przetwarza tekst kolumnowo', example: 'awk \'{print $1}\' file.log' },
  { category: 'shell', cmd: 'sed', desc: 'Edytuje strumień tekstu', example: 'sed -i "s/old/new/g" file.txt' },
  { category: 'shell', cmd: 'cut', desc: 'Wyciąga kolumny z pliku', example: 'cut -d: -f1 /etc/passwd' },
  { category: 'shell', cmd: 'sort', desc: 'Sortuje linie', example: 'sort -t: -k3 -n /etc/passwd' },
  { category: 'shell', cmd: 'uniq', desc: 'Usuwa duplikaty (wymaga sort)', example: 'sort file.txt | uniq -c' },
  { category: 'shell', cmd: 'wc', desc: 'Zlicza linie/słowa/bajty', example: 'wc -l /etc/passwd' },
  { category: 'shell', cmd: 'head', desc: 'Wyświetla pierwsze N linii', example: 'head -n 20 /var/log/syslog' },
  { category: 'shell', cmd: 'tail', desc: 'Wyświetla ostatnie N linii', example: 'tail -f /var/log/nginx/access.log' },
  { category: 'shell', cmd: 'tee', desc: 'Zapisuje do pliku i stdout', example: 'echo "data" | tee file.txt' },
  { category: 'shell', cmd: 'xargs', desc: 'Buduje argumenty z stdin', example: 'find . -name "*.tmp" | xargs rm' },
  { category: 'shell', cmd: 'alias', desc: 'Tworzy alias polecenia', example: 'alias ll="ls -la"' },
  { category: 'shell', cmd: 'history', desc: 'Pokazuje historię poleceń', example: 'history | grep git' },
  { category: 'shell', cmd: 'export', desc: 'Ustawia zmienną środowiskową', example: 'export PATH=$PATH:/custom/bin' },
  { category: 'shell', cmd: 'source', desc: 'Wykonuje skrypt w bieżącym shellu', example: 'source ~/.bashrc' },
  { category: 'shell', cmd: 'nohup', desc: 'Uruchamia proces odporny na SIGHUP', example: 'nohup ./server &' },
  { category: 'shell', cmd: 'jobs', desc: 'Listuje zadania w tle', example: 'jobs -l' },
  { category: 'shell', cmd: 'screen', desc: 'Multiplekser terminala', example: 'screen -S mysession' },
  { category: 'shell', cmd: 'tmux', desc: 'Nowoczesny multiplekser terminala', example: 'tmux new -s mysession' },
  { category: 'shell', cmd: '&& / ||', desc: 'Łańcuchowanie warunkowe', example: 'make && make install' },

  // DevOps
  { category: 'devops', cmd: 'docker', desc: 'Zarządza kontenerami', example: 'docker run -d -p 80:80 nginx' },
  { category: 'devops', cmd: 'docker-compose', desc: 'Definiuje multi-kontenerowe aplikacje', example: 'docker compose up -d' },
  { category: 'devops', cmd: 'podman', desc: 'Bezdaemonowe kontenery', example: 'podman pull fedora' },
  { category: 'devops', cmd: 'kubectl', desc: 'Zarządza klastrem Kubernetes', example: 'kubectl get pods -A' },
  { category: 'devops', cmd: 'ansible', desc: 'Automatyzacja konfiguracji', example: 'ansible all -m ping' },
  { category: 'devops', cmd: 'ansible-playbook', desc: 'Uruchamia playbook Ansible', example: 'ansible-playbook site.yml' },
  { category: 'devops', cmd: 'terraform', desc: 'Infrastruktura jako kod', example: 'terraform plan' },
  { category: 'devops', cmd: 'git', desc: 'Kontrola wersji', example: 'git commit -m "fix: ..."' },
  { category: 'devops', cmd: 'vagrant', desc: 'Zarządza maszynami wirtualnymi', example: 'vagrant up' },
  { category: 'devops', cmd: 'make', desc: 'Automatyzacja budowania', example: 'make install' },
  { category: 'devops', cmd: 'ci/cd', desc: 'Pipelines CI/CD', example: 'gitlab-ci.yml / .github/workflows' },
  { category: 'devops', cmd: 'prometheus', desc: 'Monitoring / zbieranie metryk', example: 'prometheus --config.file=prom.yml' },
  { category: 'devops', cmd: 'grafana', desc: 'Wizualizacja metryk', example: 'dashboard + alerting' },
  { category: 'devops', cmd: 'nginx', desc: 'Serwer HTTP / reverse proxy', example: 'nginx -t && systemctl reload nginx' },
  { category: 'devops', cmd: 'postfix', desc: 'Serwer poczty', example: 'postfix check' },
  { category: 'devops', cmd: 'rsync', desc: 'Synchronizacja plików', example: 'rsync -avz /src/ user@host:/dst/' },
];

export const categories = [...new Set(commands.map(c => c.category))];

export function getCommandsByCategory(cat) {
  return commands.filter(c => c.category === cat);
}

export function searchCommands(query) {
  const q = query.toLowerCase();
  return commands.filter(c =>
    c.cmd.toLowerCase().includes(q) ||
    c.desc.toLowerCase().includes(q) ||
    c.category.toLowerCase().includes(q)
  );
}
