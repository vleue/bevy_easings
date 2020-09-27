use bevy::prelude::*;

use interpolation::Ease as IEase;

use crate::{AnimationType, Ease, EaseValue, EasingComponent, MyLerp};

#[derive(Default)]
struct HandleCache<T: 'static>(std::collections::HashMap<i128, Handle<T>>);

#[derive(Debug, Clone, Copy)]
pub struct EasingsPlugin;

impl Plugin for EasingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(ease_system::<Sprite>.system())
            .add_system(ease_system::<ColorMaterial>.system())
            .add_system(ease_system::<Color>.system())
            .add_system(ease_system::<Transform>.system())
            .init_resource::<HandleCache<ColorMaterial>>()
            .add_system(handle_ease_system::<ColorMaterial>.system());
    }
}

pub fn ease_system<T: Ease + Component>(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut easing: Mut<EasingComponent<T>>,
    mut object: Mut<T>,
) where
    EaseValue<T>: interpolation::Lerp<Scalar = f32>,
{
    easing.timer.tick(time.delta_seconds);
    if easing.paused {
        if easing.timer.just_finished {
            match easing.animation_type {
                AnimationType::Once { duration } => easing.timer.duration = duration.as_secs_f32(),
                AnimationType::Loop { duration, .. } => {
                    easing.timer.duration = duration.as_secs_f32()
                }
                AnimationType::PingPong { duration, .. } => {
                    easing.timer.duration = duration.as_secs_f32()
                }
            }
            easing.timer.reset();
            easing.paused = false;
        }
    } else {
        let progress = if easing.direction.is_positive() {
            easing.timer.elapsed / easing.timer.duration
        } else {
            1. - easing.timer.elapsed / easing.timer.duration
        };
        let factor = progress.calc(easing.ease_function);
        *object = interpolation::lerp(&easing.start, &easing.end, &factor).0;
        if easing.timer.finished {
            match easing.animation_type {
                AnimationType::Once { .. } => {
                    commands.remove_one::<EasingComponent<T>>(entity);
                }
                AnimationType::Loop { pause, .. } => {
                    easing.timer.duration = pause.as_secs_f32();
                    easing.timer.reset();
                    easing.paused = true;
                }
                AnimationType::PingPong { pause, .. } => {
                    easing.timer.duration = pause.as_secs_f32();
                    easing.timer.reset();
                    easing.paused = true;
                    easing.direction = easing.direction * -1;
                }
            }
        }
    }
}

fn handle_ease_system<T: Ease + Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut assets: ResMut<Assets<T>>,
    mut handle_cache: ResMut<HandleCache<T>>,
    entity: Entity,
    mut easing: Mut<EasingComponent<Handle<T>>>,
    mut object: Mut<Handle<T>>,
) where
    EaseValue<T>: interpolation::Lerp<Scalar = f32>,
    T: MyLerp,
{
    easing.timer.tick(time.delta_seconds);
    if easing.paused {
        if easing.timer.just_finished {
            match easing.animation_type {
                AnimationType::Once { duration } => easing.timer.duration = duration.as_secs_f32(),
                AnimationType::Loop { duration, .. } => {
                    easing.timer.duration = duration.as_secs_f32()
                }
                AnimationType::PingPong { duration, .. } => {
                    easing.timer.duration = duration.as_secs_f32()
                }
            }
            easing.timer.reset();
            easing.paused = false;
        }
    } else {
        let progress = if easing.direction.is_positive() {
            easing.timer.elapsed / easing.timer.duration
        } else {
            1. - easing.timer.elapsed / easing.timer.duration
        };
        let factor = progress.calc(easing.ease_function);
        let factor_simplified = (factor * 25.) as i16;
        let handle = handle_cache
            .0
            .entry(easing.id + (easing.direction * factor_simplified) as i128)
            .or_insert_with(|| {
                let start = assets.get(&easing.start.0).unwrap().clone();
                let end = assets.get(&easing.end.0).unwrap().clone();
                let intermediate = MyLerp::lerp(EaseValue(start), EaseValue(end), factor);

                assets.add(intermediate)
            })
            .clone();
        *object = handle;
        if easing.timer.finished {
            match easing.animation_type {
                AnimationType::Once { .. } => {
                    commands.remove_one::<EasingComponent<T>>(entity);
                }
                AnimationType::Loop { pause, .. } => {
                    easing.timer.duration = pause.as_secs_f32();
                    easing.timer.reset();
                    easing.paused = true;
                }
                AnimationType::PingPong { pause, .. } => {
                    easing.timer.duration = pause.as_secs_f32();
                    easing.timer.reset();
                    easing.paused = true;
                    easing.direction = easing.direction * -1;
                }
            }
        }
    }
}
