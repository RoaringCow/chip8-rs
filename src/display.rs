
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;


const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const SDL_BLANK_COLOR: Color = Color::RGB(0x0, 0x0, 0x0);
const SDL_BLOCK_COLOR: Color = Color::RGB(0x34, 0xE6, 0x2F);


pub struct Display {
    pub canvas: WindowCanvas
}

impl Display {

    // Create a new display
    pub fn new() -> (Self, Sdl) {
        
        // Create a new SDL2 context
        let sdl_context = sdl2::init().unwrap();

        // Create a new video context
        let video_subsystem = sdl_context.video().unwrap();

        // Create a new window
        let window = video_subsystem
            .window(
                "CHIP-8 Emulator",
                SCREEN_WIDTH.try_into().unwrap(),
                SCREEN_HEIGHT.try_into().unwrap(),
            )
            .position_centered()
            .build()
            .unwrap();


        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(SDL_BLANK_COLOR);

        canvas.clear();
        

        // Return the new screen
        (Display { canvas }, sdl_context)
    }

    pub fn draw_screen(&mut self, screen: &[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if screen[y][x] {
                    self.canvas.set_draw_color(SDL_BLOCK_COLOR);
                } else {
                    self.canvas.set_draw_color(SDL_BLANK_COLOR);
                }
                self.canvas
                    .fill_rect(Rect::new(x, y, 1, 1))
                    .unwrap();
            }

            self.canvas.present();
        }
    }

    pub fn clear(&mut self) {
        // Set the canvas draw color to black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        // Clear the canvas
        self.canvas.clear();
    }
}