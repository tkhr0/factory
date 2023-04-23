use crate::Fixture;

#[derive(Debug, Default)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    fixture: Option<Box<dyn Fixture>>,
}

impl Tile {
    pub fn set_fixture(&mut self, fixture: Box<dyn Fixture>) {
        self.fixture = Some(fixture);
    }

    pub fn fixture(&self) -> Option<&Box<dyn Fixture>> {
        self.fixture.as_ref()
    }

    pub fn fixture_mut(&mut self) -> Option<&mut Box<dyn Fixture>> {
        self.fixture.as_mut()
    }

    pub fn update(&mut self, dt: f64) {
        if let Some(fixture) = self.fixture_mut() {
            fixture.update(dt);
        }
    }
}
