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
    // load the textures beforehand
    let panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/glassPanel_corners.png")
        .unwrap();

    let button_texture_handle = asset_server
        .load_sync(&mut textures, "assets/blue_button02.png")
        .unwrap();

    commands
        .spawn(
            // NodeComponents used to place the 9-Patch UI Elements
            NodeComponents {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),

                ..Default::default()
            },
        )
        .with_children(|global_parent| {
            // first element: a panel, where all margins are equal
            bevy_ninepatch::NinePatchBuilder::by_margins(20., 20., 20., 20., ())
                .apply(panel_texture_handle, &mut textures, &mut materials)
                .add(global_parent, 500., 300., |_, _| {});

            // a spacer
            global_parent.spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(100.), Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            });

            // second element: a button, not all margins are the same
            bevy_ninepatch::NinePatchBuilder::by_margins(5., 10., 6., 6., ())
                .apply(button_texture_handle, &mut textures, &mut materials)
                .add(global_parent, 450., 150., |_, _| {});
        });

    commands.spawn(UiCameraComponents::default());
}
