mod conveyer;

pub use self::conveyer::{Conveyer, ConveyerBuilder};

pub trait Clickable {
    fn on_click(&mut self);
}

pub trait MachineCore<S> {
    fn update(&mut self, dt: f64, neighbor: Option<&mut Self>) {
        self.before_update(dt);
        if self.operatable() {
            self.main(neighbor);
        }
        self.after_update();
    }

    fn main(&mut self, neighbor: Option<&mut Self>);

    fn before_update(&mut self, dt: f64) {
        self.set_cooling_time(self.cooling_time() + dt);
    }

    fn after_update(&mut self) {
        if self.operatable() {
            self.set_cooling_time(0.0);
        }
    }

    fn set_cooling_time(&mut self, dt: f64);
    fn cooling_time(&self) -> f64;

    fn operatable(&self) -> bool {
        self.cooling_time() > 0.5
    }
}
