use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, mint_to, MintTo};

declare_id!("DoMa111111111111111111111111111111111111111");

#[program]
pub mod doma_protocol {
    use super::*;

    pub fn initialize_domain(ctx: Context<InitializeDomain>, name: String, metadata_uri: String) -> Result<()> {
        let domain = &mut ctx.accounts.domain;
        domain.owner = *ctx.accounts.owner.key;
        domain.name = name;
        domain.metadata_uri = metadata_uri;
        domain.bump = *ctx.bumps.get("domain").unwrap();
        Ok(())
    }

    pub fn mint_domain_spl(ctx: Context<MintDomainSPL>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn create_lending_position(ctx: Context<CreateLending>, collateral_amount: u64, loan_amount: u64) -> Result<()> {
        let pos = &mut ctx.accounts.position;
        pos.borrower = *ctx.accounts.borrower.key;
        pos.collateral_amount = collateral_amount;
        pos.loan_amount = loan_amount;
        pos.active = true;
        Ok(())
    }

    pub fn close_lending_position(ctx: Context<CloseLending>) -> Result<()> {
        let pos = &mut ctx.accounts.position;
        require!(pos.active, ErrorCode::PositionInactive);
        pos.active = false;
        Ok(())
    }
}

#[account]
pub struct Domain {
    pub owner: Pubkey,
    pub name: String,
    pub metadata_uri: String,
    pub bump: u8,
}

#[account]
pub struct LendingPosition {
    pub borrower: Pubkey,
    pub collateral_amount: u64,
    pub loan_amount: u64,
    pub active: bool,
}

#[derive(Accounts)]
#[instruction(name: String, metadata_uri: String)]
pub struct InitializeDomain<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + name.len() + 4 + metadata_uri.len() + 1)]
    pub domain: Account<'info, Domain>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintDomainSPL<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: simplified for demo
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateLending<'info> {
    #[account(init, payer = borrower, space = 8 + 32 + 8 + 8 + 1)]
    pub position: Account<'info, LendingPosition>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseLending<'info> {
    #[account(mut, has_one = borrower)]
    pub position: Account<'info, LendingPosition>,
    pub borrower: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Lending position is inactive")]
    PositionInactive,
}
