use tracing::{info, Level};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Clone)]
enum AppType {
    Mail,
    // Calendar,
    // Addressbook,
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
struct Apps {
    mail_app: crate::apps::MailApp,
    // calendar_app: crate::apps::CalendarApp,
    // addressbook_app: crate::apps::AddressbookApp,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct App {
    app_type: AppType,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Spider {
    #[serde(skip)]
    available_apps: [App; 1],
    selected_app: App,
    authorized: bool,
    apps: Apps,
}

impl Default for Spider {
    fn default() -> Self {
        // NOTE(maltalef): The order of definition of the items of the following vector is a sensitive
        // thing to change and I WOULD NOT reccomend modifying it. Unless, of course, you know what
        // you're doing.

        // FIXME(maltalef): On the above note: This SUCKS. I don't like having hardcoded things.
        let available_apps = [
            App { app_type: AppType::Mail, name: "Mail".to_string(), },
            // App { app_type: AppType::Calendar, name: "Calendar".to_string(), },
            // App { app_type: AppType::Addressbook, name: "Addressbook".to_string(), },
        ];

        Self {
            available_apps,
            selected_app: App { app_type: AppType::Mail, name: "Mail".to_string(), },
            authorized: false,
            apps: Apps::default(),
        }
    }
}

impl Spider {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // TIP: cc.egui_ctx.set_visuals // set_fonts

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        tracing_wasm::set_as_global_default();

        Default::default()
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&App, &mut dyn eframe::App)> {
        let vec = vec![
            (
                &self.available_apps[0],
                &mut self.apps.mail_app as &mut dyn eframe::App,
            ),

            // NOTE(maltalef): When the JMAP Calendar and Addressbook drafts finally end up being RFCs,
            // I will implement such functionality.
            // (
            //     &self.available_apps[1],
            //     &mut self.apps.calendar_app as &mut dyn eframe::App,
            // ),
            // (
            //     &self.available_apps[2],
            //     &mut self.apps.addressbook_app as &mut dyn eframe::App,
            // ),
        ];

        vec.into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_app_name = self.selected_app.name.clone();
        for (app, handle) in self.apps_iter_mut() {
            if app.name == selected_app_name || ctx.memory().everything_is_visible() {
                handle.update(ctx, frame);
            }
        }
    }
}

impl eframe::App for Spider {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("app_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("spiders");

                ui.separator();

                for app in self.available_apps.clone() {
                    if ui.selectable_label(self.selected_app.name == app.name, &app.name).clicked() {
                        self.selected_app = app.clone();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if self.authorized {
                        if ui.button("Logout").clicked() {

                        }
                    } else {
                        let _ = ui.button("Login");
                    }
                    ui.label(format!("auth: {}", self.authorized));
                });
            });

        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.show_selected_app(ctx, frame);
        });
    }
}
