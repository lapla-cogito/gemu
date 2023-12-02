use sdl2::{pixels::PixelFormatEnum, render::Canvas, video::Window, Sdl};

use crate::constants::{LCD_HEIGHT, LCD_WIDTH};

pub struct Lcd(Canvas<Window>);

impl Lcd {
    pub fn new(sdl: &Sdl, scale: u32) -> Self {
        let window = sdl
            .video()
            .expect("failed to init SDL video filesystem")
            .window("gemu", LCD_WIDTH as u32 * scale, LCD_HEIGHT as u32 * scale)
            .position_centered()
            .resizable()
            .build()
            .expect("failed to create window");

        let canvas = window.into_canvas().build().unwrap();
        Self(canvas)
    }

    pub fn draw(&mut self, pixels: Box<[u8]>) {
        let texture_creator = self.0.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .unwrap();

        texture.update(None, &pixels, 480).unwrap();
        self.0.clear();
        self.0.copy(&texture, None, None).unwrap();
        self.0.present();
    }
}
