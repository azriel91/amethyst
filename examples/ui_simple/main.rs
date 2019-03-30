//! Opens a simple UI.

use amethyst::{
    assets::ProgressCounter,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{is_key_down, InputBundle},
    prelude::*,
    renderer::{DisplayConfig, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle, UiCreator},
    utils::application_root_dir,
    winit::VirtualKeyCode,
};
use log::info;

struct Example {
    /// ProgressCounter to track UI creation progress.
    progress_counter: ProgressCounter,
    /// Flag to remember if the UI has previously been loaded.
    ///
    /// We use this to make sure we only log that UI loading has been complete, once.
    ui_loaded: bool,
}

impl Example {
    fn new() -> Self {
        Example {
            progress_counter: ProgressCounter::new(),
            ui_loaded: false,
        }
    }
}

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // Load ui_simple prefab
        world.exec(|mut ui_creator: UiCreator| {
            ui_creator.create("prefab/ui_simple.ron", &mut self.progress_counter)
        });
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if !self.ui_loaded && self.progress_counter.is_complete() {
            self.ui_loaded = true;

            info!("UI loading completed.");
        }
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("examples/assets");

    let display_config_path = app_root.join("examples/ui_simple/resources/display_config.ron");
    let display_config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(AudioBundle::default())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;
    let mut game = Application::new(assets_dir, Example::new(), game_data)?;

    game.run();

    Ok(())
}
