#![windows_subsystem = "windows"]

mod builtins;
mod executor;
mod parser;
mod startup;

use eframe::egui;
use std::env;

struct MyWindow(isize);

impl raw_window_handle::HasWindowHandle for MyWindow {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let handle = raw_window_handle::Win32WindowHandle::new(std::num::NonZeroIsize::new(self.0).unwrap());
        let raw = raw_window_handle::RawWindowHandle::Win32(handle);
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(raw)) }
    }
}

impl raw_window_handle::HasDisplayHandle for MyWindow {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let raw = raw_window_handle::RawDisplayHandle::Windows(raw_window_handle::WindowsDisplayHandle::new());
        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(raw)) }
    }
}

pub fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_transparent(true)
            .with_decorations(false)
            .with_title("Iron Shell"),
        ..Default::default()
    };
    
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(500));
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
            use windows::core::s;
            if let Ok(hwnd) = FindWindowA(None, s!("Iron Shell")) {
                if !hwnd.is_invalid() {
                    let win = MyWindow(hwnd.0 as isize);
                    let _ = window_vibrancy::apply_blur(&win, Some((20, 22, 26, 180)));
                }
            }
        }
    });

    eframe::run_native(
        "Iron Shell",
        options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();
            // Translucent dark gray background
            visuals.window_fill = egui::Color32::from_rgba_unmultiplied(20, 22, 26, 180); 
            visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(20, 22, 26, 180);
            
            // Remove the black background and borders from text inputs
            visuals.extreme_bg_color = egui::Color32::TRANSPARENT;
            visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
            visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
            visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
            visuals.widgets.open.bg_stroke = egui::Stroke::NONE;
            visuals.selection.stroke = egui::Stroke::NONE;

            cc.egui_ctx.set_visuals(visuals);
            
            Ok(Box::new(IronShellApp::new()))
        }),
    )
}

struct IronShellApp {
    output_history: Vec<(String, egui::Color32)>,
    current_input: String,
}

impl IronShellApp {
    fn new() -> Self {
        let mut app = Self {
            output_history: Vec::new(),
            current_input: String::new(),
        };
        // Print welcome screen
        app.push_output(startup::get_welcome_screen(), egui::Color32::LIGHT_GRAY);
        app
    }

    fn push_output(&mut self, text: String, color: egui::Color32) {
        if !text.is_empty() {
            self.output_history.push((text, color));
        }
    }

    fn execute_command(&mut self) {
        let input = self.current_input.trim().to_string();
        if input.is_empty() {
            self.current_input.clear();
            return;
        }

        let cwd = env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| String::from("?"));
            
        let display_dir = if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
            if cwd.starts_with(&home) {
                cwd.replacen(&home, "~", 1)
            } else {
                cwd
            }
        } else {
            cwd
        };

        self.push_output(format!("{} \n❯ {}", display_dir, input), egui::Color32::from_rgb(255, 255, 100));
        
        self.current_input.clear();

        if let Some(pipeline) = parser::parse(&input) {
            let (should_continue, output) = executor::execute(pipeline);
            if !output.is_empty() {
                self.push_output(output, egui::Color32::LIGHT_GRAY);
            }
            if !should_continue {
                std::process::exit(0);
            }
        }
    }
}

impl eframe::App for IronShellApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // Completely transparent backing
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Custom transparent title bar area for dragging and closing
        egui::TopBottomPanel::top("title_bar")
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT).inner_margin(8.0))
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Iron Shell").color(egui::Color32::LIGHT_GRAY).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("✕").color(egui::Color32::LIGHT_GRAY)).clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                
                // Allow dragging the window from this top bar
                let response = ui.interact(ui.max_rect(), ui.id().with("drag"), egui::Sense::click_and_drag());
                if response.dragged() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            });

        // Change text input layout
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                // Draw history
                for (text, color) in &self.output_history {
                    ui.label(
                        egui::RichText::new(text)
                            .color(*color)
                            .family(egui::FontFamily::Monospace)
                            .size(14.0)
                    );
                }

                let cwd = env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| String::from("?"));
                    
                let display_dir = if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
                    if cwd.starts_with(&home) {
                        cwd.replacen(&home, "~", 1)
                    } else {
                        cwd
                    }
                } else {
                    cwd
                };

                ui.label(egui::RichText::new(display_dir).color(egui::Color32::LIGHT_GRAY).family(egui::FontFamily::Monospace).size(14.0));
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("❯").color(egui::Color32::YELLOW).family(egui::FontFamily::Monospace).size(14.0));
                    
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.current_input)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                            .text_color(egui::Color32::WHITE)
                    );
                    
                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.execute_command();
                    }
                    
                    response.request_focus();
                });
            });
    }
}
