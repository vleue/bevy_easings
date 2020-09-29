use std::time::Duration;

use rand::Rng;

use bevy::prelude::*;

pub use interpolation::EaseFunction;
pub use interpolation::Lerp;

mod plugin;
pub use plugin::{custom_ease_system, EasingsPlugin};
mod implemented;

#[derive(Debug)]
pub struct EaseValue<T>(pub T);

pub enum AnimationType {
    Once { duration: Duration },
    Loop { duration: Duration, pause: Duration },
    PingPong { duration: Duration, pause: Duration },
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AnimationState {
    Play,
    Paused,
}

impl std::ops::Not for AnimationState {
    type Output = AnimationState;

    fn not(self) -> Self::Output {
        match self {
            AnimationState::Paused => AnimationState::Play,
            AnimationState::Play => AnimationState::Paused,
        }
    }
}

pub struct EasingComponent<T> {
    start: Option<EaseValue<T>>,
    end: EaseValue<T>,
    ease_function: interpolation::EaseFunction,
    timer: Timer,
    pub state: AnimationState,
    paused: bool,
    animation_type: AnimationType,
    id: i128,
    direction: i16,
}

impl<T: Default> EasingComponent<T> {
    pub fn ease_to(
        self,
        end: T,
        ease_function: interpolation::EaseFunction,
        animation_type: AnimationType,
    ) -> EasingComponentChain<T> {
        let mut rng = rand::thread_rng();

        let next = EasingComponent {
            start: None,
            end: EaseValue(end),
            ease_function,
            timer: match animation_type {
                AnimationType::Once { duration } => Timer::new(duration, false),
                AnimationType::Loop { duration, .. } => Timer::new(duration, false),
                AnimationType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: AnimationState::Play,
            paused: false,
            animation_type,
            id: rng.gen(),
            direction: 1,
        };

        EasingComponentChain(vec![next, self])
    }
}

pub struct EasingComponentChain<T>(Vec<EasingComponent<T>>);

impl<T: Default> EasingComponentChain<T> {
    pub fn ease_to(
        mut self,
        end: T,
        ease_function: interpolation::EaseFunction,
        animation_type: AnimationType,
    ) -> EasingComponentChain<T> {
        let mut rng = rand::thread_rng();

        let next = EasingComponent {
            start: None,
            end: EaseValue(end),
            ease_function,
            timer: match animation_type {
                AnimationType::Once { duration } => Timer::new(duration, false),
                AnimationType::Loop { duration, .. } => Timer::new(duration, false),
                AnimationType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: AnimationState::Play,
            paused: false,
            animation_type,
            id: rng.gen(),
            direction: 1,
        };

        self.0.insert(0, next);
        self
    }
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
            start: Some(EaseValue(start)),
            end: EaseValue(end),
            ease_function,
            timer: match animation_type {
                AnimationType::Once { duration } => Timer::new(duration, false),
                AnimationType::Loop { duration, .. } => Timer::new(duration, false),
                AnimationType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: AnimationState::Play,
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

impl<T> Ease for EaseValue<T> where T: interpolation::Lerp<Scalar = f32> {}
impl<T> Ease for Handle<T> where EaseValue<T>: interpolation::Lerp<Scalar = f32> {}
impl<T> Ease for T where EaseValue<T>: interpolation::Lerp<Scalar = f32> {}

impl<T> Default for EaseValue<T>
where
    T: Default,
{
    fn default() -> Self {
        EaseValue(T::default())
    }
}

trait IntermediateLerp: Sized {
    fn lerp(start: &EaseValue<&Self>, end: &EaseValue<&Self>, scalar: &f32) -> Self;
}

pub trait CustomComponentEase: Sized {
    fn ease(
        start: Self,
        end: Self,
        ease_function: interpolation::EaseFunction,
        animation_type: AnimationType,
    ) -> EasingComponent<Self> {
        let mut rng = rand::thread_rng();
        EasingComponent {
            start: Some(EaseValue(start)),
            end: EaseValue(end),
            ease_function,
            timer: match animation_type {
                AnimationType::Once { duration } => Timer::new(duration, false),
                AnimationType::Loop { duration, .. } => Timer::new(duration, false),
                AnimationType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: AnimationState::Play,
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

impl<T> CustomComponentEase for T where T: interpolation::Lerp<Scalar = f32> {}
