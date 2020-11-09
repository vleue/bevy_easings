use bevy::prelude::*;

use bevy_ninepatch::*;

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
    commands.spawn(UiCameraComponents::default());

    // prepare the button
    let button_texture_handle = asset_server.load("blue_button02.png");
    let button_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(5, 10, 6, 6));

    commands
        .spawn(
            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
            // of this entity
            NinePatchComponents {
                style: Style {
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Px(0.),
                    },
                    size: Size::new(Val::Px(300.), Val::Px(80.)),

                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                nine_patch_data: NinePatchData {
                    nine_patch: button_nine_patch_handle.clone(),
                    texture: button_texture_handle.clone(),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .with(PatchElement::ButtonCancel);
    commands
        .spawn(
            // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
            // of this entity
            NinePatchComponents {
                style: Style {
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Px(0.),
                    },
                    size: Size::new(Val::Px(300.), Val::Px(80.)),

                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                nine_patch_data: NinePatchData {
                    nine_patch: button_nine_patch_handle,
                    texture: button_texture_handle,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .with(PatchElement::ButtonOk);
}

enum PatchElement {
    ButtonOk,
    ButtonCancel,
}

fn set_content(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    patch_query: Query<&PatchElement>,
    mut patch_content: Query<(Entity, &mut NinePatchContent<()>)>,
) {
    let font = asset_server.load("Kenney Future Narrow.ttf");

    for (entity, mut nine_patch_content) in &mut patch_content.iter_mut() {
        if !nine_patch_content.loaded {
            match *patch_query
                .get_component::<PatchElement>(nine_patch_content.parent)
                .expect("couldn't find tagged parent 9-Patch UI element")
            {
                PatchElement::ButtonOk => {
                    commands.spawn(TextComponents {
                        style: Style {
                            margin: Rect {
                                left: Val::Px(50.),
                                right: Val::Auto,
                                top: Val::Auto,
                                bottom: Val::Px(10.),
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: "OK".to_string(),
                            font: font.clone(),
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
                PatchElement::ButtonCancel => {
                    commands.spawn(TextComponents {
                        style: Style {
                            margin: Rect {
                                left: Val::Px(50.),
                                right: Val::Auto,
                                top: Val::Auto,
                                bottom: Val::Px(10.),
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: "CANCEL".to_string(),
                            font: font.clone(),
                            style: TextStyle {
                                font_size: 50.,
                                color: Color::RED,
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
}
