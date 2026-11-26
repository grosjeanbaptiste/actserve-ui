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
            nameservers: vec![
                "ns1.cloudflare.com".to_string(),
                "ns2.cloudflare.com".to_string(),
            ],
            expires_in_days: 340,
        },
        DomainItem {
            name: "actserve.io".to_string(),
            registrar: "Namecheap".to_string(),
            status: "Active".to_string(),
            auto_renew: true,
            nameservers: vec![
                "dns1.registrar-servers.com".to_string(),
                "dns2.registrar-servers.com".to_string(),
            ],
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

#[derive(Clone, Debug, PartialEq)]
pub struct WorkflowStepItem {
    pub name: String,
    pub action_type: String,
    pub details: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkflowItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStepItem>,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FleetRolloutItem {
    pub id: String,
    pub target: String,
    pub strategy: String, // "Parallel", "Rolling"
    pub total_servers: usize,
    pub successful: usize,
    pub failed: usize,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleItem {
    pub id: String,
    pub name: String,
    pub cron_expression: String,
    pub workflow_id: String,
    pub target_tag: String,
    pub enabled: bool,
    pub last_run: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StepLogItem {
    pub step_index: usize,
    pub step_name: String,
    pub status: String, // "Success", "Failed", "Running"
    pub duration_ms: u64,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobLogItem {
    pub job_id: String,
    pub workflow_name: String,
    pub server_name: String,
    pub total_duration_ms: u64,
    pub status: String,
    pub steps: Vec<StepLogItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReconciliationItem {
    pub server_id: String,
    pub server_name: String,
    pub required_packages: Vec<String>,
    pub missing_packages: Vec<String>,
    pub domain_bound: bool,
    pub wireguard_active: bool,
    pub status: String, // "InSync", "Drifted", "Healing"
}

pub fn mock_workflows() -> Vec<WorkflowItem> {
    vec![
        WorkflowItem {
            id: "wf-prod-deploy".to_string(),
            name: "Déploiement Complet Prod Web + SSL".to_string(),
            description: "Mise à jour DNS A, provision ACME auto-TLS, action systemd reload et healthcheck HTTP".to_string(),
            steps: vec![
                WorkflowStepItem {
                    name: "Upsert DNS A".to_string(),
                    action_type: "DnsRecordUpsert".to_string(),
                    details: "A app.bioxus.cloud -> 192.168.1.100".to_string(),
                },
                WorkflowStepItem {
                    name: "Installation Dépendances".to_string(),
                    action_type: "PackageInstall".to_string(),
                    details: "caddy-proxy, git".to_string(),
                },
                WorkflowStepItem {
                    name: "Redémarrage Service Caddy".to_string(),
                    action_type: "ServiceAction".to_string(),
                    details: "Restart caddy.service".to_string(),
                },
                WorkflowStepItem {
                    name: "Vérification Santé HTTP".to_string(),
                    action_type: "HttpHealthCheck".to_string(),
                    details: "GET https://app.bioxus.cloud/health (attendu 200)".to_string(),
                },
            ],
            created_at: "2026-11-18 10:00".to_string(),
        },
        WorkflowItem {
            id: "wf-security-audit".to_string(),
            name: "Audit Sécurité & Facteurs Système".to_string(),
            description: "Collecte des facts Ansible, audit UFW et renouvellement certificats WireGuard".to_string(),
            steps: vec![
                WorkflowStepItem {
                    name: "Vérification Règles Pare-feu".to_string(),
                    action_type: "ShellCommand".to_string(),
                    details: "ufw status verbose".to_string(),
                },
                WorkflowStepItem {
                    name: "Statut WireGuard".to_string(),
                    action_type: "ShellCommand".to_string(),
                    details: "wg show all".to_string(),
                },
            ],
            created_at: "2026-11-18 09:30".to_string(),
        },
    ]
}

pub fn mock_schedules() -> Vec<ScheduleItem> {
    vec![
        ScheduleItem {
            id: "sch-01".to_string(),
            name: "Audit Sécurité Quotidien".to_string(),
            cron_expression: "0 2 * * *".to_string(),
            workflow_id: "wf-security-audit".to_string(),
            target_tag: "prod".to_string(),
            enabled: true,
            last_run: Some("Hier à 02:00".to_string()),
        },
        ScheduleItem {
            id: "sch-02".to_string(),
            name: "Vérification Dérive & Auto-Healing".to_string(),
            cron_expression: "*/30 * * * *".to_string(),
            workflow_id: "wf-prod-deploy".to_string(),
            target_tag: "web-cluster".to_string(),
            enabled: true,
            last_run: Some("Il y a 12 minutes".to_string()),
        },
    ]
}

pub fn mock_reconciliations() -> Vec<ReconciliationItem> {
    vec![
        ReconciliationItem {
            server_id: "srv-01-prod".to_string(),
            server_name: "gibs-main-node".to_string(),
            required_packages: vec![
                "dev-sec-hardening".to_string(),
                "caddy-proxy".to_string(),
                "ss-sensor".to_string(),
            ],
            missing_packages: vec![],
            domain_bound: true,
            wireguard_active: true,
            status: "InSync".to_string(),
        },
        ReconciliationItem {
            server_id: "srv-03-lab".to_string(),
            server_name: "homelab-storage".to_string(),
            required_packages: vec![
                "docker-ce".to_string(),
                "vaultwarden-server".to_string(),
                "wireguard".to_string(),
            ],
            missing_packages: vec!["vaultwarden-server".to_string()],
            domain_bound: false,
            wireguard_active: false,
            status: "Drifted".to_string(),
        },
    ]
}

pub fn mock_job_logs() -> Vec<JobLogItem> {
    vec![
        JobLogItem {
            job_id: "job-8812a".to_string(),
            workflow_name: "Déploiement Complet Prod Web + SSL".to_string(),
            server_name: "gibs-main-node (192.168.1.100)".to_string(),
            total_duration_ms: 1240,
            status: "Success".to_string(),
            steps: vec![
                StepLogItem {
                    step_index: 0,
                    step_name: "Upsert DNS A".to_string(),
                    status: "Success".to_string(),
                    duration_ms: 180,
                    stdout: "Record A app.bioxus.cloud set to 192.168.1.100 with TTL 300\nZone serial incremented".to_string(),
                    stderr: "".to_string(),
                },
                StepLogItem {
                    step_index: 1,
                    step_name: "Installation Dépendances".to_string(),
                    status: "Success".to_string(),
                    duration_ms: 620,
                    stdout: "Reading package lists...\nPackage caddy-proxy (v2.8.4) installed and verified".to_string(),
                    stderr: "".to_string(),
                },
                StepLogItem {
                    step_index: 2,
                    step_name: "Redémarrage Service Caddy".to_string(),
                    status: "Success".to_string(),
                    duration_ms: 210,
                    stdout: "caddy.service restarted gracefully\nActive: active (running)".to_string(),
                    stderr: "".to_string(),
                },
                StepLogItem {
                    step_index: 3,
                    step_name: "Vérification Santé HTTP".to_string(),
                    status: "Success".to_string(),
                    duration_ms: 230,
                    stdout: "HTTP GET https://app.bioxus.cloud/health returned 200 OK (latency: 28ms)".to_string(),
                    stderr: "".to_string(),
                },
            ],
        },
    ]
}
