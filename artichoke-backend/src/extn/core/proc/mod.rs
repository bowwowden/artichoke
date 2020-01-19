use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Proc>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Proc", None, None)?;
    interp.0.borrow_mut().def_class::<Proc>(spec);
    let _ = interp.eval(&include_bytes!("proc.rb")[..])?;
    trace!("Patched Proc onto interpreter");
    Ok(())
}

pub struct Proc;
