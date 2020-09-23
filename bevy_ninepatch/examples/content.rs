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

    // create the button 9-Patch in advance. This way it can be used without complications in a closure
    let button = bevy_ninepatch::NinePatchBuilder::by_margins(5., 10., 6., 6.).apply(
        button_texture_handle,
        &mut textures,
        &mut materials,
    );

    commands
        .spawn(
            // NodeComponents used to place the 9-Patch UI element
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
            bevy_ninepatch::NinePatchBuilder::by_margins(20., 20., 20., 20.)
                .apply(panel_texture_handle, &mut textures, &mut materials)
                .add(
                    global_parent,
                    500.,
                    300.,
                    // this closure let you specify what UI elements to place inside the panel. For 9-Patch UI elements
                    // created with `by_margins`, there is only one content zone by default in the middle of the element
                    |inside, _| {
                        button.add(inside, 460., 100., |_, _| {});
                    },
                );
        });

    commands.spawn(UiCameraComponents::default());
}
