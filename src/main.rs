#![feature(get_many_mut)]
#![feature(associated_type_defaults)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    ButtonArgs, ButtonEvent, ButtonState, MouseCursorEvent, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent,
};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    field: Field,
}

type Point = [f64; 2];
type Size = [f64; 2];

impl App {
    fn initialize(&mut self) {
        self.field.initialize();
    }

    fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);
            self.field.render(gl, &c);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.field.update(args.dt);
    }

    pub fn button(&mut self, args: &ButtonArgs, mouse_pos: &Point) {
        self.field.on_click(args, mouse_pos);
    }
}

pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl GridPoint {
    pub fn new(x: usize, y: usize) -> GridPoint {
        GridPoint { x, y }
    }
}

impl Default for GridPoint {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    machine: Option<Machine>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            machine: None,
        }
    }
}

type GridRow = [Tile; 16];
type GridField = [GridRow; 16];

#[derive(Debug)]
pub struct Field {
    size: Size,
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

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn add_machine(&mut self, machine: Machine, grid_point: GridPoint) {
        self.grid[grid_point.y][grid_point.x].machine = Some(machine);
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        for row in self.grid.iter() {
            for tile in row.iter() {
                Rectangle::new_border([0.0, 0.0, 0.0, 0.1], 1.0).draw(
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
                if let Some(machine) = &tile.machine {
                    let mut context: Context = context.clone();
                    context.transform = context.transform.trans(
                        tile.x as f64 * Self::TILE_SIZE,
                        tile.y as f64 * Self::TILE_SIZE,
                    );
                    machine.render(gl, &context);
                }
                context.reset();
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        for row in self.grid.iter_mut() {
            for i in 0..row.len() {
                if let Ok([current_tile, next_tile]) = row.get_many_mut([i, i + 1]) {
                    if let Some(ref mut current_machine) = current_tile.machine {
                        println!("current: {:?}", current_machine);
                        println!("next   : {:?}", next_tile.machine);
                        if let Some(ref mut next_machine) = next_tile.machine {
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

            if let Some(machine) = &mut self.grid[y][x].machine {
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

        Self {
            size: [500.0, 500.0],
            grid,
        }
    }
}

#[derive(Debug)]
pub struct Resource {}

trait Clickable {
    fn on_click(&mut self);
}

trait MachineCore<S> {
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
        self.cooling_time() > 1.0
    }
}

#[derive(Debug)]
pub struct Container {
    slot: Vec<Option<Resource>>,
}

impl Container {
    fn new() -> Container {
        Container {
            slot: vec![None, None, None, None],
        }
    }

    fn update(&mut self) {
        for i in 0..(self.slot.len() - 1) {
            if self.slot[i].is_none() {
                self.slot.swap(i, i + 1);
            }
        }
    }

    fn pick(&mut self) -> Option<Resource> {
        self.slot.push(None);
        self.slot.swap_remove(0)
    }

    fn iter(&self) -> std::slice::Iter<'_, Option<Resource>> {
        self.slot.iter()
    }

    fn len(&self) -> usize {
        self.slot.len()
    }

    fn load(&mut self) {
        let length = self.len();
        if self.slot[length - 1].is_some() {
            return;
        }
        self.slot[length - 1] = Some(Resource {});
    }

    fn acceptable(&self) -> bool {
        self.slot[self.len() - 1].is_none()
    }

    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
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

#[derive(Debug)]
pub struct Machine {
    name: &'static str,
    container: Container,
    cooling_time: f64,
}

impl Machine {
    pub fn new(name: &'static str, cooling_time: f64) -> Machine {
        Machine {
            name,
            container: Container::new(),
            cooling_time,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn load(&mut self) {
        self.container.load();
    }

    fn pick(&mut self) -> Option<Resource> {
        self.container.pick()
    }

    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
        self.container.push(resource)
    }

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    pub fn size(&self) -> Size {
        [self.width(), self.height()]
    }

    fn top_left(&self) -> Point {
        [0.0, 0.0]
    }

    fn top_right(&self) -> Point {
        [self.width(), 0.0]
    }

    fn bottom_left(&self) -> Point {
        [0.0, self.height()]
    }

    fn bottom_right(&self) -> Point {
        [self.width(), self.height()]
    }

    pub fn collided(&self, point: &Point) -> bool {
        let x = point[0];
        let y = point[1];
        let top_left = self.top_left();
        let top = top_left[1];
        let left = top_left[0];
        let bottom_right = self.bottom_right();
        let right = bottom_right[0];
        let bottom = bottom_right[1];
        left <= x && x <= right && top <= y && y <= bottom
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        const BODY: [f32; 4] = [255.0, 186.0, 3.0, 1.0];
        const OUTLINE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let size = self.size();

        rectangle(BODY, [0.0, 0.0, size[0], size[1]], context.transform, gl);
        line(
            OUTLINE,
            1.0,
            [
                self.top_left()[0],
                self.top_left()[1],
                self.top_right()[0],
                self.top_right()[1],
            ],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.top_right()[0],
                self.top_right()[1],
                self.bottom_right()[0],
                self.bottom_right()[1],
            ],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.bottom_right()[0],
                self.bottom_right()[1],
                self.bottom_left()[0],
                self.bottom_left()[1],
            ],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.bottom_left()[0],
                self.bottom_left()[1],
                self.top_left()[0],
                self.top_left()[1],
            ],
            context.transform,
            gl,
        );

        for (i, resource) in self.container.iter().enumerate() {
            if resource.is_some() {
                ellipse(
                    OUTLINE,
                    [((i as f64) * 10.0), 0.0, 10.0, 10.0],
                    context.transform,
                    gl,
                );
            }
        }
    }
}

impl MachineCore<Resource> for Machine {
    fn set_cooling_time(&mut self, dt: f64) {
        self.cooling_time = dt;
    }

    fn cooling_time(&self) -> f64 {
        self.cooling_time
    }

    fn main(&mut self, neighbor: Option<&mut Self>) {
        self.container.update();

        if let Some(neighbor) = neighbor {
            if self.container.acceptable() {
                self.push(neighbor.pick()).unwrap();
            }
        }
    }
}

impl Clickable for Machine {
    fn on_click(&mut self) {
        self.load();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        field: Field::new(),
    };

    app.initialize();

    let mut mouse_pos = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(mouse_args) = e.mouse_cursor_args() {
            mouse_pos = mouse_args;
        }

        if let Some(args) = e.button_args() {
            println!("Button: {:?}", args);
            app.button(&args, &mouse_pos);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
