use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    pubkey::Pubkey,
    ProgramResult,
};

/// Given a native token account updates its amount field based
/// on the account's underlying `lamports`.
///
/// ### Accounts:
///   0. `[WRITE]`  The native token account to sync with its underlying
///      lamports.
pub struct SyncNative<'a> {
    /// Native Token Account
    pub native_token: &'a AccountInfo,
}

impl SyncNative<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    #[inline(always)]
    pub fn invoke_with_program(&self, program_id: &Pubkey) -> ProgramResult {
        self.invoke_signed_with_program(&[], program_id)
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        self.invoke_signed_with_program(signers, &crate::ID)
    }

    pub fn invoke_signed_with_program(
        &self,
        signers: &[Signer],
        program_id: &Pubkey,
    ) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMeta; 1] = [AccountMeta::writable(self.native_token.key())];

        let instruction = Instruction {
            program_id,
            accounts: &account_metas,
            data: &[17],
        };

        invoke_signed(&instruction, &[self.native_token], signers)
    }
}
