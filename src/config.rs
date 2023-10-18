use sdl2::pixels::Color;macro_rules! color_from_hex {
    ($hex:expr) => {
        Color::RGBA(
            ((($hex as u32) >> (3 * 8)) & 0xFF) as u8,
            ((($hex as u32) >> (2 * 8)) & 0xFF) as u8,
            ((($hex as u32) >> (1 * 8)) & 0xFF) as u8,
            ((($hex as u32) >> (0 * 8)) & 0xFF) as u8,
        )
    };
}

pub const FPS: u32 = 30;

pub const WINDOW_WIDTH: u32 = 900;
pub const WINDOW_HEIGHT: u32 = 900;

pub const CANVAS_WIDTH: u32 = 800;
pub const CANVAS_HEIGHT: u32 = 800;

pub const BOARD_WIDTH: u32 = 50;
pub const BOARD_HEIGHT: u32 = 50;

pub const CELL_WIDTH: u32 = CANVAS_WIDTH / BOARD_WIDTH;
pub const CELL_HEIGHT: u32 = CANVAS_HEIGHT / BOARD_HEIGHT;

pub const AGENT_SCALE: f32 = 0.7;
pub const FOOD_SCALE: f32 = 0.4;


// Counts
pub const AGENT_COUNT: usize = 100;
pub const WALLS_COUNT: usize = 100;
pub const FOOD_COUNT: usize = 400;

pub const RENDER_GRID: bool = false;
pub const TRAINING_MODE: bool = true;

// Agent
pub const AGENT_ENERGY: i16 = 100;
pub const AGENT_ENERGY_LOSS: i16 = 1;
pub const AGENT_ENERGY_RESTORE: i16 = 10;
pub const AGENT_VIEW_DISTANCE: u8 = 3;

// Colors
pub const BACKGROUND_COLOR: Color = color_from_hex!(0x2E3138FF);
pub const VIEWPORT_COLOR: Color = color_from_hex!(0x3F4148FF);
pub const BOARD_COLOR: Color = color_from_hex!(0x6C6F7DFF);
pub const AGENT_COLOR: Color = color_from_hex!(0xFE5F55FF);
pub const DEAD_AGENT_COLOR: Color = color_from_hex!(0xCCCCCCFF);
pub const SELECTED_AGENT_COLOR: Color = color_from_hex!(0xba453dff);
pub const AGENT_ENERGY_COLOR: Color = color_from_hex!(0xFFD460FF);
pub const WALL_COLOR: Color = BOARD_COLOR;
pub const FOOD_COLOR: Color = color_from_hex!(0x44BBA4FF);
