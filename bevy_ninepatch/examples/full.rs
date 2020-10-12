use bevy::prelude::*;

use bevy_ninepatch::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin::<Content>::default())
        .add_startup_system(setup.system())
        .add_system(set_content.system())
        .add_system(update_size.system())
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
) {
    // prepare the panel
    let cornered_panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/metalPanel_yellowCorner.png")
        .unwrap();
    let panel_nine_patch_handle = nine_patches.add(NinePatchBuilder::from_patches(vec![
        vec![
            // top left corner patch
            Patch {
                width: PatchSize::Absolute(30.),
                height: PatchSize::Absolute(35.),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            // top middle-left patch. This patch width can grow, and will contain the content for
            // `PanelContent::Title`
            Patch {
                width: PatchSize::Absolute(15.),
                height: PatchSize::Absolute(35.),
                x_growth: GrowthMode::StretchRatio(0.3),
                y_growth: GrowthMode::None,
                content: Some(Content::PanelTitle),
            },
            // top middle patch. In the original PNG, it's the yellow titled part
            Patch {
                width: PatchSize::Absolute(25.),
                height: PatchSize::Absolute(35.),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            // top middle-right patch. This patch width can grow
            Patch {
                width: PatchSize::Absolute(20.),
                height: PatchSize::Absolute(35.),
                x_growth: GrowthMode::StretchRatio(0.7),
                y_growth: GrowthMode::None,
                content: None,
            },
            // top right corner
            Patch {
                width: PatchSize::Absolute(10.),
                height: PatchSize::Absolute(35.),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
        ],
        vec![
            // left border. This patch height can grow
            Patch {
                width: PatchSize::Absolute(10.),
                height: PatchSize::Height { offset: -45. },
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::StretchRatio(1.),
                content: None,
            },
            // center. This patch can grow both in height and width, and will contain `PanelContent::Body`
            Patch {
                width: PatchSize::Width { offset: -20. },
                height: PatchSize::Height { offset: -45. },
                x_growth: GrowthMode::StretchRatio(1.),
                y_growth: GrowthMode::StretchRatio(1.),
                content: Some(Content::PanelBody),
            },
            // right border. This patch height can grow
            Patch {
                width: PatchSize::Absolute(10.),
                height: PatchSize::Height { offset: -45. },
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::StretchRatio(1.),
                content: None,
            },
        ],
        vec![
            // bottom left corner
            Patch {
                width: PatchSize::Absolute(10.),
                height: PatchSize::Absolute(10.),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            // bottom middle. This patch width can grow
            Patch {
                width: PatchSize::Width { offset: -20. },
                height: PatchSize::Absolute(10.),
                x_growth: GrowthMode::StretchRatio(1.),
                y_growth: GrowthMode::None,
                content: None,
            },
            // bottom right corner
            Patch {
                width: PatchSize::Absolute(10.),
                height: PatchSize::Absolute(10.),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
        ],
    ]));

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
                    nine_patch: panel_nine_patch_handle,
                    texture: cornered_panel_texture_handle,
                    ..Default::default()
                },
                nine_patch_size: NinePatchSize(Vec2::new(900., 600.)),
                ..Default::default()
            },
        )
        .with(ResizableContent::Panel);

    commands.spawn(UiCameraComponents::default());
}

fn set_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,

    entity: Entity,
    mut nine_patch_content: Mut<NinePatchContent<Content>>,
) {
    if !nine_patch_content.loaded {
        match nine_patch_content.content {
            Content::PanelBody => {
                let panel_texture_handle = asset_server
                    .load_sync(&mut textures, "assets/glassPanel_corners.png")
                    .unwrap();

                // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
                let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(
                    20.,
                    20.,
                    20.,
                    20.,
                    Content::InnerPanel,
                ));

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
                            nine_patch_size: NinePatchSize(Vec2::new(850., 550.)),
                            ..Default::default()
                        },
                    )
                    .with(ResizableContent::InnerPanel);
                let content_entity = commands.current_entity().unwrap();
                commands.push_children(entity, &[content_entity]);
                nine_patch_content.loaded = true;
            }
            Content::PanelTitle => {
                // load font
                let font = asset_server
                    .load("assets/Kenney Future Narrow.ttf")
                    .expect("was able to load font");

                commands.spawn(TextComponents {
                    style: Style {
                        margin: Rect {
                            left: Val::Undefined,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Px(8.),
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Example Title".to_string(),
                        font,
                        style: TextStyle {
                            font_size: 25.,
                            color: Color::BLUE,
                        },
                    },
                    ..Default::default()
                });
                let content_entity = commands.current_entity().unwrap();
                commands.push_children(entity, &[content_entity]);
                nine_patch_content.loaded = true;
            }
            Content::InnerPanel => {
                // prepare the button
                let button_texture_handle = asset_server
                    .load_sync(&mut textures, "assets/blue_button02.png")
                    .unwrap();
                let button_ok_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(
                    5.,
                    10.,
                    6.,
                    6.,
                    Content::ButtonOK,
                ));
                let button_cancel_nine_patch_handle = nine_patches.add(
                    NinePatchBuilder::by_margins(5., 10., 6., 6., Content::ButtonCancel),
                );

                commands.spawn(
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

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        nine_patch_data: NinePatchData {
                            nine_patch: button_cancel_nine_patch_handle,
                            texture: button_texture_handle,
                            ..Default::default()
                        },
                        nine_patch_size: NinePatchSize(Vec2::new(300., 80.)),
                        ..Default::default()
                    },
                );
                let button_cancel_entity = commands.current_entity().unwrap();

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
                            ..Default::default()
                        },
                        nine_patch_data: NinePatchData {
                            nine_patch: button_ok_nine_patch_handle,
                            texture: button_texture_handle,
                            ..Default::default()
                        },
                        nine_patch_size: NinePatchSize(Vec2::new(300., 80.)),
                        ..Default::default()
                    },
                );
                let button_ok_entity = commands.current_entity().unwrap();

                commands.push_children(entity, &[button_cancel_entity, button_ok_entity]);
                nine_patch_content.loaded = true;
            }
            Content::ButtonOK => {
                // load font
                let font = asset_server
                    .load("assets/Kenney Future Narrow.ttf")
                    .expect("was able to load font");

                commands.spawn(TextComponents {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(110.),
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Px(10.),
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
            Content::ButtonCancel => {
                // load font
                let font = asset_server
                    .load("assets/Kenney Future Narrow.ttf")
                    .expect("was able to load font");

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
                        font,
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

enum ResizableContent {
    Panel,
    InnerPanel,
}

#[derive(Clone)]
enum Content {
    PanelTitle,
    PanelBody,
    InnerPanel,
    ButtonOK,
    ButtonCancel,
}

// by changing the component `NinePatchSize`, the 9-Patch UI element will be resized
fn update_size(time: Res<Time>, mut size: Mut<NinePatchSize>, panel: &ResizableContent) {
    let (x, y) = time.seconds_since_startup.sin_cos();

    match panel {
        ResizableContent::Panel => {
            size.0.set_x((900. + 50. * x as f32).ceil());
            size.0.set_y((600. + 50. * y as f32).ceil());
        }
        ResizableContent::InnerPanel => {
            size.0.set_x((850. + 50. * x as f32).ceil());
            size.0.set_y((550. + 50. * y as f32).ceil());
        }
    }
}
