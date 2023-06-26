use super::*;

#[derive(Default)]
pub struct Dirt;

impl BaseBlock for Dirt {
    fn model() -> BlockModel {
        let dirt_face = BlockFace {
            rotation: utils::Rotation::Deg0,
            face_type: BlockFaceType::Full(TextureIdentifier::Path("textures/dirt.png")),
        };

        BlockModel {
            faces: [dirt_face; 6],
        }
    }
}