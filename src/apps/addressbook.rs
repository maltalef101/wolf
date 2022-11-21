use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AddressbookApp {
    none: Option<String>,
}

impl Default for AddressbookApp {
    fn default() -> Self {
        Self {
            none: None,
        }
    }
}

impl eframe::App for AddressbookApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("addressbook-app")
        });
    }
}
