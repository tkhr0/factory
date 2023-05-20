use crate::item::Material;

#[derive(Debug, Default)]
pub struct Slot {
    cargoes: Vec<Box<dyn Material>>,
}

impl Slot {
    pub fn push(&mut self, resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        if let Some(resource) = resource {
            if self.cargoes.len() >= resource.stack_size() {
                return Err("Resource overflow from slot");
            }
            self.cargoes.push(resource);
            return Ok(());
        }
        Ok(())
    }

    pub fn pick(&mut self) -> Option<Box<dyn Material>> {
        self.cargoes.pop()
    }

    pub fn is_some(&self) -> bool {
        !self.cargoes.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.cargoes.is_empty()
    }

    pub fn resource(&self) -> Option<&Box<dyn Material>> {
        self.cargoes.first()
    }
}

#[cfg(test)]
mod test {
    #[cfg(test)]
    mod push {
        use crate::item::{CoalBuilder, MaterialBuilder};
        use crate::Slot;

        #[test]
        fn pushable_some() {
            let mut slot = Slot::default();
            let resource = Some(CoalBuilder::new().build());
            assert!(slot.push(resource).is_ok());
            assert!(slot.is_some());
        }

        #[test]
        fn pushable_none() {
            let mut slot = Slot::default();
            let resource = None;
            assert!(slot.push(resource).is_ok());
            assert!(slot.is_empty());
        }

        #[test]
        fn can_not_push_when_slot_is_full() {
            let mut slot = Slot::default();
            for _ in 0..64 {
                slot.push(Some(CoalBuilder::new().build())).unwrap(); // full
            }

            assert!(slot.push(Some(CoalBuilder::new().build())).is_err());
            assert_eq!(slot.cargoes.len(), 64);
        }
    }
}
