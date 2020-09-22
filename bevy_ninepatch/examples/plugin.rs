use bevy::prelude::*;

use bevy_ninepatch::{NinePatchBuilder, NinePatchComponents, NinePatchData, NinePatchPlugin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin::<()>::default())
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

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchComponents {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            nine_patch: nine_patch_handle,
            data: NinePatchData {
                texture: panel_texture_handle,
                size: Vec2::new(500., 350.),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(UiCameraComponents::default());
}
