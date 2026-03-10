use gbx_grobner::{TraceCfg, Tracer};
use std::cell::RefCell;
use std::rc::Rc;

pub fn make_tracer() -> Rc<RefCell<Tracer>> {
    let cfg = TraceCfg { progress_every: 2000, on_new_poly: false, phases: true, breakdown: true };

    Rc::new(RefCell::new(Tracer::new(cfg)))
}
