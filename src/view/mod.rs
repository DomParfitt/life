use model::Model;
use piston_window::rectangle;
use piston_window::Context;
use piston_window::G2d;

pub struct ModelView {
    window_size: [u32; 2],
    model: Model,
}

impl ModelView {

    pub fn new(model: Model, window_size: [u32; 2]) -> Self {
        ModelView {
            window_size: window_size,
            model: model
        }
    }

    pub fn render(&mut self, context: Context, graphics: &mut G2d) {
        let x_size = self.window_size[0] as usize / self.model.width;
        let y_size = self.window_size[1] as usize / self.model.height;
        for y in 0..self.model.height {
            for x in 0..self.model.width {
                let color = if self.model.is_alive(x, y) {
                    [0.0, 0.0, 0.0, 1.0] 
                } else {
                    [1.0, 1.0, 1.0, 1.0]
                };

                rectangle(
                    color,
                    [(x * x_size) as f64, (y * y_size) as f64, x_size as f64, y_size as f64],
                    context.transform,
                    graphics,
                );
            }
        }
    }

    pub fn run_step(&mut self) {
        self.model.run_step();
    }
}
