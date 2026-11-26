use biorcrux::widgets::{Badge, Dialog, InputField, Section, Tone};
use dioxus::prelude::*;

use crate::model::{
    mock_job_logs, mock_reconciliations, mock_schedules, mock_workflows,
    ScheduleItem, WorkflowItem,
};

#[component]
pub fn OrchestrationsView() -> Element {
    let mut active_subtab = use_signal(|| "workflows".to_string());
    let workflows = use_signal(mock_workflows);
    let mut schedules = use_signal(mock_schedules);
    let reconciliations = use_signal(mock_reconciliations);
    let job_logs = use_signal(mock_job_logs);

    let mut toast_message = use_signal(|| None::<String>);
    let mut executing_wf = use_signal(|| None::<WorkflowItem>);
    let mut new_schedule_dialog = use_signal(|| false);
    let mut new_sch_name = use_signal(String::new);
    let mut new_sch_cron = use_signal(|| "0 3 * * *".to_string());
    let mut new_sch_tag = use_signal(|| "prod".to_string());

    let mut fleet_tag = use_signal(|| "prod".to_string());
    let mut fleet_strategy = use_signal(|| "Rolling".to_string());
    let mut fleet_command = use_signal(|| "apt-get update && apt-get upgrade -y".to_string());

    rsx! {
        div { class: "orchestrations-view", style: "display: flex; flex-direction: column; gap: 1.5rem;",
            // En-tête & Sous-navigation
            div { style: "display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;",
                div {
                    h2 { style: "margin: 0; font-size: 1.5rem;", "⚡ Moteur d'Orchestration Avancée" }
                    p { style: "margin: 0.25rem 0 0; color: var(--bx-muted, #6b7280); font-size: 0.9rem;",
                        "Pipelines déclaratifs, déploiements en flotte, automations récurrentes et auto-healing"
                    }
                }
                div { style: "display: flex; gap: 0.5rem; flex-wrap: wrap; background: var(--bx-surface, #f3f4f6); padding: 0.25rem; border-radius: 8px;",
                    button {
                        class: if active_subtab() == "workflows" { "bx-button bx-button-primary" } else { "bx-button" },
                        onclick: move |_| active_subtab.set("workflows".to_string()),
                        "📋 Workflows"
                    }
                    button {
                        class: if active_subtab() == "fleet" { "bx-button bx-button-primary" } else { "bx-button" },
                        onclick: move |_| active_subtab.set("fleet".to_string()),
                        "🚀 Flotte Multi-Nœuds"
                    }
                    button {
                        class: if active_subtab() == "schedules" { "bx-button bx-button-primary" } else { "bx-button" },
                        onclick: move |_| active_subtab.set("schedules".to_string()),
                        "⏱️ Tâches Programmées"
                    }
                    button {
                        class: if active_subtab() == "reconcile" { "bx-button bx-button-primary" } else { "bx-button" },
                        onclick: move |_| active_subtab.set("reconcile".to_string()),
                        "🛡️ Auto-Healing"
                    }
                    button {
                        class: if active_subtab() == "logs" { "bx-button bx-button-primary" } else { "bx-button" },
                        onclick: move |_| active_subtab.set("logs".to_string()),
                        "📜 Observabilité & Logs"
                    }
                }
            }

            // Toast de notification
            if let Some(msg) = toast_message() {
                div {
                    style: "background: var(--bx-color-ok, #22c55e); color: white; padding: 0.75rem 1rem; border-radius: 6px; display: flex; justify-content: space-between; align-items: center;",
                    span { "✓ {msg}" }
                    button {
                        style: "background: transparent; border: none; color: white; cursor: pointer; font-weight: bold;",
                        onclick: move |_| toast_message.set(None),
                        "✕"
                    }
                }
            }

            // Onglet 1 : Pipelines de Workflows
            if active_subtab() == "workflows" {
                Section { title: "Pipelines de Workflows Déclaratifs",
                    div { style: "display: flex; flex-direction: column; gap: 1rem;",
                        for wf in workflows().iter() {
                            div {
                                key: "{wf.id}",
                                style: "background: var(--bx-surface, #ffffff); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 8px; padding: 1.25rem;",
                                div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.75rem;",
                                    div {
                                        h3 { style: "margin: 0; font-size: 1.15rem;", "{wf.name}" }
                                        p { style: "margin: 0.25rem 0 0; color: var(--bx-muted, #6b7280); font-size: 0.85rem;", "{wf.description}" }
                                    }
                                    Badge { tone: Tone::Ok, "ID: {wf.id}" }
                                }

                                h4 { style: "margin: 0.75rem 0 0.5rem; font-size: 0.9rem; color: var(--bx-muted, #4b5563);", "Étapes du Pipeline :" }
                                div { style: "display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;",
                                    for (idx, step) in wf.steps.iter().enumerate() {
                                        div {
                                            key: "{idx}",
                                            style: "background: var(--bx-bg, #f9fafb); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 6px; padding: 0.6rem 0.8rem; display: flex; align-items: center; justify-content: space-between; font-size: 0.85rem;",
                                            div { style: "display: flex; align-items: center; gap: 0.6rem;",
                                                span { style: "background: var(--bx-primary, #3b82f6); color: white; border-radius: 50%; width: 22px; height: 22px; display: inline-flex; align-items: center; justify-content: center; font-size: 0.75rem; font-weight: bold;",
                                                    "{idx + 1}"
                                                }
                                                strong { "{step.name}" }
                                                span { style: "color: var(--bx-muted, #6b7280); font-family: monospace;", "({step.details})" }
                                            }
                                            Badge { tone: Tone::Neutral, "{step.action_type}" }
                                        }
                                    }
                                }

                                div { style: "display: flex; justify-content: flex-end; gap: 0.5rem;",
                                    button {
                                        class: "bx-button bx-button-primary",
                                        onclick: {
                                            let wf_clone = wf.clone();
                                            move |_| executing_wf.set(Some(wf_clone.clone()))
                                        },
                                        "▶ Lancer l'exécution..."
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Onglet 2 : Flotte Multi-Serveurs
            if active_subtab() == "fleet" {
                Section { title: "Déploiement en Flotte (Multi-Server Rollout)",
                    div { style: "background: var(--bx-surface, #ffffff); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 8px; padding: 1.25rem; display: flex; flex-direction: column; gap: 1rem;",
                        p { style: "margin: 0; color: var(--bx-muted, #6b7280); font-size: 0.9rem;",
                            "Orchestrez des opérations groupées à grande échelle avec ciblage par étiquette (tag) et stratégie d'arrêt préventif en cas d'erreur."
                        }

                        div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 1rem;",
                            div {
                                InputField {
                                    label: "Tag Serveurs Cible",
                                    value: fleet_tag(),
                                    on_input: move |v| fleet_tag.set(v),
                                }
                            }
                            div {
                                label { style: "display: block; font-weight: 600; margin-bottom: 0.35rem; font-size: 0.85rem;", "Stratégie de Déploiement" }
                                select {
                                    style: "width: 100%; padding: 0.5rem; border: 1px solid var(--bx-border, #d1d5db); border-radius: 6px; background: white;",
                                    value: fleet_strategy(),
                                    onchange: move |e| fleet_strategy.set(e.value()),
                                    option { value: "Rolling", "Rolling (Progressif un à un, stop sur erreur)" }
                                    option { value: "Parallel", "Parallel (Simultané sur toute la flotte)" }
                                }
                            }
                        }

                        div {
                            InputField {
                                label: "Commande Administrative à Diffuser",
                                value: fleet_command(),
                                on_input: move |v| fleet_command.set(v),
                            }
                        }

                        div { style: "display: flex; justify-content: flex-end;",
                            button {
                                class: "bx-button bx-button-primary",
                                onclick: move |_| {
                                    toast_message.set(Some(format!("Rollout flotte démarré : {} avec stratégie {}", fleet_command(), fleet_strategy())));
                                },
                                "🚀 Déclencher le Rollout Flotte"
                            }
                        }
                    }
                }
            }

            // Onglet 3 : Tâches Programmées (Schedules)
            if active_subtab() == "schedules" {
                Section { title: "Automations & Tâches Programmées (Cron)",
                    div { style: "display: flex; justify-content: flex-end; margin-bottom: 1rem;",
                        button {
                            class: "bx-button bx-button-primary",
                            onclick: move |_| new_schedule_dialog.set(true),
                            "+ Planifier une Nouvelle Tâche"
                        }
                    }

                    div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                        for sch in schedules().iter() {
                            div {
                                key: "{sch.id}",
                                style: "background: var(--bx-surface, #ffffff); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 8px; padding: 1rem; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.75rem;",
                                div {
                                    h4 { style: "margin: 0; font-size: 1rem;", "{sch.name}" }
                                    div { style: "display: flex; gap: 0.75rem; align-items: center; margin-top: 0.25rem; font-size: 0.85rem; color: var(--bx-muted, #6b7280);",
                                        span { style: "font-family: monospace; background: var(--bx-bg, #f3f4f6); padding: 0.15rem 0.4rem; border-radius: 4px;", "cron: {sch.cron_expression}" }
                                        span { "Cible: #{sch.target_tag}" }
                                        if let Some(last) = &sch.last_run {
                                            span { "Dernière exéc: {last}" }
                                        }
                                    }
                                }
                                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                                    Badge { tone: if sch.enabled { Tone::Ok } else { Tone::Warn }, if sch.enabled { "Actif" } else { "Désactivé" } }
                                    button {
                                        class: "bx-button",
                                        onclick: {
                                            let name = sch.name.clone();
                                            move |_| toast_message.set(Some(format!("Tâche '{}' exécutée avec succès !", name)))
                                        },
                                        "▶ Lancer maintenant"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Onglet 4 : Auto-Healing & Réconciliation Déclarative
            if active_subtab() == "reconcile" {
                Section { title: "Réconciliation d'État Déclaratif (Auto-Healing)",
                    p { style: "color: var(--bx-muted, #6b7280); font-size: 0.9rem; margin-top: 0;",
                        "Compare l'état cible (paquets déclarés, domaine SSL lié, interface WireGuard) à l'état réel et répare automatiquement les dérives."
                    }

                    div { style: "display: flex; flex-direction: column; gap: 1rem;",
                        for item in reconciliations().iter() {
                            div {
                                key: "{item.server_id}",
                                style: "background: var(--bx-surface, #ffffff); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 8px; padding: 1.25rem; display: flex; flex-direction: column; gap: 0.75rem;",
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    div {
                                        h4 { style: "margin: 0; font-size: 1.1rem;", "{item.server_name}" }
                                        span { style: "color: var(--bx-muted, #6b7280); font-family: monospace; font-size: 0.85rem;", "{item.server_id}" }
                                    }
                                    Badge {
                                        tone: match item.status.as_str() {
                                            "InSync" => Tone::Ok,
                                            "Drifted" => Tone::Danger,
                                            _ => Tone::Warn,
                                        },
                                        "{item.status}"
                                    }
                                }

                                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 0.75rem; font-size: 0.85rem;",
                                    div {
                                        strong { "Paquets Requis :" }
                                        div { style: "color: var(--bx-muted, #4b5563); margin-top: 0.2rem;",
                                            "{item.required_packages.join(\", \")}"
                                        }
                                    }
                                    div {
                                        strong { "Paquets Manquants :" }
                                        div { style: "color: var(--bx-color-danger, #ef4444); margin-top: 0.2rem;",
                                            if item.missing_packages.is_empty() { "Aucun (conforme)" } else { "{item.missing_packages.join(\", \")}" }
                                        }
                                    }
                                    div {
                                        strong { "Domaine & Auto-TLS :" }
                                        div { style: "margin-top: 0.2rem;", if item.domain_bound { "✓ Configuré" } else { "⚠ Manquant" } }
                                    }
                                    div {
                                        strong { "Tunnel WireGuard :" }
                                        div { style: "margin-top: 0.2rem;", if item.wireguard_active { "✓ Actif" } else { "⚠ Inactif" } }
                                    }
                                }

                                if item.status == "Drifted" {
                                    div { style: "display: flex; justify-content: flex-end; margin-top: 0.5rem;",
                                        button {
                                            class: "bx-button bx-button-primary",
                                            onclick: {
                                                let srv = item.server_name.clone();
                                                move |_| toast_message.set(Some(format!("Job de remédiation et d'auto-healing déclenché pour {} !", srv)))
                                            },
                                            "🛡️ Auto-Guérir (Réconcilier l'état)"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Onglet 5 : Observabilité & Console de Logs Détaillés
            if active_subtab() == "logs" {
                Section { title: "Observabilité & Logs d'Exécution par Étape",
                    div { style: "display: flex; flex-direction: column; gap: 1.25rem;",
                        for job in job_logs().iter() {
                            div {
                                key: "{job.job_id}",
                                style: "background: var(--bx-surface, #ffffff); border: 1px solid var(--bx-border, #e5e7eb); border-radius: 8px; padding: 1.25rem;",
                                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;",
                                    div {
                                        h3 { style: "margin: 0; font-size: 1.1rem;", "{job.workflow_name}" }
                                        span { style: "color: var(--bx-muted, #6b7280); font-size: 0.85rem;", "Serveur: {job.server_name} • Durée totale: {job.total_duration_ms}ms" }
                                    }
                                    Badge { tone: Tone::Ok, "{job.status}" }
                                }

                                div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                                    for step in job.steps.iter() {
                                        div {
                                            key: "{step.step_index}",
                                            style: "border: 1px solid var(--bx-border, #e5e7eb); border-radius: 6px; overflow: hidden;",
                                            div { style: "background: var(--bx-bg, #f9fafb); padding: 0.5rem 0.75rem; display: flex; justify-content: space-between; align-items: center; font-size: 0.85rem;",
                                                span {
                                                    strong { "Étape {step.step_index + 1} : {step.step_name}" }
                                                    span { style: "color: var(--bx-muted, #6b7280); margin-left: 0.5rem;", "({step.duration_ms}ms)" }
                                                }
                                                Badge { tone: Tone::Ok, "{step.status}" }
                                            }
                                            if !step.stdout.is_empty() {
                                                pre { style: "margin: 0; padding: 0.75rem; background: #1e1e1e; color: #4ade80; font-family: monospace; font-size: 0.8rem; overflow-x: auto; white-space: pre-wrap;",
                                                    "{step.stdout}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Dialogue Exécution de Workflow
            if let Some(wf) = executing_wf() {
                Dialog {
                    title: "Exécuter le Workflow",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| executing_wf.set(None),
                    div { style: "display: flex; flex-direction: column; gap: 1rem; padding: 0.5rem 0;",
                        p { "Vous êtes sur le point de déclencher le pipeline :" }
                        p { strong { "{wf.name}" } }
                        p { style: "color: var(--bx-muted, #6b7280); font-size: 0.85rem;",
                            "{wf.steps.len()} étapes seront exécutées sur le serveur cible."
                        }
                        div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;",
                            button {
                                class: "bx-button",
                                onclick: move |_| executing_wf.set(None),
                                "Annuler"
                            }
                            button {
                                class: "bx-button bx-button-primary",
                                onclick: {
                                    let wf_name = wf.name.clone();
                                    move |_| {
                                        executing_wf.set(None);
                                        toast_message.set(Some(format!("Workflow '{}' exécuté avec succès !", wf_name)));
                                    }
                                },
                                "Confirmer & Exécuter"
                            }
                        }
                    }
                }
            }

            // Dialogue Nouvelle Tâche Programmée
            if new_schedule_dialog() {
                Dialog {
                    title: "Programmer une Tâche (Cron)",
                    close_label: "Fermer",
                    open: true,
                    on_close: move |_| new_schedule_dialog.set(false),
                    div { style: "display: flex; flex-direction: column; gap: 1rem; padding: 0.5rem 0;",
                        div {
                            InputField {
                                label: "Nom de la Tâche",
                                value: new_sch_name(),
                                on_input: move |v| new_sch_name.set(v),
                            }
                        }
                        div {
                            InputField {
                                label: "Expression Cron (5 champs)",
                                value: new_sch_cron(),
                                on_input: move |v| new_sch_cron.set(v),
                            }
                        }
                        div {
                            InputField {
                                label: "Tag Serveur Cible",
                                value: new_sch_tag(),
                                on_input: move |v| new_sch_tag.set(v),
                            }
                        }
                        div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;",
                            button {
                                class: "bx-button",
                                onclick: move |_| new_schedule_dialog.set(false),
                                "Annuler"
                            }
                            button {
                                class: "bx-button bx-button-primary",
                                onclick: move |_| {
                                    let name = if new_sch_name().trim().is_empty() { "Tâche Personnalisée".to_string() } else { new_sch_name() };
                                    let mut current = schedules();
                                    current.push(ScheduleItem {
                                        id: format!("sch-0{}", current.len() + 1),
                                        name: name.clone(),
                                        cron_expression: new_sch_cron(),
                                        workflow_id: "wf-prod-deploy".to_string(),
                                        target_tag: new_sch_tag(),
                                        enabled: true,
                                        last_run: None,
                                    });
                                    schedules.set(current);
                                    new_schedule_dialog.set(false);
                                    toast_message.set(Some(format!("Tâche programmée '{}' enregistrée !", name)));
                                },
                                "Enregistrer la Tâche"
                            }
                        }
                    }
                }
            }
        }
    }
}
