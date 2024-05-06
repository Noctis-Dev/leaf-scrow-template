#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, Decode, Encode, String, TypeInfo, Vec};

pub type MilestoneId = u8;
pub type Vara = u128;

#[derive(TypeInfo, Encode, Decode)]
pub enum EscrowAction {
    Deposit {
        founder: ActorId
    },
    Aprove,
    Reject,
}

#[derive(TypeInfo, Encode, Decode)]
pub enum EscrowEvent {
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
pub struct Founder {
    pub founder_id: ActorId,
    pub amount: Vara,
}

#[derive(TypeInfo, Encode, Decode)]
pub struct Milestone {
    pub milestone_id: MilestoneId,
    pub founders: Vec<Founder>,
    pub is_active: bool,
}

#[derive(TypeInfo, Encode, Decode)]
pub struct EscrowState {
    pub collector: ActorId,
    pub milestones: Vec<Milestone>,
    pub max_milestone_number: MilestoneId,
}

pub struct ScrowMetadata {}

impl Metadata for ScrowMetadata {
    type Init = In<InitScrow>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<EscrowState>;
}