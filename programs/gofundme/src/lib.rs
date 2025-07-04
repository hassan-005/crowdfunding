use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("DaCiUp5f1XxcRgjcGNvB9xRRGYBr8yhjvyEzufVPyeRT");

#[program]
pub mod gofundme {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
        msg!("Initializing");
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.admin = *ctx.accounts.authority.key;
        Ok(())
    }
    pub fn withdraw(ctx:Context<Withdraw>, amount: u64) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.authority;
        
        if campaign.admin != *user.key{
            return Err(ProgramError::IncorrectProgramId)
        }
        let rent = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        campaign.amount_donated -= amount;

        msg!("Amount in campaign {}", campaign.amount_donated);

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount:u64)-> ProgramResult{
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.campaign.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.campaign.to_account_info()
            ]
        );
        (&mut ctx.accounts.campaign).amount_donated += amount;
        msg!("Amount in campaign {}", (&ctx.accounts.campaign).amount_donated);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create <'info> {
    #[account(init , payer=authority, space=900 )]
    pub campaign : Account<'info, Campaign>,
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>

}
#[derive(Accounts)]
pub struct Withdraw <'info> {
    #[account(mut)]
    pub campaign : Account<'info, Campaign>,
    #[account(mut)]
    pub authority : Signer<'info>,

}
#[derive(Accounts)]
pub struct Donate <'info> {
    #[account(mut)]
    pub campaign : Account<'info, Campaign>,
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Campaign{
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_donated: u64 
}
