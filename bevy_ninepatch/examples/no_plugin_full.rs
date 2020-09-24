use bevy::prelude::*;

use bevy_ninepatch::*;

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
    // load textures and font
    let font = asset_server
        .load("assets/Kenney Future Narrow.ttf")
        .expect("was able to load font");

    let cornered_panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/metalPanel_yellowCorner.png")
        .unwrap();

    let button_texture_handle = asset_server
        .load_sync(&mut textures, "assets/blue_button02.png")
        .unwrap();

    let color_none_handle = materials.add(Color::NONE.into());

    // create the button 9-Patch in advance. This way it can be used without complications in a closure
    let button = bevy_ninepatch::NinePatchBuilder::by_margins(5., 10., 6., 6., ()).apply(
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
            // this time, we create the 9-Patch UI element by specifying all the patch. Notice that the top row of patch
            // actually has 5 patches instead of 3
            NinePatchBuilder {
                patches: vec![
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
                            content: Some(PanelContent::Title),
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
                            content: Some(PanelContent::Body),
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
                ],
            }
            .apply(cornered_panel_texture_handle, &mut textures, &mut materials)
            .add(
                global_parent,
                900.,
                600.,
                // closure to specify the content. As we created two patch that accept content, each with it's own
                // value of enum `PanelContent`, we can now decide what content will get into each patch
                |inside, part| match part {
                    // for the Title patch, just add a `TextComponents`
                    PanelContent::Title => {
                        inside.spawn(TextComponents {
                            text: Text {
                                value: "Example Title".to_string(),
                                font,
                                style: TextStyle {
                                    font_size: 25.,
                                    color: Color::BLUE,
                                },
                            },
                            style: Style {
                                margin: Rect::all(Val::Auto),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    }
                    // for the Body, display two buttons each with a text inside, separated by a spacer
                    PanelContent::Body => {
                        button.add(inside, 400., 100., |in_button, _| {
                            in_button.spawn(TextComponents {
                                text: Text {
                                    value: "Cancel".to_string(),
                                    font,
                                    style: TextStyle {
                                        font_size: 50.,
                                        color: Color::RED,
                                    },
                                },
                                style: Style {
                                    margin: Rect::all(Val::Auto),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                        inside.spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Px(100.), Val::Auto),
                                ..Default::default()
                            },
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            material: color_none_handle,
                            ..Default::default()
                        });
                        button.add(inside, 400., 100., |in_button, _| {
                            in_button.spawn(TextComponents {
                                text: Text {
                                    value: "OK".to_string(),
                                    font,
                                    style: TextStyle {
                                        font_size: 50.,
                                        color: Color::GREEN,
                                    },
                                },
                                style: Style {
                                    margin: Rect::all(Val::Auto),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                    }
                },
            );
        });

    commands.spawn(UiCameraComponents::default());
}

#[derive(Clone, Debug)]
enum PanelContent {
    Title,
    Body,
}
