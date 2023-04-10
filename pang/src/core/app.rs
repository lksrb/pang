pub struct AppInfo {
    pub Title: &'static str, // Static string
    pub Width: u32,
    pub Height: u32,
    pub App: Box<dyn App> 
}

pub trait App {
    fn on_create(&self);
    fn on_shutdown(&self);
}

