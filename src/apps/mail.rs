use chrono::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Mail {
    subject: String,
    from: String,
    to: String,
    date: NaiveDateTime,
    body: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct MailApp {
    mails: Vec<Mail>
}

// impl Default for MailApp {
//     fn default() -> Self {
//     }
// }

impl eframe::App for MailApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("mail-app")
        });
    }
}
