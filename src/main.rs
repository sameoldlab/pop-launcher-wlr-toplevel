use onagre_launcher_toolkit::{
    launcher::{IconSource, Indice, PluginResponse, PluginSearchResult},
    plugin_trait::{
        async_trait,
        tracing::{error, info},
        PluginExt,
    },
};
use serde::Deserialize;
use std::{borrow::Cow, process::Command};

#[derive(Deserialize, Debug, PartialEq)]
struct ToplevelEntry {
    app_id: String,
    title: String,
}

#[derive(Default)]
struct ToplevelPlugin {
    items: Vec<ToplevelEntry>,
}

#[async_trait]
impl PluginExt for ToplevelPlugin {
    fn name(&self) -> &str {
        "toplevel"
    }

    async fn search(&mut self, query: &str) {
        let query = query.to_ascii_lowercase();

        let toplevels = get_toplevel();
        let filtered = filter_toplevels(toplevels, &query);
        self.items = filtered;
        //TODO: Get icon for from app_id
        let icon = Cow::Borrowed("application-x-executable");
        for (idx, item) in self.items.iter().enumerate() {
            self.respond_with(PluginResponse::Append(PluginSearchResult {
                id: idx as u32,
                name: item.app_id.clone(),
                description: item.title.clone(),
                icon: Some(IconSource::Mime(icon.clone())),
                ..Default::default()
            }))
            .await
        }

        self.respond_with(PluginResponse::Finished).await
    }

    async fn activate(&mut self, id: Indice) {
        match self.items.get(id as usize) {
            Some(toplvl) => {
                let status = Command::new("sh")
                    .arg("-c")
                    .arg(format!("wlrctl toplevel focus title:'{}'", &toplvl.title))
                    .status()
                    .unwrap();

                if status.success() {
                    info!("succesfully launched {}", &toplvl.title)
                } else {
                    error!("failed to launch '{}'", &toplvl.title)
                }
            }
            None => error!("Failed to get post at index {id}"),
        }

        self.respond_with(PluginResponse::Close).await;
    }
}

fn get_toplevel() -> Vec<ToplevelEntry> {
    let cmd = Command::new("wlrctl")
        .args(["toplevel", "list"])
        .output()
        .expect("failed to execute process");

    let output = if cmd.status.success() {
        String::from_utf8_lossy(&cmd.stdout).into_owned()
    } else {
        let err = String::from_utf8_lossy(&cmd.stderr);
        error!("{err}");
        String::new()
    };

    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                Some(ToplevelEntry {
                    app_id: parts[0].to_string(),
                    title: parts[1].to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn filter_toplevels(toplevels: Vec<ToplevelEntry>, filter: &str) -> Vec<ToplevelEntry> {
    toplevels
        .into_iter()
        .filter(|tle| {
            tle.app_id.to_ascii_lowercase().contains(filter)
                || tle.title.to_ascii_lowercase().contains(filter)
        })
        .collect()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    ToplevelPlugin::default().run().await;
}
