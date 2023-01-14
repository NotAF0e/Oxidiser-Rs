// use egui::*;
use std::{ fs::File, io::Read, str };
use lz4_flex::{ compress_prepend_size, decompress_size_prepended };

fn main() {
    if std::fs::metadata("test.oxid").is_ok() {
        let mut oxidised = File::open("test.oxid").unwrap_or_else(|_e|
            panic!("nurseryrhyme.txt not found!")
        );
        let mut oxidised_content = Vec::new();
        oxidised.read_to_end(&mut oxidised_content).expect("Cannot read file");

        let unoxidised = decompress_size_prepended(&oxidised_content).unwrap();

        println!("{}", str::from_utf8(&unoxidised).unwrap());

        // Saves file
        // std::fs::write("uncrompressed.txt", &mut unoxidised).expect("Unable to save file");
    } else {
        // Opens file
        let mut f = File::open("nurseryrhyme.txt").unwrap_or_else(|_e|
            panic!("nurseryrhyme.txt not found!")
        );
        let mut f_content = String::new();
        f.read_to_string(&mut f_content).expect("Cannot read file");

        // Turns file to bytes
        let f_content_bytes = f_content.as_bytes();

        // Compresses file
        let mut oxidised = compress_prepend_size(f_content_bytes);

        // Saves file
        std::fs::write("test.oxid", &mut oxidised).expect("Unable to save file");
        println!(
            "Compressed file by {}%",
            ((std::fs::metadata("test.oxid").unwrap().len() as f32) /
                (std::fs::metadata("nurseryrhyme.txt").unwrap().len() as f32)) *
                100.0
        )
    }
}