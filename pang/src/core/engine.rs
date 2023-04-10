use crate::*;

use std::env;

pub struct Engine {
    App: Box<dyn App>, 
    Args: Vec<String>,
    IsRunning: bool,
    Window: Box<dyn Window>
}

impl Engine {
    pub fn new(appInfo: AppInfo) -> Box<Engine> {
        return Box::new(Engine {
            App: appInfo.App,
            Args: env::args().collect(),
            IsRunning: true,
            Window: <dyn Window>::new(appInfo.Title, appInfo.Width, appInfo.Height)
        });
    }

    pub fn run(&mut self) {
        self.App.on_create();

        let title = String::from("Hello");

        self.Window.set_title(&title);

        while self.IsRunning {

        }
        
        self.App.on_shutdown();
    }

    pub fn shutdown(&self) {
        pang_core_info!("Engine is shutting down!");
    }
}