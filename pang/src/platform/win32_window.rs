extern crate glfw;

use glfw::Context;

use crate::{Window, WindowConfig, pang_core_error};

pub struct Win32Window {
    WindowHandle: glfw::Window,
    GlfwContext: glfw::Glfw, // FIXME: Should not be part of this struct but rather static or global
    IsMinimized: bool,
    Config: WindowConfig
}

impl Win32Window {
    pub fn new(config: WindowConfig) -> Self {
        let mut glfwContext = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfwContext.window_hint(glfw::WindowHint::Visible(false));

        let(mut glfwWindow, events) = glfwContext.create_window(
            config.Width, config.Height, &config.Title, glfw::WindowMode::Windowed
        ).expect("Could not create a window");
        
        glfwWindow.set_key_polling(true);
        glfwWindow.make_current();
        
        let window = Win32Window {
            WindowHandle: glfwWindow,
            GlfwContext: glfwContext,
            Config: config,
            IsMinimized: false
        };

        return window;
    }
}

impl Window for Win32Window {
    fn set_title(&mut self, title: &String) {
        pang_core_error!("SetTitle");
    }

    fn process_events(&mut self) {
        self.GlfwContext.poll_events();
    }

    fn show(&mut self) {
        self.WindowHandle.show();
    }
}
