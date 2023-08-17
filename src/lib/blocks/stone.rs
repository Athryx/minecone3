use super::*;

#[derive(Default)]
pub struct Stone;

impl BaseBlock for Stone {
    fn model(texture_builder: &mut TextureBuilder) -> BlockModel {
        let stone_face = texture_builder.image("textures/stone.png");

        BlockModel::new(BlockFace::Full(stone_face))
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 80,
        }
    }
}