use gstd::{exec, msg, ActorId, String};
use io::{EscrowEvent, Founder};

use crate::contract_state_mut;

#[derive(Default)]
pub struct Escrow { }

impl Escrow {

    pub fn deposit(&self, fouder: ActorId) {
        let state = contract_state_mut();
        let amount = msg::value();

        if let Some(milestone) = state.milestones.iter_mut().find(|milestone| milestone.is_active) {
            milestone.founders.push(Founder {
                founder_id: fouder.clone(),
                amount,
            });

            let _ = msg::reply(EscrowEvent::DeposidedCompleted { total_balance: exec::value_available() }, 0);
        }

        let _ = msg::reply(EscrowEvent::Error { message: String::from("Deposit error") }, 0);
    }

    pub fn aprove(&self) {
        let state = contract_state_mut();

        if let Some(milestone) = state.milestones.iter_mut().find(|milestone| milestone.is_active) {
            milestone.is_active = false;
            
            let _ = msg::send(state.collector, EscrowEvent::AprovedCompleted, exec::value_available());
            let _ = msg::reply(EscrowEvent::AprovedCompleted, exec::value_available());
        }

        let _ = msg::reply(EscrowEvent::Error { message: String::from("Approve error") }, exec::value_available());
    }

    pub fn reject(&self) {
        let state = contract_state_mut();

        if let Some(milestone) = state.milestones.iter_mut().find(|milestone| milestone.is_active) {
            milestone.is_active = false;
            
            for fouder in milestone.founders.iter() {
                let _ = msg::send(fouder.founder_id, String::from("REFOUNDED VALUE"), fouder.amount);
            }

            let _ = msg::reply(EscrowEvent::RejectedCompleted, exec::value_available());
        }

        let _ = msg::reply(EscrowEvent::RejectedCompleted, 0);
    }
}