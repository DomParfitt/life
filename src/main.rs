extern crate rand;
extern crate piston_window;

mod model;
mod view;

use piston_window::*;
use model::Model;
use view::ModelView;

const CYCLES: u8 = 10;
const WINDOW_SIZE: [u32; 2] = [400; 2];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Game of Life", WINDOW_SIZE).build().unwrap();
    let model = Model::new_random(40, 40);
    let mut view = ModelView::new(model, WINDOW_SIZE);

    let mut cycle_count = CYCLES;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);
            view.render(context, graphics);
        });

        cycle_count -= 1;
        if cycle_count <= 0 {
            view.run_step();
            cycle_count = CYCLES;
        }
    }
}
