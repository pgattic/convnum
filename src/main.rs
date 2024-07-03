use eframe::egui::{self, CentralPanel, TextEdit, Ui};

const APP_ID: &str = "com.pgattic.numconv";

fn main() {
    let app = MyApp {
        dec_input: String::from("0"),
        hex_input: String::from("0"),
        bin_input: String::from("0"),
        oct_input: String::from("0"),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([240.0, 240.0])
            .with_app_id(APP_ID),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Numconv",
        options,
        Box::new(|_cc| {
            Box::<MyApp>::new(app)
        }),
    );
}

struct MyApp {
    dec_input: String,
    hex_input: String,
    bin_input: String,
    oct_input: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Display and handle the text boxes
            show_text_boxes(ui, self);
        });
    }
}

fn show_text_boxes(ui: &mut Ui, app: &mut MyApp) {
    ui.label("Decimal:");
    let decimal_response = ui.add(
        TextEdit::singleline(&mut app.dec_input)
            .horizontal_align(egui::Align::Max)
    );
    // Update the other inputs
    if decimal_response.changed() {
        if let Ok(value) = app.dec_input.parse::<i64>() {
            app.hex_input = format!("{:X}", value);
            app.bin_input = format!("{:b}", value);
            app.oct_input = format!("{:o}", value);
        } else {
            app.hex_input.clear();
            app.bin_input.clear();
            app.oct_input.clear();
        }
    }

    ui.label("Hexadecimal:");
    let hex_response = ui.add(
        TextEdit::singleline(&mut app.hex_input)
            .horizontal_align(egui::Align::Max)
    );
    // Update the other inputs
    if hex_response.changed() {
        if let Ok(value) = i64::from_str_radix(&app.hex_input, 16) {
            app.dec_input = format!("{}", value);
            app.bin_input = format!("{:b}", value);
            app.oct_input = format!("{:o}", value);
        } else {
            app.dec_input.clear();
            app.bin_input.clear();
            app.oct_input.clear();
        }
    }

    ui.label("Binary:");
    let bin_response = ui.add(
        TextEdit::singleline(&mut app.bin_input)
            .horizontal_align(egui::Align::Max)
    );
    if bin_response.changed() {
        if let Ok(value) = i64::from_str_radix(&app.bin_input, 2) {
            app.dec_input = format!("{}", value);
            app.hex_input = format!("{:X}", value);
            app.oct_input = format!("{:o}", value);
        } else {
            app.dec_input.clear();
            app.hex_input.clear();
            app.oct_input.clear();
        }
    }

    ui.label("Octal:");
    let bin_response = ui.add(
        TextEdit::singleline(&mut app.oct_input)
            .horizontal_align(egui::Align::Max)
    );
    if bin_response.changed() {
        if let Ok(value) = i64::from_str_radix(&app.oct_input, 8) {
            app.dec_input = format!("{}", value);
            app.hex_input = format!("{:X}", value);
            app.bin_input = format!("{:b}", value);
        } else {
            app.dec_input.clear();
            app.hex_input.clear();
            app.bin_input.clear();
        }
    }

    //ui.vertical_centered(|ui| {
    //    ui.label("Made by pgattic with egui.");
    //    ui.hyperlink_to(
    //        format!("{} Star on GitHub", egui::special_emojis::GITHUB),
    //        "https://github.com/pgattic/numconv",
    //    );
    //});
}

