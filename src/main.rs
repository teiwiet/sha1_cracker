use sha1::Digest;
use std::{
    io,
    error::Error,
    fs::File,
    io::{BufRead,BufReader},
};
use std::env;
use std::io::Read;


const SHA1_HEX_STRING_LENGTH : usize = 40;


fn main()->Result<(),Box<dyn Error>>{
    let mut args : Vec<String> = env::args().collect();
    if args.len() !=3{
        println!("Usage : sha1_cracker <hash_2_crack> <word_list.txt>");
    }

    let hash_2_crack  = &args[2];
    if hash_2_crack.len() != SHA1_HEX_STRING_LENGTH{
        return Err("Hash is not valid".into());
    }
    //open file
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);
    for line in reader.lines(){
        let line = line?;
        let password = line.trim();
        if hash_2_crack == &hex::encode(sha1::Sha1::digest(password.as_bytes())){
            println!("Password found : {}",&password);
            return Ok(());
        }
    }
    Ok(())
}