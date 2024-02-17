use bevy::prelude::*;

use bevy_easings::*;

const CUBE_SIZE: f32 = 25.;

const SCREEN_X: f32 = 570.;
const SCREEN_Y: f32 = 300.;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .init_resource::<EasingList>()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
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
            EaseMethod::EaseFunction(EaseFunction::QuadraticIn),
            EaseMethod::EaseFunction(EaseFunction::QuadraticOut),
            EaseMethod::EaseFunction(EaseFunction::QuadraticInOut),
            EaseMethod::EaseFunction(EaseFunction::CubicIn),
            EaseMethod::EaseFunction(EaseFunction::CubicOut),
            EaseMethod::EaseFunction(EaseFunction::CubicInOut),
            EaseMethod::EaseFunction(EaseFunction::QuarticIn),
            EaseMethod::EaseFunction(EaseFunction::QuarticOut),
            EaseMethod::EaseFunction(EaseFunction::QuarticInOut),
            EaseMethod::EaseFunction(EaseFunction::QuinticIn),
            EaseMethod::EaseFunction(EaseFunction::QuinticOut),
            EaseMethod::EaseFunction(EaseFunction::QuinticInOut),
            EaseMethod::EaseFunction(EaseFunction::SineIn),
            EaseMethod::EaseFunction(EaseFunction::SineOut),
            EaseMethod::EaseFunction(EaseFunction::SineInOut),
            EaseMethod::EaseFunction(EaseFunction::CircularIn),
            EaseMethod::EaseFunction(EaseFunction::CircularOut),
            EaseMethod::EaseFunction(EaseFunction::CircularInOut),
            EaseMethod::EaseFunction(EaseFunction::ExponentialIn),
            EaseMethod::EaseFunction(EaseFunction::ExponentialOut),
            EaseMethod::EaseFunction(EaseFunction::ExponentialInOut),
            EaseMethod::EaseFunction(EaseFunction::ElasticIn),
            EaseMethod::EaseFunction(EaseFunction::ElasticOut),
            EaseMethod::EaseFunction(EaseFunction::ElasticInOut),
            EaseMethod::EaseFunction(EaseFunction::BackIn),
            EaseMethod::EaseFunction(EaseFunction::BackOut),
            EaseMethod::EaseFunction(EaseFunction::BackInOut),
            EaseMethod::EaseFunction(EaseFunction::BounceIn),
            EaseMethod::EaseFunction(EaseFunction::BounceOut),
            EaseMethod::EaseFunction(EaseFunction::BounceInOut),
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());

    let default_cube = AnimatedCube::default();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CUBE_SIZE, CUBE_SIZE)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
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

    commands.spawn((Text2dBundle {
        text: Text::from_section(
            format_info_text(&Vec3::ZERO, 0, None, EaseMethod::Linear),
            TextStyle {
                font,
                color: Color::WHITE,
                font_size: 18.0,
            },
        )
        .with_alignment(TextAlignment::Right),
        transform: Transform::from_translation(Vec3::new(SCREEN_X, 15., 0.)),
        text_anchor: bevy::sprite::Anchor::CenterRight,
        ..Default::default()
    },));
}

fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    ease_functions: Res<EasingList>,
    mut anim_query: Query<(Entity, &mut EasingComponent<Transform>, &mut AnimatedCube)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((_, mut easing, _)) = anim_query.get_single_mut() {
            easing.state = !easing.state;
        }
    }

    let right_pressed = keyboard_input.just_pressed(KeyCode::Right);
    let left_pressed = keyboard_input.just_pressed(KeyCode::Left);
    let up_pressed = keyboard_input.just_pressed(KeyCode::Up);
    let down_pressed = keyboard_input.just_pressed(KeyCode::Down);
    let r_pressed = keyboard_input.just_pressed(KeyCode::R);

    if right_pressed || left_pressed || up_pressed || down_pressed || r_pressed {
        if let Ok((entity, easing, mut cube)) = anim_query.get_single_mut() {
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
    mut text_query: Query<&mut Text>,
    ease_functions: Res<EasingList>,
    cube_query: Query<(Entity, &Transform, &AnimatedCube)>,
    anim_query: Query<&EasingComponent<Transform>, With<AnimatedCube>>,
) {
    for (entity, transform, animated_cube) in cube_query.iter() {
        for mut text in text_query.iter_mut() {
            let easing_state = anim_query.get(entity).map(|anim| anim.state).ok();
            text.sections[0].value = format_info_text(
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
