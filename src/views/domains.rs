use bioxus::widgets::{Badge, Dialog, InputField, Section, Tone};
use dioxus::prelude::*;

use crate::model::{mock_dns_records, mock_domains, DnsRecordItem, DomainItem};

#[component]
pub fn DomainsView() -> Element {
    let mut domains = use_signal(mock_domains);
    let mut dns_records = use_signal(mock_dns_records);
    let mut search_query = use_signal(|| "bioxus-platform.org".to_string());
    let mut check_result = use_signal(|| None::<(String, bool)>);
    let mut new_record_modal = use_signal(|| false);
    let mut new_rec_name = use_signal(String::new);
    let mut new_rec_type = use_signal(|| "A".to_string());
    let mut new_rec_value = use_signal(String::new);
    let mut toast_message = use_signal(|| None::<String>);

    rsx! {
        div { class: "domains-view",
            Section { title: "Registrar : Recherche & Enregistrement de Domaines",
                if let Some(msg) = toast_message() {
                    div {
                        style: "background: var(--bx-color-ok, #22c55e); color: white; padding: 0.75rem 1rem; border-radius: 6px; margin-bottom: 1rem;",
                        "✓ {msg}"
                    }
                }

                div { style: "display: flex; gap: 0.75rem; align-items: flex-end; flex-wrap: wrap;",
                    div { style: "flex: 1; min-width: 240px;",
                        InputField {
                            label: "Vérifier la disponibilité d'un nom de domaine",
                            value: search_query(),
                            on_input: move |v| search_query.set(v),
                        }
                    }
                    button {
                        class: "bx-button",
                        onclick: move |_| {
                            let name = search_query();
                            check_result.set(Some((name, true)));
                        },
                        "🔍 Vérifier Disponibilité"
                    }
                }

                if let Some((name, available)) = check_result() {
                    div {
                        style: "margin-top: 1rem; padding: 1rem; background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 6px; display: flex; justify-content: space-between; align-items: center;",
                        div {
                            strong { "{name} " }
                            if available {
                                Badge { tone: Tone::Ok, "Disponible (12.50 € / an)" }
                            } else {
                                Badge { tone: Tone::Danger, "Déjà Réservé" }
                            }
                        }
                        if available {
                            button {
                                class: "bx-button",
                                onclick: {
                                    let n = name.clone();
                                    move |_| {
                                        domains
                                            .with_mut(|list| {
                                                list.push(DomainItem {
                                                    name: n.clone(),
                                                    registrar: "Cloudflare".to_string(),
                                                    status: "Active".to_string(),
                                                    auto_renew: true,
                                                    nameservers: vec![
                                                        "ns1.cloudflare.com".to_string(),
                                                        "ns2.cloudflare.com".to_string(),
                                                    ],
                                                    expires_in_days: 365,
                                                });
                                            });
                                        toast_message
                                            .set(Some(format!("Domaine {} enregistré avec succès !", n)));
                                        check_result.set(None);
                                    }
                                },
                                "💳 Enregistrer le domaine"
                            }
                        }
                    }
                }
            }

            Section { title: "Noms de Domaine Actifs",
                div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                    for dom in domains().iter() {
                        div {
                            key: "{dom.name}",
                            style: "background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 6px; padding: 1rem; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem;",
                            div {
                                h4 { style: "margin: 0; font-size: 1rem;", "{dom.name}" }
                                span { style: "font-size: 0.85rem; color: var(--bx-muted);",
                                    "Registrar: {dom.registrar} • Expire dans {dom.expires_in_days} jours"
                                }
                            }
                            div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                if dom.auto_renew {
                                    Badge { tone: Tone::Ok, "Auto-Renew On" }
                                } else {
                                    Badge { tone: Tone::Warn, "Auto-Renew Off" }
                                }
                                Badge { tone: Tone::Ok, "{dom.status}" }
                            }
                        }
                    }
                }
            }

            Section { title: "Zone DNS & Enregistrements (bioxus.cloud)",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;",
                    span { style: "color: var(--bx-muted); font-size: 0.9rem;",
                        "DNSSEC : Activé • Propagation globale : 100%"
                    }
                    button {
                        class: "bx-button",
                        onclick: move |_| new_record_modal.set(true),
                        "+ Ajouter un Enregistrement DNS"
                    }
                }

                div { style: "overflow-x: auto;",
                    table { style: "width: 100%; border-collapse: collapse; text-align: left; font-size: 0.9rem;",
                        thead {
                            tr { style: "border-bottom: 2px solid var(--bx-border); color: var(--bx-muted);",
                                th { style: "padding: 0.5rem;", "Type" }
                                th { style: "padding: 0.5rem;", "Nom" }
                                th { style: "padding: 0.5rem;", "Valeur" }
                                th { style: "padding: 0.5rem;", "TTL" }
                                th { style: "padding: 0.5rem;", "Propagation" }
                                th { style: "padding: 0.5rem; text-align: right;", "Action" }
                            }
                        }
                        tbody {
                            for rec in dns_records().iter() {
                                tr {
                                    key: "{rec.id}",
                                    style: "border-bottom: 1px solid var(--bx-border);",
                                    td { style: "padding: 0.5rem; font-weight: bold;", "{rec.record_type}" }
                                    td { style: "padding: 0.5rem; font-family: monospace;", "{rec.name}" }
                                    td { style: "padding: 0.5rem; font-family: monospace;", "{rec.value}" }
                                    td { style: "padding: 0.5rem;", "{rec.ttl}s" }
                                    td { style: "padding: 0.5rem;",
                                        if rec.propagated {
                                            Badge { tone: Tone::Ok, "Propagé" }
                                        } else {
                                            Badge { tone: Tone::Warn, "En attente" }
                                        }
                                    }
                                    td { style: "padding: 0.5rem; text-align: right;",
                                        button {
                                            class: "bx-button",
                                            onclick: {
                                                let id = rec.id.clone();
                                                move |_| {
                                                    dns_records.with_mut(|list| list.retain(|r| r.id != id));
                                                    toast_message
                                                        .set(Some("Enregistrement DNS supprimé".to_string()));
                                                }
                                            },
                                            "Supprimer"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if new_record_modal() {
                Dialog {
                    title: "Ajouter un Enregistrement DNS",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| new_record_modal.set(false),
                    InputField {
                        label: "Nom d'hôte (ex: api, @, vpn)",
                        value: new_rec_name(),
                        on_input: move |v| new_rec_name.set(v),
                    }
                    div { style: "margin-top: 0.75rem;",
                        label { style: "font-size: 0.85rem; font-weight: 500;", "Type d'enregistrement" }
                        select {
                            style: "width: 100%; padding: 0.5rem; border: 1px solid var(--bx-border); border-radius: 4px; background: var(--bx-surface); margin-top: 0.25rem;",
                            value: new_rec_type(),
                            onchange: move |evt| new_rec_type.set(evt.value()),
                            option { value: "A", "A (IPv4)" }
                            option { value: "AAAA", "AAAA (IPv6)" }
                            option { value: "CNAME", "CNAME (Alias)" }
                            option { value: "TXT", "TXT (Validation / SPF)" }
                            option { value: "MX", "MX (Serveur Mail)" }
                        }
                    }
                    div { style: "margin-top: 0.75rem;",
                        InputField {
                            label: "Valeur cible (ex: 192.168.1.100 ou cible.domaine.com)",
                            value: new_rec_value(),
                            on_input: move |v| new_rec_value.set(v),
                        }
                    }
                    div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;",
                        button {
                            class: "bx-button",
                            onclick: move |_| new_record_modal.set(false),
                            "Annuler"
                        }
                        button {
                            class: "bx-button",
                            onclick: move |_| {
                                let id = format!("rec-{}", dns_records().len() + 1);
                                dns_records
                                    .with_mut(|list| {
                                        list.push(DnsRecordItem {
                                            id,
                                            name: new_rec_name(),
                                            record_type: new_rec_type(),
                                            value: new_rec_value(),
                                            ttl: 300,
                                            propagated: true,
                                        });
                                    });
                                toast_message.set(Some("Enregistrement DNS créé et propagé !".to_string()));
                                new_record_modal.set(false);
                            },
                            "Enregistrer"
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
    fn renders_domains_and_dns_records() {
        let mut dom = VirtualDom::new(DomainsView);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Registrar"));
        assert!(html.contains("bioxus.cloud"));
        assert!(html.contains("Ajouter un Enregistrement DNS"));
    }
}
