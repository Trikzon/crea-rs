pub fn app_init<'a> (engine: &'a engine::Engine<'a>) -> crate::App<'a> {
    crate::App::init(engine)
}

pub fn app_update(mut app: crate::App) {
    app.update();
}