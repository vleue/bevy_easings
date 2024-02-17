use bevy::{ecs::component::Component, prelude::*};

use crate::{EasingDirection, MyEaser};

use crate::{
    CustomComponentEase, Ease, EaseValue, EasingChainComponent, EasingComponent, EasingState,
    EasingType,
};

/// Plugin to add systems related to easing
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct EasingsPlugin;

impl Plugin for EasingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ease_system::<Transform>);
        #[cfg(feature = "sprite")]
        app.add_systems(Update, ease_system::<Sprite>);
        #[cfg(feature = "ui")]
        app.add_systems(Update, ease_system::<Style>);
        #[cfg(feature = "ui")]
        app.add_systems(Update, ease_system::<BackgroundColor>);
        #[cfg(feature = "ui")]
        app.add_systems(Update, ease_system::<BorderColor>);
    }
}

pub fn ease_system<T: Ease + Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut T)>,
    mut easing_query: Query<&mut EasingComponent<T>>,
    mut chain_query: Query<&mut EasingChainComponent<T>>,
) where
    EaseValue<T>: interpolation::Lerp<Scalar = f32>,
    T: Default,
{
    for (entity, mut object) in query.iter_mut() {
        if let Ok(ref mut easing) = easing_query.get_mut(entity) {
            if easing.state == EasingState::Play {
                easing.timer.tick(time.delta());
            }
            if easing.paused {
                if easing.timer.just_finished() {
                    match easing.easing_type {
                        EasingType::Once { duration }
                        | EasingType::Loop { duration, .. }
                        | EasingType::PingPong { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                    }
                    easing.timer.reset();
                    easing.paused = false;
                }
            } else {
                if easing.timer.duration().as_secs_f32() != 0. {
                    let progress = if easing.direction == EasingDirection::Forward {
                        easing.timer.percent()
                    } else {
                        easing.timer.percent_left()
                    };
                    let factor = progress.compute(easing.ease_function);
                    if let Some(ref start) = easing.start {
                        *object = interpolation::lerp(start, &easing.end, &factor).0;
                    } else {
                        *object =
                            interpolation::lerp(&EaseValue(T::default()), &easing.end, &factor).0;
                    }
                }
                if easing.timer.finished() {
                    match easing.easing_type {
                        EasingType::Once { .. } => {
                            commands.entity(entity).remove::<EasingComponent<T>>();
                        }
                        EasingType::Loop { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                        }
                        EasingType::PingPong { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                            easing.direction.reverse();
                        }
                    }
                }
            }
        } else if let Ok(ref mut easing_chain) = chain_query.get_mut(entity) {
            let next = easing_chain.0.pop();
            if let Some(mut next) = next {
                if next.start.is_none() {
                    next.start = Some(EaseValue(std::mem::take(&mut object)));
                }
                if let Some(ref start) = next.start {
                    *object = interpolation::lerp(start, &next.end, &0.).0;
                } else {
                    *object = interpolation::lerp(&EaseValue(T::default()), &next.end, &0.).0;
                }

                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<T>>();
            }
        }
    }
}

/// Ease system for custom component. Add this system to your application with your component as a type parameter.
pub fn custom_ease_system<T: CustomComponentEase + Component>(
    mut commands: Commands,
    time: Res<Time>,
    entity_query: Query<Entity, With<T>>,
    mut object_query: Query<&mut T>,
    mut easing_query: Query<&mut EasingComponent<T>>,
    mut chain_query: Query<&mut EasingChainComponent<T>>,
) where
    T: interpolation::Lerp<Scalar = f32> + Default,
{
    for entity in entity_query.iter() {
        if let Ok(ref mut easing) = easing_query.get_mut(entity) {
            if easing.state == EasingState::Play {
                easing.timer.tick(time.delta());
            }
            if easing.paused {
                if easing.timer.just_finished() {
                    match easing.easing_type {
                        EasingType::Once { duration }
                        | EasingType::Loop { duration, .. }
                        | EasingType::PingPong { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                    }
                    easing.timer.reset();
                    easing.paused = false;
                }
            } else {
                if easing.timer.duration().as_secs_f32() != 0. {
                    let mut object = object_query.get_mut(entity).unwrap();

                    let progress = if easing.direction == EasingDirection::Forward {
                        easing.timer.percent()
                    } else {
                        easing.timer.percent_left()
                    };
                    let factor = progress.compute(easing.ease_function);
                    if let Some(ref start) = easing.start {
                        *object = interpolation::lerp(&start.0, &easing.end.0, &factor);
                    } else {
                        *object = interpolation::lerp(&T::default(), &easing.end.0, &factor);
                    }
                }
                if easing.timer.finished() {
                    match easing.easing_type {
                        EasingType::Once { .. } => {
                            commands.entity(entity).remove::<EasingComponent<T>>();
                        }
                        EasingType::Loop { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                        }
                        EasingType::PingPong { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                            easing.direction.reverse();
                        }
                    }
                }
            }
        } else if let Ok(ref mut easing_chain) = chain_query.get_mut(entity) {
            let next = easing_chain.0.pop();
            if let Some(mut next) = next {
                let mut object = object_query.get_mut(entity).unwrap();

                if next.start.is_none() {
                    next.start = Some(EaseValue(std::mem::take(&mut object)));
                }
                if let Some(ref start) = next.start {
                    *object = interpolation::lerp(&start.0, &next.end.0, &0.);
                } else {
                    *object = interpolation::lerp(&T::default(), &next.end.0, &0.);
                }

                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<T>>();
            }
        }
    }
}
