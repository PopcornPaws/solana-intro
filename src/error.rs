// program specific errors
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    /// Address is not rent exempt 
    #[error("Address is not rent exempt")]
    NotRentExempt,
    /// Expected amount doesn't match
    #[error("Expected amount was different than the given amount")]
    ExpectedAmountMismatch,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
