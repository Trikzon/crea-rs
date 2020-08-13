pub struct Display {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
    viewport: Viewport,
    color_buffer: ColorBuffer,
    gl_context: sdl2::video::GLContext,
    gl: gl::GL
}

impl Display {
    pub fn create(title: &str, width: u32, height: u32) -> Display {

        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem
            .window(title, width, height)
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::GL::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        let viewport = Viewport::from_dims(width, height);
        let color_buffer = ColorBuffer::from_rgb(0.3, 0.3, 0.5);
        viewport.bind(&gl);
        color_buffer.bind(&gl);

        Display { sdl, video_subsystem, sdl_window: window, viewport, color_buffer, gl_context, gl }
    }

    pub fn update(&self) {
        self.sdl_window.gl_swap_window();
        self.color_buffer.clear(&self.gl);
    }

    pub fn poll_events(&mut self) -> bool {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { ..} => return true,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    self.viewport.update(w, h);
                    self.viewport.bind(&self.gl);
                },
                _ => {}
            }
        }
        false
    }

    pub fn gl(&self) -> &gl::Gl {
        &self.gl
    }
}

struct Viewport {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Viewport {
    fn from_dims(w: u32, h: u32) -> Viewport {
        Viewport { x: 0, y: 0, w: w as i32, h: h as i32 }
    }

    fn update(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
    }

    fn bind(&self, gl: &gl::GL) {
        unsafe {
            gl.Viewport(self.x, self.y, self.w, self.h);
        }
    }
}

struct ColorBuffer {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl ColorBuffer {
    fn from_rgb(r: f32, g: f32, b: f32) -> ColorBuffer {
        ColorBuffer { r, g, b, a: 1.0 }
    }

    fn update_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.ClearColor(self.r, self.g, self.b, self.a);
        }
    }

    fn clear(&self, gl: &gl::GL) {
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}