use raylib::prelude::*;

#[derive(Default, Debug)]
pub struct VirtualPetBuilder {
    // Your typical vsync
    vsync: bool,
    // Show red background where the window is
    show_hitbox: bool,
    // Title of the app
    title: String,
    // How much the window refreshes
    fps: u32,
    // How much the animation frame wait
    animation_fps: u32,
    // Stuff for the animation sprite
    img: ImgBuilderInfo,
    // Stuff for the animation sprite
    window_position: Vector2,
}

#[derive(Default, Debug)]
pub struct ImgBuilderInfo {
    data: ImgFile,
    info: ImgInfo,
}

#[derive(Default, Debug)]
pub enum ImgFile {
    #[default]
    Empty,
    External(String),
    // I don't know if raylib support internal bytes
    // Internal(&'static [u8; 0]),
}

impl ImgBuilderInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filepath(mut self, path: &str) -> Self {
        self.data = ImgFile::External(String::from(path));
        self
    }

    pub fn width_slice(mut self, width: u32) -> Self {
        self.info.width_slice = width;
        self
    }

    pub fn height_slice(mut self, height: u32) -> Self {
        self.info.height_slice = height;
        self
    }

    pub fn idle_row(mut self, row: u32) -> Self {
        self.info.idle_row = row;
        self
    }

    pub fn move_row(mut self, row: u32) -> Self {
        self.info.move_row = row;
        self
    }

    pub fn max_animation_frame(mut self, max: u32) -> Self {
        self.info.max_animation_frame = max;
        self
    }
}

#[derive(Default, Debug)]
pub struct ImgInfo {
    // width: u32,
    // height: u32,
    idle_row: u32,
    move_row: u32,
    width_slice: u32,
    height_slice: u32,
    max_animation_frame: u32,
}

#[derive(Debug)]
pub struct VirtualPet {
    rl: RaylibHandle,
    thread: RaylibThread,
    show_hitbox: bool,

    animation: SpriteAnimation,

    should_exit: bool,
    window_pos: Vector2,
    pan_offset: Vector2,
    dragged: bool,
}

#[derive(Debug)]
pub struct SpriteAnimation {
    texture: Texture2D,
    render_texture: RenderTexture2D,
    current_frame: u32,
    current_fps: u32,
    target_fps: u32,

    info: ImgInfo,
}

impl SpriteAnimation {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        img: ImgBuilderInfo,
        animation_fps: u32,
    ) -> SpriteAnimation {
        // INFO : I should extract this to somewhere
        let pet = match img.data {
            ImgFile::Empty => panic!("Whoa, no img?"),
            ImgFile::External(file) => rl.load_texture(&thread, &file).expect("File not found:("),
        };

        let render_texture = rl
            .load_render_texture(&thread, img.info.width_slice, img.info.height_slice)
            .expect("Render Texture fail:(");

        Self {
            current_fps: 0,
            current_frame: 0,
            target_fps: animation_fps,
            texture: pet,
            info: img.info,
            render_texture,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread, dragged: bool) {
        self.current_fps += 1;
        if self.current_fps > self.target_fps {
            self.current_fps = 0;

            self.current_frame = (self.current_frame + 1) % self.info.max_animation_frame;

            let row = if dragged {
                self.info.move_row
            } else {
                self.info.idle_row
            };

            let source = Rectangle::new(
                self.info.width_slice as f32 * self.current_frame as f32,
                self.info.height_slice as f32 * row as f32,
                self.info.width_slice as f32,
                self.info.height_slice as f32,
            );

            {
                let mut mode = d.begin_texture_mode(&thread, &mut self.render_texture);
                mode.clear_background(Color::BLANK);
                mode.draw_texture_pro(
                    &self.texture,
                    source,
                    Rectangle::new(
                        0.,
                        0.,
                        self.info.width_slice as f32,
                        self.info.height_slice as f32,
                    ),
                    Vector2::new(self.info.width_slice as f32, self.info.height_slice as f32),
                    180.,
                    Color::WHITE,
                );
            }
        }
    }
}

impl VirtualPetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vsync(mut self) -> Self {
        self.vsync = true;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }

    pub fn show_hitbox(mut self) -> Self {
        self.show_hitbox = true;
        self
    }

    pub fn window_position(mut self, x: f32, y: f32) -> Self {
        self.window_position = Vector2::new(x, y);
        self
    }

    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn animation_fps(mut self, fps: u32) -> Self {
        self.animation_fps = fps;
        self
    }

    pub fn img(mut self, img: ImgBuilderInfo) -> Self {
        self.img = img;
        self
    }

    pub fn build(self) -> VirtualPet {
        let mut binding = raylib::init();
        let rl = binding
            .size(
                self.img.info.width_slice as i32,
                self.img.info.height_slice as i32,
            )
            .title(&self.title)
            .undecorated()
            .transparent();

        if self.vsync {
            rl.vsync();
        }

        let (mut rl, thread) = rl.build();
        rl.set_target_fps(self.fps);
        let animation = SpriteAnimation::new(&mut rl, &thread, self.img, self.animation_fps);

        VirtualPet {
            rl,
            thread,
            show_hitbox: self.show_hitbox,

            animation,

            should_exit: false,
            pan_offset: Vector2::new(0., 0.),
            dragged: false,
            window_pos: self.window_position,
        }
    }
}

impl VirtualPet {
    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            if self.should_exit {
                break;
            }
            self.update();
            self.draw();
        }
    }
    fn update(&mut self) {
        let curr_mos = self.rl.get_mouse_position();
        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
        {
            self.should_exit = true;
        }

        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            self.dragged = true;
            self.pan_offset = curr_mos;
        }

        if self.dragged {
            self.window_pos += curr_mos - self.pan_offset;
            self.rl
                .set_window_position(self.window_pos.x as i32, self.window_pos.y as i32);

            if self
                .rl
                .is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.dragged = false;
            }
        }
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        if self.show_hitbox {
            d.clear_background(Color::new(50, 0, 0, 128));
        } else {
            d.clear_background(Color::BLANK);
        }

        {
            self.animation.draw(&mut d, &self.thread, self.dragged);
        }

        d.draw_texture_ex(
            &self.animation.render_texture,
            Vector2::new(0., 0.),
            0.,
            1.,
            Color::WHITE,
        );
    }
}
