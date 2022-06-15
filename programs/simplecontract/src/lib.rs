use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod simplecontract {
    use super::*;

    pub fn create_global_state(ctx: Context<CreateGlobalState>) -> ProgramResult {
        let mut global_state = ctx.accounts.global_state.load_init()?;
        global_state.admin = ctx.accounts.admin.key();
        global_state.users = 0;
        global_state.bump = *ctx.bumps.get("global_state").unwrap();

        Ok(())
    }


    pub fn initialize_vault (ctx: Context<InitializeValut>) -> ProgramResult {
        let mut vault = ctx.accounts.vault.load_init()?;
        
        vault.token_x = ctx.accounts.token_x.key();
        vault.token_y = ctx.accounts.token_y.key();
        vault.token_x_account = ctx.accounts.token_x_account.key();
        vault.token_y_account = ctx.accounts.token_y_account.key();
        vault.token_x_amount = 0;
        vault.token_y_amount = 0;
        vault.bump = *ctx.bumps.get("vault").unwrap();
        
        Ok(())
    }


    pub fn initialize_user(ctx: Context<InitializeUser>) -> ProgramResult {
        let mut global_state = ctx.accounts.global_state.load_mut()?;
        global_state.users += 1;

        let mut user_struct = ctx.accounts.user_struct.load_init()?;
        
        user_struct.wallet = ctx.accounts.wallet.key();
        user_struct.token_x_account = ctx.accounts.token_x_account.key();
        user_struct.token_y_account = ctx.accounts.token_y_account.key();
        user_struct.token_x_amount = 0;
        user_struct.token_y_amount = 0;
        user_struct.id = global_state.users;
        user_struct.bump = *ctx.bumps.get("user_struct").unwrap();
        
        Ok(())
    }


    pub fn storage_token(ctx: Context<StorageToken>, t_x_amount: u64, t_y_amount: u64) -> ProgramResult {
        let mut user = ctx.accounts.user_struct.load_mut()?;
        let mut vault = ctx.accounts.vault.load_mut()?;
        user.token_x_amount += t_x_amount;
        vault.token_x_amount += t_x_amount;
        user.token_y_amount += t_y_amount;
        vault.token_y_amount += t_y_amount;
        Ok(())
    }


    pub fn withdraw_token(ctx: Context<WithdrawToken>, t_x_amount: u64, t_y_amount: u64) -> ProgramResult {
        let mut user = ctx.accounts.user_struct.load_mut()?;
        let mut vault = ctx.accounts.vault.load_mut()?;
        user.token_x_amount -= t_x_amount;
        vault.token_x_amount -= t_x_amount;
        user.token_y_amount -= t_y_amount;
        vault.token_y_amount -= t_y_amount;
        
        Ok(())
    }

}



#[derive(Accounts)]
pub struct StorageToken<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [wallet.key().as_ref()], bump = user_struct.load()?.bump)]
    pub user_struct: AccountLoader<'info, User>,
    #[account(mut, constraint = user_x_account.mint == token_x.key(), constraint = user_x_account.key() == user_struct.load()?.token_x_account)]
    pub user_x_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = user_y_account.mint == token_y.key(), constraint = user_y_account.key() == user_struct.load()?.token_y_account)]
    pub user_y_account: Account<'info, TokenAccount>,
    pub wallet: AccountInfo<'info>,
    pub token_x: Account<'info, Mint>,
    pub token_y: Account<'info, Mint>,
    #[account(mut, constraint = token_x_account.mint == token_x.key(), constraint = token_x_account.key() == vault.load()?.token_x_account)]
    pub token_x_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = token_y_account.mint == token_y.key(), constraint = token_y_account.key() == vault.load()?.token_y_account)]
    pub token_y_account: Account<'info, TokenAccount>,
    pub vault: AccountLoader<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [wallet.key().as_ref()], bump = user_struct.load()?.bump)]
    pub user_struct: AccountLoader<'info, User>,
    #[account(mut, constraint = user_x_account.mint == token_x.key(), constraint = user_x_account.key() == user_struct.load()?.token_x_account)]
    pub user_x_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = user_y_account.mint == token_y.key(), constraint = user_y_account.key() == user_struct.load()?.token_y_account)]
    pub user_y_account: Account<'info, TokenAccount>,
    pub wallet: AccountInfo<'info>,
    pub token_x: Account<'info, Mint>,
    pub token_y: Account<'info, Mint>,
    #[account(mut, constraint = token_x_account.mint == token_x.key(), constraint = token_x_account.key() == vault.load()?.token_x_account)]
    pub token_x_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = token_y_account.mint == token_y.key(), constraint = token_y_account.key() == vault.load()?.token_y_account)]
    pub token_y_account: Account<'info, TokenAccount>,
    pub vault: AccountLoader<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateGlobalState<'info> {
    #[account(init, seeds = [b"global_state"], payer = admin, bump)]
    pub global_state: AccountLoader<'info, GlobalState>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeValut<'info> {
    pub global_state: AccountLoader<'info, GlobalState>,
    #[account(init, seeds = [token_x.key().as_ref(), token_y.key().as_ref()], payer = admin, bump)]
    pub vault: AccountLoader<'info, Vault>,
    pub token_x: Account<'info, Mint>,
    pub token_y: Account<'info, Mint>,
    #[account(constraint = token_x_account.mint == token_x.key(), constraint = token_x_account.owner == admin.key())]
    pub token_x_account: Account<'info, TokenAccount>,
    #[account(constraint = token_y_account.mint == token_y.key(), constraint = token_y_account.owner == admin.key())]
    pub token_y_account: Account<'info, TokenAccount>,
    #[account(constraint = admin.key() == global_state.load()?.admin)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct InitializeUser<'info> {
    pub global_state: AccountLoader<'info, GlobalState>,
    #[account(init, seeds = [wallet.key().as_ref()], payer = user, bump)]
    pub user_struct: AccountLoader<'info, User>,
    #[account(seeds = [token_x.key().as_ref(), token_y.key().as_ref()], bump = vault.load()?.bump)]
    pub vault: AccountLoader<'info, Vault>,
    pub token_x: Account<'info, Mint>,
    pub token_y: Account<'info, Mint>,
    #[account(constraint = token_x_account.mint == token_x.key(), constraint = token_x_account.owner == user.key())]
    pub token_x_account: Account<'info, TokenAccount>,
    #[account(constraint = token_y_account.mint == token_y.key(), constraint = token_y_account.owner == user.key())]
    pub token_y_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[account(zero_copy)]
#[repr(packed)]
#[derive(Debug, Default)]
pub struct GlobalState {
    pub admin: Pubkey,
    pub users: u64,
    pub bump: u8,
}


#[account(zero_copy)]
#[repr(packed)]
#[derive(Debug, Default)]
pub struct Vault {
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_x_account: Pubkey,
    pub token_y_account: Pubkey,
    pub token_x_amount: u64,
    pub token_y_amount: u64,
    pub bump: u8,
}


#[account(zero_copy)]
#[repr(packed)]
#[derive(Debug, Default)]
pub struct User {
    pub wallet: Pubkey,
    pub token_x_account: Pubkey,
    pub token_y_account: Pubkey,
    pub token_x_amount: u64,
    pub token_y_amount: u64,
    pub id: u64,
    pub bump: u8,
}
