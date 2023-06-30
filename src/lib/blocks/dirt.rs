use super::*;

#[derive(Default)]
pub struct Dirt;

impl BaseBlock for Dirt {
    fn model() -> BlockModel {
        let dirt_face = BlockFace {
            rotation: utils::Rotation::Deg0,
            face_type: BlockFaceType::Full("textures/dirt.png"),
            texture_data: None,
        };

        BlockModel {
            faces: [dirt_face; 6],
        }
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 10,
        }
    }
}