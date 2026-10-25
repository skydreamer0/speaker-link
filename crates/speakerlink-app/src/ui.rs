//! Native layout preview only. This module has no adapter, filesystem or network calls.
use crate::presentation::{AppModel, SessionState, ViewState};
use eframe::egui;
use speakerlink_core::model::{
    BatteryPercent, BatterySource, CapabilityState, DeviceRef, Freshness, Observation, Snapshot,
    SpeakerDevice, SpeakerTargetRef, UnknownReason,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Preview {
    #[default]
    Empty,
    Unknown,
    Stale,
    Unsupported,
    Busy,
}

impl Preview {
    pub const ALL: [Self; 5] = [
        Self::Empty,
        Self::Unknown,
        Self::Stale,
        Self::Unsupported,
        Self::Busy,
    ];
    pub const fn label(self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Unknown => "Unknown",
            Self::Stale => "Stale",
            Self::Unsupported => "Unsupported",
            Self::Busy => "Busy",
        }
    }
    pub fn model(self) -> AppModel {
        if self == Self::Empty {
            return AppModel::default();
        }
        // Fixed invented identifiers are never displayed or sent anywhere.
        let mut snapshot = Snapshot::unobserved(
            DeviceRef::new("synthetic-preview").expect("fixed valid fixture ID"),
        );
        let mut session = SessionState::Idle;
        match self {
            Self::Stale => {
                snapshot.freshness = Freshness::Stale;
                snapshot.observed_at = Some(1_700_000_000_000);
                snapshot.battery.percent = Observation::Known(
                    BatteryPercent::new(67).expect("fixed valid fixture percentage"),
                );
                snapshot.battery.source = Observation::Known(BatterySource::SpeakerProtocol);
                snapshot.speaker_devices = Observation::Known(vec![SpeakerDevice {
                    target_ref: SpeakerTargetRef::new("synthetic-source")
                        .expect("fixed valid fixture ID"),
                    paired: Observation::Known(true),
                    connected: Observation::Known(false),
                    active_audio: Observation::Unknown(UnknownReason::NotObserved),
                }]);
            }
            Self::Unsupported => {
                snapshot.capabilities.battery = CapabilityState::Unsupported;
                snapshot.capabilities.speaker_devices = CapabilityState::Unsupported;
                snapshot.capabilities.set_connection = CapabilityState::Unsupported;
                snapshot.battery.percent = Observation::Unknown(UnknownReason::Unsupported);
                snapshot.speaker_devices = Observation::Unknown(UnknownReason::Unsupported);
            }
            Self::Busy => session = SessionState::Busy,
            Self::Empty | Self::Unknown => {}
        }
        AppModel::from_snapshot(&snapshot, session)
    }
}

#[derive(Default)]
pub struct SpeakerLinkApp {
    preview: Preview,
    model: AppModel,
}

impl SpeakerLinkApp {
    pub fn select_preview(&mut self, preview: Preview) {
        self.preview = preview;
        self.model = preview.model();
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Speaker Link");
                ui.label("Device-free UI preview");
                ui.colored_label(ui.visuals().warn_fg_color,
                    "SYNTHETIC DATA ONLY · No speaker access · No Bluetooth operations");
                ui.separator();
                let mut selected = self.preview;
                ui.horizontal_wrapped(|ui| {
                    ui.label("Preview state:");
                    egui::ComboBox::from_id_salt("preview-state")
                        .selected_text(selected.label()).show_ui(ui, |ui| {
                            for preview in Preview::ALL {
                                ui.selectable_value(&mut selected, preview, preview.label());
                            }
                        });
                    if ui.button("Clear preview").clicked() { selected = Preview::Empty; }
                });
                if selected != self.preview { self.select_preview(selected); }
                ui.small("Changing this selector only changes an in-memory fixture. It does not scan or connect.");
                ui.add_space(12.0);
                if self.model.session == SessionState::Busy {
                    ui.group(|ui| {
                        ui.strong("Busy — preview scenario");
                        ui.label("A future session may be owned by another client. No connection attempt or retry is made here.");
                    });
                }
                match &self.model.view {
                    ViewState::Empty => {
                        ui.group(|ui| {
                            ui.heading("No device selected");
                            ui.label("Discovery is not connected in this preview. Choose a synthetic state above to inspect the layout.");
                        });
                    }
                    ViewState::UnsupportedSchema { received } => {
                        ui.label(format!("Unsupported snapshot schema {received}. No device state is displayed."));
                    }
                    ViewState::Snapshot(view) => {
                        ui.heading("Observation");
                        ui.strong(view.freshness);
                        ui.label(format!("Observed at: {}", view.observed_at));
                        ui.add_space(8.0);
                        ui.group(|ui| {
                            ui.heading("Battery");
                            ui.label(format!("Level: {}", view.battery));
                            ui.label(format!("Charging: {}", view.charging));
                            ui.label(format!("Source: {}", view.battery_source));
                        });
                        ui.add_space(8.0);
                        ui.heading("Speaker-side sources");
                        ui.label(&view.devices_summary);
                        ui.small("This is not the Windows paired-device list. Paired, connected and active audio are independent.");
                        for device in &view.devices {
                            ui.group(|ui| {
                                ui.strong(&device.label);
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(format!("Paired: {}", device.paired));
                                    ui.label(format!("Connected: {}", device.connected));
                                    ui.label(format!("Active audio: {}", device.active_audio));
                                });
                            });
                        }
                        ui.add_space(8.0);
                        ui.heading("Capabilities");
                        for (label, capability) in ["Battery", "Speaker sources", "Connection changes", "Diagnostics"]
                            .iter().zip(view.capabilities) {
                            ui.label(format!("{label}: {}", capability.as_str()));
                        }
                    }
                }
                ui.add_space(12.0);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    for label in ["Scan", "Refresh", "Connect", "Disconnect", "Export diagnostics"] {
                        ui.add_enabled(self.model.controls_enabled(), egui::Button::new(label));
                    }
                });
                ui.label("All device and export actions are disabled in this build. No report is written or uploaded.");
                ui.small("F1–F4 hardware acceptance and CLI/App parity are still pending. Preview data is not device support evidence.");
            });
        });
    }
}

impl eframe::App for SpeakerLinkApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_preview_has_disabled_controls() {
        for preview in Preview::ALL {
            assert!(!preview.model().controls_enabled());
        }
    }
    #[test]
    fn clear_and_repeated_switches_remove_stale_state() {
        let mut app = SpeakerLinkApp::default();
        for _ in 0..3 {
            for preview in Preview::ALL {
                app.select_preview(preview);
            }
            app.select_preview(Preview::Empty);
            assert_eq!(app.model, AppModel::default());
        }
    }
    #[test]
    fn headless_layout_all_states_at_small_and_large_sizes() {
        for size in [[420.0, 640.0], [1000.0, 760.0]] {
            for preview in Preview::ALL {
                let context = egui::Context::default();
                let mut app = SpeakerLinkApp::default();
                app.select_preview(preview);
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(size[0], size[1]),
                    )),
                    ..Default::default()
                };
                let mut output = context.run_ui(input, |ui| app.draw(ui));
                output.textures_delta.clear();
                assert!(!output.shapes.is_empty());
                assert!(
                    output.platform_output.commands.is_empty(),
                    "no clipboard or URL commands"
                );
            }
        }
    }
    #[test]
    fn actual_clear_button_click_resets_busy_preview() {
        let context = egui::Context::default();
        let mut app = SpeakerLinkApp::default();
        app.select_preview(Preview::Busy);
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1000.0, 760.0));
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |ui| app.draw(ui),
        );
        output.textures_delta.clear();
        let button_position = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) if text.galley.text() == "Clear preview" => {
                    Some(text.pos + text.galley.rect.size() * 0.5)
                }
                _ => None,
            })
            .expect("clear button text must be rendered");
        for pressed in [true, false] {
            let output = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    events: vec![
                        egui::Event::PointerMoved(button_position),
                        egui::Event::PointerButton {
                            pos: button_position,
                            button: egui::PointerButton::Primary,
                            pressed,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                    ..Default::default()
                },
                |ui| app.draw(ui),
            );
            output.drop_without_applying_deltas();
        }
        assert_eq!(app.model, AppModel::default());
        assert_eq!(app.preview, Preview::Empty);
    }
}
