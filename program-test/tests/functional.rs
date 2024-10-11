use {
    num_enum::IntoPrimitive,
    solana_program::{
        instruction::{AccountMeta, Instruction, InstructionError},
        pubkey::Pubkey,
        system_instruction,
    },
    solana_program_test::{tokio, ProgramTest, ProgramTestContext},
    solana_sdk::{
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    test_case::test_case,
};

mod helloworld {
    solana_program::declare_id!("Zigc1Hc97L8Pebma74jDzYiyoUvdxxcj7Gxppg9VRxK");
}

fn program_test() -> ProgramTest {
    ProgramTest::new("helloworld", helloworld::id(), None)
}

#[repr(u8)]
#[derive(Clone, Copy, IntoPrimitive)]
enum AccountType {
    Uninitialized,
    SmallInt,
    BigInt,
}

async fn create_and_init(
    context: &mut ProgramTestContext,
    account_type: AccountType,
) -> Result<Pubkey, TransactionError> {
    let rent = context.banks_client.get_rent().await.unwrap();
    let size = match account_type {
        AccountType::SmallInt => 4,
        AccountType::BigInt => 32,
        AccountType::Uninitialized => 1,
    };
    let space = size + 1;
    let lamports = rent.minimum_balance(space);
    let new_account = Keypair::new();
    let blockhash = context.banks_client.get_latest_blockhash().await.unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &new_account.pubkey(),
                lamports,
                space as u64,
                &helloworld::id(),
            ),
            Instruction {
                program_id: helloworld::id(),
                accounts: vec![AccountMeta::new(new_account.pubkey(), false)],
                data: vec![0, u8::from(account_type)],
            },
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, &new_account],
        blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .map(|_| new_account.pubkey())
        .map_err(|e| e.unwrap())
}

#[tokio::test]
async fn fail_init_uninitialized() {
    let pt = program_test();
    let mut context = pt.start_with_context().await;
    let err = create_and_init(&mut context, AccountType::Uninitialized)
        .await
        .unwrap_err();
    assert_eq!(
        err,
        TransactionError::InstructionError(1, InstructionError::Custom(3))
    );
}

#[test_case(AccountType::SmallInt; "small")]
#[test_case(AccountType::BigInt; "big")]
#[tokio::test]
async fn init(account_type: AccountType) {
    let pt = program_test();
    let mut context = pt.start_with_context().await;
    let account_pubkey = create_and_init(&mut context, account_type).await.unwrap();

    let account = context
        .banks_client
        .get_account(account_pubkey)
        .await
        .unwrap()
        .unwrap();
    let mut expected_data = vec![u8::from(account_type)];
    match account_type {
        AccountType::SmallInt => expected_data.extend_from_slice(&[0; 4]),
        AccountType::BigInt => expected_data.extend_from_slice(&[0; 32]),
        AccountType::Uninitialized => unreachable!(),
    }
    assert_eq!(account.data, expected_data);
}

#[test_case(AccountType::SmallInt; "small")]
#[test_case(AccountType::BigInt; "big")]
#[tokio::test]
async fn increment(account_type: AccountType) {
    let pt = program_test();
    let mut context = pt.start_with_context().await;
    let account_pubkey = create_and_init(&mut context, account_type).await.unwrap();

    let blockhash = context.banks_client.get_latest_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: helloworld::id(),
            accounts: vec![AccountMeta::new(account_pubkey, false)],
            data: vec![1],
        }],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let account = context
        .banks_client
        .get_account(account_pubkey)
        .await
        .unwrap()
        .unwrap();
    let mut expected_data = vec![u8::from(account_type), 1];
    match account_type {
        AccountType::SmallInt => expected_data.extend_from_slice(&[0; 3]),
        AccountType::BigInt => expected_data.extend_from_slice(&[0; 31]),
        AccountType::Uninitialized => unreachable!(),
    }
    assert_eq!(account.data, expected_data);
}

#[test_case(AccountType::SmallInt; "small")]
#[test_case(AccountType::BigInt; "big")]
#[tokio::test]
async fn add(account_type: AccountType) {
    let pt = program_test();
    let mut context = pt.start_with_context().await;
    let account_pubkey = create_and_init(&mut context, account_type).await.unwrap();

    let blockhash = context.banks_client.get_latest_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: helloworld::id(),
            accounts: vec![AccountMeta::new(account_pubkey, false)],
            data: vec![2, 255, 255, 255, 255],
        }],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let account = context
        .banks_client
        .get_account(account_pubkey)
        .await
        .unwrap()
        .unwrap();
    let mut expected_data = vec![u8::from(account_type), 255, 255, 255, 255];
    match account_type {
        AccountType::SmallInt => expected_data.extend_from_slice(&[0; 0]),
        AccountType::BigInt => expected_data.extend_from_slice(&[0; 28]),
        AccountType::Uninitialized => unreachable!(),
    }
    assert_eq!(account.data, expected_data);
}
