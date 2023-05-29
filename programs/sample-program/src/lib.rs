/*
  Copyright 2023 Bitoku Labs

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use anchor_lang::prelude::*;
use bitoku_sdk::*;

// const MAX_ENTRY_LEN: usize = 1_000_000;

declare_id!("9484XY8sPuWKySxZ976fSutET8v2AT2gKSej2MzHQ9iU");

#[program]
pub mod sample_program {
    use super::*;

    //This function will register a diary client.
    pub fn init_diary(ctx: Context<InitDiary>) -> Result<()> {
        //sdk agent logic

        let accounts = &CpiAccounts {
            fee_payer: ctx.accounts.owner.to_account_info(),
            bookkeeper: ctx.accounts.bookkeeper.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            bitoku_agent_program_id: ctx.accounts.agent.to_account_info(),
            request: ctx.accounts.request.to_account_info(),
            sys_var_program: ctx.accounts.rent.to_account_info(),
        };

        let my_id = reg_client(accounts);
        msg!("{}", my_id);
        //actual program logic
        ctx.accounts.diary.owner = *ctx.accounts.owner.key;
        ctx.accounts.diary.last_update = ctx.accounts.clock.unix_timestamp;
        ctx.accounts.diary.bitoku_client_id = my_id;

        Ok(())
    }

    pub fn add_entry(ctx: Context<AddEntry>, diary_content: String) -> Result<()> {
        //sdk logic
        let accounts = &CpiAccounts {
            fee_payer: ctx.accounts.owner.to_account_info(),
            bookkeeper: ctx.accounts.bookkeeper.to_account_info(),
            request: ctx.accounts.request.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            sys_var_program: ctx.accounts.rent.to_account_info(),
            bitoku_agent_program_id: ctx.accounts.agent.to_account_info(),
        };

        // Actual program logic

        let mut arr: [u8; 512] = [0; 512];
        let bytes = diary_content.as_bytes();
        arr[..bytes.len()].copy_from_slice(bytes);

        let file_name = "diary".to_string();

        write_file(
            &accounts,
            &file_name,
            arr,
            1,
            ctx.accounts.diary.bitoku_client_id,
        );

        ctx.accounts.diary.last_update = ctx.accounts.clock.unix_timestamp;
        ctx.accounts.diary.num_entries += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDiary<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        init,
        payer= owner,
        space= 8+32+1+8+4,
        seeds= ["diary".as_ref(), owner.key().as_ref()],
        bump
    )]
    diary: Account<'info, DiaryDescriptor>,

    ///CHECK
    #[account(
        mut,
        seeds =["bookkeeper".as_bytes()],
        seeds::program=agent,
        bump
     )]
    bookkeeper: UncheckedAccount<'info>,
    ///CHECK
    #[account(
        mut,
        seeds = [
            "request".as_bytes(),
            owner.key().as_ref(),
        ],
        seeds::program=agent,
        bump
    )]
    request: UncheckedAccount<'info>,
    ///CHECK
    agent: UncheckedAccount<'info>,

    clock: Sysvar<'info, Clock>,

    system_program: Program<'info, System>,

    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddEntry<'info> {
    owner: Signer<'info>,

    ///CHECK
    #[account(
        mut,
        seeds =["bookkeeper".as_bytes()],
        seeds::program=agent,
        bump
     )]
    bookkeeper: UncheckedAccount<'info>,
    ///CHECK
    agent: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = ["diary".as_bytes(),owner.key().as_ref()],
        bump
    )]
    diary: Account<'info, DiaryDescriptor>,

    ///CHECK
    #[account(
        mut,
        seeds = [
            "request".as_bytes(),
            owner.key().as_ref(),
        ],
        seeds::program=agent,
        bump
    )]
    request: UncheckedAccount<'info>,

    clock: Sysvar<'info, Clock>,

    system_program: Program<'info, System>,

    rent: Sysvar<'info, Rent>,
}

#[account]
pub struct DiaryDescriptor {
    owner: Pubkey,
    bitoku_client_id: u8,
    last_update: i64,
    num_entries: u32,
}
