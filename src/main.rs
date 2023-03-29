use std::time::Instant;

fn main() {
    loop {
        let start = Instant::now();
        let _resp = reqwest::blocking::get("https://example.com");
        let duration = start.elapsed();
        println!("{:?}", duration);
        match _resp {
            Ok(_resp) => {
                continue;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
