use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};

use crate::grid_point::GridPoint;
use crate::machine::ConveyerBuilder;
use crate::tile::Tile;
use crate::types::{Direction, Point};
use crate::Fixture;

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
        self.add_fixture(
            ConveyerBuilder::new("A")
                .set_direction(Direction::East)
                .build(),
            GridPoint::new(2, 3),
        );
        self.add_fixture(
            ConveyerBuilder::new("B")
                .set_direction(Direction::East)
                .build(),
            GridPoint::new(3, 3),
        );
        self.add_fixture(
            ConveyerBuilder::new("C")
                .set_direction(Direction::South)
                .build(),
            GridPoint::new(4, 3),
        );
        self.add_fixture(
            ConveyerBuilder::new("D")
                .set_direction(Direction::South)
                .build(),
            GridPoint::new(4, 4),
        );
        self.add_fixture(
            ConveyerBuilder::new("E")
                .set_direction(Direction::West)
                .build(),
            GridPoint::new(4, 5),
        );
        self.add_fixture(
            ConveyerBuilder::new("F")
                .set_direction(Direction::West)
                .build(),
            GridPoint::new(3, 5),
        );
        self.add_fixture(
            ConveyerBuilder::new("G")
                .set_direction(Direction::North)
                .build(),
            GridPoint::new(2, 5),
        );
        self.add_fixture(
            ConveyerBuilder::new("H")
                .set_direction(Direction::North)
                .build(),
            GridPoint::new(2, 4),
        );
    }

    pub fn add_fixture(&mut self, fixture: Box<dyn Fixture>, grid_point: GridPoint) {
        self.tiles[grid_point.to_index(WIDTH)].set_fixture(fixture);
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
            if let Some(fixture) = tile.fixture() {
                let mut context: Context = *context;
                context.transform = context.transform.trans(
                    tile.x as f64 * Self::TILE_SIZE,
                    tile.y as f64 * Self::TILE_SIZE,
                );
                fixture.render(gl, &context);
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
        for tile in self.tiles.iter_mut() {
            tile.update(dt);
        }
    }

    pub fn on_click(&mut self, args: &ButtonArgs, mouse_pos: &Point) {
        if args.state == ButtonState::Press {
            let x = (mouse_pos.x / Self::TILE_SIZE) as usize;
            let y = (mouse_pos.y / Self::TILE_SIZE) as usize;
            let point: GridPoint = GridPoint::new(x, y);

            println!("clicked: x: {}, y: {}", point.x, point.y);

            match args.button {
                piston::Button::Mouse(piston::MouseButton::Left) => {
                    if let Some(fixture) = &mut self.tiles[point.to_index(WIDTH)].fixture_mut() {
                        fixture.on_click();
                    } else {
                        self.add_fixture(ConveyerBuilder::new("D").build(), GridPoint::new(x, y));
                    }
                }
                piston::Button::Keyboard(piston::Key::R) => {
                    if let Some(fixture) = &mut self.tiles[point.to_index(WIDTH)].fixture_mut() {
                        fixture.rotate();
                    }
                }
                _ => (),
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
