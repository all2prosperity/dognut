#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
pub mod department;
mod util;

pub mod proto {
    pub mod debugger {
        include!(concat!(env!("OUT_DIR"), "/debugger.rs"));
    }
}
