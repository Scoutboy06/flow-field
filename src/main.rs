mod flow_field;
mod vector2d;

// use flo_canvas::*;
use flo_draw::{
    canvas::{Color, GraphicsContext, GraphicsPrimitives, Transform2D},
    *,
};
use flow_field::FlowField;

pub fn main() {
    with_2d_graphics(|| {
        let width = 800;
        let height = 600;
        let tile_size = 20;

        debug_assert_eq!(
            width as f64 / tile_size as f64,
            (width as f64 / tile_size as f64).floor()
        );
        debug_assert_eq!(
            height as f64 / tile_size as f64,
            (height as f64 / tile_size as f64).floor()
        );

        let num_cols = width / tile_size;
        let num_rows = height / tile_size;

        let flow_field = FlowField::new(num_rows, num_cols, 20);
        let canvas = create_canvas_window("Flow Field");

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(height as f32);
            gc.transform(Transform2D::scale(1.0, -1.0));
            gc.center_region(0.0, 0.0, width as f32, height as f32);

            gc.new_path();

            for row in 0..(num_rows) {
                for col in 0..(num_cols) {
                    let x = (col * tile_size) as f32;
                    let y = (row * tile_size) as f32;

                    // let mut tile_vec = flow_field.tiles[row * num_cols + col].copy();

                    let tile_vec = flow_field.from_coords(row, col);
                    match tile_vec {
                        Some(tile) => {
                            let mut copy = tile.copy();
                            copy.mult(15.0);
                            gc.circle(x, y, 2.0);

                            gc.move_to(x, y);
                            gc.line_to(x + copy.x, y + copy.y);
                        }
                        None => {}
                    }
                }
            }

            for point in flow_field.points.iter() {
                gc.circle(point.x * num_cols as f32, point.y * num_rows as f32, 10.0);
            }

            gc.circle(100.0, 100.0, 5.0);

            gc.fill_color(Color::Rgba(255.0, 255.0, 255.0, 1.0));
            gc.stroke_color(Color::Rgba(255.0, 255.0, 255.0, 1.0));
            gc.fill();
            gc.stroke();
        })
    });
}
