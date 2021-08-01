// Entrypoint to the program
use solana_program::account_info::AccountInfo;
use solana_program::{entrypoint, entrypoint::ProgramResult};
use solana_program::msg;
use solana_program::pubkey::Pubkey;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}
