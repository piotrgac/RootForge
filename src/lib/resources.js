export const resources = {
  linux: {
    name: 'Linux',
    icon: '🐧',
    links: [
      { title: 'Linux Documentation Project', url: 'https://www.tldp.org/', desc: 'Oficjalna dokumentacja Linux' },
      { title: 'Linux Journey', url: 'https://linuxjourney.com/', desc: 'Interaktywny kurs Linux od podstaw' },
      { title: 'Arch Wiki', url: 'https://wiki.archlinux.org/', desc: 'Najlepsza wiki Linux – nie tylko Arch' },
      { title: 'Linux Command Library', url: 'https://linuxcommandlibrary.com/', desc: 'Baza poleceń z przykładami' },
    ],
  },
  system: {
    name: 'System',
    icon: '⚙️',
    links: [
      { title: 'systemd Documentation', url: 'https://systemd.io/', desc: 'Oficjalna dokumentacja systemd' },
      { title: 'Freedesktop.org', url: 'https://www.freedesktop.org/wiki/', desc: 'Standardy Linux (XDG, D-Bus)' },
      { title: 'Kernel Newbies', url: 'https://kernelnewbies.org/', desc: 'Wprowadzenie do jądra Linux' },
      { title: 'Linux Performance (Brendan Gregg)', url: 'https://www.brendangregg.com/linuxperf.html', desc: 'Narzędzia i metody analizy wydajności' },
    ],
  },
  network: {
    name: 'Sieć',
    icon: '🌐',
    links: [
      { title: 'Beesky\'s Network Guide', url: 'https://www.beej.us/guide/bgnet/', desc: 'Programowanie gniazd sieciowych' },
      { title: 'nmap Guide', url: 'https://nmap.org/docs.html', desc: 'Oficjalna dokumentacja nmap' },
      { title: 'tcpdump Tutorial', url: 'https://danielmiessler.com/p/tcpdump/', desc: 'Praktyczny przewodnik po tcpdump' },
      { title: 'Wireshark Docs', url: 'https://www.wireshark.org/docs/', desc: 'Analiza pakietów Wireshark' },
    ],
  },
  security: {
    name: 'Bezpieczeństwo',
    icon: '🔒',
    links: [
      { title: 'OWASP', url: 'https://owasp.org/', desc: 'Otwarty projekt bezpieczeństwa aplikacji' },
      { title: 'CIS Benchmarks', url: 'https://www.cisecurity.org/cis-benchmarks/', desc: 'Standardy bezpiecznej konfiguracji' },
      { title: 'Stig Viewer', url: 'https://www.stigviewer.com/', desc: 'DISA STIG – hardening Linux' },
      { title: 'SSH Academy', url: 'https://www.ssh.com/academy/', desc: 'Kompendium wiedzy o SSH' },
    ],
  },
  shell: {
    name: 'Shell',
    icon: '💻',
    links: [
      { title: 'GNU Bash Manual', url: 'https://www.gnu.org/software/bash/manual/', desc: 'Oficjalny podręcznik Bash' },
      { title: 'Shell Scripting Tutorial', url: 'https://www.shellscript.sh/', desc: 'Wprowadzenie do skryptów shell' },
      { title: 'Explain Shell', url: 'https://explainshell.com/', desc: 'Rozszyfrowuje polecenia shell' },
      { title: 'Command Line Power User', url: 'https://commandlinepoweruser.com/', desc: 'Kurs linii poleceń dla zaawansowanych' },
    ],
  },
  devops: {
    name: 'DevOps',
    icon: '🚀',
    links: [
      { title: 'Docker Docs', url: 'https://docs.docker.com/', desc: 'Oficjalna dokumentacja Docker' },
      { title: 'Kubernetes Docs', url: 'https://kubernetes.io/docs/', desc: 'Oficjalna dokumentacja K8s' },
      { title: 'Ansible Documentation', url: 'https://docs.ansible.com/', desc: 'Dokumentacja Ansible' },
      { title: 'Terraform Learn', url: 'https://learn.hashicorp.com/terraform', desc: 'Nauka Terraform z przykładami' },
      { title: 'Git Pro Book', url: 'https://git-scm.com/book/pl/v2', desc: 'Książka o Git (po polsku)' },
      { title: 'DevOps Roadmap', url: 'https://roadmap.sh/devops', desc: 'Mapa drogi DevOps z zasobami' },
    ],
  },
};

export function getCategoryResources(cat) {
  return resources[cat] || null;
}

export function getAllCategories() {
  return Object.keys(resources);
}
