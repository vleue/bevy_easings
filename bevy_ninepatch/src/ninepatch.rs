use bevy::{
    asset::{Assets, Handle},
    ecs::Entity,
    math::{Rect, Size, Vec2},
    render::{
        color::Color,
        draw::Draw,
        texture::{Texture, TextureFormat},
    },
    sprite::ColorMaterial,
    transform::hierarchy::{BuildChildren, ChildBuilder},
    ui::{
        entity::{ImageComponents, NodeComponents},
        AlignContent, FlexDirection, FocusPolicy, PositionType, Style, Val,
    },
};
use rand::Rng;

/// How a patch can grow
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GrowthMode {
    /// This patch doesn't grow
    None,
    /// This patch grows according to a ratio
    StretchRatio(f32),
}

/// Size of a patch in the original image
#[derive(Debug, Clone, Copy)]
pub enum PatchSize {
    /// Absolute size
    Absolute(f32),
    /// Size relative to the total width of the original image
    Width {
        /// Offset relative to the width
        offset: f32,
    },
    /// Size relative to the total height of the original image
    Height {
        /// Offset relative to the height
        offset: f32,
    },
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

/// Describe a patch in the original image, how it should grow and if it can have content
#[derive(Debug, Clone)]
pub struct Patch<T: Clone + Send + Sync + 'static> {
    /// Width of the patch
    pub width: PatchSize,
    /// Height of the patch
    pub height: PatchSize,
    /// How does it grow on x axis
    pub x_growth: GrowthMode,
    /// How does it grow on y axis
    pub y_growth: GrowthMode,
    /// Does this patch can contain content
    pub content: Option<T>,
}

/// Holds the patches of a nine patch texture
#[derive(Debug)]
pub struct NinePatchBuilder<T: Clone + Send + Sync + 'static = ()> {
    /// Patches for a nine patch texture. See example `full.rs` on how to use directly
    pub patches: Vec<Vec<Patch<T>>>,
    pub(crate) patch_textures: Option<Vec<Handle<ColorMaterial>>>,
    pub(crate) original_texture: Option<Handle<Texture>>,
}

impl<T: Clone + Send + Sync + 'static> NinePatchBuilder<T> {
    /// Create a `NinePatchBuilder` from it's patches
    pub fn from_patches(patches: Vec<Vec<Patch<T>>>) -> Self {
        Self {
            patches,
            patch_textures: None,
            original_texture: None,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> NinePatchBuilder<T> {
    /// Create a simple nine patch split by creating fixed patch for the margins, and growing patches inside
    pub fn by_margins(
        top_margin: f32,
        bottom_margin: f32,
        left_margin: f32,
        right_margin: f32,
        content: T,
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
                content: Some(content),
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
            patch_textures: None,
            original_texture: None,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> NinePatchBuilder<T> {
    /// Apply a `NinePatchBuilder` to a texture to get a `NinePatch` ready to be added to entities. This will split
    /// the given texture according to the patches.
    pub fn apply(
        &mut self,
        texture_handle: Handle<Texture>,
        textures: &mut Assets<Texture>,
        materials: &mut Assets<ColorMaterial>,
    ) -> NinePatch<T> {
        let (texture_size, texture_data) = {
            let t = textures
                .get(&texture_handle)
                .expect("could not get texture from handle");
            (t.size, t.data.clone())
        };
        if self.patch_textures.is_none() || self.original_texture != Some(texture_handle) {
            let mut patch_textures = vec![];
            let mut accu_y = 0.;
            for row in &self.patches {
                let mut accu_x = 0.;
                for column_item in row {
                    let start_x = accu_x;
                    let end_x = accu_x + column_item.width.to_value(texture_size);

                    let start_y = accu_y;
                    let end_y = accu_y + column_item.height.to_value(texture_size);

                    let mut patch_texture_data = vec![];
                    for j in start_y as usize..end_y as usize {
                        for i in start_x as usize..end_x as usize {
                            let base = (i + j * texture_size.x() as usize) * 4;
                            patch_texture_data.push(texture_data[base]);
                            patch_texture_data.push(texture_data[base + 1]);
                            patch_texture_data.push(texture_data[base + 2]);
                            patch_texture_data.push(texture_data[base + 3]);
                        }
                    }

                    let patch_texture = Texture {
                        size: Vec2::new(
                            column_item.width.to_value(texture_size),
                            column_item.height.to_value(texture_size),
                        ),
                        data: patch_texture_data,
                        format: TextureFormat::Rgba8UnormSrgb,
                    };
                    let patch_texture_handle = textures.add(patch_texture);
                    let material = materials.add(patch_texture_handle.into());
                    patch_textures.push(material);
                    accu_x += column_item.width.to_value(texture_size);
                }
                accu_y += row[0].height.to_value(texture_size);
            }
            self.patch_textures = Some(patch_textures);
            self.original_texture = Some(texture_handle);
        }
        NinePatch {
            patches: self.patches.clone(),
            texture_size,
            background: materials.add(Color::NONE.into()),
            splitted_texture: self.patch_textures.as_ref().cloned().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct BuildedNinePatchGrowthAxis {
    pub fixed: f32,
    pub ratio: f32,
}

#[derive(Clone)]
pub struct BuildedNinePatchGrowth {
    pub x: Option<BuildedNinePatchGrowthAxis>,
    pub y: Option<BuildedNinePatchGrowthAxis>,
}

#[derive(Clone)]
pub struct NinePatchId(pub u128);

/// Component to mark the entity placed for the content of the 9-Patch UI element
#[derive(Clone, Debug)]
pub struct NinePatchContent<T: Send + Sync + 'static> {
    /// Name of the content patch
    pub content: T,
    /// Has it been already loaded
    pub loaded: bool,
    #[cfg(feature = "manual")]
    /// Entity parent of the 9-Patch UI element
    pub parent: Option<Entity>,
    #[cfg(not(feature = "manual"))]
    /// Entity parent of the 9-Patch UI element
    pub parent: Entity,
}

/// `NinePatch` ready to be added to entities.
#[derive(Debug)]
pub struct NinePatch<T: Clone + Send + Sync + 'static> {
    patches: Vec<Vec<Patch<T>>>,
    texture_size: Vec2,
    background: Handle<ColorMaterial>,
    splitted_texture: Vec<Handle<ColorMaterial>>,
}
impl<T: Clone + Send + Sync + 'static> NinePatch<T> {
    /// Add the `NinePatch` to entities. This will create several entities as children.
    #[cfg(feature = "manual")]
    pub fn add<F>(
        &self,
        parent: &mut ChildBuilder,
        width: f32,
        height: f32,
        content_builder: F,
    ) -> u128
    where
        F: FnMut(&mut ChildBuilder, &T) + Copy,
    {
        self.add_with_parent(parent, width, height, None, content_builder)
    }

    pub(crate) fn add_with_parent<F>(
        &self,
        parent: &mut ChildBuilder,
        width: f32,
        height: f32,
        #[cfg(feature = "manual")] grandparent: Option<Entity>,
        #[cfg(not(feature = "manual"))] grandparent: Entity,
        mut content_builder: F,
    ) -> u128
    where
        F: FnMut(&mut ChildBuilder, &T) + Copy,
    {
        let mut rng = rand::thread_rng();
        // // temp entity that will be erased when getting actual entity
        // // TODO: use `current_entity` once https://github.com/bevyengine/bevy/pull/595 is released
        // let mut parent_entity = Entity::new(0);
        // parent.for_current_entity(|entity| parent_entity = entity);

        let id: u128 = rng.gen();
        parent
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_content: AlignContent::Stretch,
                    ..Default::default()
                },
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                material: self.background,
                ..Default::default()
            })
            .with(FocusPolicy::Pass)
            .with_bundle((
                NinePatchId(id),
                BuildedNinePatchGrowth {
                    x: Some(BuildedNinePatchGrowthAxis {
                        fixed: 0.,
                        ratio: 1.,
                    }),
                    y: Some(BuildedNinePatchGrowthAxis {
                        fixed: 0.,
                        ratio: 1.,
                    }),
                },
            ))
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
                                size: Size::new(Val::Px(width), Val::Undefined),
                                flex_direction: FlexDirection::Row,
                                align_content: AlignContent::Stretch,
                                ..Default::default()
                            },
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            material: self.background,
                            ..Default::default()
                        })
                        .with_bundle((
                            NinePatchId(id),
                            BuildedNinePatchGrowth {
                                x: Some(BuildedNinePatchGrowthAxis {
                                    fixed: 0.,
                                    ratio: 1.,
                                }),
                                y: None,
                            },
                        ))
                        .with(FocusPolicy::Pass);
                    inner_parent.with_children(|row_parent| {
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
                                GrowthMode::None => column_item.width.to_value(self.texture_size),
                                GrowthMode::StretchRatio(ratio) => {
                                    (width - fixed_width_in_row) * ratio
                                }
                            };
                            let size_y = match column_item.y_growth {
                                GrowthMode::None => column_item.height.to_value(self.texture_size),
                                GrowthMode::StretchRatio(ratio) => {
                                    (height - fixed_height_in_column) * ratio
                                }
                            };
                            row_parent
                                .spawn(ImageComponents {
                                    material: self.splitted_texture[n],
                                    style: Style {
                                        size: Size::new(Val::Px(size_x), Val::Px(size_y)),
                                        ..Default::default()
                                    },
                                    draw: Draw {
                                        is_transparent: true,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with(FocusPolicy::Pass);
                            let growth_bundle = (
                                NinePatchId(id),
                                BuildedNinePatchGrowth {
                                    x: match column_item.x_growth {
                                        GrowthMode::None => None,
                                        GrowthMode::StretchRatio(ratio) => {
                                            Some(BuildedNinePatchGrowthAxis {
                                                fixed: fixed_width_in_row,
                                                ratio,
                                            })
                                        }
                                    },
                                    y: match column_item.y_growth {
                                        GrowthMode::None => None,
                                        GrowthMode::StretchRatio(ratio) => {
                                            Some(BuildedNinePatchGrowthAxis {
                                                fixed: fixed_height_in_column,
                                                ratio,
                                            })
                                        }
                                    },
                                },
                            );
                            if (column_item.x_growth != GrowthMode::None)
                                || (column_item.y_growth != GrowthMode::None)
                            {
                                row_parent.with_bundle(growth_bundle.clone());
                            }
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
                                    .with(FocusPolicy::Pass)
                                    .with(NinePatchContent {
                                        content: content_part.clone(),
                                        loaded: false,
                                        #[cfg(feature = "manual")]
                                        parent: grandparent.clone(),
                                        #[cfg(not(feature = "manual"))]
                                        parent: grandparent,
                                    });
                                if (column_item.x_growth != GrowthMode::None)
                                    || (column_item.y_growth != GrowthMode::None)
                                {
                                    row_parent.with_bundle(growth_bundle);
                                }

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
        id
    }
}
