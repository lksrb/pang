mod sandbox;
use sandbox::SandboxApplication;

fn main() {
    let app_info =  pang::AppInfo { 
        Title: "Sandbox",
        Width: 1280,
        Height: 720,
        App: SandboxApplication::new()
    };

    let mut engine = pang::Engine::new(app_info);
    engine.run();
    engine.shutdown();
}