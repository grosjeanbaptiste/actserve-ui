use bioxus::widgets::{AppShell, NavItem};
use dioxus::prelude::*;

use crate::views::{DashboardView, DomainsView, EcosystemView, ServersView, WireGuardView};

#[component]
pub fn App() -> Element {
    let mut active = use_signal(|| "dashboard".to_string());

    let items = vec![
        NavItem::new("dashboard", "Tableau de Bord").with_icon("📊"),
        NavItem::new("servers", "Serveurs").with_icon("🖥️"),
        NavItem::new("domains", "Domaines & DNS").with_icon("🌐"),
        NavItem::new("wireguard", "WireGuard VPN").with_icon("🔒"),
        NavItem::new("ecosystem", "Dépôts & Presets").with_icon("📦"),
    ];

    rsx! {
        style { {bioxus::design::STYLESHEET} }
        style { {bioxus::widgets::STYLESHEET} }
        style { {bioxus::charts::STYLESHEET} }

        div { "data-theme": "ocean", style: "min-height: 100vh; background: var(--bx-bg); color: var(--bx-fg);",
            AppShell {
                items,
                active: active(),
                nav_label: "Servai",
                title: "Servai • Bioxus UI",
                on_select: move |id| active.set(id),

                match active().as_str() {
                    "servers" => rsx! {
                        ServersView {}
                    },
                    "domains" => rsx! {
                        DomainsView {}
                    },
                    "wireguard" => rsx! {
                        WireGuardView {}
                    },
                    "ecosystem" => rsx! {
                        EcosystemView {}
                    },
                    _ => rsx! {
                        DashboardView {
                            on_navigate: move |target| active.set(target),
                        }
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_servai_bioxus_shell() {
        let mut dom = VirtualDom::new(App);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("bx-shell"));
        assert!(html.contains("Tableau de Bord"));
        assert!(html.contains("Serveurs"));
        assert!(html.contains("WireGuard VPN"));
        assert!(html.contains("Domaines"));
    }
}
