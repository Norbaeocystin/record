//! Program state
use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;

/// Header type for recorded account data
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct RecordData {
    /// Struct version, allows for upgrades to the program
    pub version: u8,

    /// The account allowed to update the data
    pub authority: Pubkey,
}

impl RecordData {
    /// Version to fill in on new created accounts
    pub const CURRENT_VERSION: u8 = 1;

    /// Start of writable account data, after version and authority
    pub const WRITABLE_START_INDEX: usize = 33;
}

impl RecordData {
    /// Is initialized
    pub fn is_initialized(&self) -> bool {
        self.version == Self::CURRENT_VERSION
    }
}
