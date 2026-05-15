use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Minimal egui App"),
        ..Default::default()
    };

    eframe::run_native(
        "Minimal egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    counter: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Минимальное egui приложение");
            ui.separator();

            // Текстовое поле
            ui.horizontal(|ui| {
                ui.label("Введите имя: ");
                ui.text_edit_singleline(&mut self.name);
            });

            // Кнопка и счетчик
            ui.horizontal(|ui| {
                if ui.button("Увеличить счетчик").clicked() {
                    self.counter += 1;
                }
                ui.label(format!("Счетчик: {}", self.counter));
            });

            // Вывод данных
            if !self.name.is_empty() {
                ui.separator();
                ui.label(format!(
                    "Привет, {}! Твой счетчик: {}",
                    self.name, self.counter
                ));
            }

            ui.separator();

            // Кнопка сброса
            if ui.button("Сбросить всё").clicked() {
                self.name.clear();
                self.counter = 0;
            }
        });
    }
}
