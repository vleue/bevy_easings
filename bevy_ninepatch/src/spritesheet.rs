use bevy::asset::Handle;
use bevy::ecs::Bundle;
use bevy::render::{
    draw::Draw,
    mesh::Mesh,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
};
use bevy::sprite::QUAD_HANDLE;
use bevy::sprite::{TextureAtlas, TextureAtlasSprite};
use bevy::transform::{components::GlobalTransform, prelude::Transform};
use bevy::ui::Node;
use bevy::ui::{widget::Image, CalculatedSize, Style};

#[derive(Bundle)]
pub struct SpriteSheetImageComponents {
    pub node: Node,
    pub style: Style,
    pub image: Image,
    pub calculated_size: CalculatedSize,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    /// The specific sprite from the texture atlas to be drawn
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    pub global_transform: GlobalTransform,
}

impl Default for SpriteSheetImageComponents {
    fn default() -> Self {
        SpriteSheetImageComponents {
            mesh: QUAD_HANDLE,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                bevy::sprite::SPRITE_SHEET_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 2,
                            binding: 0,
                        },
                        // Node_size
                        DynamicBinding {
                            bind_group: 2,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            node: Default::default(),
            image: Default::default(),
            calculated_size: Default::default(),
            style: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            sprite: Default::default(),
            texture_atlas: Default::default(),
            global_transform: Default::default(),
        }
    }
}
