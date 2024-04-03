#![no_std]
use contract::Scrow;
use gstd::{msg, Vec};
use io::{InitScrow, ScrowState};

pub mod contract;
pub mod handler;

pub static mut SCROW_STATE:Option<ScrowState> = None;
pub static mut SCROW_CONTRACT: Option<Scrow> = None;

pub fn contract_mut() -> &'static mut Scrow {
    unsafe {
        SCROW_CONTRACT.get_or_insert(Default::default())
    }
}

pub fn contract_state_mut() -> &'static mut ScrowState {
    unsafe {
        let state = SCROW_STATE.as_mut();
        state.unwrap_unchecked()
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_message: InitScrow = msg::load().expect("Can't decode the incoming message");

    unsafe {
        SCROW_STATE = Some( ScrowState {
            collector: init_message.collector,
            founders: Vec::new(),
            max_milestone_number: init_message.max_milestone_number,
            actual_milestone: 0,
        });
    };
}

#[no_mangle]
extern "C" fn state() {
    let state = contract_state_mut();
    let _ = msg::reply(state, 0);
}