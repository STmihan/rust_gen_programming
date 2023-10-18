use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::config;
use crate::models::coord::Coord;

fn render_wall(at: &Coord, canvas: &mut Canvas<Window>) {

    canvas.set_draw_color(config::WALL_COLOR);
    canvas.fill_rect(Rect::new(
        at.x * config::CELL_WIDTH as i32,
        at.y * config::CELL_HEIGHT as i32,
        config::CELL_WIDTH,
        config::CELL_HEIGHT,
    )).unwrap();
}
pub fn render_walls(walls: &Vec<Coord>, canvas: &mut Canvas<Window>) {
    for wall in walls {
        render_wall(wall, canvas);
    }
}