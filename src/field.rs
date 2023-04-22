use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};

use crate::grid_point::GridPoint;
use crate::machine::{Clickable, Machine, MachineCore};
use crate::tile::Tile;
use crate::types::{Direction, Point};

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug)]
pub struct Field {
    tiles: Vec<Tile>, // FIXME: Use a array instead of a vector
}

impl Field {
    const TILE_SIZE: f64 = 50.0;

    pub fn new() -> Field {
        Self::default()
    }

    pub fn initialize(&mut self) {
        self.add_machine(Machine::new("A"), GridPoint::new(2, 3));
        self.add_machine(Machine::new("B"), GridPoint::new(3, 3));
        self.add_machine(Machine::new("C"), GridPoint::new(4, 3));
    }

    pub fn add_machine(&mut self, machine: Machine, grid_point: GridPoint) {
        self.tiles[grid_point.to_index(WIDTH)].set_machine(machine);
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        for tile in self.tiles.iter() {
            graphics::Rectangle::new_border([0.0, 0.0, 0.0, 0.1], 1.0).draw(
                [
                    Self::TILE_SIZE * tile.x as f64,
                    Self::TILE_SIZE * tile.y as f64,
                    Self::TILE_SIZE,
                    Self::TILE_SIZE,
                ],
                &context.draw_state,
                context.transform,
                gl,
            );
            if let Some(machine) = tile.machine() {
                let mut context: Context = *context;
                context.transform = context.transform.trans(
                    tile.x as f64 * Self::TILE_SIZE,
                    tile.y as f64 * Self::TILE_SIZE,
                );
                machine.render(gl, &context);
                context.reset();
            }
        }
    }

    fn relative_index(&self, index: usize, direction: &Direction) -> Option<usize> {
        let x = index % WIDTH;
        let y = index / WIDTH;

        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some(index - WIDTH)
                }
            }
            Direction::South => {
                if y == HEIGHT - 1 {
                    None
                } else {
                    Some(index + WIDTH)
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some(index - 1)
                }
            }
            Direction::East => {
                if x == WIDTH - 1 {
                    None
                } else {
                    Some(index + 1)
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        for i in 0..self.tiles.len() - 1 {
            let target_index = if let Some(machine) = self.tiles[i].machine() {
                self.relative_index(i, machine.direction())
            } else {
                None
            };

            if self.tiles[i].machine().is_some() {
                if let Some(target_index) = target_index {
                    if let Ok([current_tile, target_tile]) =
                        self.tiles.get_many_mut([i, target_index])
                    {
                        if let Some(ref mut current_machine) = current_tile.machine_mut() {
                            if let Some(ref mut target_machine) = target_tile.machine_mut() {
                                current_machine.update(dt, Some(target_machine));
                            } else {
                                current_machine.update(dt, None);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn on_click(&mut self, args: &ButtonArgs, mouse_pos: &Point) {
        if args.state == ButtonState::Press
            && args.button == piston::Button::Mouse(piston::MouseButton::Left)
        {
            let x = (mouse_pos.x / Self::TILE_SIZE) as usize;
            let y = (mouse_pos.y / Self::TILE_SIZE) as usize;
            let point: GridPoint = GridPoint::new(x, y);

            println!("clicked: x: {}, y: {}", point.x, point.y);

            if let Some(machine) = &mut self.tiles[point.to_index(WIDTH)].machine_mut() {
                machine.on_click();
            } else {
                self.add_machine(Machine::new("D"), GridPoint::new(x, y));
            }
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        let mut tiles = Vec::with_capacity(SIZE);

        for i in 0..SIZE {
            let mut tile: Tile = Default::default();
            tile.x = i % WIDTH;
            tile.y = i / WIDTH;
            tiles.push(tile);
        }

        Self { tiles }
    }
}
