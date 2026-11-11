#[derive(Clone, Debug, PartialEq)]
pub struct ServerItem {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub status: String, // "Online", "Pending", "Maintenance", "Offline"
    pub cpu_pct: f64,
    pub ram_pct: f64,
    pub disk_pct: f64,
    pub bound_domain: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DomainItem {
    pub name: String,
    pub registrar: String,
    pub status: String,
    pub auto_renew: bool,
    pub nameservers: Vec<String>,
    pub expires_in_days: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DnsRecordItem {
    pub id: String,
    pub name: String,
    pub record_type: String,
    pub value: String,
    pub ttl: u32,
    pub propagated: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WireGuardPeerItem {
    pub public_key: String,
    pub allowed_ips: String,
    pub endpoint: Option<String>,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub last_handshake: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PresetItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
    pub category: String,
}

pub fn mock_servers() -> Vec<ServerItem> {
    vec![
        ServerItem {
            id: "srv-01-prod".to_string(),
            name: "gibs-main-node".to_string(),
            ip: "192.168.1.100".to_string(),
            status: "Online".to_string(),
            cpu_pct: 24.5,
            ram_pct: 62.0,
            disk_pct: 48.0,
            bound_domain: Some("app.bioxus.cloud".to_string()),
        },
        ServerItem {
            id: "srv-02-edge".to_string(),
            name: "gibs-edge-router".to_string(),
            ip: "192.168.1.101".to_string(),
            status: "Online".to_string(),
            cpu_pct: 12.0,
            ram_pct: 35.5,
            disk_pct: 22.0,
            bound_domain: Some("vpn.bioxus.cloud".to_string()),
        },
        ServerItem {
            id: "srv-03-lab".to_string(),
            name: "homelab-storage".to_string(),
            ip: "192.168.1.150".to_string(),
            status: "Maintenance".to_string(),
            cpu_pct: 5.0,
            ram_pct: 85.0,
            disk_pct: 78.5,
            bound_domain: None,
        },
    ]
}

pub fn mock_domains() -> Vec<DomainItem> {
    vec![
        DomainItem {
            name: "bioxus.cloud".to_string(),
            registrar: "Cloudflare".to_string(),
            status: "Active".to_string(),
            auto_renew: true,
            nameservers: vec!["ns1.cloudflare.com".to_string(), "ns2.cloudflare.com".to_string()],
            expires_in_days: 340,
        },
        DomainItem {
            name: "servai.io".to_string(),
            registrar: "Namecheap".to_string(),
            status: "Active".to_string(),
            auto_renew: true,
            nameservers: vec!["dns1.registrar-servers.com".to_string(), "dns2.registrar-servers.com".to_string()],
            expires_in_days: 285,
        },
    ]
}

pub fn mock_dns_records() -> Vec<DnsRecordItem> {
    vec![
        DnsRecordItem {
            id: "rec-1".to_string(),
            name: "@".to_string(),
            record_type: "A".to_string(),
            value: "192.168.1.100".to_string(),
            ttl: 300,
            propagated: true,
        },
        DnsRecordItem {
            id: "rec-2".to_string(),
            name: "app".to_string(),
            record_type: "A".to_string(),
            value: "192.168.1.100".to_string(),
            ttl: 300,
            propagated: true,
        },
        DnsRecordItem {
            id: "rec-3".to_string(),
            name: "vpn".to_string(),
            record_type: "A".to_string(),
            value: "192.168.1.101".to_string(),
            ttl: 300,
            propagated: true,
        },
        DnsRecordItem {
            id: "rec-4".to_string(),
            name: "_acme-challenge".to_string(),
            record_type: "TXT".to_string(),
            value: "letsencrypt-dns-01-token-validated".to_string(),
            ttl: 60,
            propagated: true,
        },
    ]
}

pub fn mock_wireguard_peers() -> Vec<WireGuardPeerItem> {
    vec![
        WireGuardPeerItem {
            public_key: "k9+RoadWarriorGibsMacBookProKey=".to_string(),
            allowed_ips: "10.0.0.2/32".to_string(),
            endpoint: Some("82.65.12.34:51820".to_string()),
            rx_bytes: 145_890_200,
            tx_bytes: 842_110_450,
            last_handshake: "Il y a 42 secondes".to_string(),
        },
        WireGuardPeerItem {
            public_key: "p4+MobileClientiPhonePeerKey=".to_string(),
            allowed_ips: "10.0.0.3/32".to_string(),
            endpoint: Some("90.12.45.67:51820".to_string()),
            rx_bytes: 25_100_000,
            tx_bytes: 110_500_000,
            last_handshake: "Il y a 2 minutes".to_string(),
        },
    ]
}

pub fn mock_presets() -> Vec<PresetItem> {
    vec![
        PresetItem {
            id: "gibs-developer-node".to_string(),
            name: "GIBS Developer Node".to_string(),
            description: "Hardening Linux + Docker Engine + Dotfiles (GNU Stow) + SS Telemetry Sensor + WireGuard VPN".to_string(),
            packages: vec![
                "dev-sec-hardening".to_string(),
                "geerlingguy-docker".to_string(),
                "dotfiles-stow".to_string(),
                "ss-sensor".to_string(),
                "linuxserver-wireguard".to_string(),
            ],
            category: "Production".to_string(),
        },
        PresetItem {
            id: "secure-web-baseline".to_string(),
            name: "Secure Web Baseline".to_string(),
            description: "Hardening Linux CIS + UFW/Fail2ban + Caddy HTTPS avec certificats SSL automatiques".to_string(),
            packages: vec![
                "dev-sec-hardening".to_string(),
                "geerlingguy-security".to_string(),
                "caddy-proxy".to_string(),
            ],
            category: "Web".to_string(),
        },
        PresetItem {
            id: "homelab-private-cloud".to_string(),
            name: "Homelab Private Cloud".to_string(),
            description: "Hardening + Docker + WireGuard VPN + Nextcloud stockage privé + Vaultwarden mots de passe".to_string(),
            packages: vec![
                "dev-sec-hardening".to_string(),
                "geerlingguy-docker".to_string(),
                "linuxserver-wireguard".to_string(),
                "linuxserver-nextcloud".to_string(),
                "vaultwarden-server".to_string(),
            ],
            category: "Cloud Privé".to_string(),
        },
    ]
}
