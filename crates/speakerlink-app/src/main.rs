//! Native entry for an explicitly device-free layout preview.
#![forbid(unsafe_code)]

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([960.0, 760.0])
            .with_min_inner_size([420.0, 480.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Speaker Link — Device-free UI preview",
        options,
        Box::new(|_creation_context| Ok(Box::<speakerlink_app::ui::SpeakerLinkApp>::default())),
    )
}
