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

    pub fn natural_resource_mut(&mut self) -> Option<&mut dyn NaturalResource> {
        if let Some(natural_resource) = self.natural_resource.as_mut() {
            Some(natural_resource.as_mut())
        } else {
            None
        }
    }

    pub fn set_natural_resource(&mut self, natural_resource: Box<dyn NaturalResource>) {
        self.natural_resource = Some(natural_resource);
    }

    pub fn mine(&mut self, amount: usize) -> Result<usize, &'static str> {
        return if let Some(natural_resource) = self.natural_resource_mut() {
            natural_resource.extract(amount)
        } else {
            Err("Natural resources are nothing")
        };
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

    pub fn take_fixture(&mut self) -> Option<Box<dyn Fixture>> {
        self.fixture.take()
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

    pub fn affect_self(&mut self) {
        if let Some(mut fixture) = self.take_fixture() {
            fixture.affect(self, &types::Direction::Origin);
            self.set_fixture(fixture)
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
