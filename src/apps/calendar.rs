use chrono::prelude::*;

use chrono::{NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct CalendarApp {
    selected_date: Option<NaiveDate>,
}

impl Default for CalendarApp {
    fn default() -> Self {
        Self {
            selected_date: None,
        }
    }
}

impl eframe::App for CalendarApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("calendar-app");
        });
    }
}
