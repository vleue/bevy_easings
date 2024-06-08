use bevy::{color::palettes, prelude::*};

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let size = 140.;

    let spacing = 1.15;
    let screen_x = window.single().width();
    let screen_y = window.single().height();
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
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    x + size / 2.0,
                    y - size / 2.0,
                    0.,
                )),
                ..Default::default()
            },
            Sprite {
                custom_size: Some(Vec2::new(size, size)),
                color: palettes::basic::RED.into(),
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(size, size)),
                    color: palettes::basic::BLUE.into(),
                    ..Default::default()
                },
                *ease_function,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(2),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            ),
        ));
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{:?}", ease_function),
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(x + size / 2.0, y - size / 2.0, 0.)),

            ..default()
        });
        x += size * spacing;
        if x + size > screen_x {
            x = -screen_x;
            y -= size * spacing;
        }
    }
}
