use crate::{Window, WindowConfig, pang_core_error};

pub struct Win32Window {
    Config: WindowConfig
}

impl Win32Window {
    pub fn new(config: WindowConfig) -> Self {
        Self { Config: config }
    }
}

impl Window for Win32Window {
    fn set_title(&mut self, title: &String) {
        pang_core_error!("SetTitle");
    }
}
