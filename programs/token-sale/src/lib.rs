use anchor_lang::{prelude::*, solana_program::system_program};
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_sale {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>, _mint_bump: u8, _mint_authority_bump: u8) -> ProgramResult {
        msg!("We just initialized a mint!");
        Ok(())
    }

    pub fn mint_some_tokens(ctx: Context<MintSomeTokens>, _mint_bump: u8, mint_authority_bump: u8) -> ProgramResult {
        msg!("Total supply = {}", ctx.accounts.mint.supply);
        let ts = ctx.accounts.mint.supply;
        let  tokenAmount;
        if ts >= 12 {
            return Err(ProgramError::Custom(1));
        }

        if ts >=0 && ts < 5 {
            tokenAmount = 0500000000;
        } else if ts >= 5 && ts <= 10 {
            tokenAmount = 1000000000;
        }
        else {
            tokenAmount = 1500000000;
        }
       
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.wallet.key,
            ctx.accounts.mint_authority.key,
            tokenAmount
        );
        anchor_lang::solana_program::program::invoke(&transfer_ix, &[
            ctx.accounts.wallet.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
        ])?;    
     
        // if ts > 5 && ts <= 10 {
        //     let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        //         ctx.accounts.wallet.key,
        //         ctx.accounts.mint_authority.key,
        //         1000000000
        //     );
        //     anchor_lang::solana_program::program::invoke(&transfer_ix, &[
        //         ctx.accounts.wallet.to_account_info(),
        //         ctx.accounts.mint_authority.to_account_info(),
        //     ])?;    
        // }


        anchor_spl::token::mint_to(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_destination.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info()
            },
            &[&[b"mint-authority".as_ref(), &[mint_authority_bump]]]
        ), 1)?;

        ctx.accounts.mint.reload()?;
        msg!("Total supply = {}", ctx.accounts.mint.supply);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, mint_authority_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"mint".as_ref()], bump = mint_bump, payer = wallet, mint::decimals = 0, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(seeds = [b"mint-authority".as_ref()], bump = mint_authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, mint_authority_bump: u8)]
pub struct MintSomeTokens<'info> {
    #[account(mut, seeds = [b"mint".as_ref()], bump = mint_bump)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(mut)]
    pub token_destination: AccountInfo<'info>, //Account<'info, TokenAccount>,

    #[account(mut, seeds = [b"mint-authority".as_ref()], bump = mint_authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}
