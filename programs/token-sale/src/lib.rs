
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
        price: f64,
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
           
            // anchor_spl::token::set_authority(
            //     CpiContext::new_with_signer(
            //         ctx.accounts.token_program.to_account_info(),
            //         anchor_spl::token::SetAuthority {
            //         current_authority: ctx.accounts.mint_authority.to_account_info(),
            //         account_or_mint: ctx.accounts.mint.to_account_info(),
            //     },
            //     &[&[b"mint-authority".as_ref(), &[mint_authority_bump]]],
            //     ), 
            //    // spl_token::instruction::Authority::MintTokens,
               
            //    spl_token::instruction::AuthorityType::MintTokens,
            //    Some(*ctx.accounts.wallet.key),
            // )?;

    
            
        
            Ok(Self {
                beneficiary: *ctx.accounts.beneficiary.key,
                supply: initialSupply,
                price: calc_price(initialSupply),
                lost_authority: true
            })
        }

        pub fn change_authority_to_deployer(&mut self,
            ctx: Context<ChangeAuthorityToDeployer>, _mint_bump: u8, mint_authority_bump: u8) -> Result<(), ProgramError> {
                
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
                
                Ok(())
            
        }

        pub fn change_authority_to_program(&mut self,
            ctx: Context<ChangeAuthorityToProgram>,) -> Result<(), ProgramError> {

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
            token_count: u64,
        ) -> Result<(), ProgramError> {

            if token_count == 0 || token_count > 10 {
                return Err(ProgramError::Custom(1));
            }

            if ctx.accounts.beneficiary.key != &self.beneficiary {
                    return Err(ProgramError::Custom(2));
            }
            let ts = ctx.accounts.mint.supply;
            if ts >= 10500  {
                return Err(ProgramError::Custom(3));
            }

            let mut token_amount:f64 = calc_price(ts);
            let token_amount2:f64 = calc_price(ts + token_count - 1);

            if token_amount == token_amount2 {
                token_amount = token_amount * token_count as f64;
            } else {
                token_amount = (token_count - ( (ts + token_count) % 10)) as f64 * token_amount + ((ts + token_count) % 10) as f64 * token_amount2;
            }
            msg!("We are the beneficiary! {}", ctx.accounts.beneficiary.key);
            msg!("We are the beneficiary! {}", &self.beneficiary);

            let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                ctx.accounts.wallet.key,
                ctx.accounts.beneficiary.key,
                token_amount as u64,
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
                token_count,
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

pub fn calc_price(supply: u64) -> f64 {
    let fee: f64;
    if supply < 500 {
        fee = 0.0;
    } else if supply >=500 && supply < 750 {
        fee = 1.0;
    } else if supply >= 750 && supply < 1250 {
        fee = 1.5;
    } else if supply >= 1250 && supply < 3250 {
        fee = 2.0;
    } else if supply >= 3250 && supply < 5250 {
        fee = 2.5;
    } else if supply >= 5250 && supply < 8750 {
        fee = 3.0;
    } else if supply >= 8750 && supply < 10250 {
        fee = 3.5;
    } else if supply >= 10250 && supply < 10500 {
        fee = 4.0;
    } else {
        fee = 999999.9;
    }


   // return fee * (10_00_00_00_00);
    return fee  * 1000000000.0

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

    pub beneficiary: AccountInfo<'info>,

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
pub struct ChangeAuthorityToProgram<'info> {


    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, mint_authority_bump: u8)]
pub struct ChangeAuthorityToDeployer<'info> {

    #[account(mut, seeds = [b"mint".as_ref()], bump = mint_bump)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(seeds = [b"mint-authority".as_ref()], bump = mint_authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

}