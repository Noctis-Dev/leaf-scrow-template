#![no_std]
use contract::Escrow;
use gstd::{msg, Vec};
use io::{EscrowState, InitScrow};

pub mod contract;
pub mod handler;

pub static mut SCROW_STATE:Option<EscrowState> = None;
pub static mut SCROW_CONTRACT: Option<Escrow> = None;

pub fn contract_mut() -> &'static mut Escrow {
    unsafe {
        SCROW_CONTRACT.get_or_insert(Default::default())
    }
}

pub fn contract_state_mut() -> &'static mut EscrowState {
    unsafe {
        let state = SCROW_STATE.as_mut();
        state.unwrap_unchecked()
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_message: InitScrow = msg::load().expect("Can't decode the incoming message");

    unsafe {
        SCROW_STATE = Some( EscrowState {
            collector: init_message.collector,
            milestones: Vec::new(),            
            max_milestone_number: init_message.max_milestone_number,
        });
    };
}

#[no_mangle]
extern "C" fn state() {
    let state = contract_state_mut();
    let _ = msg::reply(state, 0);
}