pub const PPU_ENABLE: u8 = 1 << 7;
pub const WINDOW_TILE_MAP_SELECT: u8 = 1 << 6;
pub const WINDOW_ENABLE: u8 = 1 << 5;
pub const BG_WINDOW_TILE_DATA_SELECT: u8 = 1 << 4;
pub const BG_TILE_MAP_SELECT: u8 = 1 << 3;
pub const OBJ_SIZE: u8 = 1 << 2;
pub const OBJ_ENABLE: u8 = 1 << 1;
pub const BG_DISPLAY_ENABLE: u8 = 1 << 0;
pub const LYC_EQ_LY_INT: u8 = 1 << 6;
pub const OAM_SCAN_INT: u8 = 1 << 5;
pub const VBLANK_INT: u8 = 1 << 4;
pub const HBLANK_INT: u8 = 1 << 3;
pub const LYC_EQ_LY: u8 = 1 << 2;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;
