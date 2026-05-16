use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let vault_state_key = self.vault_state.to_account_info().key();
        let amount = self.vault.lamports();

        let seeds = &[
            b"vault",
            vault_state_key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds: [&[&[u8]]; 1] = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            },
            &signer_seeds,
        );

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
