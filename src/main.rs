#![allow(unused)]
use sha1::Digest;
use std::fs::File;
use hex;
use std::{
    io,
    io::{BufRead,BufReader},
    error::Error,
};
use std::env;
const HASH_LENGTH : usize = 40;
fn main() -> Result<(),Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    if args.len()!=3{
        println!("Usage : sha1_cracker <wordlist> <hash_2_crack>");
    }

    let hash_2_crack = &args[2];
    if hash_2_crack.len()!= HASH_LENGTH{
        return Err("Invalid hash value".into());
    }
    let mut lmao : Vec<String> = vec![];
    //open file
    let file = File::open(args[1].trim())?;
    let reader = BufReader::new(&file);
    for line in reader.lines(){
        let line = line?;
        let password = line.trim();
        if hash_2_crack==&hex::encode(sha1::Sha1::digest(&password)){
            println!("Found password : {}",password);
            return Ok(());
        }
    }
    println!("Password not found");
    Ok(())
}