//fn main() {
//    println!("¡Hola Mundo!");
//}

use borsh::{BorshDeserealize, BorshSeriable};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::Programresult, 
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};

//Define the type of state stored in accounts
#[derive(BorshSeriable,BorshDeserealize, Debug)]
pub struct GreetingAccount{
    //Number of greetings
    pub counter: u32,
}

//Declare and export the program's entrypoint
entrypoint!(process_instruction);

//Program entrypoint's implementation
pub fn process process_instruction(
    program_id: &Pubkey, //Public key of the account hello world
    accounts: &[AccountInfo], the account to say hello
    _instruction_data: &[u8], //ignored, all helooworld instructions
) -> Programresult{
    msg! ("Hello world Rust program entrypoint");

    //Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    //Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    //The account must be owned by the program in order to modify to
    if account.owner != program_id{
        msg! ("Greeted account does not have the correct program id");
        return Err (ProgramError::IncorrectProgramId);
    }
}

//Increment and store the number of times account has been are
let mut greetingAccount = GreetingAccount::try_form_slice(&account);
greetingAccount.conter += 1;
greetingAccount.serialize(&mut &mut account.data.borrow_mut()[..])

msg!("Greeted {} time(s)!", greeting_account.counter);

Ok(())