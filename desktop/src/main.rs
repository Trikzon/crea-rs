extern crate app;
extern crate engine;

fn main() {
    let engine = engine::platform::engine_init(test_callback);
    let mut app = app::platform::app_init(&engine);

    loop {
        app.update();
    }
}

fn test_callback(foo: f32) {
    println!("{}", foo);
}
