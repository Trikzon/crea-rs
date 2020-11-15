mod bindings {
    #[cfg(not(feature = "debug"))]
    include!("./bindings.rs");
    #[cfg(feature = "debug")]
    include!("./debug_bindings.rs");
}

pub use bindings::*;

use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>
}

impl Gl {
    pub fn load_with<F>(load_fn: F) -> Self where
        F: FnMut(&'static str) -> *const types::GLvoid {

        Self {
            inner: Rc::new(bindings::Gl::load_with(load_fn))
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}
