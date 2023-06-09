use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState, RenderArgs, UpdateArgs};
use piston::{Button, Key, ResizeArgs};

use crate::coordinate;
use crate::EventHandleState;
use crate::Field;
use crate::Hud;
use crate::PlayerState;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    field: Field,
    hud: Hud,
}

impl Game {
    pub fn new(window_size: coordinate::Size, gl: GlGraphics, quick_slot_len: usize) -> Self {
        Self {
            gl,
            field: Field::new(),
            hud: Hud::new(window_size, quick_slot_len),
        }
    }

    pub fn initialize(&mut self) {
        self.field.initialize();
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
        player_state: &PlayerState,
        mouse_pos: &coordinate::Point,
    ) {
        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BACKGROUND, gl);
            self.field.render(gl, &c);
            let tile_state = self.field.tile_state(mouse_pos);
            self.hud.render(&c, gl, player_state, mouse_pos, tile_state);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.field.update(args.dt);
    }

    pub fn button(
        &mut self,
        args: &ButtonArgs,
        mouse_pos: &coordinate::Point,
        player_state: &mut PlayerState,
    ) {
        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(Key::E) => {
                    player_state.toggle_inventory();
                }
                Button::Keyboard(Key::Q) => {
                    player_state.release_item();
                }
                _ => {}
            }
        }

        // Click
        {
            let mut state: EventHandleState = Default::default();
            state = self.hud.click(args, mouse_pos, state, player_state);

            if !state.consumed() {
                self.field.on_click(
                    args,
                    mouse_pos,
                    player_state.holding_item().map(|v| v.as_machine().unwrap()),
                );
            }
        }
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.hud.resize(args);
    }
}
