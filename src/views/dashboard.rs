use biorcrux::charts::{Bar, BarChart};
use biorcrux::widgets::{Panel, Section, Stat, Tone};
use dioxus::prelude::*;

use crate::model::{mock_domains, mock_servers, mock_wireguard_peers};

#[component]
pub fn DashboardView(on_navigate: EventHandler<String>) -> Element {
    let servers = mock_servers();
    let domains = mock_domains();
    let peers = mock_wireguard_peers();

    let online_servers = servers.iter().filter(|s| s.status == "Online").count();
    let active_peers = peers.len();
    let total_domains = domains.len();

    let cpu_bars = servers
        .iter()
        .map(|s| {
            let mut bar = Bar::new(s.name.clone(), s.cpu_pct, format!("{:.1}%", s.cpu_pct));
            bar.tone = if s.cpu_pct > 80.0 {
                Tone::Danger
            } else if s.cpu_pct > 50.0 {
                Tone::Warn
            } else {
                Tone::Ok
            };
            bar
        })
        .collect();

    let mem_bars = servers
        .iter()
        .map(|s| {
            let mut bar = Bar::new(s.name.clone(), s.ram_pct, format!("{:.1}%", s.ram_pct));
            bar.tone = if s.ram_pct > 80.0 {
                Tone::Warn
            } else {
                Tone::Neutral
            };
            bar
        })
        .collect();

    rsx! {
        div { class: "dashboard-view",
            Section { title: "État Global de l'Infrastructure",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;",
                    Stat {
                        label: "Serveurs En Ligne",
                        value: "{online_servers} / {servers.len()}",
                    }
                    Stat {
                        label: "Domaines & Zones DNS",
                        value: "{total_domains} gérés",
                    }
                    Stat {
                        label: "Pairs WireGuard Actifs",
                        value: "{active_peers} connectés",
                    }
                    Stat {
                        label: "Sécurité & SSL/TLS",
                        value: "100% Validé",
                    }
                }
            }

            Section { title: "Télémétrie & Charge Système (Moteur ss)",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.5rem;",
                    Panel { title: "Charge Processeur (CPU %)",
                        BarChart {
                            title: "Utilisation CPU par serveur",
                            bars: cpu_bars,
                            max: Some(100.0),
                        }
                    }
                    Panel { title: "Occupation Mémoire Vive (RAM %)",
                        BarChart {
                            title: "Utilisation RAM par serveur",
                            bars: mem_bars,
                            max: Some(100.0),
                        }
                    }
                }
            }

            Section { title: "Actions Rapides d'Orchestration",
                div { style: "display: flex; gap: 0.75rem; flex-wrap: wrap; margin-top: 1rem;",
                    button {
                        class: "bx-button",
                        onclick: move |_| on_navigate.call("servers".to_string()),
                        "🖥️ Déployer / Gérer un Serveur"
                    }
                    button {
                        class: "bx-button",
                        onclick: move |_| on_navigate.call("domains".to_string()),
                        "🌐 Nouveau Domaine & DNS"
                    }
                    button {
                        class: "bx-button",
                        onclick: move |_| on_navigate.call("vpn".to_string()),
                        "🔒 Ajouter un Pair WireGuard"
                    }
                    button {
                        class: "bx-button",
                        onclick: move |_| on_navigate.call("ecosystem".to_string()),
                        "📦 Déployer Preset GIBS (Dotfiles + SS)"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn DashboardTest() -> Element {
        rsx! {
            DashboardView {
                on_navigate: |_| (),
            }
        }
    }

    #[test]
    fn renders_dashboard_stats_and_charts() {
        let mut dom = VirtualDom::new(DashboardTest);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Serveurs En Ligne"));
        assert!(html.contains("Télémétrie"));
    }
}
