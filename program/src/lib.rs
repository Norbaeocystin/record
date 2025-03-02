// #![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

// use solana_account_info::AccountInfo;
// use solana_program::{declare_id, entrypoint};
// use solana_program_error::ProgramResult;
// use solana_pubkey::Pubkey;

use pinocchio::{account_info::AccountInfo, program_entrypoint, pubkey::Pubkey, ProgramResult};

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

pub use pinocchio;

//solana_sdk::declare_id!("11111111111111111111111111111111");

pub const ID: Pubkey = [
   178,26,162,175,50,174,64,192,126,95,126,95,182,78,116,38,69,35,33,211,65,82,99,87,99,29,42,235,108,43,50,98
];

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process_instruction(program_id, accounts, instruction_data)
}

//entrypoint!(process_instruction);
program_entrypoint!(process_instruction);
