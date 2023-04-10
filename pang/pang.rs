#![allow(non_snake_case, dead_code, unused_variables)]

#[path = "src/core/log.rs"] pub mod log;
#[path = "src/core/window.rs"] pub mod window;
#[path = "src/core/engine.rs"] pub mod engine;
#[path = "src/core/app.rs"] pub mod app;
use window::{Window, WindowConfig};

pub use engine::{ Engine };
pub use app::{AppInfo, App};