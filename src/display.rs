use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    pub frame_buffer: [[bool; WIDTH]; HEIGHT],
    pub window: Window,
}

impl Display {
    pub fn default() -> Self {
        let mut display = Display { 
            frame_buffer: [[false; WIDTH]; HEIGHT], 
            window: Window::new(
                "Idek",
                WIDTH,
                HEIGHT, 
                WindowOptions {
                    scale: minifb::Scale::X16,
                    ..WindowOptions::default()
                },
            ).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
        };

        display.window.set_target_fps(60);

        display
    }

    pub fn render(&mut self) {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        //convert 2D bool array to 1D u32 vec
        let mut vec_index = 0;
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if self.frame_buffer[row][col] {
                    buffer[vec_index] = 0xFFFFFFFF;
                } else {
                    buffer[vec_index] = 0x00000000;
                }
                vec_index += 1;
            }
        }

        self.window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn clear(&mut self) {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                self.frame_buffer[row][col] = false;
            }
        }
    }
}