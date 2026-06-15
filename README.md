# i wish i were a network swtich and someone plugged two of my ports into each other 

```rust
use eframe::egui;

struct MyApp {
    // Your app state goes here
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 1. Set custom fonts, visual themes, or styles
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = 5.0.into();
        cc.egui_ctx.set_visuals(visuals);

        // 2. Restore state from previously saved storage (if applicable)
        if let Some(storage) = cc.storage {
            // e.g., load previous app data
        }

        // 3. Perform network fetches, file reads, or other setup
        // ...

        Self {
            // Initialize your struct fields
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // GUI frame loop (runs every frame)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}
```
