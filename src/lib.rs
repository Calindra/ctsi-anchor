pub use anchor_spl::*;

pub mod token {
    use anchor_lang::context::CpiContext;
    use anchor_lang::solana_program::msg;
    use anchor_lang::Result;
    pub use anchor_spl::token::*;

    pub fn transfer2<'a, 'b, 'c, 'info>(
        _ctx: CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>,
        _amount: u64,
    ) -> Result<()> {
        msg!("At this point we intercept the CPI");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
