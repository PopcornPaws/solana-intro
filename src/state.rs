// program objects, (de)serializing state

use arrayref::{array_mut_ref, mut_array_refs, array_ref, array_refs};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use solana_program::pubkey::Pubkey;

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_sender_pubkey: Pubkey,    // alice
    pub temp_token_account_pubkey: Pubkey,    // temp escrow account
    pub initializer_recipient_pubkey: Pubkey, // bob
    pub expected_amount: u64,
}

impl Sealed for Escrow {} // similar to rust's Sized trait

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 105;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_sender_pubkey,
            temp_token_account_pubkey,
            initializer_recipient_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_sender_pubkey: Pubkey::new_from_array(*initializer_sender_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_recipient_pubkey: Pubkey::new_from_array(*initializer_recipient_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_sender_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_recipient_pubkey_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        initializer_sender_pubkey_dst.copy_from_slice(self.initializer_sender_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(self.temp_token_account_pubkey.as_ref());
        initializer_recipient_pubkey_dst.copy_from_slice(self.initializer_recipient_pubkey.as_ref());
        *expected_amount_dst = self.expected_amount.to_le_bytes();
    }
}
