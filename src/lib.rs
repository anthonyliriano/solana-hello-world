use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

//Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount{
/// number of greetings
pub counter: u32,
}

//Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Accepts the program_id, a public key where the program is being deployed and the accountInfo, which is the account
//used to say hello to.
pub fn process_instruction(
    program_id: &Pubkey, //Public key of the account the hello program was loaded into.
    accounts: &[AccountInfo], //The account to say hello to.
    _instruction_date: &[u8], //Ignored, all helloword instruction are hellos
) -> ProgramResult{ //Stores the main logic of the program. In this case, prints a message.
    msg!("Hello World Rust Program entrypoint");
    let accounts_iter = &mut accounts.iter(); //Iterating accounts is safer then indexing
    let account = next_account_info(accounts_iter)?; // Get the account to say hello to

    //The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    //Finally,  function fetches the existing account's stored number, raises the value by 1, writes back the result
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Greeted {} time(s)!", greeting_account.counter);
    Ok(())
}