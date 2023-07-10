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
impl Lerp for EaseValue<BackgroundColor> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(BackgroundColor(
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
            margin: EaseValue(self.0.margin)
                .lerp(&EaseValue(other.0.margin), scalar)
                .0,
            padding: EaseValue(self.0.padding)
                .lerp(&EaseValue(other.0.padding), scalar)
                .0,
            border: EaseValue(self.0.border)
                .lerp(&EaseValue(other.0.border), scalar)
                .0,
            width: EaseValue(self.0.width)
                .lerp(&EaseValue(other.0.width), scalar)
                .0,
            height: EaseValue(self.0.height)
                .lerp(&EaseValue(other.0.height), scalar)
                .0,
            min_width: EaseValue(self.0.min_width)
                .lerp(&EaseValue(other.0.min_width), scalar)
                .0,
            min_height: EaseValue(self.0.min_height)
                .lerp(&EaseValue(other.0.min_height), scalar)
                .0,
            max_width: EaseValue(self.0.max_width)
                .lerp(&EaseValue(other.0.max_width), scalar)
                .0,
            max_height: EaseValue(self.0.max_height)
                .lerp(&EaseValue(other.0.max_height), scalar)
                .0,
            ..default()
        })
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<UiRect> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(UiRect {
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
        let color = match (self.0, other.0) {
            (
                Color::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::Rgba {
                    red: redo,
                    green: greeno,
                    blue: blueo,
                    alpha: alphao,
                },
            ) => Color::Rgba {
                red: red + (redo + (red * -1.0)) * *scalar,
                green: green + (greeno + (green * -1.0)) * *scalar,
                blue: blue + (blueo + (blue * -1.0)) * *scalar,
                alpha: alpha + (alphao + (alpha * -1.0)) * *scalar,
            },
            (
                Color::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::RgbaLinear {
                    red: redo,
                    green: greeno,
                    blue: blueo,
                    alpha: alphao,
                },
            ) => Color::RgbaLinear {
                red: red + (redo + (red * -1.0)) * *scalar,
                green: green + (greeno + (green * -1.0)) * *scalar,
                blue: blue + (blueo + (blue * -1.0)) * *scalar,
                alpha: alpha + (alphao + (alpha * -1.0)) * *scalar,
            },
            (
                Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Color::Hsla {
                    hue: hueo,
                    saturation: saturationo,
                    lightness: lightnesso,
                    alpha: alphao,
                },
            ) => Color::Hsla {
                hue: hue + (hueo + (hue * -1.0)) * *scalar,
                saturation: saturation + (saturationo + (saturation * -1.0)) * *scalar,
                lightness: lightness + (lightnesso + (lightness * -1.0)) * *scalar,
                alpha: alpha + (alphao + (alpha * -1.0)) * *scalar,
            },
            _ => self.0 + (other.0 + (self.0 * -1.)) * *scalar,
        };
        EaseValue(color)
    }
}
