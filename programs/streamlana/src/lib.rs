use anchor_lang::{
    prelude::*,
    solana_program::{ self, program::{ invoke, invoke_signed }, system_program::ID as SYSID },
};
use anchor_spl::token::{ spl_token, Token };

declare_id!("CQapQthxRSpDEEoj1EQ2S9Esj4uZxPSwC49pKufQfo3z");

#[program]
pub mod streamlana {
    use super::*;

    pub fn start_stream(ctx: Context<StartStream>, stream_data: StreamPda) -> Result<()> {
        if stream_data.currency.eq(&SYSID) {
            let sol_ix = solana_program::system_instruction::transfer(
                &ctx.accounts.sender.key(),
                &ctx.accounts.user_stream_pda.key(),
                0
            );
        } else {
            let token_ix = spl_token::instruction::transfer(
                &ctx.accounts.token_program.key,
                &ctx.accounts.sender.key(),
                &ctx.accounts.user_stream_pda.key(),
                &ctx.accounts.sender.key(),
                &[&ctx.accounts.sender.key()],
                0
            )?;

            invoke(
                &token_ix,
                &[
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.destinary.to_account_info(),
                    ctx.accounts.sender.to_account_info(),
                    ctx.accounts.user_stream_pda.to_account_info(),
                ]
            )?;
        }

        let now = Clock::get()?.unix_timestamp;

        ctx.accounts.user_stream_pda.destinary = ctx.accounts.destinary.key();
        ctx.accounts.user_stream_pda.flow_rate = 0;
        ctx.accounts.user_stream_pda.last_balance = 0;
        ctx.accounts.user_stream_pda.start_time = now;
        ctx.accounts.user_stream_pda.last_claimed = now;

        Ok(())
    }

    // lock
    // unlock
    // start_stream
    // update_stream
}

#[derive(Accounts)]
pub struct StartStream<'info> {
    #[account(
        init,
        payer = sender,
        seeds = [&destinary.key().as_ref()],
        bump,
        space = StreamPda::LEN
    )]
    user_stream_pda: Box<Account<'info, StreamPda>>,
    #[account(mut)]
    sender: Signer<'info>,
    destinary: AccountInfo<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct creatdSharePda<'info> {
    #[account(
        init,
        payer = sender,
        seeds = [b"shares".as_ref(), sender.key().as_ref()],
        bump,
        space = StreamPda::LEN
    )]
    user_stream_pda: Box<Account<'info, StreamPda>>,
    #[account(mut)]
    sender: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct StreamPda {
    pub last_owner: Pubkey,
    pub destinary: Pubkey,
    pub currency: Pubkey,
    pub flow_rate: u64, // lamport/time
    pub last_claimed: i64, // timestamp
    pub start_time: i64, // timestamp
    pub last_balance: u64, // lamport
}

#[account]
#[derive(Default)]
pub struct comissionPDA {
    pub owner: Pubkey,
    pub shares: i64,
}
impl StreamPda {
    const LEN: usize =
        8 + //Base
        8 * 4 + //u32
        32;

    // pub fn size(swap_data: SwapData) -> usize {
    //     let nft_size = NftSwapItem::LEN.checked_mul(swap_data.nb_items.nft as usize).unwrap();
    //     let token_size = TokenSwapItem::LEN.checked_mul(
    //         swap_data.nb_items.tokens as usize
    //     ).unwrap();
    //     msg!("nft_size {} token_size {}", nft_size, token_size);
    //     return SwapData::LEN.checked_add(nft_size.checked_add(token_size).unwrap()).unwrap();
    // }
}

// #[derive(Accounts)]
// #[instruction(seed: Vec<u8>)]
// pub struct ClaimPNft<'info> {
//     #[account(init, payer = signer, seeds = [&seed[..]], bump, space = SwapData::size(swap_data))]
//     swap_data_account: Box<Account<'info, SwapData>>,
//     #[account(mut)]
//     signer: Signer<'info>,

//     /// CHECK: user Account
//     #[account(mut,
//         constraint = user_ata.owner == user.key() @ MYERROR::IncorrectOwner
//         )]
//     user: AccountInfo<'info>,
//     #[account(
//         mut,
//         constraint = swap_data_account_ata.mint == user_ata.mint  @ MYERROR::MintIncorrect,
//         constraint = swap_data_account_ata.owner == swap_data_account.key()  @ MYERROR::IncorrectOwner
//     )]
//     swap_data_account_ata: Account<'info, TokenAccount>,
//     #[account(
//         mut,
//         constraint = user_ata.owner == user.key() @ MYERROR::IncorrectOwner
//     )]
//     user_ata: Account<'info, TokenAccount>,

//     #[account(constraint = swap_data_account_ata.mint == mint.key()  @ MYERROR::MintIncorrect)]
//     mint: Account<'info, Mint>,
//     /// CHECK: in constraints
//     #[account(mut,
//         seeds =[
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref()],
//         bump,
//         owner = metadata_program.key() @ MYERROR::IncorrectMetadata,
//         seeds::program = metadata_program.key()
//     )]
//     nft_metadata: AccountInfo<'info>,
//     /// CHECK: in constraints
//     nft_master_edition: AccountInfo<'info>,
//     /// CHECK: in constraints
//     #[account(mut)]
//     owner_token_record: AccountInfo<'info>,
//     /// CHECK: in constraints
//     #[account(mut)]
//     destination_token_record: AccountInfo<'info>,
//     /// CHECK: account checked in CPI
//     auth_rules_program: AccountInfo<'info>,
//     /// CHECK: account checked in CPI
//     auth_rules: AccountInfo<'info>,
//     system_program: Program<'info, System>,
//     token_program: Program<'info, Token>,
//     /// CHECK: in constraint
//     #[account(constraint = metadata_program.key().eq(&mpl_token_metadata::ID) @ MYERROR::IncorrectMetadata)]
//     metadata_program: AccountInfo<'info>,
//     /// CHECK: in constraint
//     #[account(constraint = sysvar_instructions.key().eq(&solana_program::sysvar::instructions::ID) @ MYERROR::IncorrectSysvar)]
//     sysvar_instructions: AccountInfo<'info>,
//     spl_ata_program: Program<'info, AssociatedToken>,
// }

// #[account]
// #[derive(Default)]
// pub struct SwapData {
//     pub initializer: Pubkey, // Initializer is admin of the PDA
//     pub status: u8, // Gives the status of the current swap with TradeStatus
//     pub nb_items: NbItems, // Required to initialize the PDA account data size
//     pub pre_seed: String, // String to initialize PDA's seed
//     pub seed_string: String, // String to initialize PDA's seed
//     pub nft_items: Vec<NftSwapItem>, // List of items engaged in a swap (can be SOL or NFT)
//     pub token_items: Vec<TokenSwapItem>, // List of items engaged in a swap (can be SOL or NFT)
//     pub receiving_streams: Vec<TokenSwapItem>, // List of items engaged in a swap (can be SOL or NFT)
//     pub sending_streams: Vec<TokenSwapItem>, // List of items engaged in a swap (can be SOL or NFT)
//     pub accepted_payement: Pubkey, // List of tokens accepted for payment
//     pub start_time: i64, // Timestamp of the opening of the swap
//     pub duration: i64, // Duration of the swap
// }

// impl SwapData {
//     const LEN: usize =
//         8 + //Base
//         1 + //u8
//         4 * 2 + //u32
//         32 +
//         32 + // max 32 char pre_seed
//         50 + // max 50 char seed_string
//         32 * 2; //Pubkey

//     pub fn size(swap_data: SwapData) -> usize {
//         let nft_size = NftSwapItem::LEN.checked_mul(swap_data.nb_items.nft as usize).unwrap();
//         let token_size = TokenSwapItem::LEN.checked_mul(
//             swap_data.nb_items.tokens as usize
//         ).unwrap();
//         msg!("nft_size {} token_size {}", nft_size, token_size);
//         return SwapData::LEN.checked_add(nft_size.checked_add(token_size).unwrap()).unwrap();
//     }
// }

// invoke_signed(
//     &ix,
//     &[
//         ctx.accounts.token_program.to_account_info(),
//         ctx.accounts.signer.to_account_info(),
//         ctx.accounts.user_ata.to_account_info(),
//         ctx.accounts.user_ata.to_account_info(),
//         ctx.accounts.swap_data_account_ata.to_account_info(),
//     ],
//     &[&[&seed[..], &[bump]]]
// )?;
