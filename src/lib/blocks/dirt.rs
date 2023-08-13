use super::*;

#[derive(Default)]
pub struct Dirt;

impl BaseBlock for Dirt {
    fn model() -> BlockModel {
        let dirt_face = BlockFace::solid_face("textures/dirt.png");

        BlockModel::new(dirt_face)
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 10,
        }
    }
}