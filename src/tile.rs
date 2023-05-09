use crate::item::{Fixture, ResourceObj};
use crate::types;

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

    pub fn fixture(&self) -> Option<&dyn Fixture> {
        if let Some(fixture) = self.fixture.as_ref() {
            Some(fixture.as_ref())
        } else {
            None
        }
    }

    pub fn fixture_mut(&mut self) -> Option<&mut Box<dyn Fixture>> {
        self.fixture.as_mut()
    }

    pub fn update(&mut self, dt: f64) {
        if let Some(fixture) = self.fixture_mut() {
            fixture.update(dt);
        }
    }

    pub fn effect_range(&self) -> Option<types::GridSize> {
        if let Some(fixture) = self.fixture() {
            fixture.effect_range()
        } else {
            None
        }
    }

    pub fn affect(&mut self, other: &mut Self, direction: types::Direction) {
        if self.fixture.is_none() {
            return;
        }

        if let Some(fixture) = self.fixture_mut() {
            fixture.affect(other, &direction);
        }
    }

    pub fn insertable(&self) -> bool {
        if let Some(fixture) = self.fixture() {
            fixture.insertable()
        } else {
            false
        }
    }

    pub fn insert(&mut self, resource: ResourceObj) -> Result<(), &'static str> {
        if let Some(fixture) = self.fixture_mut() {
            fixture.insert(resource)
        } else {
            Err("No fixture")
        }
    }

    pub fn pushable(&self) -> bool {
        if let Some(fixture) = self.fixture() {
            fixture.pushable()
        } else {
            false
        }
    }

    pub fn push(&mut self, resource: Option<ResourceObj>) -> Result<(), &'static str> {
        if let Some(fixture) = self.fixture_mut() {
            fixture.push(resource)
        } else {
            Err("No fixture")
        }
    }

    pub fn request(&mut self) -> Option<ResourceObj> {
        if let Some(fixture) = self.fixture_mut() {
            fixture.request()
        } else {
            None
        }
    }
}
