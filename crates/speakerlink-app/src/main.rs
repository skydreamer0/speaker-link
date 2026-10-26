//! Native entry for an explicitly device-free layout preview.
#![forbid(unsafe_code)]

fn selected_renderer(
    args: impl IntoIterator<Item = String>,
) -> Result<eframe::Renderer, &'static str> {
    let args: Vec<_> = args.into_iter().collect();
    match args.as_slice() {
        [] => Ok(eframe::Renderer::Wgpu),
        [flag, value] if flag == "--renderer" && value == "wgpu" => Ok(eframe::Renderer::Wgpu),
        [flag, value] if flag == "--renderer" && value == "glow" => Ok(eframe::Renderer::Glow),
        _ => Err("usage: speakerlink-app [--renderer wgpu|glow]"),
    }
}

fn main() -> eframe::Result {
    let renderer = match selected_renderer(std::env::args().skip(1)) {
        Ok(renderer) => renderer,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([960.0, 760.0])
            .with_min_inner_size([420.0, 480.0]),
        renderer,
        ..Default::default()
    };
    eframe::run_native(
        "Speaker Link — Device-free UI preview",
        options,
        Box::new(|context| {
            // Fixed enums only: no adapter name, driver string, path or user data.
            if let Some(state) = &context.wgpu_render_state {
                let info = state.adapter.get_info();
                println!(
                    "speakerlink-renderer=Wgpu backend={:?} device_type={:?}",
                    info.backend, info.device_type
                );
            } else {
                println!("speakerlink-renderer=Glow");
            }
            Ok(Box::<speakerlink_app::ui::SpeakerLinkApp>::default())
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<eframe::Renderer, &'static str> {
        selected_renderer(args.iter().map(|arg| (*arg).to_owned()))
    }

    #[test]
    fn wgpu_is_default_and_glow_is_explicit_comparison_only() {
        assert_eq!(parse(&[]).unwrap(), eframe::Renderer::Wgpu);
        assert_eq!(
            parse(&["--renderer", "wgpu"]).unwrap(),
            eframe::Renderer::Wgpu
        );
        assert_eq!(
            parse(&["--renderer", "glow"]).unwrap(),
            eframe::Renderer::Glow
        );
    }

    #[test]
    fn invalid_or_extra_arguments_do_not_silently_fallback() {
        for args in [
            vec!["glow"],
            vec!["--renderer"],
            vec!["--renderer", "unknown"],
            vec!["--renderer", "glow", "extra"],
        ] {
            assert!(parse(&args).is_err());
        }
    }
}
