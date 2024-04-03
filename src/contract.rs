use gstd::{exec, msg, ActorId};
use io::ScrowEvent;

use crate::contract_state_mut;

#[derive(Default)]
pub struct Scrow { }

impl Scrow {

    pub fn deposit(&self, fouder: ActorId) {
        let state = contract_state_mut();
        let amount = msg::value();

        state.founders.push((fouder, (amount, state.actual_milestone)));
        let _ = msg::reply(ScrowEvent::DeposidedCompleted { total_balance: exec::value_available() }, 0);
    }

    pub fn aprove(&self) {
        let state = contract_state_mut();

        if state.actual_milestone == state.max_milestone_number {
            panic!("this contract has been ended its lifecycle");
        }

        state.actual_milestone += 1;
        
        let _ = msg::reply(ScrowEvent::AprovedCompleted, exec::value_available());
    }

    pub fn reject(&self) {
        let state = contract_state_mut();

        for founder in state.founders.iter() {
            if founder.1.1 == state.actual_milestone {
                let _ = msg::send(founder.0, "rejected value", founder.1.0);
            }
        }

        let _ = msg::reply(ScrowEvent::RejectedCompleted, 0);
    }
}