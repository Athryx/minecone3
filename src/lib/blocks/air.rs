use super::*;

#[derive(Default)]
pub struct Air;

static AIR_PROPERTIES: BlockProperties = BlockProperties {
    max_hp: 0,
};

impl BaseBlock for Air {
    fn model() -> BlockModel {
        let dirt_face = BlockFace {
            rotation: utils::Rotation::Deg0,
            face_type: BlockFaceType::Full(TextureIdentifier::Path("textures/dirt.png")),
        };

        BlockModel {
            faces: [dirt_face; 6],
        }
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 0
        }
    }
}