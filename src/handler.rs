use gstd::msg;
use io::ScrowAction;

use crate::contract_mut;

#[no_mangle]
extern "C" fn handle() {
    let message: ScrowAction = msg::load().expect("Unable to load message");
    let contract = contract_mut();

    match message {
        ScrowAction::Deposit { founder } => {
            contract.deposit(founder);
        },
        ScrowAction::Aprove => {
            contract.aprove();
        },
        ScrowAction::Reject => {
            contract.reject();
        },
    }
}