extern crate rpassword;

use rpassword::read_password;

use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::thread;

use std::time::Instant;
use sha1::{Digest, Sha1};
use std::str;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    let path: &str = &std::env::var("PATH_TO_PWND_FILE").expect("PATH_TO_PWND_FILE must be set!");
    let length: usize = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len to usize");

    const BLOCK_SIZE: usize = 128*1024*1024;
    const THREADS: usize = 16;


    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, &password);
    let result = hasher.finalize_reset();
    let hex = &result.iter().map(|b| format!("{:02x}", b)).collect::<String>().to_ascii_uppercase();

    println!("{}", length);
    println!("Password Hash: {}", hex);
    println!("================================\n");

    let division: usize = ((length / THREADS) as f64).ceil() as usize;

    thread::scope(|scope| {
        for i in 0..THREADS {
            scope.spawn(move || {
                let start = Instant::now();
                println!("Start Thread: {}", i);
               let mut thread_file = File::open(&path).expect("Can't open file");
                let mut contents = vec![0_u8; BLOCK_SIZE];

                let mut read_length: usize = 1;
                let mut read_total: usize = 0;
                let offset: u64 = (i * division) as u64;

                thread_file
                    .seek(SeekFrom::Start(offset))
                    .expect("Couldn't seek to position in file");

                while (read_total < division) && (read_length != 0) {
                    if read_total + BLOCK_SIZE > division {
                        contents.truncate(division - read_total);
                    }
                    read_length = thread_file.read(&mut contents).expect("Couldn't read file");
                    let string_contents = unsafe { str::from_utf8_unchecked(&contents) };
                    string_contents.split("\n").for_each(|line| {
                        if line.contains(":") {
                            let splitted = line.split(":").collect::<Vec<&str>>();
                            // println!("{}", splitted[0]);
                            if hex == splitted[0] {
                                println!("\x1b[41m\x1b[93m!!ATTENTION!!\x1b[0m: Password found! Count: {}", splitted[1]);
                            }
                        }
                    });
                    read_total += read_length;
                }

                let duration = start.elapsed();

                println!("Thread {} finished in \x1b[92m{:.2?}s\x1b[0m", i, duration.as_secs());
            });
        }
    })
}