use super::*;

#[derive(Default)]
pub struct Air;

impl BaseBlock for Air {
    fn model() -> BlockModel {
        let air_face = BlockFace {
            rotation: utils::Rotation::Deg0,
            face_type: BlockFaceType::Empty,
            texture_data: None,
        };

        BlockModel {
            faces: [air_face; 6],
        }
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 0
        }
    }
}