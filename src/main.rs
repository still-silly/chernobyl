use eframe::egui;
use gumdrop::Options;
use log::{LevelFilter, info};
use pnet::datalink::{self, NetworkInterface};

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

#[derive(Default)]
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

        // 2. Build the ComboBox UI
        egui::ComboBox::from_label("Dynamic Dropdown")
            .selected_text(interfaces[selected_index])
            .show_ui(ui, |ui| {
                // Iterate over the dynamic vector to populate elements
                for (index, item) in my_items.iter().enumerate() {
                    // Alternatively use selectable_value for state binding
                    ui.selectable_value(&mut selected_index, index, *item);
                }
            });

        EguiProgram {
            interfaces: datalink::interfaces(),
        }
    }
}

impl eframe::App for EguiProgram {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello World!");
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
        ..Default::default()
    };

    // entry point is in EguiProgram::new

    eframe::run_native(
        "Chernobyl",
        options,
        Box::new(|cc| Ok(Box::new(EguiProgram::new(cc)))),
    )
}
