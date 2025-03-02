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
//default_panic_handler!();
//no_allocator!();

// // Declare and export the program's entrypoint
// entrypoint!(process_instruction);

// // Program entrypoint's implementation
// pub fn process_instruction(
//     _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
//     accounts: &[AccountInfo], // The account to say hello to
//     instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
// ) -> ProgramResult {
//     let mut data = instruction_data.to_vec();
//     // last byte is less or not less
//     let less = data.pop().unwrap();
//     let max_tokens = u64::from_le_bytes(data.try_into().unwrap());
//     let accounts_iter = &mut accounts.iter();
//     let token_account = accounts_iter
//         .next()
//         .ok_or(ProgramError::NotEnoughAccountKeys)?;
//     let amount = u64::from_le_bytes(token_account.data.borrow().to_vec()[64..72].to_vec().try_into().unwrap());
//     if less == 0 {
//         // if amount in TA is more that as max tokens from ix - throw error
//         if amount > max_tokens {
//             return Err(ProgramError::ArithmeticOverflow);
//         }
//     }
//     if less == 1 {
//         // if amount in TA is less than as max tokens from ix - throw error
//         if amount < max_tokens {
//             return Err(ProgramError::ArithmeticOverflow);
//         }
//     }
//     Ok(())
// }
