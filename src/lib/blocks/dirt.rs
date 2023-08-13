use super::*;

#[derive(Default)]
pub struct Dirt;

impl BaseBlock for Dirt {
    fn model(texture_builder: &mut TextureBuilder) -> BlockModel {
        let dirt_face = texture_builder.image("textures/dirt.png");

        BlockModel::new(BlockFace::Full(dirt_face))
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 10,
        }
    }
}