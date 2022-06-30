use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();
    let nonce=instruction_data[0];
    // Get the account to say hello to
    let open_orders = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let market = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let create_program = next_account_info(accounts_iter)?;
    let rent=next_account_info(accounts_iter)?;
    let swap_program_id=next_account_info(accounts_iter)?;
    let dex_program_id=next_account_info(accounts_iter)?;
    let seeds=&[&create_program.key.to_bytes()[..32], &[nonce]];
    let signer_seeds = &[&seeds[..]];
    let ix=solana_program::system_instruction::create_account(
        authority.key,
        open_orders.key,
        234567,
        3228,
        &serum_swap::ID
    );
    solana_program::program::invoke_signed(
        &ix, 
        &[
            authority.clone(),
            open_orders.clone(),
            system_program.clone()
        ],
        signer_seeds,
    )?;
  init_open_orders(open_orders,authority,market,rent,dex_program_id,swap_program_id)?;
   
  Ok(())

}

pub fn init_open_orders<'info>(
    open_orders:&solana_program::account_info::AccountInfo<'info>,
    authority:&solana_program::account_info::AccountInfo<'info>,
    market:&solana_program::account_info::AccountInfo<'info>,
    rent:&solana_program::account_info::AccountInfo<'info>,
    dex_program:&AccountInfo<'info>,
    serum_swap_id:&solana_program::account_info::AccountInfo<'info>,

)->ProgramResult{
  
    let init_account=serum_swap::cpi::accounts::InitAccount{
        open_orders:open_orders.clone(),
        authority:authority.clone(),
        market:market.clone(),
        dex_program:dex_program.clone(),
        rent:rent.clone()};
        let cpi_ctx = serum_swap::anchor_lang::CpiContext::new(serum_swap_id.clone(),init_account);
        serum_swap::cpi::init_account(cpi_ctx)?;
   

    Ok(())
}


