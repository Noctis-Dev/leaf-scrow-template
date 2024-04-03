#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, Decode, Encode, String, TypeInfo, Vec};

pub type MilestoneId = u8;

#[derive(TypeInfo, Encode, Decode)]
pub enum ScrowAction {
    Deposit {
        founder: ActorId
    },
    Aprove,
    Reject,
}

#[derive(TypeInfo, Encode, Decode)]
pub enum ScrowEvent {
    DeposidedCompleted {
        total_balance: u128,
    },
    AprovedCompleted,
    RejectedCompleted,
    Error {
        message: String
    }
}

#[derive(TypeInfo, Encode, Decode, Default)]
pub struct InitScrow {
    pub collector: ActorId,
    pub max_milestone_number: MilestoneId
}

#[derive(TypeInfo, Encode, Decode)]
pub struct ScrowState {
    pub collector: ActorId,
    pub founders: Vec<(ActorId, (u128, MilestoneId))>,
    pub max_milestone_number: MilestoneId,
    pub actual_milestone: MilestoneId
}

pub struct ScrowMetadata {}

impl Metadata for ScrowMetadata {
    type Init = In<InitScrow>;
    type Handle = InOut<ScrowAction, ScrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<ScrowState>;
}