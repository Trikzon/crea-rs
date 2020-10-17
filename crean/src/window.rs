pub struct Window<'a> {
    _title: &'a str,
    width: i32,
    height: i32,

    sdl: sdl2::Sdl,
    sdl_window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    gl: gl::Gl
}

impl<'a> Window<'a> {
    pub fn new(title: &'a str, width: u32, height: u32) -> Self {
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
        let gl = gl::Gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        let width = width as i32;
        let height = height as i32;

        unsafe {
            gl.Viewport(0, 0, width, height);
            gl.ClearColor(1.0, 0.0, 0.0, 1.0);
        }

        Self {
            _title: title,
            width,
            height,
            sdl,
            sdl_window: window,
            _gl_context: gl_context,
            gl
        }
    }

    pub fn update(&self) {
        self.sdl_window.gl_swap_window();
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Returns true if program should quit.
    ///
    /// Processes window events like quitting, resizing, etc.
    /// Should be called each frame or game loop
    pub fn poll_events(&mut self) -> bool {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return true,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    self.width = w;
                    self.height = h;
                },
                _ => {}
            }
        }
        false
    }
}
