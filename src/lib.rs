use std::borrow::BorrowMut;
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use wasm_bindgen::prelude::*;
use crate::env::Env;
use crate::parser::parse;

mod parser;
mod interpreter;
mod object;
mod error;
mod env;


fn env() -> &'static Mutex<Env> {
    // Create an uninitialized static
    static mut ENV: MaybeUninit<Mutex<Env>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let debugger = Env::base();
            ENV.write(Mutex::new(debugger));
        });
        ENV.assume_init_ref()
    }
}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> Result<String, String> {
    let env = env();
    let parsed = parse(input).map_err(|e| e.message)?;
    let result = interpreter::eval(parsed, env.lock().unwrap().borrow_mut()).map_err(|e| e.message)?;
    Ok(format!("{}", result))
}
