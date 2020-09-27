use std::time::Duration;

use rand::Rng;

use bevy::prelude::*;

pub use interpolation::EaseFunction;
pub use interpolation::Lerp;

mod plugin;
pub use plugin::EasingsPlugin;
mod implemented;

#[derive(Debug)]
pub struct EaseValue<T>(pub T);

impl<T> Ease for Handle<T> where EaseValue<T>: interpolation::Lerp<Scalar = f32> {}
impl<T> Ease for T where EaseValue<T>: interpolation::Lerp<Scalar = f32> {}

pub enum AnimationType {
    Once { duration: Duration },
    Loop { duration: Duration, pause: Duration },
    PingPong { duration: Duration, pause: Duration },
}

pub struct EasingComponent<T> {
    start: EaseValue<T>,
    end: EaseValue<T>,
    ease_function: interpolation::EaseFunction,
    timer: Timer,
    paused: bool,
    animation_type: AnimationType,
    id: i128,
    direction: i16,
}

pub trait Ease: Sized {
    fn ease(
        start: Self,
        end: Self,
        ease_function: interpolation::EaseFunction,
        animation_type: AnimationType,
    ) -> EasingComponent<Self> {
        let mut rng = rand::thread_rng();
        EasingComponent {
            start: EaseValue(start),
            end: EaseValue(end),
            ease_function,
            timer: match animation_type {
                AnimationType::Once { duration } => Timer::new(duration, false),
                AnimationType::Loop { duration, .. } => Timer::new(duration, false),
                AnimationType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            paused: false,
            animation_type,
            id: rng.gen(),
            direction: 1,
        }
    }
    fn ease_to(
        self,
        target: Self,
        ease_function: interpolation::EaseFunction,
        animation_type: AnimationType,
    ) -> EasingComponent<Self> {
        Self::ease(self, target, ease_function, animation_type)
    }
}

trait MyLerp: Sized {
    fn lerp(start: EaseValue<&Self>, end: EaseValue<&Self>, scalar: f32) -> Self;
}
