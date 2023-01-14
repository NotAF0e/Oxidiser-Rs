// use egui::*;
// use magic_crypt::{ new_magic_crypt, MagicCryptTrait };
use std::{ fs::File, io::Read, str };
use lz4_flex::{ compress_prepend_size, decompress_size_prepended };

fn main() {
    //let mc = new_magic_crypt!("magickey", 128);
    //let base64 = mc.encrypt_str_to_base64(&file_contents);

    //println!("{}", base64);
    //println!("{}", mc.decrypt_base64_to_string(&base64).unwrap());

    // Opens file
    let mut file = File::open("nurseryrhyme.txt").unwrap_or_else(|_e|
        panic!("nurseryrhyme.txt not found!")
    );
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Cannot read file");

    // Turns file to bytes
    let file_content_bytes = file_content.as_bytes();
    
    // Compresses file
    let mut compressed = compress_prepend_size(file_content_bytes);

    // Saves file
    std::fs::write("test.oxid", &mut compressed).expect("Unable to save file");

    let uncompressed = decompress_size_prepended(&compressed).unwrap();

    println!("{}", str::from_utf8(&uncompressed).unwrap());
}