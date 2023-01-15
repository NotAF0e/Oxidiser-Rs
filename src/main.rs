// #![windows_subsystem = "windows"]
use egui::*;
use wfd::{ self };
use eframe::egui;
use std::{ fs::File, io::Read, str };
use lz4_flex::{ compress_prepend_size, decompress_size_prepended };

struct App {}

fn main() {
    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            ctx.set_pixels_per_point(1.75);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().spacing.button_padding = (41.0, 16.0).into();

                // Bottom settings panel
                egui::TopBottomPanel::bottom("settings").show(ctx, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        egui::CollapsingHeader
                            ::new("THEME")
                            .show(ui, egui::widgets::global_dark_light_mode_buttons);
                    });
                });

                egui::Grid::new("Button_grid").show(ui, |ui| {
                    // Button which opens dialog for files to be oxidised(compressed)
                    if ui.button("Oxidise").clicked() {
                        // Stores dialog for all types of files
                        let unoxid_dialog = wfd::open_dialog(Default::default());

                        if unoxid_dialog.is_ok() {
                            let file_path = unoxid_dialog.unwrap().selected_file_path;
                            println!("{:?}", file_path);

                            oxidise(file_path.as_path());
                        }
                    }

                    // Button which opens dialog for files to be unoxidised(uncompressed)
                    if ui.button("Unoxidise").clicked() {
                        // Stores dialog for oxidised files
                        let oxid_dialog = wfd::open_dialog(wfd::DialogParams {
                            file_types: vec![("Oxidised Files", "*.oxid")],
                            ..Default::default()
                        });

                        if oxid_dialog.is_ok() {
                            let file_path = oxid_dialog.unwrap().selected_file_path;
                            println!("{:?}", file_path);

                            unoxidise(file_path.as_path());
                        }
                    }
                });
            });
        }
    }

    impl Default for App {
        fn default() -> Self {
            Self {}
        }
    }
    // Custom options
    let options = eframe::NativeOptions {
        maximized: true,
        min_window_size: Option::from(Vec2::new(500_f32, 575_f32)),
        max_window_size: Option::from(Vec2::new(500_f32, 575_f32)),
        vsync: true,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        centered: true,
        ..Default::default()
    };

    // Runs the application
    eframe::run_native(
        "PopSim",
        options,
        Box::new(|_cc| Box::new(App::default()))
    );

    // Functions
    fn oxidise(file_path: &std::path::Path) {
        // Opens file
        let mut f = File::open(file_path).unwrap_or_else(|_e|
            panic!("'{:?}' not found!", file_path)
        );
        let mut f_content = String::new();
        f.read_to_string(&mut f_content).expect("Cannot read file");

        // Turns file to bytes
        let f_content_bytes = f_content.as_bytes();

        // Compresses file
        let mut oxidised = compress_prepend_size(f_content_bytes);

        // Saves file
        std::fs::write("file.oxid", &mut oxidised).expect("Unable to save file");
        println!(
            "Compressed file by {}%",
            100.0 -
                ((std::fs::metadata("file.oxid").unwrap().len() as f32) /
                    (std::fs::metadata(file_path).unwrap().len() as f32)) *
                    100.0
        );
        println!("Compressed file saved");
    }
    fn unoxidise(file_path: &std::path::Path) {
        if std::fs::metadata(file_path).is_ok() {
            let mut oxidised = File::open(file_path).unwrap_or_else(|_e|
                panic!("'{:?}' not found!", file_path)
            );
            let mut oxidised_content = Vec::new();
            oxidised.read_to_end(&mut oxidised_content).expect("Cannot read file");

            let mut unoxidised = decompress_size_prepended(&oxidised_content).unwrap();

            println!("{}", str::from_utf8(&unoxidised).unwrap());

            //Saves file
            std::fs::write("uncrompressed.txt", &mut unoxidised).expect("Unable to save file");
            println!("Uncrompressed file saved")
        }
    }
}