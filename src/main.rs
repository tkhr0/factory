#![feature(get_many_mut)]
#![feature(associated_type_defaults)]
#![feature(trait_upcasting)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, MouseCursorEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::ResizeEvent;

mod coordinate;
mod event_handle_state;
mod field;
mod game;
mod hud;
mod inventory;
mod item;
mod natural_resource;
mod natural_resource_variant;
mod player_state;
mod quick_slot;
mod slot;
mod tile;
mod tile_state;
mod types;

use event_handle_state::EventHandleState;
use field::Field;
use game::Game;
use hud::Hud;
use inventory::Inventory;
use natural_resource::NaturalResource;
use natural_resource_variant::NaturalResourceVariant;
use player_state::PlayerState;
use quick_slot::QuickSlot;
use slot::Slot;
use tile::Tile;
use tile_state::TileState;

fn main() {
    const WINDOW_SIZE: coordinate::Size = coordinate::Size::new(1300.0, 700.0);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("factory", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player_state: PlayerState = Default::default();
    let quick_slot = player_state.quick_slot_mut();
    quick_slot.set_item(0, item::MaterialVariant::Inserter);
    quick_slot.set_item(1, item::MaterialVariant::Conveyer);
    quick_slot.set_item(2, item::MaterialVariant::Container);

    let mut game = Game::new(
        WINDOW_SIZE,
        GlGraphics::new(opengl),
        player_state.quick_slot().len(),
    );

    game.initialize();

    let mut mouse_pos = coordinate::Point::new(0.0, 0.0);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.resize_args() {
            game.resize(&args);
        }

        if let Some(mouse_args) = e.mouse_cursor_args() {
            mouse_pos = coordinate::Point::new(mouse_args[0], mouse_args[1]);
        }

        if let Some(args) = e.button_args() {
            game.button(&args, &mouse_pos, &mut player_state);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.render_args() {
            game.render(&args, &player_state, &mouse_pos);
        }
    }
}
