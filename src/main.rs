mod des_implementation;

use des_implementation::DES;

fn main() {

    let key : u64 = 
    0b_00110001_00110010_00110011_00110100_00110101_00110110_00110111_00111000; 

    let test_plaintext_vector: u64 = 
    0b00110000_00110001_00110010_00110011_00110100_00110101_00110110_00110111;

    let mut cipher = DES::new(&key);
    println!("DES encrypt procedure");
    let ciphertext = cipher.encrypt_block(&test_plaintext_vector);
    println!("DES decrypt procedure");
    let decrypted = cipher.decrypt_block(&ciphertext);


    println!("Key = {:064b}", key);
    println!("Plaintext = {:064b}", test_plaintext_vector);
    println!("Ciphertext = {:064b}", ciphertext);
    println!("Decrypted ciphertext = {:064b}", decrypted);
}
