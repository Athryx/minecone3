use super::*;

#[derive(Default)]
pub struct Grass;

impl BaseBlock for Grass {
    fn model(texture_builder: &mut TextureBuilder) -> BlockModel {
        let dirt_face = texture_builder.image("textures/dirt.png");
        let grass_face = texture_builder.image("textures/grass.png");

        let grass_side = texture_builder.image("textures/grass_side.png");
        let grass_side = texture_builder.overlay(
            grass_side,
            dirt_face,
        );

        BlockModel::new(BlockFace::Full(grass_side))
            .set_top(BlockFace::Full(grass_face))
            .set_bottom(BlockFace::Full(dirt_face))
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 10,
        }
    }
}