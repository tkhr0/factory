use crate::resource::Resource;

#[derive(Debug)]
struct Container {
    slot: Vec<Option<Resource>>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            slot: vec![None, None, None, None],
        }
    }

    pub fn update(&mut self) {
        for i in 0..(self.slot.len() - 1) {
            if self.slot[i].is_none() {
                self.slot.swap(i, i + 1);
            }
        }
    }

    pub fn pick(&mut self) -> Option<Resource> {
        self.slot.push(None);
        self.slot.swap_remove(0)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Resource>> {
        self.slot.iter()
    }

    fn len(&self) -> usize {
        self.slot.len()
    }

    pub fn load(&mut self) {
        let length = self.len();
        if self.slot[length - 1].is_some() {
            return;
        }
        self.slot[length - 1] = Some(Resource::default());
    }

    pub fn acceptable(&self) -> bool {
        self.slot[self.len() - 1].is_none()
    }

    pub fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
        if resource.is_none() {
            return Ok(());
        }

        let index = self.len() - 1;
        if self.slot[index].is_some() {
            return Err("Resource overflow from container");
        }

        self.slot[index] = resource;
        Ok(())
    }
}
