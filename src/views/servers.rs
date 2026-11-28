use biorcrux::widgets::{Badge, Dialog, InputField, Section, Tone};
use dioxus::prelude::*;

use crate::model::{mock_servers, ServerItem};

#[component]
pub fn ServersView() -> Element {
    let mut servers = use_signal(mock_servers);
    let mut selected_server = use_signal(|| None::<ServerItem>);
    let mut bind_domain_target = use_signal(|| None::<ServerItem>);
    let mut domain_input = use_signal(String::new);
    let mut toast_message = use_signal(|| None::<String>);

    rsx! {
        div { class: "servers-view",
            Section { title: "Serveurs et Nœuds Gérés",
                if let Some(msg) = toast_message() {
                    div {
                        style: "background: var(--bx-color-ok, #22c55e); color: white; padding: 0.75rem 1rem; border-radius: 6px; margin-bottom: 1rem;",
                        "✓ {msg}"
                    }
                }

                div { style: "display: flex; flex-direction: column; gap: 1rem;",
                    for server in servers().iter() {
                        div {
                            key: "{server.id}",
                            style: "background: var(--bx-surface, #ffffff); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 8px; padding: 1.25rem; display: flex; flex-direction: column; gap: 0.75rem;",
                            div { style: "display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem;",
                                div { style: "display: flex; align-items: center; gap: 0.75rem;",
                                    h3 { style: "margin: 0; font-size: 1.1rem;", "{server.name}" }
                                    span { style: "color: var(--bx-muted, #6b7280); font-family: monospace; font-size: 0.9rem;",
                                        "{server.ip}"
                                    }
                                }
                                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                    Badge {
                                        tone: match server.status.as_str() {
                                            "Online" => Tone::Ok,
                                            "Maintenance" => Tone::Warn,
                                            _ => Tone::Danger,
                                        },
                                        "{server.status}"
                                    }
                                    if let Some(domain) = &server.bound_domain {
                                        Badge { tone: Tone::Ok, "🔒 {domain}" }
                                    }
                                }
                            }

                            div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 0.75rem; font-size: 0.85rem; color: var(--bx-muted, #4b5563);",
                                div { "CPU: {server.cpu_pct:.1}%" }
                                div { "RAM: {server.ram_pct:.1}%" }
                                div { "Disque: {server.disk_pct:.1}%" }
                                div { "ID: {server.id}" }
                            }

                            div { style: "display: flex; gap: 0.5rem; flex-wrap: wrap; margin-top: 0.5rem;",
                                button {
                                    class: "bx-button",
                                    onclick: {
                                        let s = server.clone();
                                        move |_| {
                                            domain_input.set(format!("{}.bioxus.cloud", s.name));
                                            bind_domain_target.set(Some(s.clone()));
                                        }
                                    },
                                    "🌐 Lier un Domaine (Auto-SSL)"
                                }
                                button {
                                    class: "bx-button",
                                    onclick: {
                                        let s = server.clone();
                                        move |_| selected_server.set(Some(s.clone()))
                                    },
                                    "⚙️ Actions & Services"
                                }
                                button {
                                    class: "bx-button",
                                    onclick: {
                                        let name = server.name.clone();
                                        move |_| {
                                            toast_message
                                                .set(Some(format!("Ordre de redémarrage envoyé à {}", name)));
                                        }
                                    },
                                    "🔄 Reboot"
                                }
                            }
                        }
                    }
                }
            }

            // Modal Lier un Domaine (Auto-SSL Let's Encrypt + Record A)
            if let Some(target) = bind_domain_target() {
                Dialog {
                    title: "Lier un Domaine & Certificat SSL Let's Encrypt",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| bind_domain_target.set(None),
                    p { "Associer automatiquement un nom de domaine à {target.name} ({target.ip}) :" }
                    InputField {
                        label: "Nom de domaine complet (FQDN)",
                        value: domain_input(),
                        on_input: move |val| domain_input.set(val),
                    }
                    p { style: "font-size: 0.85rem; color: var(--bx-muted); margin-top: 0.5rem;",
                        "Servai va automatiquement créer l'enregistrement DNS A sur la zone et émettre un certificat TLS Let's Encrypt via challenge DNS-01."
                    }
                    div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;",
                        button {
                            class: "bx-button",
                            onclick: move |_| bind_domain_target.set(None),
                            "Annuler"
                        }
                        button {
                            class: "bx-button",
                            onclick: {
                                let target_id = target.id.clone();
                                move |_| {
                                    let dom = domain_input();
                                    servers
                                        .with_mut(|list| {
                                            if let Some(s) = list.iter_mut().find(|s| s.id == target_id) {
                                                s.bound_domain = Some(dom.clone());
                                            }
                                        });
                                    toast_message
                                        .set(
                                            Some(
                                                format!(
                                                    "Domaine {} lié avec succès ! Certificat TLS délivré.",
                                                    dom,
                                                ),
                                            ),
                                        );
                                    bind_domain_target.set(None);
                                }
                            },
                            "Délivrer le certificat et lier"
                        }
                    }
                }
            }

            // Modal Actions de service
            if let Some(server) = selected_server() {
                Dialog {
                    title: "Gestion des Services Systemd - {server.name}",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| selected_server.set(None),
                    p { "Piloter les services d'administration sur le serveur :" }
                    div { style: "display: flex; flex-direction: column; gap: 0.75rem; margin-top: 1rem;",
                        for svc in ["ss-sensor", "wireguard", "podman", "caddy"] {
                            div {
                                key: "{svc}",
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 0.5rem; background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 4px;",
                                span { style: "font-weight: 500;", "{svc}.service" }
                                div { style: "display: flex; gap: 0.25rem;",
                                    button {
                                        class: "bx-button",
                                        onclick: {
                                            let s_name = server.name.clone();
                                            move |_| {
                                                toast_message
                                                    .set(
                                                        Some(
                                                            format!("Service {} redémarré sur {}", svc, s_name),
                                                        ),
                                                    );
                                            }
                                        },
                                        "Restart"
                                    }
                                }
                            }
                        }
                    }
                    div { style: "display: flex; justify-content: flex-end; margin-top: 1rem;",
                        button {
                            class: "bx-button",
                            onclick: move |_| selected_server.set(None),
                            "Fermer"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_servers_list_and_bind_domain_button() {
        let mut dom = VirtualDom::new(ServersView);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Serveurs et Nœuds Gérés"));
        assert!(html.contains("gibs-main-node"));
        assert!(html.contains("Lier un Domaine (Auto-SSL)"));
    }
}
