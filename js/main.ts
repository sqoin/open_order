import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    Account,
    SYSVAR_RENT_PUBKEY,
  } from '@solana/web3.js';


async function createAndInitOpenOrder(){
    let createAccount=new Account()
    let openOrder=new Account();
    let programId=new PublicKey("BZMn9wczraZbwBErg2A7HXexE4qLi1syRH2SQ2jjHQoB");
    let connection=new Connection("http://127.0.0.1:8899")
    console.log("openOrderopenOrderopenOrder ",openOrder.publicKey.toBase58())
    let [programAddress, nonce] = await PublicKey.findProgramAddress(
      [createAccount.publicKey.toBuffer()],
      programId,
    ); 
    let authority=new Account([197,77,191,226,212,208,95,83,23,165,194,47,62,189,32,7,75,135,221,89,54,61,223,75,171,254,56,104,219,96,206,131,177,204,147,100,86,155,74,144,149,79,243,251,128,144,119,238,227,2,10,35,25,45,241,214,150,80,60,242,187,131,76,248])
    let market=new PublicKey("3tJDRWYaZdge1bYAAsf3bhytx8rRQXX7eWubeyZzNHJp");
    let swap_serum_program_id=new PublicKey("CxXDXjGBJ6RwMKKLqkd9KCAR5yfswNd8iXcQPmFFeDvU");
    let dex_program_id=new PublicKey("7RA6GmbCYRBB66QfuDa1peHAE2fWDbeR7Vr2sGmNtGFC");
    let rent=SYSVAR_RENT_PUBKEY;
  
    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: openOrder.publicKey, isSigner: true, isWritable: true},
        {pubkey: authority.publicKey, isSigner: true, isWritable: false},
        {pubkey: market, isSigner: false, isWritable: false},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        {pubkey: createAccount.publicKey, isSigner: false, isWritable: false},
        {pubkey:rent, isSigner: false, isWritable: false},
        {pubkey:swap_serum_program_id, isSigner: false, isWritable: false},
        {pubkey:dex_program_id, isSigner: false, isWritable: false},
      ],
      programId,
      data: Buffer.from([nonce]), // All instructions are hellos
    });
    await sendAndConfirmTransaction(
      connection,
      new Transaction().add(instruction),
      [authority,openOrder],
    );
}
createAndInitOpenOrder();
