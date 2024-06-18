use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum CounterInstructions {
    Increment,
    Decrement,
    Update(UpdateArgs),
    Reset,
    Initialize { counter: u32 },
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment,
            1 => Self::Decrement,
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            4 => Self::Initialize{counter::try_from_slice(rest).unwrap()},
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}