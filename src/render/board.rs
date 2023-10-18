use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn render_board(canvas: &mut Canvas<Window>, width: usize, height: usize, cell_width: u32, cell_height: u32, color: Color) {
    canvas.set_draw_color(color);
    for x in 0..width {
        for y in 0..height {
            canvas.draw_rect(Rect::new(
                x as i32 * cell_width as i32, y as i32 * cell_height as i32,
                cell_width,
                cell_height,
            )).unwrap();
        }
    }
}
