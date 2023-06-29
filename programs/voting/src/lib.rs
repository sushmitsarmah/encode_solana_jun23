use anchor_lang::prelude::*;

declare_id!("BQjhXTevM6DdxfRqfn8itMykbuD7gYc4a58v7ahv1Yh1");

#[program]
pub mod voting {
    use super::*;

    pub fn init_vote_bank(ctx: Context<InitVote>) -> Result<()> {
        // Open vote bank for public to vote on our favourite "NFT"
        ctx.accounts.vote_account.is_open_to_vote = true;
        Ok(())
    }

    pub fn nft_vote(ctx: Context<NftVote>, vote_type: VoteType) -> Result<()> {
        // If vote_type is GM increment GM by 1 else increment GN by 1
        match vote_type {
            VoteType::Like => {
                msg!("Liked this NFT");
                ctx.accounts.vote_account.gm += 1; 
            },
            VoteType::Dislike => {
                msg!("DisLiked this NFT");
                ctx.accounts.vote_account.gn += 1; 
            },
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVote<'info> {
    // Making a global account for storing votes
    #[account(
        init, 
        payer = signer, 
        space = 8 + 1 + 8 + 8, 
    )] 
    pub vote_account: Account<'info, VoteBank>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NftVote<'info> {
    // Storing Votes in global account
    #[account(mut)] 
    pub vote_account: Account<'info, VoteBank>,

    pub signer: Signer<'info>,
}


#[account]
#[derive(Default)]
pub struct VoteBank {
    is_open_to_vote: bool,
    gm: u64, // 8 bytes in size
    gn: u64, // 8 bytes in size
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    Like,
    Dislike
}
