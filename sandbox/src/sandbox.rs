#![allow(non_snake_case, dead_code, unused_variables)]

use pang::*;

pub struct SandboxApplication {
    testValue: i32
}

impl SandboxApplication {
    pub fn new() -> Box<SandboxApplication> {
        Box::new(SandboxApplication {
            testValue: 54
        })
    }
}

impl pang::App for SandboxApplication 
{
    fn on_create(&self) {
        pang_info!("on_create called!");
    }

    fn on_shutdown(&self) {
        pang_info!("on_shutdown called!");
    }
} 

