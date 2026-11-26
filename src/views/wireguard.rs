use biorcrux::widgets::{Badge, Dialog, InputField, Section, Stat, Tone};
use dioxus::prelude::*;

use crate::model::{mock_wireguard_peers, WireGuardPeerItem};

#[component]
pub fn WireGuardView() -> Element {
    let mut peers = use_signal(mock_wireguard_peers);
    let mut add_peer_modal = use_signal(|| false);
    let mut peer_pubkey = use_signal(String::new);
    let mut peer_ips = use_signal(|| "10.0.0.4/32".to_string());
    let mut generated_keys = use_signal(|| None::<(String, String)>);
    let mut toast_message = use_signal(|| None::<String>);

    rsx! {
        div { class: "wireguard-view",
            Section { title: "Interface VPN WireGuard (wg0)",
                if let Some(msg) = toast_message() {
                    div {
                        style: "background: var(--bx-color-ok, #22c55e); color: white; padding: 0.75rem 1rem; border-radius: 6px; margin-bottom: 1rem;",
                        "✓ {msg}"
                    }
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 1rem;",
                    Stat {
                        label: "Statut Interface",
                        value: "UP (Écoute sur 51820)",
                    }
                    Stat {
                        label: "Adresse Passerelle",
                        value: "10.0.0.1/24",
                    }
                    Stat {
                        label: "Trafic Réseau Total",
                        value: "1.12 Go échangés",
                    }
                    Stat {
                        label: "Pairs Autorisés",
                        value: "{peers().len()} actifs",
                    }
                }

                div {
                    style: "background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 6px; padding: 1rem; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem;",
                    div {
                        span { style: "color: var(--bx-muted); font-size: 0.85rem;", "Clé Publique du Serveur :" }
                        div { style: "font-family: monospace; font-weight: bold; margin-top: 0.25rem;",
                            "s8+ServaiWireGuardGatewayMainPublicKey="
                        }
                    }
                    div { style: "display: flex; gap: 0.5rem;",
                        button {
                            class: "bx-button",
                            onclick: move |_| {
                                generated_keys
                                    .set(
                                        Some((
                                            "e7+GeneratedPrivateKeyCurve25519Sec=".to_string(),
                                            "p1+GeneratedPublicKeyCurve25519Pub=".to_string(),
                                        )),
                                    );
                            },
                            "🔑 Générer Paire de Clés"
                        }
                        button {
                            class: "bx-button",
                            onclick: move |_| add_peer_modal.set(true),
                            "+ Ajouter un Pair VPN"
                        }
                    }
                }

                if let Some((priv_k, pub_k)) = generated_keys() {
                    div {
                        style: "margin-top: 1rem; padding: 1rem; background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 6px;",
                        h4 { style: "margin: 0 0 0.5rem 0;", "Nouvelle Paire Cryptographique Générée" }
                        div { style: "font-size: 0.85rem; font-family: monospace; color: var(--bx-muted);",
                            div { "Clé Privée : {priv_k}" }
                            div { "Clé Publique : {pub_k}" }
                        }
                    }
                }
            }

            Section { title: "Clients et Tunnels VPN Actifs",
                div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                    for peer in peers().iter() {
                        div {
                            key: "{peer.public_key}",
                            style: "background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 6px; padding: 1rem; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.75rem;",
                            div {
                                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                    span { style: "font-family: monospace; font-weight: bold;",
                                        "{peer.allowed_ips}"
                                    }
                                    Badge { tone: Tone::Ok, "Connecté" }
                                }
                                div { style: "font-family: monospace; font-size: 0.8rem; color: var(--bx-muted); margin-top: 0.25rem;",
                                    "Clé : {peer.public_key}"
                                }
                                div { style: "font-size: 0.8rem; color: var(--bx-muted); margin-top: 0.25rem;",
                                    "Endpoint : {peer.endpoint.as_deref().unwrap_or(\"N/A\")} • Handshake : {peer.last_handshake}"
                                }
                            }
                            div { style: "display: flex; align-items: center; gap: 1rem;",
                                div { style: "text-align: right; font-size: 0.85rem;",
                                    div { "Rx: {peer.rx_bytes / 1024 / 1024} Mo" }
                                    div { "Tx: {peer.tx_bytes / 1024 / 1024} Mo" }
                                }
                                button {
                                    class: "bx-button",
                                    onclick: {
                                        let key = peer.public_key.clone();
                                        move |_| {
                                            peers.with_mut(|list| list.retain(|p| p.public_key != key));
                                            toast_message.set(Some("Pair VPN révoqué".to_string()));
                                        }
                                    },
                                    "Révoquer"
                                }
                            }
                        }
                    }
                }
            }

            if add_peer_modal() {
                Dialog {
                    title: "Enregistrer un Pair VPN WireGuard",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| add_peer_modal.set(false),
                    InputField {
                        label: "Clé Publique du Client (Base64)",
                        value: peer_pubkey(),
                        on_input: move |v| peer_pubkey.set(v),
                    }
                    div { style: "margin-top: 0.75rem;",
                        InputField {
                            label: "IP interne autorisée (AllowedIPs)",
                            value: peer_ips(),
                            on_input: move |v| peer_ips.set(v),
                        }
                    }
                    div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;",
                        button {
                            class: "bx-button",
                            onclick: move |_| add_peer_modal.set(false),
                            "Annuler"
                        }
                        button {
                            class: "bx-button",
                            onclick: move |_| {
                                let key = if peer_pubkey().is_empty() {
                                    "client-auto-pubkey-base64=".to_string()
                                } else {
                                    peer_pubkey()
                                };
                                peers
                                    .with_mut(|list| {
                                        list.push(WireGuardPeerItem {
                                            public_key: key,
                                            allowed_ips: peer_ips(),
                                            endpoint: None,
                                            rx_bytes: 0,
                                            tx_bytes: 0,
                                            last_handshake: "En attente".to_string(),
                                        });
                                    });
                                toast_message.set(Some("Pair WireGuard configuré avec succès !".to_string()));
                                add_peer_modal.set(false);
                            },
                            "Ajouter le Pair"
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
    fn renders_wireguard_interface_and_peers() {
        let mut dom = VirtualDom::new(WireGuardView);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Interface VPN WireGuard (wg0)"));
        assert!(html.contains("10.0.0.1/24"));
        assert!(html.contains("Ajouter un Pair VPN"));
    }
}
