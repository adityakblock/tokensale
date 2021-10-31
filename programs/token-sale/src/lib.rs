use anchor_lang::{prelude::*, solana_program::system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_lang::solana_program;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{Accounts, CpiContext};


declare_id!("FbPURn5SWk6PiBWgdFApT5oNkgZUyN6aeiEtPw43r8Jn");

#[program]
pub mod token_sale {

    use super::*;
    #[state]
    pub struct MyProgram {
        beneficiary: Pubkey,
        supply: u64,
        price: u64,
        lost_authority: bool,
    }

    impl MyProgram {
        pub fn new(ctx: Context<Initialize>, _mint_bump: u8, mint_authority_bump: u8) -> Result<Self, ProgramError> {
            msg!("We just initialized a mint!");

            let initialSupply:u64= 500;

            anchor_spl::token::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::MintTo {
                        mint: ctx.accounts.mint.to_account_info(),
                        to: ctx.accounts.destination.to_account_info(),
                        authority: ctx.accounts.mint_authority.to_account_info(),
                    },
                    &[&[b"mint-authority".as_ref(), &[mint_authority_bump]]],
                ),
                initialSupply,
            )?;
           
            anchor_spl::token::set_authority(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::SetAuthority {
                    current_authority: ctx.accounts.mint_authority.to_account_info(),
                    account_or_mint: ctx.accounts.mint.to_account_info(),
                },
                &[&[b"mint-authority".as_ref(), &[mint_authority_bump]]],
                ), 
               // spl_token::instruction::Authority::MintTokens,
               
               spl_token::instruction::AuthorityType::MintTokens,
               Some(*ctx.accounts.wallet.key),
            )?;

    
            
        
            Ok(Self {
                beneficiary: *ctx.accounts.wallet.key,
                supply: initialSupply,
                price: calc_price(initialSupply),
                lost_authority: true
            })
        }



        pub fn change_authority(&mut self,
            ctx: Context<ChangeAuthority>,) -> Result<(), ProgramError> {

                if ctx.accounts.wallet.key != &self.beneficiary {
                    return Err(ProgramError::Custom(2));
                }

                if !self.lost_authority {
                    return Err(ProgramError::Custom(2));
                }

                // anchor_spl::token::set_authority(
                //     CpiContext::new_with_signer(
                //         ctx.accounts.token_program.to_account_info(),
                //         anchor_spl::token::SetAuthority {
                //         current_authority: ctx.accounts.wallet.to_account_info(),
                //         account_or_mint: ctx.accounts.mint.to_account_info(),
                //     },
                //     &[&[&[], &[]]],
                //     ), 
                //    // spl_token::instruction::Authority::MintTokens,
                   
                //    spl_token::instruction::AuthorityType::MintTokens,
                //    Some(*ctx.accounts.mint_authority.key),
                // )?;

                self.lost_authority = false;
                
                Ok(())
            } 


        pub fn mint_some_tokens(
            &mut self,
            ctx: Context<MintSomeTokens>,
            _mint_bump: u8,
            mint_authority_bump: u8,
        ) -> Result<(), ProgramError> {
            msg!("Total supply = {}", ctx.accounts.mint.supply);
            let ts = ctx.accounts.mint.supply;
            let token_amount:u64 = calc_price(ts);
            if ts >= 5000  {
                return Err(ProgramError::Custom(1));
            }

            msg!("We are the beneficiary! {}", ctx.accounts.beneficiary.key);
            msg!("We are the beneficiary! {}", &self.beneficiary);

            if ctx.accounts.beneficiary.key != &self.beneficiary {
                return Err(ProgramError::Custom(2));
            }

            let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                ctx.accounts.wallet.key,
                ctx.accounts.beneficiary.key,
                token_amount,
            );

            msg!("transfering {} lamports from {} to {}", token_amount, ctx.accounts.wallet.key , ctx.accounts.beneficiary.key);

            anchor_lang::solana_program::program::invoke(
                &transfer_ix,
                &[
                    ctx.accounts.wallet.to_account_info(),
                    ctx.accounts.beneficiary.to_account_info(),
                    ctx.accounts.mint_authority.to_account_info(),
                ],
            )?;

            msg!("minting token to {}", ctx.accounts.destination.key());

            anchor_spl::token::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::MintTo {
                        mint: ctx.accounts.mint.to_account_info(),
                        to: ctx.accounts.destination.to_account_info(),
                        authority: ctx.accounts.mint_authority.to_account_info(),
                    },
                    &[&[b"mint-authority".as_ref(), &[mint_authority_bump]]],
                ),
                1,
            )?;

            ctx.accounts.mint.reload()?;
            msg!("Total supply = {}", ctx.accounts.mint.supply);

            self.supply = ctx.accounts.mint.supply;
            self.price = calc_price(self.supply);

            msg!("new calculated price {} lamports", calc_price(self.supply));
            msg!("setting updated price {} lamports", self.price);

            Ok(())
        }
    }
}

pub fn calc_price(supply: u64) -> u64 {
    let fee:u64;
    if supply < 500 {
        fee = 0;
    } else if supply < 510 {
        fee = 1;
    } else if supply < 515 {
        fee = 2;
    } else if supply < 520 {
        fee = 3;
    } else if supply < 525 {
        fee = 4;
    } else if supply < 530 {
        fee = 5;
    } else {
        fee = 999999999;
    }

    return fee * (10_00_00_00_00);

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

    #[account(init_if_needed, payer = wallet, associated_token::mint = mint, associated_token::authority = wallet)]
    pub destination: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, mint_authority_bump: u8)]
pub struct MintSomeTokens<'info> {
    #[account(mut, seeds = [b"mint".as_ref()], bump = mint_bump)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(init_if_needed, payer = wallet, associated_token::mint = mint, associated_token::authority = wallet)]
    pub destination: Account<'info, TokenAccount>,

    #[account(seeds = [b"mint-authority".as_ref()], bump = mint_authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ChangeAuthority<'info> {


    #[account(mut)]
    pub wallet: Signer<'info>,
}