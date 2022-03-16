use wasm_bindgen::prelude::*;
//use hex_literal::hex;
use sha2::{Sha256, Digest};


fn readerOfPasswd() -> String {

    // get value of password field
    let passwd: String = "jackson".to_string();
    print!("\npasswd= {}\n",passwd);

    return passwd;
}

fn salt(test: &str) -> String {
    
    let common_salt: &str = "SmartNotary";
    let mut salted_passwd: String = String::from(test);

    salted_passwd.push_str(common_salt);

    println!("salt = {}\n", common_salt);
    println!("salted_passwd = {}\n", salted_passwd);

    return salted_passwd;
}

#[wasm_bindgen]
pub fn sha2() -> String {

    // create a Sha256 object
    let mut hasher = Sha256::new();
    let passwd: String = readerOfPasswd();
    let salted_passwd: String = salt(&passwd);

    // write input message
    hasher.update(salted_passwd);

    // read hash digest and consume hasher
    let hashed_passwd = hasher.finalize();

    println!("myhash-1 = {:?}\n",hashed_passwd);
    
    return format!("{:?}", hashed_passwd);
}