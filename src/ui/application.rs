use super::NonZeroUsizeInput;
use crate::{Cipher, Cracker, PotentialKey, TextEncoding};
use eframe::{App, Frame};
use egui::scroll_area::ScrollArea;
use egui::text::LayoutJob;
use egui::{
    Align, CentralPanel, Color32, ComboBox, Context, DroppedFile, Hyperlink, Layout, RichText,
    TopBottomPanel, Ui, Window,
};
use std::num::NonZeroUsize;

pub struct Application {
    file: Option<DroppedFile>,
    message: Option<LayoutJob>,
    key: Option<PotentialKey>,
    selected_key: Option<usize>,
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
            ComboBox::from_id_source("Encoding")
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
                        self.cracker = Cracker::new(&self.encoding);
                    }
                });
        });

        if ui.button("Crack").clicked() && self.file.is_some() {
            let key_option = self
                .file
                .as_ref()
                .and_then(|file| file.bytes.as_ref())
                .map(|bytes| self.cracker.crack(bytes, *self.key_length));
            if let Some(key) = key_option {
                self.cipher = Cipher::new(key.get_current_key());
                self.key = Some(key);
            }
            self.refresh_content();
        }

        if let Some(key) = &mut self.key {
            let current_key = key.get_current_key();
            let mut should_refresh = false;

            ui.separator();

            ui.label("Key (click non-green value to correct it): ");
            for index in 0..(self.key_length.get() / 16 + 1) {
                ui.horizontal(|ui| {
                    for i in 0..16 {
                        let index = index * 16 + i;

                        if index >= self.key_length.get() {
                            break;
                        }

                        let mut text = RichText::new(format!("{:02X?}", current_key[index]));

                        if key.is_decoded(index) {
                            text = text.color(Color32::LIGHT_GREEN);
                        } else if !key.is_uncertain(index) {
                            text = text.color(Color32::YELLOW);
                        } else if key.is_error(index) {
                            text = text.color(Color32::LIGHT_RED);
                        }

                        if key.is_decoded(index) {
                            ui.label(text);
                        } else if ui.button(text).clicked() {
                            self.selected_key = if let Some(prev) = self.selected_key {
                                if prev == index {
                                    None
                                } else {
                                    Some(index)
                                }
                            } else {
                                Some(index)
                            };
                            should_refresh = true;
                        }
                    }
                });
            }

            if let Some(index) = self.selected_key {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(format!("Selected key: {:02X?}", current_key[index]));
                    if ui.button("Confirm value").clicked() {
                        key.accept_value(index);
                        self.selected_key = None;
                        should_refresh = true;
                    }
                });
                ui.label("Possibilities:");

                let possibility = key.get_possibilities(index);

                let mut new_value = None;

                for i in 0..(possibility.len() / 16 + 1) {
                    ui.horizontal(|ui| {
                        for j in 0..16 {
                            let possibility_index = i * 16 + j;

                            if possibility_index >= possibility.len() {
                                break;
                            }

                            let mut text =
                                RichText::new(format!("{:02X?}", possibility[possibility_index]));

                            if possibility[possibility_index] == current_key[index] {
                                text = text.color(Color32::LIGHT_GREEN);
                            }

                            if ui.button(text).clicked() {
                                new_value = Some(possibility[possibility_index]);
                            }
                        }
                    });
                }

                if let Some(new_value) = new_value {
                    key.set_value(index, new_value);
                    should_refresh = true;
                }
            }

            if should_refresh {
                self.cipher = Cipher::new(key.get_current_key());
                self.refresh_content();
            }
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
            if let Some(message) = &self.message {
                ui.label(message.clone());
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
        let mut job = LayoutJob::default();

        if let Some(text) = text_option {
            if let Some(key) = &mut self.key {
                for (index, character) in text.chars().enumerate() {
                    let index = index % self.key_length.get();

                    let mut text_format = egui::TextFormat::default();

                    if key.is_decoded(index) {
                        text_format.color = Color32::LIGHT_GREEN;
                    } else if !key.is_uncertain(index) {
                        text_format.color = Color32::YELLOW;
                    } else if key.is_error(index) {
                        text_format.color = Color32::LIGHT_RED;
                    }

                    if let Some(highlighted) = self.selected_key {
                        if highlighted == index {
                            text_format.background = Color32::DARK_BLUE;
                        }
                    }

                    job.append(&character.to_string(), 0.0, text_format);
                }
            } else {
                job.text = text;
            }
        } else {
            job.text = "Non decodable text".to_owned();
        }
        self.message = Some(job);
    }
}

impl Default for Application {
    fn default() -> Self {
        let encoding = TextEncoding::default();
        Self {
            file: None,
            message: None,
            key: None,
            cracker: Cracker::new(&encoding),
            encoding,
            key_length: NonZeroUsize::new(256).unwrap().into(),
            selected_key: None,
            cipher: Cipher::default(),
        }
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| self.create_controls(ui));

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add(Hyperlink::from_label_and_url(
                    "Source code",
                    "https://github.com/SamPanDonte/many_time_pad/tree/master/",
                ));
            });
        });

        CentralPanel::default().show(ctx, |ui| self.create_content(ui));

        ctx.input(|input| {
            if let Some(file) = input.raw.dropped_files.first() {
                self.file = Some(file.clone());
                self.refresh_content();
            }
        });
    }
}
