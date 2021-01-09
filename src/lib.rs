#[macro_use] extern crate log;

mod util;
pub mod gl_types;
pub mod shader;
pub mod mesh;

pub struct Program {
    //SDL2
    sdl: sdl2::Sdl,
    sdl_video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,

    //GL
    gl_context: sdl2::video::GLContext,
}

pub struct WindowSettings<'a> {
    pub title: &'a str,
    pub resolution: (u32, u32),

    //Flags
    pub vsync: bool,
    pub gl_version: (u8, u8),
}

impl Program {
    pub fn new(win_settings: WindowSettings) -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL2!");
        let video_subsystem = sdl.video().expect("Failed to initialize SDL2 video system!");
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core); //TODO: Flag?
        gl_attr.set_context_version(win_settings.gl_version.0, win_settings.gl_version.1);

        let window = video_subsystem
            .window(win_settings.title, win_settings.resolution.0, win_settings.resolution.1)
            .resizable()
            .opengl()
            .build()
            .expect("Failed to initialize SDL2 window!");

        let gl_context = window.gl_create_context().expect("Failed to initialize OGL context!");

        let swap_interval = match win_settings.vsync {
            false => sdl2::video::SwapInterval::Immediate,
            true => sdl2::video::SwapInterval::VSync,
        };
        video_subsystem.gl_set_swap_interval(swap_interval).expect("Failed to set window swap interval!");

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        Self {
            sdl: sdl,
            sdl_video: video_subsystem,
            window: window,

            gl_context: gl_context,
        }
    }

    //Helper functions
    pub fn sdl(&self) -> &sdl2::Sdl {
        &self.sdl
    }

    pub fn sdl_video(&self) -> &sdl2::VideoSubsystem {
        &self.sdl_video
    }

    pub fn sdl_window(&self) -> &sdl2::video::Window {
        &self.window
    }

    pub fn sdl_mut(&mut self) -> &mut sdl2::Sdl {
        &mut self.sdl
    }

    pub fn sdl_video_mut(&mut self) -> &mut sdl2::VideoSubsystem {
        &mut self.sdl_video
    }

    pub fn sdl_window_mut(&mut self) -> &mut sdl2::video::Window {
        &mut self.window
    }
}
