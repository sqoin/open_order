use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
  
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
    let nonce = instruction_data[0];
    // Get the account to say hello to
    let open_orders = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let market = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let create_program = next_account_info(accounts_iter)?;
   let rent_account = next_account_info(accounts_iter)?;
    let dex_program_id = next_account_info(accounts_iter)?;
    let seeds = &[&create_program.key.to_bytes()[..32], &[nonce]];
    let signer_seeds = &[&seeds[..]];
    let serum_open_orders_account_size = 3228;
let rent=Rent::get().unwrap();
    let ix = solana_program::system_instruction::create_account(
        authority.key,
        open_orders.key,
        rent.minimum_balance(serum_open_orders_account_size),
        serum_open_orders_account_size as u64,
        dex_program_id.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            authority.clone(),
            open_orders.clone(),
            system_program.clone(),
        ],
        signer_seeds,
    )?;
        
    init_open_orders(
        open_orders,
        authority,
        market,
        rent_account,
        dex_program_id,
        signer_seeds,
    )?;

    Ok(())
}


fn init_open_orders<'info>(
    open_orders: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    market: &AccountInfo<'info>,
    rent: &AccountInfo<'info>,
    dex_program: &AccountInfo<'info>,
    signer_seeds:&[&[&[u8]]]
) -> ProgramResult {
   
    let ix = match serum_dex::instruction::init_open_orders(
        dex_program.key,
        open_orders.key,
        authority.key,
        market.key,
        None)
     {
        Err(err) => return Err(ProgramError::from(err).into()),
        Ok(ix) => ix,
    };
    solana_program::program::invoke_signed(
        &ix
        , &[
            open_orders.clone(),
            authority.clone(),
            market.clone(),
            rent.clone()
        ],
         signer_seeds)?;
    Ok(())
}

/* fn verfy_account(x:&AccountInfo)->Option<Pubkey>{
    if x.data_is_empty() {
        return  None
    } else {
        return Some(*x.key)
    };
} */