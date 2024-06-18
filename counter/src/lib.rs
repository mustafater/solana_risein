use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};
pub mod instructions;

use crate::instructions::CounterInstructions;



#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;
   

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment => {
            counter_account.counter += 1;
        }
        CounterInstructions::Decrement => {
            counter_account.counter -= 1;
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
        CounterInstructions::Initialize { counter } => process_initialize(program_id, accounts, counter),
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    counter: u32,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
 
    let new_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
 
    let account_data = CounterAccount { counter };
    let size = account_data.try_to_vec()?.len();
    let lamports = (Rent::get()?).minimum_balance(size);
 
    invoke(
        &create_account(
            signer.key,
            new_account.key,
            lamports,
            size as u32,
            program_id,
        ),
        &[signer.clone(), new_account.clone(), system_program.clone()],
    )?;
 
    account_data.serialize(&mut *new_account.data.borrow_mut())?;
    msg!("Changed data to: {:?}!", counter);
    Ok(())
}
