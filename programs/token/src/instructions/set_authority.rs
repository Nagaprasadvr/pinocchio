use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    pubkey::Pubkey,
    ProgramResult,
};

use crate::{FromOptPubkeyToOptBytes, IxData, UNINIT_BYTE};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AuthorityType {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

/// Sets a new authority of a mint or account.
///
/// ### Accounts:
///   0. `[WRITE]` The mint or account to change the authority of.
///   1. `[SIGNER]` The current authority of the mint or account.
pub struct SetAuthority<'a> {
    /// Account (Mint or Token)
    pub account: &'a AccountInfo,

    /// Authority of the Account.
    pub authority: &'a AccountInfo,

    /// The type of authority to update.
    pub authority_type: AuthorityType,

    /// The new authority
    pub new_authority: Option<&'a Pubkey>,
}

impl<'a> SetAuthority<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMeta; 2] = [
            AccountMeta::writable(self.account.key()),
            AccountMeta::readonly_signer(self.authority.key()),
        ];

        // instruction data
        // -  [0]: instruction discriminator
        // -  [1]: authority_type
        // -  [2..35] new_authority
        let mut ix_buffer = [UNINIT_BYTE; 35];
        let mut ix_data = IxData::new(&mut ix_buffer);

        // Set discriminator as u8 at offset [0]
        ix_data.write_bytes(&[6]);
        // Set authority_type as u8 at offset [1]
        ix_data.write_bytes(&[self.authority_type as u8]);
        // Set new_authority as [u8; 32] at offset [2..35]
        ix_data.write_optional_bytes(self.new_authority.to_opt_slice());

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: ix_data.read_bytes(),
        };

        invoke_signed(&instruction, &[self.account, self.authority], signers)
    }
}
