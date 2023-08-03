#[derive(Debug)]
pub enum Item {}

#[derive(Debug)]
pub struct ItemStack {
    pub item: Item,
    pub count: usize,
}

macro_rules! register_items {
    () => {
        
    };
}