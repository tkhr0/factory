use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};

use crate::grid_point::GridPoint;
use crate::machine::{Clickable, Machine, MachineCore};
use crate::tile::Tile;
use crate::types::Point;

type GridRow = [Tile; 16];
type GridField = [GridRow; 16];

#[derive(Debug)]
pub struct Field {
    grid: GridField,
}

impl Field {
    const TILE_SIZE: f64 = 50.0;

    pub fn new() -> Field {
        Self::default()
    }

    pub fn initialize(&mut self) {
        self.add_machine(Machine::new("A", 0.0), GridPoint::new(2, 3));
        self.add_machine(Machine::new("B", 0.3), GridPoint::new(3, 3));
        self.add_machine(Machine::new("C", 0.8), GridPoint::new(4, 3));
    }

    pub fn add_machine(&mut self, machine: Machine, grid_point: GridPoint) {
        self.grid[grid_point.y][grid_point.x].set_machine(machine);
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        for row in self.grid.iter() {
            for tile in row.iter() {
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
                    let mut context: Context = context.clone();
                    context.transform = context.transform.trans(
                        tile.x as f64 * Self::TILE_SIZE,
                        tile.y as f64 * Self::TILE_SIZE,
                    );
                    machine.render(gl, &context);
                    context.reset();
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        for row in self.grid.iter_mut() {
            for i in 0..row.len() {
                if let Ok([current_tile, next_tile]) = row.get_many_mut([i, i + 1]) {
                    if let Some(ref mut current_machine) = current_tile.machine_mut() {
                        println!("current: {:?}", current_machine);
                        println!("next   : {:?}", next_tile.machine());
                        if let Some(ref mut next_machine) = next_tile.machine_mut() {
                            current_machine.update(dt, Some(next_machine));
                        } else {
                            current_machine.update(dt, None);
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
            let x = (mouse_pos[0] / Self::TILE_SIZE) as usize;
            let y = (mouse_pos[1] / Self::TILE_SIZE) as usize;

            println!("clicked: x: {}, y: {}", x, y);

            if let Some(machine) = &mut self.grid[y][x].machine_mut() {
                machine.on_click();
            }
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        let mut grid: GridField = Default::default();

        for x in 0..16 {
            for y in 0..16 {
                grid[y][x].x = x;
                grid[y][x].y = y;
            }
        }

        Self { grid }
    }
}
