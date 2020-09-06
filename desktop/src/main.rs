extern crate app;
extern crate engine;

fn main() {
    let platform = engine::platform::Platform::init(test_callback);
    let engine = engine::Engine::init(platform);
    let mut app = app::App::init(engine);

    loop {
        app.update();
    }
}

fn test_callback(foo: f32) {
    println!("{}", foo);
}
