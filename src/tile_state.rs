use crate::NaturalResource;

#[derive(Debug)]
struct NaturalResourceState {
    name: String,
    reserves: usize,
}

#[derive(Debug, Default)]
pub struct TileState {
    natural_resource_state: Option<NaturalResourceState>,
}

impl TileState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_natural_resource(mut self, natural_resource: Option<&dyn NaturalResource>) -> Self {
        if let Some(natural_resource) = natural_resource {
            self.natural_resource_state = Some(NaturalResourceState {
                name: natural_resource.variant().to_string(),
                reserves: natural_resource.reserves(),
            });
        }
        self
    }
}
