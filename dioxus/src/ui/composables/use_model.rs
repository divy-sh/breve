use crate::core::models::{controller as models_ctrl, models::Model};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub struct UseModel {
    pub available: Signal<Vec<(String, Model)>>,
    pub downloaded: Signal<Vec<String>>,
    pub default_model: Signal<String>,
    pub downloading: Signal<HashMap<String, f64>>,
    pub selecting: Signal<Option<String>>,
}

impl UseModel {
    pub fn available(&self) -> Vec<(String, Model)> {
        (self.available)()
    }

    pub fn is_downloaded(&self, name: &str) -> bool {
        (self.downloaded)().contains(&name.to_string())
    }

    pub fn is_default(&self, name: &str) -> bool {
        (self.default_model)() == name
    }

    pub fn progress(&self, name: &str) -> Option<f64> {
        (self.downloading)().get(name).copied()
    }

    pub fn is_selecting(&self, name: &str) -> bool {
        (self.selecting)().as_deref() == Some(name)
    }

    pub fn select_model(&self, name: String, on_success: Option<EventHandler<()>>) {
        let mut selecting = self.selecting;
        let mut default_model = self.default_model;

        spawn(async move {
            selecting.set(Some(name.clone()));
            let result =
                tokio::task::spawn_blocking(move || models_ctrl::set_default_model(name)).await;
            selecting.set(None);

            if matches!(result, Ok(Ok(()))) {
                default_model.set(models_ctrl::get_default_model());
                if let Some(handler) = on_success {
                    handler.call(());
                }
            }
        });
    }

    pub fn download_model(&self, name: String) {
        let mut downloading = self.downloading;
        let mut downloaded = self.downloaded;

        spawn(async move {
            downloading.write().insert(name.clone(), 0.0);
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<f64>();

            let handle = {
                let name = name.clone();
                tokio::task::spawn_blocking(move || {
                    models_ctrl::download_model(name, move |pct: f64| {
                        let _ = tx.send(pct);
                    })
                })
            };

            while let Some(pct) = rx.recv().await {
                downloading.write().insert(name.clone(), pct);
            }

            let result = handle.await;
            downloading.write().remove(&name);

            if matches!(result, Ok(Ok(()))) {
                downloaded.set(models_ctrl::list_downloaded_models());
            }
        });
    }
}

pub fn use_model() -> UseModel {
    let available = use_signal(|| {
        let mut models: Vec<(String, Model)> = models_ctrl::get_available_models()
            .iter()
            .map(|(name, model)| (name.clone(), model.clone()))
            .collect();
        models.sort_by(|a, b| a.1.size.partial_cmp(&b.1.size).unwrap());
        models
    });

    let downloaded = use_signal(models_ctrl::list_downloaded_models);
    let default_model = use_signal(models_ctrl::get_default_model);
    let downloading = use_signal(HashMap::new);
    let selecting = use_signal(|| None);

    UseModel {
        available,
        downloaded,
        default_model,
        downloading,
        selecting,
    }
}
