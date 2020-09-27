use bevy::prelude::*;

use crate::{EaseValue, MyLerp};

impl interpolation::Lerp for EaseValue<Sprite> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Sprite {
            size: self.0.size + (other.0.size - self.0.size) * *scalar,
            resize_mode: match self.0.resize_mode {
                SpriteResizeMode::Manual => SpriteResizeMode::Manual,
                SpriteResizeMode::Automatic => SpriteResizeMode::Automatic,
            },
        })
    }
}

impl interpolation::Lerp for EaseValue<Transform> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Transform::new(
            *self.0.value() + (*other.0.value() - *self.0.value()) * *scalar,
        ))
    }
}

impl interpolation::Lerp for EaseValue<ColorMaterial> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        if self.0.texture.is_none() {
            EaseValue(ColorMaterial {
                color: self.0.color + (other.0.color + (self.0.color * -1.)) * *scalar,
                texture: None,
            })
        } else {
            EaseValue(ColorMaterial {
                color: self.0.color,
                texture: self.0.texture.clone(),
            })
        }
    }
}

impl interpolation::Lerp for EaseValue<Color> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(self.0 + (other.0 + (self.0 * -1.)) * *scalar)
    }
}

impl MyLerp for ColorMaterial {
    fn lerp(start: EaseValue<&Self>, end: EaseValue<&Self>, scalar: f32) -> Self {
        if start.0.texture.is_none() {
            ColorMaterial {
                color: start.0.color + (end.0.color + (start.0.color * -1.)) * scalar,
                texture: None,
            }
        } else {
            ColorMaterial {
                color: start.0.color,
                texture: start.0.texture.clone(),
            }
        }
    }
}
