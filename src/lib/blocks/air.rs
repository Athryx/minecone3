use super::*;

#[derive(Default)]
pub struct Air;

impl BaseBlock for Air {
    fn model(texture_builder: &mut TextureBuilder) -> BlockModel {
        BlockModel::new(BlockFace::Empty)
    }

    fn properties() -> BlockProperties {
        BlockProperties {
            max_hp: 0
        }
    }
}