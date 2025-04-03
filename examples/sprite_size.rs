use bevy::{color::palettes, prelude::*};

use bevy_easings::Ease;

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

    let initial_size = 10.;
    let final_size = 140.;

    let spacing = 1.15;
    let screen_x = window.single().unwrap().width() / 2.0;
    let screen_y = window.single().unwrap().height() / 2.0;
    let mut x = -screen_x;
    let mut y = screen_y;

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
            Transform::from_translation(Vec3::new(x + final_size / 2.0, y - final_size / 2.0, 0.)),
            Sprite {
                custom_size: Some(Vec2::new(initial_size, initial_size)),
                color: palettes::basic::RED.into(),
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(final_size, final_size)),
                    color: palettes::basic::RED.into(),
                    ..Default::default()
                },
                *ease_function,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(2),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            )
            .with_original_value(),
        ));

        commands.spawn((
            Text2d::new(format!("{:?}", ease_function)),
            Transform::from_translation(Vec3::new(x + final_size / 2.0, y - final_size / 2.0, 1.)),
            TextFont {
                font_size: 16.0,
                ..Default::default()
            },
        ));

        x += final_size * spacing;
        if x + final_size > screen_x {
            x = -screen_x;
            y -= final_size * spacing;
        }
    }
}
