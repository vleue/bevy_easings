use bevy::prelude::*;

use bevy_easings::{Ease, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin::default())
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    commands.spawn(Camera2d);

    let color_pairs = [
        (Color::Srgba(Srgba::RED), Color::Srgba(Srgba::BLUE)),
        (
            Color::LinearRgba(LinearRgba::RED),
            Color::LinearRgba(LinearRgba::BLUE),
        ),
        (
            Color::Laba(Laba::new(0.5438, 0.8626, 0.7607, 1.0)),
            Color::Laba(Laba::new(0.3230, 0.7920, -1.0786, 1.0)),
        ),
        (
            Color::Oklaba(Oklaba::new(0.6279554, 0.22486295, 0.1258463, 1.0)),
            Color::Oklaba(Oklaba::new(0.4520137, -0.032456964, -0.31152815, 1.0)),
        ),
        (
            Color::Lcha(Lcha::new(0.53, 1.04576, 40.0, 1.0)),
            Color::Lcha(Lcha::new(0.32, 1.33816, 306.287, 1.0)),
        ),
        (
            Color::Oklcha(Oklcha::new(0.6279554, 0.25768322, 29.233896, 1.0)),
            Color::Oklcha(Oklcha::new(0.4520137, 0.31321436, 264.05203, 1.0)),
        ),
        (
            Color::Xyza(Xyza::new(0.4124564, 0.2126729, 0.0193339, 1.0)),
            Color::Xyza(Xyza::new(0.1804375, 0.072175, 0.9503041, 1.0)),
        ),
        (
            Color::Hsla(Hsla::new(0.0, 1.0, 0.5, 1.0)),
            Color::Hsla(Hsla::new(240.0, 1.0, 0.5, 1.0)),
        ),
        (
            Color::Hsva(Hsva::new(0.0, 1.0, 1.0, 1.0)),
            Color::Hsva(Hsva::new(240.0, 1.0, 1.0, 1.0)),
        ),
        (
            Color::Hwba(Hwba::new(0.0, 0.0, 0.0, 1.0)),
            Color::Hwba(Hwba::new(240.0, 0.0, 0.0, 1.0)),
        ),
    ];

    let size = window.single().height() / color_pairs.len() as f32;

    for (i, colors) in color_pairs.iter().enumerate() {
        let y = (color_pairs.len() as i32 / 2 - i as i32) as f32 * size - size / 2.0;
        commands.spawn((
            Transform::from_translation(Vec3::new(0.0, y, 0.)),
            Sprite {
                custom_size: Some(Vec2::new(size * 30.0, size)),
                color: colors.0,
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(size * 30.0, size)),
                    color: colors.1,
                    ..Default::default()
                },
                EaseMethod::Linear,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(2),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            )
            .with_original_value(),
        ));
        commands.spawn((
            Text2d::new(color_space_name(colors.0)),
            Transform::from_translation(Vec3::new(0.0, y, 1.)),
        ));
    }
}

fn color_space_name(color: Color) -> String {
    match color {
        Color::Srgba(_) => "Srgba",
        Color::LinearRgba(_) => "LinearRgba",
        Color::Hsla(_) => "Hsla",
        Color::Hsva(_) => "Hsva",
        Color::Hwba(_) => "Hwba",
        Color::Laba(_) => "Laba",
        Color::Lcha(_) => "Lcha",
        Color::Oklaba(_) => "Oklaba",
        Color::Oklcha(_) => "Oklcha",
        Color::Xyza(_) => "Xyza",
    }
    .to_string()
}
