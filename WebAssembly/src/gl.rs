use std::ops::Deref;
use std::rc::Rc;
use web_sys::WebGlProgram;

pub struct Gl {
    raw: Rc<WebGlProgram>
}

impl Gl {
    pub fn new(raw: WebGlProgram) -> Gl {
        Gl {
            raw: Rc::new(raw)
        }
    }
}

impl Clone for Gl {
    fn clone(&self) -> Gl {
        Gl {
            raw: self.raw.clone()
        }
    }
}

impl Deref for Gl {
    type Target = WebGlProgram;

    fn deref(&self) -> &WebGlProgram {
        &self.raw.deref()
    }
}
