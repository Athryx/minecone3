use super::*;

#[derive(Default)]
pub struct Grass;

impl BaseBlock for Grass {
    fn model(texture_builder: &mut TextureBuilder) -> BlockModel {
        let dirt_face = texture_builder.image("textures/dirt.png");
        let grass_face = texture_builder.image("textures/grass.png");

        BlockModel::new(BlockFace::Full(dirt_face))
            .set_top(BlockFace::Full(grass_face))
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 10,
        }
    }
}