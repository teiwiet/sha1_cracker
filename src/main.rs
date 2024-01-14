use std::{
    io,
    fs::File,
    error::Error,
    io::{BufRead,BufReader}
};
use sha1::Digest;
use std::env;
const SHA1_STRING_LENGTH : usize = 40;
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = env::args().collect();
    if args.len() !=3{
        println!("Usage : sha1_cracker <wordlist.txt> <hash_2_crack>");
        return Ok(());
    }
    let hash_2_crack = args[2].trim();
    if hash_2_crack.len() != SHA1_STRING_LENGTH{
        return Err("Invalid hash value".into());
    }
    //open file
    let file = File::open(&args[1])?;
    let reader = BufReader::new(&file);
    for line in reader.lines(){
        let line = line?;
        let password = line.trim();
        if hash_2_crack == hex::encode(sha1::Sha1::digest(password.as_bytes())){
            println!("Password found : {}",&password);
            return Ok(())
        }
    }
    println!("Password not found");
    Ok(())
}