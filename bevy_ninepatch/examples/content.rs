use bevy::prelude::*;

use bevy_ninepatch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/glassPanel_corners.png")
        .unwrap();

    let button_texture_handle = asset_server
        .load_sync(&mut textures, "assets/blue_button02.png")
        .unwrap();

    let button = bevy_ninepatch::NinePatchBuilder::by_margins(5., 10., 6., 6.).apply(
        button_texture_handle,
        &mut textures,
        &mut materials,
    );

    commands
        .spawn(NodeComponents {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),

            ..Default::default()
        })
        .with_children(|global_parent| {
            bevy_ninepatch::NinePatchBuilder::by_margins(20., 20., 20., 20.)
                .apply(panel_texture_handle, &mut textures, &mut materials)
                .add(global_parent, 500., 300., |inside, _| {
                    button.add(inside, 460., 100., |_, _| {});
                });
        });

    commands.spawn(UiCameraComponents::default());
}
