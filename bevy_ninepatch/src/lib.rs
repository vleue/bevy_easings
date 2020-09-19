use bevy::prelude::{
    Assets, BuildChildren, ChildBuilder, Color, ColorMaterial, Draw, FlexDirection, Handle,
    NodeComponents, PositionType, Rect, ResMut, Size, Style, Texture, TextureAtlas,
    TextureAtlasSprite, Transform, Val, Vec2, Vec3,
};

pub mod spritesheet;
use spritesheet::SpriteSheetImageComponents;

#[derive(PartialEq, Debug)]
enum GrowthMode {
    None,
    StretchRatio(f32),
}

#[derive(Debug)]
enum PatchSize {
    Absolute(f32),
    Width { offset: f32 },
    Height { offset: f32 },
}

impl PatchSize {
    fn to_value(&self, texture_size: Vec2) -> f32 {
        match self {
            PatchSize::Absolute(v) => *v,
            PatchSize::Width { offset } => texture_size.x() + offset,
            PatchSize::Height { offset } => texture_size.y() + offset,
        }
    }
}

#[derive(Debug)]
struct Patch<T> {
    width: PatchSize,
    height: PatchSize,
    x_growth: GrowthMode,
    y_growth: GrowthMode,
    content: Option<T>,
}

#[derive(Debug)]
pub struct NinePatchBuilder<T> {
    patches: Vec<Vec<Patch<T>>>,
}

impl NinePatchBuilder<()> {
    pub fn simple(
        top_margin: f32,
        bottom_margin: f32,
        left_margin: f32,
        right_margin: f32,
    ) -> Self {
        let top = vec![
            Patch {
                width: PatchSize::Absolute(left_margin),
                height: PatchSize::Absolute(top_margin),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Width {
                    offset: -left_margin - right_margin,
                },
                height: PatchSize::Absolute(top_margin),
                x_growth: GrowthMode::StretchRatio(1.),
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(right_margin),
                height: PatchSize::Absolute(top_margin),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
        ];
        let middle = vec![
            Patch {
                width: PatchSize::Absolute(left_margin),
                height: PatchSize::Height {
                    offset: -top_margin - bottom_margin,
                },
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::StretchRatio(1.),
                content: None,
            },
            Patch {
                width: PatchSize::Width {
                    offset: -left_margin - right_margin,
                },
                height: PatchSize::Height {
                    offset: -top_margin - bottom_margin,
                },
                x_growth: GrowthMode::StretchRatio(1.),
                y_growth: GrowthMode::StretchRatio(1.),
                content: Some(()),
            },
            Patch {
                width: PatchSize::Absolute(right_margin),
                height: PatchSize::Height {
                    offset: -top_margin - bottom_margin,
                },
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::StretchRatio(1.),
                content: None,
            },
        ];
        let bottom = vec![
            Patch {
                width: PatchSize::Absolute(left_margin),
                height: PatchSize::Absolute(bottom_margin),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Width {
                    offset: -left_margin - right_margin,
                },
                height: PatchSize::Absolute(bottom_margin),
                x_growth: GrowthMode::StretchRatio(1.),
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(right_margin),
                height: PatchSize::Absolute(bottom_margin),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
        ];
        Self {
            patches: vec![top, middle, bottom],
        }
    }
}

impl<T> NinePatchBuilder<T> {
    pub fn apply(
        self,
        texture_handle: Handle<Texture>,
        textures: &ResMut<Assets<Texture>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> NinePatch<T> {
        let texture = textures.get(&texture_handle).unwrap();

        let mut rects = vec![];
        let mut accu_y = 0.;
        for row in &self.patches {
            let mut accu_x = 0.;
            for column_item in row {
                rects.push(bevy::sprite::Rect {
                    min: Vec2::new(accu_x, accu_y),
                    max: Vec2::new(
                        accu_x + column_item.width.to_value(texture.size),
                        accu_y + column_item.height.to_value(texture.size),
                    ),
                });
                accu_x += column_item.width.to_value(texture.size);
            }
            accu_y += row[0].height.to_value(texture.size);
        }
        let texture_atlas = TextureAtlas {
            texture: texture_handle,
            size: texture.size,
            textures: rects,
            texture_handles: None,
        };
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        NinePatch {
            patches: self.patches,
            texture_size: texture.size,
            texture_atlas_handle,
            background: materials.add(Color::NONE.into()),
        }
    }
}
pub struct NinePatch<T> {
    patches: Vec<Vec<Patch<T>>>,
    texture_size: Vec2,
    texture_atlas_handle: Handle<TextureAtlas>,
    background: Handle<ColorMaterial>,
}
impl<T> NinePatch<T> {
    pub fn add<F>(&self, parent: &mut ChildBuilder, width: f32, height: f32, mut content_builder: F)
    where
        F: FnMut(&mut ChildBuilder, &T) + Copy,
    {
        parent
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                material: self.background,
                ..Default::default()
            })
            .with(bevy::ui::FocusPolicy::Pass)
            .with_children(|inner_parent| {
                let mut n = 0;
                for row in self.patches.iter() {
                    let fixed_width_in_row: f32 = row
                        .iter()
                        .filter(|p| p.x_growth == GrowthMode::None)
                        .map(|p| p.width.to_value(self.texture_size))
                        .sum();
                    inner_parent
                        .spawn(NodeComponents {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            material: self.background,
                            ..Default::default()
                        })
                        .with(bevy::ui::FocusPolicy::Pass)
                        .with_children(|row_parent| {
                            let mut accu_x = 0.;
                            for (j, column_item) in row.iter().enumerate() {
                                let fixed_height_in_column: f32 = self
                                    .patches
                                    .iter()
                                    .filter_map(|col| col.get(j))
                                    .filter(|p| p.y_growth == GrowthMode::None)
                                    .map(|p| p.height.to_value(self.texture_size))
                                    .sum();
                                let size_x = match column_item.x_growth {
                                    GrowthMode::None => {
                                        column_item.width.to_value(self.texture_size)
                                    }
                                    GrowthMode::StretchRatio(ratio) => {
                                        width * ratio - fixed_width_in_row
                                    }
                                };
                                let size_y = match column_item.y_growth {
                                    GrowthMode::None => {
                                        column_item.height.to_value(self.texture_size)
                                    }
                                    GrowthMode::StretchRatio(ratio) => {
                                        height * ratio - fixed_height_in_column
                                    }
                                };
                                row_parent
                                    .spawn(SpriteSheetImageComponents {
                                        texture_atlas: self.texture_atlas_handle,
                                        sprite: TextureAtlasSprite::new(n),
                                        style: Style {
                                            size: Size::new(Val::Px(size_x), Val::Px(size_y)),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_non_uniform_scale(Vec3::new(
                                            match column_item.x_growth {
                                                GrowthMode::None => 1.,
                                                GrowthMode::StretchRatio(_) => {
                                                    size_x
                                                        / (self.texture_size.x()
                                                            - fixed_width_in_row)
                                                }
                                            },
                                            match column_item.y_growth {
                                                GrowthMode::None => 1.,
                                                GrowthMode::StretchRatio(_) => {
                                                    size_y
                                                        / (self.texture_size.y()
                                                            - fixed_height_in_column)
                                                }
                                            },
                                            1.,
                                        )),
                                        draw: Draw {
                                            is_transparent: true,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with(bevy::ui::FocusPolicy::Pass);
                                if let Some(content_part) = column_item.content.as_ref() {
                                    row_parent
                                        .spawn(NodeComponents {
                                            style: Style {
                                                position_type: PositionType::Absolute,
                                                position: Rect {
                                                    left: Val::Px(accu_x),
                                                    ..Default::default()
                                                },
                                                size: Size::new(Val::Px(size_x), Val::Px(size_y)),
                                                ..Default::default()
                                            },
                                            draw: Draw {
                                                is_transparent: true,
                                                is_visible: false,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .with(bevy::ui::FocusPolicy::Pass);
                                    row_parent.with_children(|child_builder| {
                                        content_builder(child_builder, content_part);
                                    });
                                }
                                accu_x += size_x;

                                n += 1;
                            }
                        });
                }
            });
    }
}
