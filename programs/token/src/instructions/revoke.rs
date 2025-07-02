use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    pubkey::Pubkey,
    ProgramResult,
};

/// Revokes the delegate's authority.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[SIGNER]` The source account owner.
pub struct Revoke<'a> {
    /// Source Account.
    pub source: &'a AccountInfo,
    ///  Source Owner Account.
    pub authority: &'a AccountInfo,
}

impl Revoke<'_> {
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
        let account_metas: [AccountMeta; 2] = [
            AccountMeta::writable(self.source.key()),
            AccountMeta::readonly_signer(self.authority.key()),
        ];

        let instruction = Instruction {
            program_id,
            accounts: &account_metas,
            data: &[5],
        };

        invoke_signed(&instruction, &[self.source, self.authority], signers)
    }
}
