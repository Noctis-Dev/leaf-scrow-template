use gstd::msg;
use io::EscrowAction;

use crate::contract_mut;

#[no_mangle]
extern "C" fn handle() {
    let message: EscrowAction = msg::load().expect("Unable to load message");
    let contract = contract_mut();

    match message {
        EscrowAction::Deposit { founder } => {
            contract.deposit(founder);
        },
        EscrowAction::Aprove => {
            contract.aprove();
        },
        EscrowAction::Reject => {
            contract.reject();
        },
    }
}