pub type GameMap = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Self { blocked: false, block_sight: false }
    }

    pub fn wall() -> Self {
        Self { blocked: true, block_sight: true }
    }
}
