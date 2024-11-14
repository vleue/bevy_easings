use bevy::{color::palettes, prelude::*};

use bevy_easings::{Ease, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let size = 25.;

    let spacing = 1.5;
    let screen_x = 570.;
    let screen_y = 300.;
    let mut x = -screen_x;

    for ease_function in &[
        bevy_easings::EaseFunction::QuadraticIn,
        bevy_easings::EaseFunction::QuadraticOut,
        bevy_easings::EaseFunction::QuadraticInOut,
        bevy_easings::EaseFunction::CubicIn,
        bevy_easings::EaseFunction::CubicOut,
        bevy_easings::EaseFunction::CubicInOut,
        bevy_easings::EaseFunction::QuarticIn,
        bevy_easings::EaseFunction::QuarticOut,
        bevy_easings::EaseFunction::QuarticInOut,
        bevy_easings::EaseFunction::QuinticIn,
        bevy_easings::EaseFunction::QuinticOut,
        bevy_easings::EaseFunction::QuinticInOut,
        bevy_easings::EaseFunction::SineIn,
        bevy_easings::EaseFunction::SineOut,
        bevy_easings::EaseFunction::SineInOut,
        bevy_easings::EaseFunction::CircularIn,
        bevy_easings::EaseFunction::CircularOut,
        bevy_easings::EaseFunction::CircularInOut,
        bevy_easings::EaseFunction::ExponentialIn,
        bevy_easings::EaseFunction::ExponentialOut,
        bevy_easings::EaseFunction::ExponentialInOut,
        bevy_easings::EaseFunction::ElasticIn,
        bevy_easings::EaseFunction::ElasticOut,
        bevy_easings::EaseFunction::ElasticInOut,
        bevy_easings::EaseFunction::BackIn,
        bevy_easings::EaseFunction::BackOut,
        bevy_easings::EaseFunction::BackInOut,
        bevy_easings::EaseFunction::BounceIn,
        bevy_easings::EaseFunction::BounceOut,
        bevy_easings::EaseFunction::BounceInOut,
    ] {
        commands.spawn((
            Sprite::from_color(palettes::basic::RED, Vec2::new(size, size)),
            Transform::from_translation(Vec3::new(x, screen_y, 0.)).ease_to(
                Transform::from_translation(Vec3::new(x, -screen_y, 0.)),
                *ease_function,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(1),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            ),
        ));
        x += size * spacing;
    }

    commands.spawn((
        Sprite::from_color(palettes::basic::RED, Vec2::new(size, size)),
        Transform::from_translation(Vec3::new(x, screen_y, 0.)).ease_to(
            Transform::from_translation(Vec3::new(x, -screen_y, 0.)),
            EaseMethod::Linear,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: Some(std::time::Duration::from_millis(500)),
            },
        ),
    ));
    x += size * spacing;

    commands.spawn((
        Sprite::from_color(palettes::basic::RED, Vec2::new(size, size)),
        Transform::from_translation(Vec3::new(x, screen_y, 0.)).ease_to(
            Transform::from_translation(Vec3::new(x, -screen_y, 0.)),
            EaseMethod::Discrete,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: Some(std::time::Duration::from_millis(500)),
            },
        ),
    ));
    x += size * spacing;

    commands.spawn((
        Sprite::from_color(palettes::basic::RED, Vec2::new(size, size)),
        Transform::from_translation(Vec3::new(x, screen_y, 0.)).ease_to(
            Transform::from_translation(Vec3::new(x, -screen_y, 0.)),
            EaseMethod::CustomFunction(|x| x / 4.),
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: Some(std::time::Duration::from_millis(500)),
            },
        ),
    ));
}
