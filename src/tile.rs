use crate::machine::Machine;

#[derive(Debug, Default)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    machine: Option<Machine>,
}

impl Tile {
    pub fn set_machine(&mut self, machine: Machine) {
        self.machine = Some(machine);
    }

    pub fn machine(&self) -> Option<&Machine> {
        self.machine.as_ref()
    }

    pub fn machine_mut(&mut self) -> Option<&mut Machine> {
        self.machine.as_mut()
    }
}


