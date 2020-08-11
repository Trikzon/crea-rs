mod bindings {
    // include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    include!("./bindings.rs");
}

pub use bindings::*;

use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct GL {
    inner: Rc<bindings::Gl>,
}

impl GL {
    pub fn load_with<F>(loadfn: F) -> GL
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        GL {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

impl Deref for GL {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl { &self.inner }
}