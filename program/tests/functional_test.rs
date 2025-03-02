use solana_program::instruction::{AccountMeta, Instruction};
use {
    record::state::RecordData,
    solana_program_test::*,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    },
};

use record::instruction::RecordInstruction;
use solana_program::rent::Rent;

async fn initialize_storage_account(
    context: &mut ProgramTestContext,
    authority: &Keypair,
    account: &Keypair,
    data: &[u8],
) {
    let account_length = std::mem::size_of::<RecordData>()
        .checked_add(data.len())
        .unwrap();
    let custom_program_id = Pubkey::new_from_array(record::ID);
    let ix_data = RecordInstruction::Initialize.pack();
    let ix = Instruction {
        program_id: custom_program_id,
        accounts: vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), false),
        ],
        data:ix_data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &account.pubkey(),
                1.max(Rent::default().minimum_balance(account_length)),
                account_length as u64,
                &custom_program_id,
            ),
            // autority = fee payer
            ix,
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, account],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Write IX
    let ix_data = RecordInstruction::Write { offset: 0, data }.pack();
    let ix = Instruction {
        program_id: custom_program_id,
        accounts: vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
        ],
        data: ix_data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, authority],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

#[tokio::test]
async fn initialize_and_write_success() {
    let custom_program_id = Pubkey::new_from_array(record::ID);
    let mut context = ProgramTest::new("record", custom_program_id, None)
        .start_with_context()
        .await;

    let authority = Keypair::new();
    let account = Keypair::new();
    let data = &[1u8; 32];
    initialize_storage_account(
        &mut context,
        &authority,
        &account,
        data,
    )
    .await;

    let record_account = context
        .banks_client
        .get_account(account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let record_value = &record_account.data
            [RecordData::WRITABLE_START_INDEX..(RecordData::WRITABLE_START_INDEX + data.len())];
    assert!(record_value.starts_with(data.as_slice()));
}

#[tokio::test]
async fn set_authority_success() {
    let custom_program_id = Pubkey::new_from_array(record::ID);
    let program_test = ProgramTest::new("record", custom_program_id, None);

    let mut context: ProgramTestContext = program_test.start_with_context().await;

    let authority = Keypair::new();
    let account = Keypair::new();
    let new_authority = Keypair::new();

    let data = &[0u8; 8];
    initialize_storage_account(
        &mut context,
        &authority,
        &account,
        data,
    )
    .await;

    let mut record_account = context
        .banks_client
        .get_account(account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let old_account_data = bytemuck::try_from_bytes_mut::<RecordData>(
        &mut record_account.data[..RecordData::WRITABLE_START_INDEX],
    )
    .unwrap();

    let data = RecordInstruction::SetAuthority.pack();
    let ix = Instruction {
        program_id: custom_program_id,
        accounts: vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new_readonly(new_authority.pubkey(), false),
        ],
        data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        //&[&context.payer],
        &[&context.payer, &authority],
        context.last_blockhash,
    );
    assert!(context
        .banks_client
        .process_transaction(transaction)
        .await
        .is_ok());

    let mut record_account = context
        .banks_client
        .get_account(account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let new_account_data = bytemuck::try_from_bytes_mut::<RecordData>(
        &mut record_account.data[..RecordData::WRITABLE_START_INDEX],
    )
    .unwrap();
    assert_eq!(old_account_data.authority, authority.pubkey().to_bytes());
    assert_eq!(
        new_account_data.authority,
        new_authority.pubkey().to_bytes()
    );
}

#[tokio::test]
async fn close_account_success() {
    let custom_program_id = Pubkey::new_from_array(record::ID);
    let mut context = ProgramTest::new("record", custom_program_id, None)
        .start_with_context()
        .await;

    let authority = Keypair::new();
    let account = Keypair::new();

    let data = &[0u8; 16];

    initialize_storage_account(
        &mut context,
        &authority,
        &account,
        data,
    )
    .await;

    let record_account = context
        .banks_client
        .get_account(account.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert!(record_account.lamports > 0);
    assert!(!record_account.data.is_empty());
    let data = RecordInstruction::CloseAccount.pack();
    let ix = Instruction {
        program_id: custom_program_id,
        accounts: vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new(authority.pubkey(), false),
        ],
        data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        //&[&context.payer],
        &[&context.payer, &authority],
        context.last_blockhash,
    );
    assert!(context
        .banks_client
        .process_transaction(transaction)
        .await
        .is_ok());

    let record_account = context
        .banks_client
        .get_account(account.pubkey())
        .await
        .unwrap();
    assert!(record_account.is_none());
}
