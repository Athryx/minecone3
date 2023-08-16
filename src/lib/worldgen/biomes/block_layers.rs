use crate::blocks::BlockType;

/// A layer of blocks on the surface of a biome
pub struct BlockLayer {
    pub block: BlockType,
    pub thickness: u32,
}

/// Describes the makeup of the blocks in a biome
pub struct BiomeLayers {
    /// The top layers of the biome, with index 0 being at the very top
    pub layers: &'static [BlockLayer],
    /// Block that will fill the rest of the biome
    pub bottom: BlockType,
}

impl BiomeLayers {
    /// Gets the block at the given depth, with depth == 0 beingh the surface, depth < 0 being underground, adn depth > 0 being air
    // NOTE: this is O(n) with number of layers, so if there is ever a biome with lots of layers use binary search instead
    pub fn get_block_at_depth(&self, depth: i32) -> BlockType {
        if depth > 0 {
            BlockType::Air
        } else {
            let mut inv_depth = (-depth) as u32;
            for layer in self.layers {
                if inv_depth < layer.thickness {
                    return layer.block;
                }

                inv_depth -= layer.thickness;
            }

            self.bottom
        }
    }
}