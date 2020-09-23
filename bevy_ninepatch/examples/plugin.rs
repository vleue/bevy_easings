use bevy::prelude::*;

use bevy_ninepatch::{
    NinePatchBuilder, NinePatchComponents, NinePatchData, NinePatchPlugin, NinePatchSize,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin)
        .add_system(update_size.system())
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    let panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/glassPanel_corners.png")
        .unwrap();

    // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20., 20., 20., 20.));

    commands
        .spawn(
            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
            // of this entity
            NinePatchComponents {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                nine_patch_data: NinePatchData {
                    nine_patch: nine_patch_handle,
                    texture: panel_texture_handle,
                    ..Default::default()
                },
                nine_patch_size: NinePatchSize(Vec2::new(500., 100.)),
                ..Default::default()
            },
        )
        .with(Timer::from_seconds(0.05, true));

    commands.spawn(UiCameraComponents::default());
}

fn update_size(timer: &Timer, mut size: Mut<NinePatchSize>) {
    if timer.just_finished {
        let y = size.0.y_mut();
        *y += 2.;
        if size.0.y() > 500. {
            size.0.set_y(100.);
        }
    }
}
