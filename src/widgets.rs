use egui::FontId;

fn mail_list_item_ui(ui: &mut egui::Ui, subject: &String, short_body: &String) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(10.0, 2.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        response.mark_changed();
    }

    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, subject));

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);

        let rect = rect.expand(visuals.expansion);
        ui.painter()
            .rect(rect, 0.5, visuals.bg_fill, visuals.bg_stroke);

        let galley = ui.painter().layout_no_wrap(subject.to_owned(), FontId::default(), visuals.text_color());
        ui.painter()
            .galley(
                egui::pos2(rect.left() + 10.0, rect.top() + 5.0),
                galley
        );
    }

    response
}

pub fn mail_list_item<'a>(subject: &'a String, short_body: &'a String) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| mail_list_item_ui(ui, subject, short_body)
}
