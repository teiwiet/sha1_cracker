use std::{
    io,
    error::Error,
};
use std::env;


const SHA1_HEX_STRING_LENGTH : usize = 40;


fn main()->Result<(),Box<dyn Error>>{
    let mut args : Vec<String> = env::args().collect();
    if args.lent() !=3{
        println!("Usage : sha1_cracker <hash_2_crack> <word_list.txt>");
    }
    if args[2].len() != SHA1_HEX_STRING_LENGTH{
        return Err("Hash is not valid".into());
    }
    Ok(())
}