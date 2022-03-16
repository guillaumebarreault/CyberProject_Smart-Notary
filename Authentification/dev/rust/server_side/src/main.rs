use rand::Rng;
use argon2::{self, Config, ThreadMode, Variant, Version};
/* AES */
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};


fn random_salt() -> [u8; 32] {
    // salt: 256-bits 
    let rnd_salt: [u8; 32] = rand::thread_rng().gen();
    println!("\nrandom_salt = {:?}\n", rnd_salt);

    return rnd_salt;
}

fn argon2() -> String {

    let config = Config {
	    variant: Variant::Argon2d, 	// Select: Argon2i; Argon2d; Argon2id;
	    version: Version::Version13,
	    mem_cost: 65536,
	    time_cost: 10,  // Iterations
	    lanes: 4,   // Parallelism Factor
	    thread_mode: ThreadMode::Parallel,
	    secret: &[],
	    ad: &[],
	    hash_length: 32, // 256-bits
	};
    // get value of hash-1 (sha256)
    let salt: [u8; 32] = random_salt();
    let hash1: &str = "b22924d2235b022f3340fe32461c148951e5339883579b0110f70f6a84750597";    
    let hash2: String = argon2::hash_encoded(hash1.as_bytes(), &salt, &config).unwrap();

    println!("hash1 = {:?} \n",hash1);
	println!("hash2 = {:?} \n",hash2);

    return hash2;
}

// cipher data => send to DB 
struct MessageAES {
    //salt: [u8; 32],
    iv: [u8; 12],
    key_aes: [u8; 32],    
    cipher_hash: Vec<u8>,   
}

fn aes_gcm_encrypt() -> MessageAES {

    /* aes key: 256-bits; unique per user */
    let key: [u8; 32] = rand::thread_rng().gen();
    let secret_key = Key::from_slice(&key);
    let cipher = Aes256Gcm::new(secret_key);

    // IV: 96-bits; unique per message
    let rnd: [u8; 12] = rand::thread_rng().gen();
    let iv = Nonce::from_slice(&rnd);

    // get hash: 256-bits; convert string to byte
    let data: String = argon2();
    println!("hash2 in Bytes = {:?}\n", data.as_bytes());

    // encrypt (convert data in byte)
    let ciphertext: Vec<u8> = cipher.encrypt(iv, data.as_bytes().as_ref())
        .expect("encryption failure!");    
    assert_ne!(&data.as_bytes(), &ciphertext);

    let message = MessageAES {iv: rnd, key_aes: key, cipher_hash: ciphertext};
    println!("iv1 = {:?}\n\nkey1 = {:?}\n\nciphertext1 = {:?}\n\n", message.iv, message.key_aes, message.cipher_hash);
    
    return message;
}

fn aes_gcm_decrypt(iv: [u8; 12], key: [u8; 32], cipherdata: Vec<u8>) -> Vec<u8> {
  
    // get secret_key aes_gcm
    let secret_key = Key::from_slice(&key);
    let cipher = Aes256Gcm::new(secret_key);    
    // get iv
    let nonce = Nonce::from_slice(&iv);

    // decrypt
    let plaintext: Vec<u8> = cipher.decrypt(nonce, cipherdata.as_ref())
    .expect("decryption failure!");
    /* let plaintext = cipher.decrypt(&nonce, encrypted_message.cipherText.as_ref())
    .expect("decryption failure!"); */
    assert_ne!(&plaintext, &cipherdata);

    let hashed = String::from_utf8_lossy(&plaintext);
    println!("clear hash = {}", hashed); 

    return plaintext;
}



fn main() {

    let message = aes_gcm_encrypt();
    //println!("this message:\n\niv2 = {:?}\n\nkeyaes2 = {:?}\n\ncipher2 = {:?}\n\n", message.iv, message.key_aes, message.cipher_hash);

    let plaint  = aes_gcm_decrypt(message.iv, message.key_aes, message.cipher_hash);
    println!("\ndecode {:?}\n", plaint);
    
}