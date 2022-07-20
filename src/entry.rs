use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::processor::process;

pub fn program_entry(
    _: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    process(accounts, data)
}

entrypoint!(program_entry);

#[cfg(test)]
mod tests {
    use solana_program_test::{tokio, ProgramTest};

    #[tokio::test]
    async fn test_initialize_custom_token() {
        let program_test = ProgramTest::default();

        let (mut a, b, c) = program_test.start().await;
    }
}
