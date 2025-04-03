use bevy::{color::palettes, prelude::*, sprite::Anchor};

use bevy_easings::{Ease, *};

const CUBE_SIZE: f32 = 25.;

const SCREEN_X: f32 = 570.;
const SCREEN_Y: f32 = 300.;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .init_resource::<EasingList>()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (update_text, handle_input))
        .add_systems(FixedUpdate, check_value)
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .run();

    Ok(())
}

#[derive(Resource)]
pub struct EasingList(Vec<EaseMethod>);

impl Default for EasingList {
    fn default() -> Self {
        Self(vec![
            EaseMethod::Linear,
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuadraticIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuadraticOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuadraticInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::CubicIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::CubicOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::CubicInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuarticIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuarticOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuarticInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuinticIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuinticOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::QuinticInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::SineIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::SineOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::SineInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::CircularIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::CircularOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::CircularInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::ExponentialIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::ExponentialOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::ExponentialInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::ElasticIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::ElasticOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::ElasticInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::BackIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::BackOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::BackInOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::BounceIn),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::BounceOut),
            EaseMethod::EaseFunction(bevy_easings::EaseFunction::BounceInOut),
        ])
    }
}

#[derive(Component)]
pub struct AnimatedCube {
    duration: u64,
    easing_id: usize,
}
impl Default for AnimatedCube {
    fn default() -> Self {
        Self {
            duration: 1000,
            easing_id: 0,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let default_cube = AnimatedCube::default();

    commands.spawn((
        Sprite::from_color(palettes::basic::RED, Vec2::new(CUBE_SIZE, CUBE_SIZE)),
        create_animated_transform(
            -SCREEN_X,
            SCREEN_Y,
            -SCREEN_Y,
            default_cube.duration,
            EasingState::Play,
            EaseMethod::Linear,
        ),
        default_cube,
    ));

    commands.spawn((
        Text2d::new("".to_string()),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(SCREEN_X, 15., 0.)),
        Anchor::CenterRight,
    ));
}

fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ease_functions: Res<EasingList>,
    mut anim_query: Query<(Entity, &mut EasingComponent<Transform>, &mut AnimatedCube)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((_, mut easing, _)) = anim_query.single_mut() {
            easing.state = !easing.state;
        }
    }

    let right_pressed = keyboard_input.just_pressed(KeyCode::ArrowRight);
    let left_pressed = keyboard_input.just_pressed(KeyCode::ArrowLeft);
    let up_pressed = keyboard_input.just_pressed(KeyCode::ArrowUp);
    let down_pressed = keyboard_input.just_pressed(KeyCode::ArrowDown);
    let r_pressed = keyboard_input.just_pressed(KeyCode::KeyR);

    if right_pressed || left_pressed || up_pressed || down_pressed || r_pressed {
        if let Ok((entity, easing, mut cube)) = anim_query.single_mut() {
            // Change easing
            if right_pressed || left_pressed {
                cube.easing_id = if right_pressed {
                    (cube.easing_id + 1) % ease_functions.0.len()
                } else {
                    (cube.easing_id + ease_functions.0.len() - 1) % ease_functions.0.len()
                };
            }

            // Change duration
            if up_pressed || down_pressed {
                cube.duration = if up_pressed {
                    cube.duration + 100
                } else {
                    (cube.duration - 100).max(100)
                };
            }

            // Reset cube
            if r_pressed {
                let default_cube = AnimatedCube::default();
                cube.duration = default_cube.duration;
                cube.easing_id = default_cube.easing_id;
            }

            // Replace animation
            commands.entity(entity).insert(create_animated_transform(
                -SCREEN_X,
                SCREEN_Y,
                -SCREEN_Y,
                cube.duration,
                easing.state,
                ease_functions.0[cube.easing_id],
            ));
        }
    }
}

fn check_value(cube_query: Query<&Transform, Changed<Transform>>) {
    for transform in cube_query.iter() {
        println!("new cube pos -> {:?}", transform.translation);
    }
}

fn update_text(
    text_query: Query<Entity, With<Text2d>>,
    ease_functions: Res<EasingList>,
    cube_query: Query<(Entity, &Transform, &AnimatedCube)>,
    anim_query: Query<&EasingComponent<Transform>, With<AnimatedCube>>,
    mut text_writer: Text2dWriter,
) {
    for (entity, transform, animated_cube) in cube_query.iter() {
        for text in &text_query {
            let easing_state = anim_query.get(entity).map(|anim| anim.state).ok();
            *text_writer.text(text, 0) = format_info_text(
                &transform.translation,
                animated_cube.duration,
                easing_state,
                ease_functions.0[animated_cube.easing_id],
            );
        }
    }
}

fn create_animated_transform(
    x: f32,
    start_y: f32,
    end_y: f32,
    duration: u64,
    initial_state: EasingState,
    ease_method: impl Into<EaseMethod>,
) -> EasingComponent<Transform> {
    let mut easing_component = Transform::from_translation(Vec3::new(x, start_y, 0.)).ease_to(
        Transform::from_translation(Vec3::new(x, end_y, 0.)),
        ease_method,
        EasingType::PingPong {
            duration: std::time::Duration::from_millis(duration),
            pause: Some(std::time::Duration::from_millis(500)),
        },
    );

    easing_component.state = initial_state;

    easing_component
}

const INSTRUCTIONS: &str =
    "Space to play/pause\nLeft/Right to change easing\nUp/Down to change duration\nR to reset";

fn format_info_text(
    translation: &Vec3,
    duration: u64,
    easing_state: Option<EasingState>,
    easing_method: EaseMethod,
) -> String {
    let text_state = easing_state.map_or("Stopped", |easing_state| match easing_state {
        EasingState::Play => "Playing",
        EasingState::Paused => "Paused",
    });
    let easing_mode = match easing_method {
        EaseMethod::EaseFunction(ease_func) => format!("{:?}", ease_func),
        EaseMethod::Linear => "Linear".to_string(),
        EaseMethod::Discrete => "Discrete".to_string(),
        EaseMethod::CustomFunction(_) => "Custom".to_string(),
    };
    let pos = format!("{:.2}x{:.2}", translation.x, translation.y);

    format!(
        "State: {}\nEasing Mode: {}\nDuration: {}ms\nValue: {}\n\n{}",
        text_state, easing_mode, duration, pos, INSTRUCTIONS
    )
}
