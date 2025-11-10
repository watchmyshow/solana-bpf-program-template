use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

/// Demo instruction for Solana developers.
/// Shows how to trigger a simple log message inside a BPF program.
pub fn handle_demo_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    msg!("Demo instruction executed successfully 🚀");
    Ok(())
}
