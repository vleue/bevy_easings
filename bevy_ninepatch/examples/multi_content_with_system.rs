use bevy::prelude::*;

use bevy_ninepatch::{
    NinePatchBuilder, NinePatchComponents, NinePatchContent, NinePatchData, NinePatchPlugin,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin::<Content>::default())
        .add_startup_system(setup.system())
        .add_system(set_content.system())
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
) {
    // prepare the panel
    let panel_texture_handle = asset_server.load("glassPanel_corners.png");
    let panel_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins_with_content(
        20,
        20,
        20,
        20,
        Content::Panel,
    ));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchComponents {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(500.), Val::Px(300.)),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: panel_nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(UiCameraComponents::default());
}

fn set_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
    entity: Entity,
    mut nine_patch_content: Mut<NinePatchContent<Content>>,
) {
    if !nine_patch_content.loaded {
        match nine_patch_content.content {
            Content::Panel => {
                // prepare the button
                let button_texture_handle = asset_server.load("blue_button02.png");
                let button_nine_patch_handle = nine_patches.add(
                    NinePatchBuilder::by_margins_with_content(5, 10, 6, 6, Content::Button),
                );

                commands.spawn(
                    // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
                    // of this entity
                    NinePatchComponents {
                        style: Style {
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Px(0.),
                                top: Val::Auto,
                                bottom: Val::Px(0.),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(200.), Val::Px(100.)),
                            ..Default::default()
                        },
                        nine_patch_data: NinePatchData {
                            nine_patch: button_nine_patch_handle,
                            texture: button_texture_handle,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                );
                let content_entity = commands.current_entity().unwrap();
                commands.push_children(entity, &[content_entity]);
                nine_patch_content.loaded = true;
            }
            Content::Button => {
                // load font
                let font = asset_server.load("Kenney Future Narrow.ttf");

                commands.spawn(TextComponents {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(60.),
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Px(20.),
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "OK".to_string(),
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
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash)]
enum Content {
    Panel,
    Button,
}
