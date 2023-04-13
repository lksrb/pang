#[cfg(target_os = "windows")] #[path ="../platform/win32_window.rs"] mod win32_window;
#[cfg(target_os = "windows")] use win32_window::Win32Window;

#[cfg(target_os = "linux")] #[path ="../platform/linux_window.rs"] mod linux_window;
#[cfg(target_os = "linux")] use linux_window::LinuxWindow;

pub struct WindowConfig {
    pub Title: String,
    pub Width: u32,
    pub Height: u32,
    pub VSync: bool
}

pub trait Window {
    fn set_title(&mut self, title: &String);
    fn process_events(&mut self);
    fn show(&mut self);
}

impl dyn Window {
    pub fn new(title: &'static str, width: u32, height: u32) -> Box<dyn Window> {
        let windowConfig = WindowConfig { 
            Title: String::from(title),
            Width: width,
            Height: height,
            VSync: true
        };

        return Self::new_platform(windowConfig);
    }

    fn new_platform(windowConfig: WindowConfig) -> Box<dyn Window> {
        #[cfg(target_os = "windows")]
        return Box::new(Win32Window::new(windowConfig));
        #[cfg(target_os = "linux")] 
        return Box::new(LinuxWindow::new(windowConfig));
    }
}
