use bevy::prelude::*;

use interpolation::Lerp;

use crate::EaseValue;

#[cfg(feature = "sprite")]
impl Lerp for EaseValue<Sprite> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Sprite {
            custom_size: match (self.0.custom_size, other.0.custom_size) {
                (None, None) => None,
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => Some(a.lerp(b, *scalar)),
            },
            #[cfg(feature = "render")]
            color: EaseValue(self.0.color)
                .lerp(&EaseValue(other.0.color), scalar)
                .0,
            ..Sprite::default()
        })
    }
}

#[cfg(all(feature = "ui", feature = "render"))]
impl Lerp for EaseValue<UiColor> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(UiColor(
            EaseValue(self.0 .0).lerp(&EaseValue(other.0 .0), scalar).0,
        ))
    }
}

impl Lerp for EaseValue<Transform> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Transform {
            translation: self.0.translation.lerp(other.0.translation, *scalar),
            scale: self.0.scale.lerp(other.0.scale, *scalar),
            rotation: self.0.rotation.lerp(other.0.rotation, *scalar),
        })
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<Style> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Style {
            position: EaseValue(self.0.position)
                .lerp(&EaseValue(other.0.position), scalar)
                .0,
            margin: EaseValue(self.0.margin)
                .lerp(&EaseValue(other.0.margin), scalar)
                .0,
            padding: EaseValue(self.0.padding)
                .lerp(&EaseValue(other.0.padding), scalar)
                .0,
            border: EaseValue(self.0.border)
                .lerp(&EaseValue(other.0.border), scalar)
                .0,
            size: EaseValue(self.0.size)
                .lerp(&EaseValue(other.0.size), scalar)
                .0,
            min_size: EaseValue(self.0.min_size)
                .lerp(&EaseValue(other.0.min_size), scalar)
                .0,
            max_size: EaseValue(self.0.max_size)
                .lerp(&EaseValue(other.0.max_size), scalar)
                .0,
            ..self.0
        })
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<Rect<Val>> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Rect {
            left: EaseValue(self.0.left)
                .lerp(&EaseValue(other.0.left), scalar)
                .0,
            right: EaseValue(self.0.right)
                .lerp(&EaseValue(other.0.right), scalar)
                .0,
            top: EaseValue(self.0.top)
                .lerp(&EaseValue(other.0.top), scalar)
                .0,
            bottom: EaseValue(self.0.bottom)
                .lerp(&EaseValue(other.0.bottom), scalar)
                .0,
        })
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<Size<Val>> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Size {
            width: EaseValue(self.0.width)
                .lerp(&EaseValue(other.0.width), scalar)
                .0,
            height: EaseValue(self.0.height)
                .lerp(&EaseValue(other.0.height), scalar)
                .0,
        })
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<Val> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        match (self.0, other.0) {
            (Val::Percent(self_val), Val::Percent(other_val)) => {
                EaseValue(Val::Percent(Lerp::lerp(&self_val, &other_val, scalar)))
            }
            (Val::Px(self_val), Val::Px(other_val)) => {
                EaseValue(Val::Px(Lerp::lerp(&self_val, &other_val, scalar)))
            }
            _ => EaseValue(self.0),
        }
    }
}

#[cfg(feature = "render")]
impl Lerp for EaseValue<Color> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(self.0 + (other.0 + (self.0 * -1.)) * *scalar)
    }
}
