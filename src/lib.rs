#![warn(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

use std::time::Duration;

use bevy::prelude::*;

use interpolation::Ease as IEase;
pub use interpolation::EaseFunction;
pub use interpolation::Lerp;

mod plugin;
pub use plugin::{custom_ease_system, EasingsLabel, EasingsPlugin};
mod implemented;

/// Wrapper around a type that can be eased.
#[derive(Debug, Clone, Copy)]
pub struct EaseValue<T>(pub T);

/// How should this easing loop repeat
#[derive(Clone, Copy)]
pub enum EasingType {
    /// Only happen once
    Once {
        /// duration of the easing
        duration: Duration,
    },
    /// Looping, restarting from the start once finished
    Loop {
        /// duration of the easing
        duration: Duration,
        /// duration of the pause between two loops
        pause: Option<Duration>,
    },
    /// Repeat the animation back and forth
    PingPong {
        /// duration of the easing
        duration: Duration,
        /// duration of the pause before starting again in the other direction
        pause: Option<Duration>,
    },
}

/// Control if an easing is played
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EasingState {
    /// Play the easing
    Play,
    /// Pause the easing
    Paused,
}

impl std::ops::Not for EasingState {
    type Output = EasingState;

    fn not(self) -> Self::Output {
        match self {
            EasingState::Paused => EasingState::Play,
            EasingState::Play => EasingState::Paused,
        }
    }
}

/// Describe how eased value should be computed
#[derive(Clone, Copy)]
pub enum EaseMethod {
    /// Follow `EaseFunction`
    EaseFunction(EaseFunction),
    /// Linear interpolation, with no function
    Linear,
    /// Discrete interpolation, eased value will jump from start to end
    Discrete,
    /// Use a custom function to interpolate the value
    CustomFunction(fn(f32) -> f32),
}

#[allow(clippy::from_over_into)]
impl Into<EaseMethod> for EaseFunction {
    fn into(self) -> EaseMethod {
        EaseMethod::EaseFunction(self)
    }
}

trait MyEaser {
    fn compute(self, function: EaseMethod) -> Self;
}
impl MyEaser for f32 {
    fn compute(self, function: EaseMethod) -> f32 {
        match function {
            EaseMethod::EaseFunction(function) => self.calc(function),
            EaseMethod::Linear => {
                let delta = 0.01;
                if self < 0. + delta {
                    0.
                } else if self > 1. - delta {
                    1.
                } else {
                    self
                }
            }
            EaseMethod::Discrete => {
                if self > 0.5 {
                    1.
                } else {
                    0.
                }
            }
            EaseMethod::CustomFunction(function) => function(self),
        }
    }
}

/// Component to control an easing
#[derive(Component, Clone)]
pub struct EasingComponent<T> {
    start: Option<EaseValue<T>>,
    end: EaseValue<T>,
    ease_function: EaseMethod,
    timer: Timer,
    /// Control if this easing is played or not
    pub state: EasingState,
    paused: bool,
    easing_type: EasingType,
    direction: EasingDirection,
}

/// Direction of an easing. It can be backward with an [`EasingType::PingPong`]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EasingDirection {
    /// Easing is moving forward
    Forward = 1,
    /// Easing is moving backward
    Backward = -1,
}

impl EasingDirection {
    fn reverse(&mut self) {
        *self = match self {
            EasingDirection::Backward => EasingDirection::Forward,
            EasingDirection::Forward => EasingDirection::Backward,
        };
    }
}

impl<T> EasingComponent<T> {
    /// For [EasingType::PingPong], gets the current direction as -1 or 1.
    ///
    /// Positive is in the direction of the "ping" (first iteration).
    pub fn direction(&self) -> EasingDirection {
        self.direction
    }

    /// Adds a delay before triggering the easing.
    pub fn delay(mut self, duration: Duration) -> Self {
        self.paused = true;
        self.timer = Timer::new(duration, TimerMode::Once);
        self
    }
}

impl<T> EasingComponent<T>
where
    T: Clone,
{
    /// Returns a bundle containing the starting value additionally to this [EasingComponent].
    pub fn with_original_value(self) -> (T, Self) {
        let starting_value = self.start.clone().unwrap().0;

        (starting_value, self)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for EasingComponent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EasingComponent")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("state", &self.state)
            .finish()
    }
}

impl<T: Default> EasingComponent<T> {
    /// Start a chain of easing, adding a new one after the first one
    pub fn ease_to(
        self,
        end: T,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingChainComponent<T> {
        let next = EasingComponent {
            start: None,
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration }
                | EasingType::Loop { duration, .. }
                | EasingType::PingPong { duration, .. } => Timer::new(duration, TimerMode::Once),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            direction: EasingDirection::Forward,
        };

        EasingChainComponent(vec![next, self])
    }
}

/// Component to control a chain of easing
#[derive(Component)]
pub struct EasingChainComponent<T>(Vec<EasingComponent<T>>);

impl<T: Default> EasingChainComponent<T> {
    /// Add a new easing at the end of the current chain
    pub fn ease_to(
        mut self,
        end: T,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingChainComponent<T> {
        let next = EasingComponent {
            start: None,
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration }
                | EasingType::Loop { duration, .. }
                | EasingType::PingPong { duration, .. } => Timer::new(duration, TimerMode::Once),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            direction: EasingDirection::Forward,
        };

        self.0.insert(0, next);
        self
    }

    /// Repeat the chain `n` times.
    pub fn repeat(mut self, n: usize) -> EasingChainComponent<T>
    where
        EasingComponent<T>: Clone,
    {
        let mut tmp = self.0.clone();
        for _ in 1..n {
            tmp.extend(self.0.clone());
        }
        self.0 = tmp;
        self
    }
}

/// Trait marking components that can be eased
pub trait Ease: Sized {
    /// Create a new easing. If no start is provided, it will try to use the current value of the component for the target entity
    fn ease(
        start: Option<Self>,
        end: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        EasingComponent {
            start: start.map(EaseValue),
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration }
                | EasingType::Loop { duration, .. }
                | EasingType::PingPong { duration, .. } => Timer::new(duration, TimerMode::Once),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            direction: EasingDirection::Forward,
        }
    }

    /// Create a new easing with the current component value as a starting point
    fn ease_to(
        self,
        target: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        Self::ease(Some(self), target, ease_function, easing_type)
    }
}

impl<T> Ease for EaseValue<T> where T: Lerp<Scalar = f32> {}
impl<T> Ease for T where EaseValue<T>: Lerp<Scalar = f32> {}

impl<T> Default for EaseValue<T>
where
    T: Default,
{
    fn default() -> Self {
        EaseValue(T::default())
    }
}

/// Trait to mark custom component that can be eased. It will be automatically implemented if the custom component implement `Lerp`
pub trait CustomComponentEase: Sized {
    /// Create a new easing. If no start is provided, it will try to use the current value of the component for the target entity
    fn ease(
        start: Option<Self>,
        end: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        EasingComponent {
            start: start.map(EaseValue),
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration }
                | EasingType::Loop { duration, .. }
                | EasingType::PingPong { duration, .. } => Timer::new(duration, TimerMode::Once),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            direction: EasingDirection::Forward,
        }
    }

    /// Create a new easing with the current component value as a starting point
    fn ease_to(
        self,
        target: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        Self::ease(Some(self), target, ease_function, easing_type)
    }
}

impl<T> CustomComponentEase for T where T: Lerp<Scalar = f32> {}
