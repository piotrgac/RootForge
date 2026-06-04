export const certificationRoadmaps = [
  {
    name: 'LPIC-1',
    fullName: 'Linux Professional Institute Certification – Level 1',
    description: 'Certyfikat potwierdzający podstawową wiedzę o administracji Linux. Wymagany do LPIC-2.',
    url: 'https://www.lpi.org/our-certifications/lpic-1-overview/',
    exams: [
      { code: '101-500', name: 'Architektura systemu, instalacja, zarządzanie pakietami, sprzęt' },
      { code: '102-500', name: 'Shell, skrypty, interfejsy, zadania, bezpieczeństwo' },
    ],
    topics: [
      { category: 'linux', name: 'Architektura systemu Linux', tasks: ['Uruchamianie systemu', 'Inicjatywy', 'Procesy', 'Moduły jądra'] },
      { category: 'system', name: 'Instalacja Linux i zarządzanie pakietami', tasks: ['Dysk i partycje', 'Bootloader', 'RPM/DPKG', 'Repozytoria'] },
      { category: 'shell', name: 'Polecenia GNU i Unix', tasks: ['Linia poleceń', 'Potoki', 'Przetwarzanie tekstu', 'Zarządzanie plikami'] },
      { category: 'linux', name: 'Urządzenia i systemy plików', tasks: ['Montowanie', 'ACL', 'RAID', 'LVM', 'Quota'] },
      { category: 'network', name: 'Sieci', tasks: ['Konfiguracja sieci', 'DNS', 'NTP', 'Firewall'] },
      { category: 'security', name: 'Bezpieczeństwo systemu', tasks: ['Prawa dostępu', 'Hasła', 'Sudo', 'SSH', 'GP'] },
    ],
  },
  {
    name: 'LPIC-2',
    fullName: 'Linux Professional Institute Certification – Level 2',
    description: 'Zaawansowana administracja: sieci, usługi, bezpieczeństwo, jądro.',
    url: 'https://www.lpi.org/our-certifications/lpic-2-overview/',
    exams: [
      { code: '201-450', name: 'Jądro, system plików, storage, networking, DNS, HTTP' },
      { code: '202-450', name: 'DHCP, Poczta, SSH, VPN, bezpieczeństwo, monitoring' },
    ],
    topics: [
      { category: 'system', name: 'Jądro i system plików', tasks: ['Kompilacja jądra', 'Systemy plików', 'LVM', 'RAID programowy', 'Udziały NFS/Samba'] },
      { category: 'network', name: 'Usługi sieciowe', tasks: ['DNS (BIND)', 'HTTP (Apache)', 'DHCP', 'Poczta (Postfix)', 'FTP'] },
      { category: 'security', name: 'Bezpieczeństwo sieci', tasks: ['OpenVPN', 'SSHd', 'iptables', 'TCP Wrappers', 'GPG'] },
      { category: 'system', name: 'Monitoring', tasks: ['Syslog', 'Logrotate', 'Nagios/Icinga', 'SNMP', 'MIB'] },
    ],
  },
  {
    name: 'RHCSA',
    fullName: 'Red Hat Certified System Administrator',
    description: 'Praktyczny egzamin na żywym systemie Red Hat Enterprise Linux. Ceniony przez pracodawców.',
    url: 'https://www.redhat.com/en/services/training/ex200-red-hat-certified-system-administrator-rhcsa-exam',
    topics: [
      { category: 'system', name: 'Zarządzanie systemem', tasks: ['systemd', 'usługi', 'targets', 'timery', 'logowanie'] },
      { category: 'linux', name: 'System plików', tasks: ['LVM', 'Stratis', 'VDO', 'RAID', 'partycje', 'montowanie'] },
      { category: 'shell', name: 'Skrypty i automatyzacja', tasks: ['Bash', 'warunki', 'pętle', 'wyrażenia regularne'] },
      { category: 'security', name: 'Bezpieczeństwo', tasks: ['SELinux', 'firewalld', 'SSH', 'sudo', 'ACL'] },
      { category: 'network', name: 'Sieci', tasks: ['Konfiguracja sieci', 'bonding', 'team', 'DNS', 'NTP'] },
      { category: 'devops', name: 'Kontenery', tasks: ['Podman', 'Skopeo', 'Buildah', 'Containerfiles'] },
    ],
  },
  {
    name: 'RHCE',
    fullName: 'Red Hat Certified Engineer',
    description: 'Zaawansowany egzamin: automatyzacja Ansible, role, playbooki.',
    url: 'https://www.redhat.com/en/services/training/ex294-red-hat-certified-engineer-rhce-exam',
    topics: [
      { category: 'devops', name: 'Ansible', tasks: ['Playbooki', 'role', 'szablony', 'handlery', 'zmienne', 'vault'] },
      { category: 'system', name: 'Automatyzacja systemu', tasks: ['Ansible do konfiguracji LVM, sieci, SELinux, firewalld'] },
      { category: 'security', name: 'Ansible Vault i bezpieczeństwo', tasks: ['Szyfrowanie haseł', 'zarządzanie kluczami'] },
    ],
  },
];

export function getRoadmapByName(name) {
  return certificationRoadmaps.find(r => r.name === name) || null;
}

export function getTopicsByCategory(category) {
  const result = [];
  for (const road of certificationRoadmaps) {
    for (const topic of road.topics) {
      if (topic.category === category) {
        result.push({ ...topic, cert: road.name });
      }
    }
  }
  return result;
}
