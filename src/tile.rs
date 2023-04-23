use crate::machine::Conveyer;

#[derive(Debug, Default)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    conveyer: Option<Conveyer>,
}

impl Tile {
    pub fn set_conveyer(&mut self, conveyer: Conveyer) {
        self.conveyer = Some(conveyer);
    }

    pub fn conveyer(&self) -> Option<&Conveyer> {
        self.conveyer.as_ref()
    }

    pub fn conveyer_mut(&mut self) -> Option<&mut Conveyer> {
        self.conveyer.as_mut()
    }
}
