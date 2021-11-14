use anchor_lang::prelude::*;

declare_id!("EP1hwCGfZfSw6uWxMs7DtXfJgknfP8QYBidZRynwVZMm");

#[program]
pub mod myepicproject {
    use super::*;

    // Used to initiate the program
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a ref to the base account of the current program
        let base_account = &mut ctx.accounts.base_account;

        // Initialize total gifs on the base account reference
        base_account.total_gifs = 0;

        Ok(())
    }

    // Used to add a gif to the base account
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;

        let user = &mut ctx.accounts.user;

        // Build the Item struct
        let item = ItemStruct {
            gif_link,
            user_address: *user.to_account_info().key,
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Context object that derives from Accounts
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // Initialize an account with this parameters
    // init -> creates a new account owned by the program
    // payer -> the account that pays for the account (user in this case is us)
    // space -> the amount of space in bytes that the account will have
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Create an account and decide on what to hold inside it
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}
