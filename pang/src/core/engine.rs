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

        // Show window after initialization
        self.Window.show();

        while self.IsRunning {
            self.Window.process_events();
        }
        
        self.App.on_shutdown();
    }

    pub fn shutdown(&mut self) {
        self.IsRunning = false;
        pang_core_info!("Engine is shutting down!");
    }
}