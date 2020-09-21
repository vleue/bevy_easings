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
            NinePatchBuilder {
                patches: vec![
                    vec![
                        Patch {
                            width: PatchSize::Absolute(30.),
                            height: PatchSize::Absolute(35.),
                            x_growth: GrowthMode::None,
                            y_growth: GrowthMode::None,
                            content: None,
                        },
                        Patch {
                            width: PatchSize::Absolute(15.),
                            height: PatchSize::Absolute(35.),
                            x_growth: GrowthMode::StretchRatio(0.3),
                            y_growth: GrowthMode::None,
                            content: Some(PanelContent::Title),
                        },
                        Patch {
                            width: PatchSize::Absolute(25.),
                            height: PatchSize::Absolute(35.),
                            x_growth: GrowthMode::None,
                            y_growth: GrowthMode::None,
                            content: None,
                        },
                        Patch {
                            width: PatchSize::Absolute(20.),
                            height: PatchSize::Absolute(35.),
                            x_growth: GrowthMode::StretchRatio(0.7),
                            y_growth: GrowthMode::None,
                            content: None,
                        },
                        Patch {
                            width: PatchSize::Absolute(10.),
                            height: PatchSize::Absolute(35.),
                            x_growth: GrowthMode::None,
                            y_growth: GrowthMode::None,
                            content: None,
                        },
                    ],
                    vec![
                        Patch {
                            width: PatchSize::Absolute(10.),
                            height: PatchSize::Height { offset: -45. },
                            x_growth: GrowthMode::None,
                            y_growth: GrowthMode::StretchRatio(1.),
                            content: None,
                        },
                        Patch {
                            width: PatchSize::Width { offset: -20. },
                            height: PatchSize::Height { offset: -45. },
                            x_growth: GrowthMode::StretchRatio(1.),
                            y_growth: GrowthMode::StretchRatio(1.),
                            content: Some(PanelContent::Body),
                        },
                        Patch {
                            width: PatchSize::Absolute(10.),
                            height: PatchSize::Height { offset: -45. },
                            x_growth: GrowthMode::None,
                            y_growth: GrowthMode::StretchRatio(1.),
                            content: None,
                        },
                    ],
                    vec![
                        Patch {
                            width: PatchSize::Absolute(10.),
                            height: PatchSize::Absolute(10.),
                            x_growth: GrowthMode::None,
                            y_growth: GrowthMode::None,
                            content: None,
                        },
                        Patch {
                            width: PatchSize::Width { offset: -20. },
                            height: PatchSize::Absolute(10.),
                            x_growth: GrowthMode::StretchRatio(1.),
                            y_growth: GrowthMode::None,
                            content: None,
                        },
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
            .add(global_parent, 900., 600., |inside, part| match part {
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
            });
        });

    commands.spawn(UiCameraComponents::default());
}

#[derive(Clone)]
enum PanelContent {
    Title,
    Body,
}
