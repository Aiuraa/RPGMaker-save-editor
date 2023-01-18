
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let path = Path::new("normal-text.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Cannot open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("Cannot read {}: {}", display, why),
        Ok(_) => {
            let buf = lz_str::decompress_from_base64(&s).expect("cannot be decompressed!");
            let buf = String::from_utf16(&buf).expect("This is not valid UTF16");
            let out = jsonxf::pretty_print(&buf).unwrap();
            
            let another_path = Path::new("decrypted-text.json");

            let mut output = File::create(&another_path).expect("Cannot create file!");
            writeln!(output, "{}", out).expect("Cannot write file!");
        },
    };
}
