use eframe::egui;
use crate::vault::Vault;
use crate::entry::ServiceInfo;
pub struct App {
    pub vault: Vault,
    pub master: String,
    pub input_service: String,
    pub input_username: String,
    pub input_password: String,
}
impl Default for App {
    fn default() -> Self {
        Self {
            vault: Vault::new(""),
            master: String::new(),
            input_service: String::new(),
            input_username: String::new(),
            input_password: String::new(),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Vault");
            ui.text_edit_singleline(&mut self.master);
            if ui.button("Unlock").clicked() {
            }
            ui.separator();
            ui.text_edit_singleline(&mut self.input_service);
            ui.text_edit_singleline(&mut self.input_username);
            ui.text_edit_singleline(&mut self.input_password);
            if ui.button("Add Entry").clicked() {
                let e = ServiceInfo::new(self.input_service.clone(), self.input_username.clone(), self.input_password.clone());
                self.vault.add(e);
            }
            ui.separator();
            for e in self.vault.entries() {
                ui.label(format!("{} | {} | {}", e.service, e.username, e.password));
            }
        });
    }
}
pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Password Vault",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
} 