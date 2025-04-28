#[cfg(feature = "color")]
use bevy_color::{
    Color, ColorToComponents, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza,
};
use bevy_math::Rect;
use bevy_transform::components::Transform;
#[cfg(feature = "ui")]
use bevy_ui::Val;
use interpolation::Lerp;

use crate::EaseValue;

#[cfg(feature = "sprite")]
impl Lerp for EaseValue<bevy_sprite::Sprite> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(bevy_sprite::Sprite {
            custom_size: match (self.0.custom_size, other.0.custom_size) {
                (None, None) => None,
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => Some(a.lerp(b, *scalar)),
            },
            rect: match (self.0.rect, other.0.rect) {
                (None, None) => None,
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => Some(EaseValue(a).lerp(&EaseValue(b), scalar).0),
            },
            #[cfg(feature = "render")]
            color: EaseValue(self.0.color)
                .lerp(&EaseValue(other.0.color), scalar)
                .0,
            ..self.0.clone()
        })
    }
}

#[cfg(all(feature = "ui", feature = "render"))]
impl Lerp for EaseValue<bevy_ui::BackgroundColor> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(bevy_ui::BackgroundColor(
            EaseValue(self.0.0).lerp(&EaseValue(other.0.0), scalar).0,
        ))
    }
}

#[cfg(all(feature = "ui", feature = "render"))]
impl Lerp for EaseValue<bevy_ui::BorderColor> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(bevy_ui::BorderColor(
            EaseValue(self.0.0).lerp(&EaseValue(other.0.0), scalar).0,
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
impl Lerp for EaseValue<bevy_ui::Node> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(bevy_ui::Node {
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
            grid_auto_columns: self.0.grid_auto_columns.clone(),
            grid_auto_rows: self.0.grid_auto_rows.clone(),
            grid_template_columns: self.0.grid_template_columns.clone(),
            grid_template_rows: self.0.grid_template_rows.clone(),
            ..self.0
        })
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<bevy_ui::UiRect> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(bevy_ui::UiRect {
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
            (Val::Vw(self_val), Val::Vw(other_val)) => {
                EaseValue(Val::Vw(Lerp::lerp(&self_val, &other_val, scalar)))
            }
            (Val::Vh(self_val), Val::Vh(other_val)) => {
                EaseValue(Val::Vh(Lerp::lerp(&self_val, &other_val, scalar)))
            }
            _ => EaseValue(self.0),
        }
    }
}

#[cfg(feature = "color")]
impl Lerp for EaseValue<Color> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        let color = match (self.0, other.0) {
            (Color::Srgba(color), other) => {
                let lerped = Srgba::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Srgba>::into(other).to_vec4(), *scalar),
                );
                Color::Srgba(lerped)
            }
            (Color::LinearRgba(color), other) => {
                let lerped = LinearRgba::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<LinearRgba>::into(other).to_vec4(), *scalar),
                );
                Color::LinearRgba(lerped)
            }
            (Color::Hsla(color), other) => {
                let lerped = Hsla::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Hsla>::into(other).to_vec4(), *scalar),
                );
                Color::Hsla(lerped)
            }
            (Color::Hsva(color), other) => {
                let lerped = Hsva::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Hsva>::into(other).to_vec4(), *scalar),
                );
                Color::Hsva(lerped)
            }
            (Color::Hwba(color), other) => {
                let lerped = Hwba::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Hwba>::into(other).to_vec4(), *scalar),
                );
                Color::Hwba(lerped)
            }
            (Color::Laba(color), other) => {
                let lerped = Laba::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Laba>::into(other).to_vec4(), *scalar),
                );
                Color::Laba(lerped)
            }
            (Color::Lcha(color), other) => {
                let lerped = Lcha::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Lcha>::into(other).to_vec4(), *scalar),
                );
                Color::Lcha(lerped)
            }
            (Color::Oklaba(color), other) => {
                let lerped = Oklaba::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Oklaba>::into(other).to_vec4(), *scalar),
                );
                Color::Oklaba(lerped)
            }
            (Color::Oklcha(color), other) => {
                let lerped = Oklcha::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Oklcha>::into(other).to_vec4(), *scalar),
                );
                Color::Oklcha(lerped)
            }
            (Color::Xyza(color), other) => {
                let lerped = Xyza::from_vec4(
                    color
                        .to_vec4()
                        .lerp(Into::<Xyza>::into(other).to_vec4(), *scalar),
                );
                Color::Xyza(lerped)
            }
        };
        EaseValue(color)
    }
}

#[cfg(feature = "ui")]
impl Lerp for EaseValue<bevy_text::TextColor> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(bevy_text::TextColor(
            EaseValue(self.0.0).lerp(&EaseValue(other.0.0), scalar).0,
        ))
    }
}

impl Lerp for EaseValue<Rect> {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        EaseValue(Rect {
            min: self.0.min.lerp(other.0.min, *scalar),
            max: self.0.max.lerp(other.0.max, *scalar),
        })
    }
}
