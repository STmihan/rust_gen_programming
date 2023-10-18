use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::config;
use crate::models::food::Food;

fn render_single_food(food: &Food, canvas: &Canvas<Window>) {
    let width = config::CELL_WIDTH as i32;
    let height = config::CELL_HEIGHT as i32;

    let x = food.pos.x * width + width / 2;
    let y = food.pos.y * height + height / 2;

    let r = ((width / 2) as f32 * config::FOOD_SCALE).floor() as i16;

    canvas.filled_circle(
        x as i16,
        y as i16,
        r,
        config::FOOD_COLOR,
    ).unwrap();
}

pub fn render_food(food: &Vec<Food>, canvas: &Canvas<Window>) {
    for food in food {
        render_single_food(food, canvas);
    }
}
