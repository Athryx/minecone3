use super::*;

#[derive(Default)]
pub struct Grass;

impl BaseBlock for Grass {
    fn model() -> BlockModel {
        let dirt_face = BlockFace::solid_face("textures/dirt.png");
        let grass_face = BlockFace::solid_face("textures/grass.png");

        BlockModel::new(dirt_face)
            .set_top(grass_face)
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 10,
        }
    }
}