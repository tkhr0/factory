use std::iter::Iterator;

use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};

use crate::item::{Machine, MachineFactory, MaterialVariant};
use crate::natural_resource::IronOre;
use crate::types::{Direction, GridPoint, Point, Size};
use crate::Tile;

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
        self.tiles[0].set_natural_resource(Box::new(IronOre::new()));

        self.add_fixture(
            MachineFactory::build(MaterialVariant::MiningDrill).unwrap(),
            GridPoint::new(0, 0),
        );

        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(2, 3),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(3, 3),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(4, 3),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(4, 4),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(4, 5),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(3, 5),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(2, 5),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(2, 4),
        );

        self.add_fixture(
            MachineFactory::build(MaterialVariant::Container).unwrap(),
            GridPoint::new(6, 3),
        );
    }

    pub fn add_fixture(&mut self, fixture: Box<dyn Machine>, grid_point: GridPoint) {
        self.tiles[grid_point.as_index(WIDTH)].set_fixture(fixture);
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        let tile_size = Size::new(Self::TILE_SIZE, Self::TILE_SIZE);

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

            if let Some(natural_resource) = tile.natural_resource() {
                let mut context: Context = *context;
                context.transform = context.transform.trans(
                    tile.x as f64 * Self::TILE_SIZE,
                    tile.y as f64 * Self::TILE_SIZE,
                );
                natural_resource.render(gl, &context, &tile_size);
            }

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

    fn relative_direction(&self, base_index: usize, other_index: usize) -> Direction {
        let base_x = base_index % WIDTH;
        let base_y = base_index / WIDTH;
        let other_x = other_index % WIDTH;
        let other_y = other_index / WIDTH;

        if base_x == other_x {
            if base_y == other_y {
                Direction::None
            } else if base_y - 1 == other_y {
                Direction::North
            } else if base_y + 1 == other_y {
                Direction::South
            } else {
                Direction::None
            }
        } else if base_y == other_y {
            if base_x + 1 == other_x {
                Direction::East
            } else if base_x - 1 == other_x {
                Direction::West
            } else {
                Direction::None
            }
        } else {
            Direction::None
        }
    }

    pub fn update(&mut self, dt: f64) {
        for tile in self.tiles.iter_mut() {
            tile.update(dt);
        }

        for i in 0..self.tiles.len() {
            for other_index in self.effect_range(i) {
                let direction = self.relative_direction(i, other_index);
                if let Ok([tile, other]) = self.tiles.get_many_mut([i, other_index]) {
                    tile.affect(other, direction);
                }
            }
        }
    }

    pub fn on_click(
        &mut self,
        args: &ButtonArgs,
        mouse_pos: &Point,
        holding_item: Option<Box<dyn Machine>>,
    ) {
        if args.state == ButtonState::Press {
            let x = (mouse_pos.x / Self::TILE_SIZE) as usize;
            let y = (mouse_pos.y / Self::TILE_SIZE) as usize;
            let point: GridPoint = GridPoint::new(x, y);

            match args.button {
                piston::Button::Mouse(piston::MouseButton::Left) => {
                    if let Some(fixture) = &mut self.tiles[point.as_index(WIDTH)].fixture_mut() {
                        fixture.on_click();
                    } else if let Some(item) = holding_item {
                        self.add_fixture(item, point);
                    }
                }
                piston::Button::Keyboard(piston::Key::R) => {
                    if let Some(fixture) = &mut self.tiles[point.as_index(WIDTH)].fixture_mut() {
                        fixture.rotate();
                    }
                }
                _ => (),
            }
        }
    }

    // tile_index のタイルが影響を与える範囲にあるタイルのイテレータを返す
    fn effect_range(&self, tile_index: usize) -> TileIterator {
        TileIterator::new(tile_index, &self.tiles)
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

#[derive(Debug)]
struct TileRange {
    start: GridPoint,
    end: GridPoint,
    current: Option<GridPoint>,
}

impl TileRange {
    pub fn new(start: GridPoint, end: GridPoint) -> Self {
        Self {
            start,
            end,
            current: None,
        }
    }
}

impl Iterator for &mut TileRange {
    type Item = GridPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.x > self.end.x || self.start.y > self.end.y {
            None
        } else {
            if self.current.is_none() {
                self.current = Some(self.start);
            } else {
                let current = self.current.as_ref().unwrap_or(&self.start);
                let next = GridPoint::new(current.x + 1, current.y);

                let next = if self.end.x < next.x {
                    GridPoint::new(self.start.x, current.y + 1)
                } else {
                    next
                };

                if self.end.x < next.x || self.end.y < next.y {
                    self.current = None;
                } else {
                    self.current = Some(next);
                }
            }

            self.current
        }
    }
}

#[cfg(test)]
mod test {
    use self::super::TileRange;
    use crate::types::GridPoint;

    #[test]
    fn test_tile_range() {
        let mut range = TileRange::new(GridPoint::new(0, 0), GridPoint::new(2, 2));

        let mut iter = range.into_iter();

        assert_eq!(iter.next(), Some(GridPoint::new(0, 0)));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 0)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 0)));
        assert_eq!(iter.next(), Some(GridPoint::new(0, 1)));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 1)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 1)));
        assert_eq!(iter.next(), Some(GridPoint::new(0, 2)));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 2)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_range_misaligned_orien() {
        let mut range = TileRange::new(GridPoint::new(1, 1), GridPoint::new(2, 2));
        let mut iter = range.into_iter();

        assert_eq!(iter.next(), Some(GridPoint::new(1, 1)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 1)));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 2)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_range_contains_start_when_start_is_equal_end() {
        let mut range = TileRange::new(GridPoint::new(1, 1), GridPoint::new(1, 1));
        let mut iter = range.into_iter();

        assert_eq!(iter.next(), Some(GridPoint::new(1, 1)));
        assert_eq!(iter.next(), None);
    }
}

#[derive(Debug)]
enum TileIteratorState {
    None,
    Range(TileRange),
}

#[derive(Debug)]
struct TileIterator {
    target_index: usize,
    range: TileIteratorState,
}

impl TileIterator {
    pub fn new(target_index: usize, tiles: &[Tile]) -> Self {
        let (x, y) = {
            let target = &tiles[target_index];
            (target.x, target.y)
        };
        let diff = {
            let target = &tiles[target_index];
            target.effect_range()
        };

        if diff.is_none() {
            return Self {
                target_index,
                range: TileIteratorState::None,
            };
        }
        let diff = diff.unwrap();
        let diff_width = (diff.width as f64 / 2.0) as usize;
        let diff_height = (diff.height as f64 / 2.0) as usize;

        let start = GridPoint::new(
            if 0 < x { x - diff_width } else { x },
            if 0 < y { y - diff_height } else { y },
        );
        // FIXME: サイズオーバーする
        let end = GridPoint::new(x + diff_width, y + diff_height);

        Self {
            target_index,
            range: TileIteratorState::Range(TileRange::new(start, end)),
        }
    }
}

impl Iterator for TileIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.range {
            TileIteratorState::None => None,
            TileIteratorState::Range(range) => {
                for point in range {
                    let index = point.as_index(WIDTH);
                    if index == self.target_index {
                        continue;
                    }
                    return Some(index);
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod test_tile_iterator {
    use crate::{
        field::{SIZE, WIDTH},
        item::{MachineFactory, MaterialVariant},
        tile::Tile,
        types::GridPoint,
    };

    #[test]
    fn test_tile_contains_target_range() {
        use self::super::TileIterator;

        let mut tiles = Vec::new();
        for i in 0..(SIZE) {
            let mut tile: Tile = Default::default();
            let point = GridPoint::from_index(i, WIDTH);
            tile.x = point.x;
            tile.y = point.y;
            tiles.push(tile);
        }
        let index = GridPoint::new(2, 2).as_index(WIDTH);
        tiles[index].set_fixture(MachineFactory::build(MaterialVariant::Conveyer));

        let mut iter = TileIterator::new(index, &tiles);

        assert_eq!(iter.next(), Some(GridPoint::new(1, 1).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 1).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(3, 1).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 2).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(3, 2).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 3).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 3).as_index(WIDTH)));
        assert_eq!(iter.next(), Some(GridPoint::new(3, 3).as_index(WIDTH)));
        assert_eq!(iter.next(), None);
    }
}
