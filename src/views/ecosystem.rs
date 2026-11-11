use bioxus::widgets::{Badge, Dialog, Section, Tone};
use dioxus::prelude::*;

use crate::model::{mock_presets, mock_servers, PresetItem};

#[component]
pub fn EcosystemView() -> Element {
    let presets = mock_presets();
    let servers = mock_servers();
    let mut selected_preset = use_signal(|| None::<PresetItem>);
    let mut target_server_id = use_signal(|| "srv-01-prod".to_string());
    let mut toast_message = use_signal(|| None::<String>);

    rsx! {
        div { class: "ecosystem-view",
            Section { title: "Profils & Presets Multi-Dépôts (1-Clic)",
                if let Some(msg) = toast_message() {
                    div {
                        style: "background: var(--bx-color-ok, #22c55e); color: white; padding: 0.75rem 1rem; border-radius: 6px; margin-bottom: 1rem;",
                        "✓ {msg}"
                    }
                }

                p { style: "color: var(--bx-muted); margin-bottom: 1rem;",
                    "Déployez des configurations d'administration complètes combinant vos dépôts Git, Ansible Galaxy, LinuxServer.io et vos dotfiles."
                }

                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1rem;",
                    for preset in presets.iter() {
                        div {
                            key: "{preset.id}",
                            style: "background: var(--bx-surface); border: 1px solid var(--bx-border); border-radius: 8px; padding: 1.25rem; display: flex; flex-direction: column; justify-content: space-between;",
                            div {
                                div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;",
                                    h3 { style: "margin: 0; font-size: 1.1rem;", "{preset.name}" }
                                    Badge { tone: Tone::Ok, "{preset.category}" }
                                }
                                p { style: "font-size: 0.9rem; color: var(--bx-muted); line-height: 1.4;",
                                    "{preset.description}"
                                }
                                div { style: "margin-top: 0.75rem;",
                                    span { style: "font-size: 0.8rem; font-weight: bold; color: var(--bx-muted);",
                                        "Dépôts inclus :"
                                    }
                                    div { style: "display: flex; flex-wrap: wrap; gap: 0.35rem; margin-top: 0.35rem;",
                                        for pkg in preset.packages.iter() {
                                            span {
                                                key: "{pkg}",
                                                style: "background: var(--bx-border); font-size: 0.75rem; font-family: monospace; padding: 0.15rem 0.4rem; border-radius: 4px;",
                                                "{pkg}"
                                            }
                                        }
                                    }
                                }
                            }
                            div { style: "margin-top: 1.25rem;",
                                button {
                                    class: "bx-button",
                                    style: "width: 100%;",
                                    onclick: {
                                        let p = preset.clone();
                                        move |_| selected_preset.set(Some(p.clone()))
                                    },
                                    "⚡ Déployer le Preset"
                                }
                            }
                        }
                    }
                }
            }

            if let Some(preset) = selected_preset() {
                Dialog {
                    title: "Déployer : {preset.name}",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| selected_preset.set(None),
                    p { "Sélectionnez le serveur cible pour exécuter le déploiement du profil :" }
                    div { style: "margin-top: 1rem;",
                        label { style: "font-size: 0.85rem; font-weight: 500;", "Serveur Cible" }
                        select {
                            style: "width: 100%; padding: 0.5rem; border: 1px solid var(--bx-border); border-radius: 4px; background: var(--bx-surface); margin-top: 0.25rem;",
                            value: target_server_id(),
                            onchange: move |evt| target_server_id.set(evt.value()),
                            for s in servers.iter() {
                                option { key: "{s.id}", value: "{s.id}", "{s.name} ({s.ip})" }
                            }
                        }
                    }
                    p { style: "font-size: 0.85rem; color: var(--bx-muted); margin-top: 1rem;",
                        "Servai va résoudre les dépendances, cloner les dépôts (incluant les dotfiles et le capteur ss), exécuter les playbooks et vérifier le bon démarrage de tous les services."
                    }
                    div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1.5rem;",
                        button {
                            class: "bx-button",
                            onclick: move |_| selected_preset.set(None),
                            "Annuler"
                        }
                        button {
                            class: "bx-button",
                            onclick: {
                                let p_name = preset.name.clone();
                                move |_| {
                                    toast_message
                                        .set(
                                            Some(
                                                format!(
                                                    "Déploiement du profil '{}' lancé avec succès sur le serveur !",
                                                    p_name,
                                                ),
                                            ),
                                        );
                                    selected_preset.set(None);
                                }
                            },
                            "Lancer le Déploiement"
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
    fn renders_ecosystem_presets_including_gibs_node() {
        let mut dom = VirtualDom::new(EcosystemView);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("GIBS Developer Node"));
        assert!(html.contains("dotfiles-stow"));
        assert!(html.contains("ss-sensor"));
        assert!(html.contains("Déployer le Preset"));
    }
}
