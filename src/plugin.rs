use bevy::{ecs::component::Component, prelude::*};

use crate::{EasingDirection, MyEaser};

use crate::{
    CustomComponentEase, Ease, EaseValue, EasingChainComponent, EasingComponent, EasingState,
    EasingType,
};

/// Plugin to add systems related to easing
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct EasingsPlugin<Time: Default = ()> {
    _marker: std::marker::PhantomData<Time>,
}

impl Default for EasingsPlugin<()> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Default> EasingsPlugin<T> {
    /// Create a new instance of the plugin
    pub fn with_time() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

/// Label to coordinate new easing spawns
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct EasingsLabel;

impl<T: Default + Send + Sync + 'static> Plugin for EasingsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ease_system::<T, Transform>.in_set(EasingsLabel));
        #[cfg(feature = "sprite")]
        app.add_systems(Update, ease_system::<T, Sprite>.in_set(EasingsLabel));
        #[cfg(feature = "ui")]
        app.add_systems(Update, ease_system::<T, Node>.in_set(EasingsLabel));
        #[cfg(feature = "ui")]
        app.add_systems(
            Update,
            ease_system::<T, BackgroundColor>.in_set(EasingsLabel),
        );
        #[cfg(feature = "ui")]
        app.add_systems(Update, ease_system::<T, BorderColor>.in_set(EasingsLabel));
        #[cfg(feature = "ui")]
        app.add_systems(Update, ease_system::<T, TextColor>.in_set(EasingsLabel));
    }
}

pub fn ease_system<T: Default + Send + Sync + 'static, C: Ease + Component + Default>(
    mut commands: Commands,
    time: Res<Time<T>>,
    entity_query: Query<Entity, With<C>>,
    mut object_query: Query<&mut C>,
    mut easing_query: Query<&mut EasingComponent<C>>,
    mut chain_query: Query<&mut EasingChainComponent<C>>,
) where
    EaseValue<C>: interpolation::Lerp<Scalar = f32>,
{
    for entity in entity_query.iter() {
        if let Ok(ref mut easing) = easing_query.get_mut(entity) {
            if easing.state == EasingState::Play {
                easing.timer.tick(time.delta());
            } else {
                continue;
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
                        easing.timer.fraction()
                    } else {
                        easing.timer.fraction_remaining()
                    };
                    let factor = progress.compute(easing.ease_function);
                    if let Some(ref start) = easing.start {
                        *object = interpolation::lerp(start, &easing.end, &factor).0;
                    } else {
                        *object =
                            interpolation::lerp(&EaseValue(C::default()), &easing.end, &factor).0;
                    }
                }
                if easing.timer.finished() {
                    match easing.easing_type {
                        EasingType::Once { .. } => {
                            commands.entity(entity).remove::<EasingComponent<C>>();
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
                    *object = interpolation::lerp(start, &next.end, &0.).0;
                } else {
                    *object = interpolation::lerp(&EaseValue(C::default()), &next.end, &0.).0;
                }

                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<C>>();
            }
        }
    }
}

/// Ease system for custom component. Add this system to your application with your component as a type parameter.
pub fn custom_ease_system<
    T: Default + Send + Sync + 'static,
    C: CustomComponentEase + Component + interpolation::Lerp<Scalar = f32> + Default,
>(
    mut commands: Commands,
    time: Res<Time<T>>,
    entity_query: Query<Entity, With<C>>,
    mut object_query: Query<&mut C>,
    mut easing_query: Query<&mut EasingComponent<C>>,
    mut chain_query: Query<&mut EasingChainComponent<C>>,
) {
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
                        easing.timer.fraction()
                    } else {
                        easing.timer.fraction_remaining()
                    };
                    let factor = progress.compute(easing.ease_function);
                    if let Some(ref start) = easing.start {
                        *object = interpolation::lerp(&start.0, &easing.end.0, &factor);
                    } else {
                        *object = interpolation::lerp(&C::default(), &easing.end.0, &factor);
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
                    *object = interpolation::lerp(&C::default(), &next.end.0, &0.);
                }

                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<C>>();
            }
        }
    }
}
