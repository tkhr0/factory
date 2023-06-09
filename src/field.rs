use std::iter::Iterator;

use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};

use crate::coordinate::{GridPoint, Point, Size, TILE_COLUMNS, TILE_LENGTH, TILE_SIZE};
use crate::item::{Machine, MachineFactory, MaterialVariant};
use crate::natural_resource::{Coal, IronOre, NaturalResource};
use crate::types::Direction;
use crate::Tile;
use crate::TileState;

#[derive(Debug)]
pub struct Field {
    tiles: Vec<Tile>, // FIXME: Use a array instead of a vector
}

impl Field {
    pub fn new() -> Field {
        Self::default()
    }

    pub fn initialize(&mut self) {
        self.add_natural_resource(Box::new(IronOre::new()), GridPoint::new(2, 2));

        self.add_fixture(
            MachineFactory::build(MaterialVariant::MiningDrill).unwrap(),
            GridPoint::new(2, 2),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Inserter).unwrap(),
            GridPoint::new(3, 2),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(4, 2),
        );

        self.add_natural_resource(Box::new(Coal::new()), GridPoint::new(2, 3));
        self.add_fixture(
            MachineFactory::build(MaterialVariant::MiningDrill).unwrap(),
            GridPoint::new(2, 3),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Inserter).unwrap(),
            GridPoint::new(3, 3),
        );
        self.add_fixture(
            MachineFactory::build(MaterialVariant::Conveyer).unwrap(),
            GridPoint::new(4, 3),
        );

        self.add_fixture(
            MachineFactory::build(MaterialVariant::Container).unwrap(),
            GridPoint::new(6, 3),
        );
    }

    pub fn add_fixture(&mut self, fixture: Box<dyn Machine>, grid_point: GridPoint) {
        self.tiles[grid_point.as_index()].set_fixture(fixture);
    }

    pub fn add_natural_resource(
        &mut self,
        natural_resource: Box<dyn NaturalResource>,
        grid_point: GridPoint,
    ) {
        self.tiles[grid_point.as_index()].set_natural_resource(natural_resource);
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        let tile_size = Size::new(TILE_SIZE, TILE_SIZE);

        for tile in self.tiles.iter() {
            graphics::Rectangle::new_border([0.0, 0.0, 0.0, 0.1], 1.0).draw(
                [
                    TILE_SIZE * tile.x as f64,
                    TILE_SIZE * tile.y as f64,
                    TILE_SIZE,
                    TILE_SIZE,
                ],
                &context.draw_state,
                context.transform,
                gl,
            );

            if let Some(natural_resource) = tile.natural_resource() {
                let mut context: Context = *context;
                context.transform = context
                    .transform
                    .trans(tile.x as f64 * TILE_SIZE, tile.y as f64 * TILE_SIZE);
                natural_resource.render(gl, &context, &tile_size);
            }

            if let Some(fixture) = tile.fixture() {
                let mut context: Context = *context;
                context.transform = context
                    .transform
                    .trans(tile.x as f64 * TILE_SIZE, tile.y as f64 * TILE_SIZE);
                fixture.render(gl, &context);
                context.reset();
            }
        }
    }

    pub fn tile_state(&self, point: &Point) -> TileState {
        let x = (point.x / TILE_SIZE) as usize;
        let y = (point.y / TILE_SIZE) as usize;

        if let Some(tile) = self.tiles.get(y * TILE_COLUMNS + x) {
            tile.to_tile_state()
        } else {
            TileState::default()
        }
    }

    fn relative_direction(&self, base_index: usize, other_index: usize) -> Direction {
        let base_x = base_index % TILE_COLUMNS;
        let base_y = base_index / TILE_COLUMNS;
        let other_x = other_index % TILE_COLUMNS;
        let other_y = other_index / TILE_COLUMNS;

        if base_x == other_x {
            if base_y == other_y {
                Direction::Origin
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
                if i == other_index {
                    self.tiles[i].affect_self();
                } else if let Ok([tile, other]) = self.tiles.get_many_mut([i, other_index]) {
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
            let x = (mouse_pos.x / TILE_SIZE) as usize;
            let y = (mouse_pos.y / TILE_SIZE) as usize;
            let point: GridPoint = GridPoint::new(x, y);

            match args.button {
                piston::Button::Mouse(piston::MouseButton::Left) => {
                    if let Some(fixture) = &mut self.tiles[point.as_index()].fixture_mut() {
                        fixture.on_click();
                    } else if let Some(item) = holding_item {
                        self.add_fixture(item, point);
                    }
                }
                piston::Button::Keyboard(piston::Key::R) => {
                    if let Some(fixture) = &mut self.tiles[point.as_index()].fixture_mut() {
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
        let mut tiles = Vec::with_capacity(TILE_LENGTH);

        for i in 0..TILE_LENGTH {
            let mut tile: Tile = Default::default();
            tile.x = i % TILE_COLUMNS;
            tile.y = i / TILE_COLUMNS;
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
    use crate::coordinate::GridPoint;

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
                    let index = point.as_index();
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
        coordinate::GridPoint,
        field::TILE_LENGTH,
        item::{MachineFactory, MaterialVariant},
        tile::Tile,
    };

    #[test]
    fn test_tile_contains_target_range() {
        use self::super::TileIterator;

        let mut tiles = Vec::new();
        for i in 0..(TILE_LENGTH) {
            let mut tile: Tile = Default::default();
            let point = GridPoint::from_index(i);
            tile.x = point.x;
            tile.y = point.y;
            tiles.push(tile);
        }
        let index = GridPoint::new(2, 2).as_index();
        tiles[index].set_fixture(MachineFactory::build(MaterialVariant::Conveyer).unwrap());

        let mut iter = TileIterator::new(index, &tiles);

        assert_eq!(iter.next(), Some(GridPoint::new(1, 1).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 1).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(3, 1).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 2).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 2).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(3, 2).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(1, 3).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(2, 3).as_index()));
        assert_eq!(iter.next(), Some(GridPoint::new(3, 3).as_index()));
        assert_eq!(iter.next(), None);
    }
}
