#[cfg(target_os = "windows")]
#[path ="../platform/win32_window.rs"] mod win32_window;
use win32_window::Win32Window;

pub struct WindowConfig {
    pub Title: String,
    pub Width: u32,
    pub Height: u32,
    pub VSync: bool
}

pub trait Window {
    fn set_title(&mut self, title: &String);
}

impl dyn Window {
    #[cfg(target_os = "windows")]
    pub fn new(title: &'static str, width: u32, height: u32) -> Box<dyn Window> {
        let windowConfig = WindowConfig { 
            Title: String::from(title),
            Width: width,
            Height: height,
            VSync: true
        };

        return Box::new(Win32Window::new(windowConfig));
    }
}
