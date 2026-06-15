use eframe::egui;
use gumdrop::Options;
use log::{LevelFilter, info};
use pnet::datalink::{self, NetworkInterface};
use serde::{Deserialize, Serialize};

mod egui_theme;

#[derive(Debug, Options)]
struct CommandOptions {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "be verbose")]
    verbose: bool,

    #[options(command)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "send a packet")]
    Start(SendPacketOptions),
}

#[derive(Debug, Options)]
struct SendPacketOptions {}

#[derive(Default, Serialize, Deserialize)]
struct EguiProgram {
    interfaces: Vec<NetworkInterface>,
    selected_iface_index: usize,
}

impl EguiProgram {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set styling
        let mut style = (*cc.egui_ctx.global_style()).clone();
        egui_theme::set_style_theme(&mut style, egui_theme::MACCHIATO);
        egui_theme::set_style_widgets(&mut style);
        cc.egui_ctx.set_visuals(style.visuals);

        let interfaces = datalink::interfaces();

        // check for previously saved state

        if let Some(storage) = cc.storage {
            // retrieve state, but ignore the list of interfaces as they may have changed
            eframe::get_value(storage, eframe::APP_KEY)
                .map(|mut program: EguiProgram| {
                    program.selected_iface_index =
                        program.selected_iface_index.max(interfaces.len());
                    program.interfaces = interfaces.clone();
                    program
                })
                .unwrap_or(EguiProgram {
                    interfaces,
                    selected_iface_index: 0,
                })
        } else {
            EguiProgram {
                interfaces,
                selected_iface_index: 0,
            }
        }
    }
}

impl eframe::App for EguiProgram {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("fuck my life");

            if !self.interfaces.is_empty() && self.selected_iface_index < self.interfaces.len() {
                let current_selection = &self.interfaces[self.selected_iface_index].name;

                egui::ComboBox::from_label("network interface")
                    .selected_text(current_selection)
                    .show_ui(ui, |ui| {
                        for (index, item) in self.interfaces.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_iface_index, index, &item.name);
                        }
                    });
            } else {
                ui.label("No interfaces available or invalid index.");
            }
        });
    }
}

fn main() -> eframe::Result {
    let opts = CommandOptions::parse_args_default_or_exit();

    let mut builder = env_logger::Builder::new();

    if opts.verbose {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.filter_level(LevelFilter::Info);
    }
    builder.init();

    info!("Starting");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([768.0, 768.0]),
        persist_window: false,
        ..Default::default()
    };

    // entry point is in EguiProgram::new

    eframe::run_native(
        "Chernobyl",
        options,
        Box::new(|cc| Ok(Box::new(EguiProgram::new(cc)))),
    )
}
