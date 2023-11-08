use super::NonZeroUsizeInput;
use super::PotentialKey;
use super::TextEncoding;
use crate::{Cipher, Cracker};
use eframe::{App, Frame};
use egui::scroll_area::ScrollArea;
use egui::{Align, CentralPanel, Context, DroppedFile, Layout, Ui, Window};
use std::num::NonZeroUsize;

pub struct Application {
    file: Option<DroppedFile>,
    message: Option<String>,
    key: Option<PotentialKey>,
    encoding: TextEncoding,
    key_length: NonZeroUsizeInput,
    cracker: Cracker,
    cipher: Cipher,
}

impl Application {
    fn create_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Key length: ");
            if ui.text_edit_singleline(&mut self.key_length).changed() {
                self.cipher = Cipher::default();
                self.refresh_content();
            }
        });
        ui.horizontal(|ui| {
            ui.label("Encoding: ");
            egui::ComboBox::from_id_source("Encoding")
                .selected_text(self.encoding.to_string())
                .show_ui(ui, |ui| {
                    let mut changed = ui
                        .selectable_value(
                            &mut self.encoding,
                            TextEncoding::UTF8,
                            TextEncoding::UTF8.to_string(),
                        )
                        .changed();
                    changed |= ui
                        .selectable_value(
                            &mut self.encoding,
                            TextEncoding::WINDOWS1250,
                            TextEncoding::WINDOWS1250.to_string(),
                        )
                        .changed();
                    if changed {
                        self.cracker = Cracker::new(&self.encoding.alphabet());
                    }
                });
        });
        if ui.button("Crack").clicked() && self.file.is_some() {
            let key_option = self
                .file
                .as_ref()
                .and_then(|file| file.bytes.as_ref())
                .and_then(|bytes| self.cracker.crack(bytes, *self.key_length));
            if let Some(key) = key_option {
                let key = key
                    .into_iter()
                    .map(|key| key.into_iter().collect())
                    .collect();
                let key = PotentialKey::new(key);
                self.cipher = Cipher::new(key.get_current_key());
                self.key = Some(key);
            }
            self.refresh_content();
        }
    }

    fn create_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                if let Some(file) = &self.file {
                    ui.label(&file.name);
                } else {
                    ui.label("Drop files here!");
                }
            });
        });

        ui.separator();

        ScrollArea::new([false, true]).show(ui, |ui| {
            if let Some(text) = &self.message {
                ui.label(text);
            }
        });
    }

    fn refresh_content(&mut self) {
        let text_option = self
            .file
            .as_ref()
            .and_then(|file| file.bytes.as_ref())
            .map(|bytes| self.cipher.decrypt(bytes))
            .and_then(|bytes| self.encoding.decode(&bytes));
        if let Some(text) = text_option {
            self.message = Some(text);
        } else {
            self.message = Some("Non decodable text".to_owned());
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        let encoding = TextEncoding::default();
        Self {
            file: None,
            message: None,
            key: None,
            cracker: Cracker::new(&encoding.alphabet()),
            encoding,
            key_length: NonZeroUsize::new(256).unwrap().into(),
            cipher: Cipher::default(),
        }
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| self.create_controls(ui));
        CentralPanel::default().show(ctx, |ui| self.create_content(ui));

        ctx.input(|input| {
            if let Some(file) = input.raw.dropped_files.first() {
                self.file = Some(file.clone());
                self.refresh_content();
            }
        });
    }
}