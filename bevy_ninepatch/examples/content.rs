use bevy::prelude::*;

use bevy_ninepatch::{
    NinePatchBuilder, NinePatchBundle, NinePatchContent, NinePatchData, NinePatchPlugin,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_startup_system(setup.system())
        .add_system(set_content.system())
        .run();

    Ok(())
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    let panel_texture_handle = asset_server.load("glassPanel_corners.png");

    // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(500.), Val::Px(300.)),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(UiCameraBundle::default());
}

fn set_content(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut NinePatchContent<()>)>,
) {
    for (entity, mut nine_patch_content) in query.iter_mut() {
        if !nine_patch_content.loaded {
            // load font
            let font = asset_server.load("Kenney Future Narrow.ttf");

            commands.spawn(TextBundle {
                text: Text {
                    value: "Hello".to_string(),
                    font,
                    style: TextStyle {
                        font_size: 50.,
                        color: Color::GREEN,
                    },
                },
                ..Default::default()
            });
            let content_entity = commands.current_entity().unwrap();
            commands.push_children(entity, &[content_entity]);
            nine_patch_content.loaded = true;
        }
    }
}
