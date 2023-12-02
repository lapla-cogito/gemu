pub const PPU_ENABLE: u8 = 1 << 7;
pub const BG_WINDOW_TILE_DATA_SELECT: u8 = 1 << 4;
pub const BG_TILE_MAP_SELECT: u8 = 1 << 3;
pub const BG_DISPLAY_ENABLE: u8 = 1 << 0;
pub const LYC_EQ_LY: u8 = 1 << 2;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;

const M_CYCLE_CLOCK: u128 = 4;
const CPU_CLOCK_HZ: u128 = 4194304;
pub const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub const VBLANK: u8 = 1 << 0;
pub const LCD_STAT: u8 = 1 << 1;
pub const TIMER: u8 = 1 << 2;
pub const SERIAL: u8 = 1 << 3;
pub const JOYPAD: u8 = 1 << 4;
