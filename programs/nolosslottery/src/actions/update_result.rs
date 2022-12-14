use crate::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
pub use switchboard_v2::VrfAccountData;

#[derive(Accounts)]
#[instruction(params: UpdateResultParams)]
pub struct UpdateResult<'info> {
    #[account(mut)]
    pub state: AccountLoader<'info, VrfClient>,
    /// CHECK:
    pub vrf: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateResultParams {}

impl UpdateResult<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &UpdateResultParams) -> ProgramResult {
        Ok(())
    }

    pub fn process(ctx: &Context<Self>, _params: &UpdateResultParams) -> ProgramResult {
        let vrf_account_info = &ctx.accounts.vrf;
        let vrf = VrfAccountData::new(vrf_account_info)?;
        let result_buffer = vrf.get_result()?;
        if result_buffer == [0u8; 32] {
            return Ok(());
        }

        let state = &mut ctx.accounts.state.load_mut()?;
        let max_result = state.max_result;
        if result_buffer == state.result_buffer {
            return Ok(());
        }

        let value: &[u128] = bytemuck::cast_slice(&result_buffer[..]);
        let result = value[0] % max_result as u128;

        if state.result != result {
            state.result_buffer = result_buffer;
            state.result = result;
            state.last_timestamp = clock::Clock::get().unwrap().unix_timestamp;
        }

        Ok(())
    }
}
