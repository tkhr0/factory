use crate::item::{Fixture, Material};
use crate::types;
use crate::NaturalResource;

#[derive(Debug, Default)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    fixture: Option<Box<dyn Fixture>>,
    natural_resource: Option<Box<dyn NaturalResource>>,
}

impl Tile {
    // natural_resource

    pub fn natural_resource(&self) -> Option<&dyn NaturalResource> {
        if let Some(natural_resource) = self.natural_resource.as_ref() {
            Some(natural_resource.as_ref())
        } else {
            None
        }
    }

    pub fn set_natural_resource(&mut self, natural_resource: Box<dyn NaturalResource>) {
        self.natural_resource = Some(natural_resource);
    }

    // fixture

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

    pub fn insert(&mut self, resource: Box<dyn Material>) -> Result<(), &'static str> {
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

    pub fn push(&mut self, resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        if let Some(fixture) = self.fixture_mut() {
            fixture.push(resource)
        } else {
            Err("No fixture")
        }
    }

    pub fn request(&mut self) -> Option<Box<dyn Material>> {
        if let Some(fixture) = self.fixture_mut() {
            fixture.request()
        } else {
            None
        }
    }
}
