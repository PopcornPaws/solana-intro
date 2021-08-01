// Program API (de)serializing instruction data

use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and
    /// transferring ownership of the given temp token account to the PDA
    ///
    /// Accounts expected:
    /// 0. `[signer]` the account of the person initializing the escrow
    /// 1. `[writable]` temporary token account that should be created prior to
    ///    this instruction and owned by the initializer
    /// 2. `[]` the initializer's token account for the token they will receive
    ///    should the trade goes through
    /// 3. `[writable]` the escrow account, which holds all necessary info
    ///    about the trade
    /// 4. `[]` the rent sysvar
    /// 5. `[]` the token program
    InitEscrow {
        /// the amount party A expects to receive of token Y
        amount: u64,
    }
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        match tag {
            0 => Ok(Self::InitEscrow {
                amount: unpack_amount(rest)?,
            }),
            _ => Err(InvalidInstruction.into()),
        }
    }
}

fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;

    Ok(amount)
}
