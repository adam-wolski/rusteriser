//! Window module so we can display our backbuffer somewhere.
use sdl2;

pub struct Window<'a> {
    context: sdl2::Sdl,
    renderer: sdl2::render::Renderer<'a>,
    buffer_texture: sdl2::render::Texture,
    backbuffer: Vec<u8>,
    width: u32,
    height: u32,
}

impl<'a> Window<'a> {
    /// Initialize and return Window.
    ///
    /// # Panics
    ///
    /// SDL2 Initialization in here is really hoping for correct values as most of the results
    /// are just unwraped.
    ///
    pub fn new(title: &str, width: u32, height: u32) -> Window<'a> {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window(title, width, height).build().unwrap();
        let renderer = window.renderer().build().unwrap();
        // Create texture that will be used as a buffer.
        let bt = renderer.create_texture_static(sdl2::pixels::PixelFormatEnum::ARGB8888,
                                                width,
                                                height)
                         .unwrap();
        // Backbuffer has the same amount of pixels as window, we store data as u8 for
        // compatibility with sdl_texture so we multiply that by 4 to get ARGB8 format.
        let backbuffer: Vec<u8> = vec!(0; (width * height * 4) as usize);
        Window {
            context: context,
            renderer: renderer,
            buffer_texture: bt,
            width: width,
            height: height,
            backbuffer: backbuffer,
        }
    }

    /// Fill backbuffer with given data.
    pub fn backbuffer_fill(&mut self, data: &[u8]) {
        let max = self.backbuffer.capacity();
        for (i, v) in data.into_iter()
                          .enumerate()
                          .take_while(|&(i, _)| i < max) {
            self.backbuffer[i] = *v;
        }
    }

    /// Swap current presented buffer with stored one.
    pub fn swap(&mut self) {
        self.buffer_texture
            .update(None, self.backbuffer.as_ref(), (self.width * 4) as usize)
            .unwrap();
        self.renderer.clear();
        self.renderer.copy(&self.buffer_texture, None, None);
        self.renderer.present();
    }

    /// Check events and find out if our window still is open.
    pub fn is_running(&self) -> bool {
        let mut event_pump = match self.context.event_pump() {
            Ok(pump) => pump,
            Err(_) => return false,
        };

        let mut running = true;
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            running = match event {
                Event::Quit {..} => false,
                _ => true,
            };
            if !running {
                break;
            }

        }
        running
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
