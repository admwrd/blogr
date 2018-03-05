# Rust Snippets

## Dotenv
Cargo.toml:
`dotenv = "0.10.1"`

// Load environmental variables from the .env file
extern crate dotenv;
use dotenv::dotenv;

dotenv().ok();

## Testing
To show println output: cargo test -- --color always --nocapture
`cargo test -- --nocapture`
Or use: `set RUST_TEST_NOCAPTURE=1`
The tests dir treats all rust files in it as a test, you must still use the #[test]
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // ...
    }
    
    #[test]
    #[should_panic]
    // or use:
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn it_works2() {
    }
    #[test]
    #[ignore]
    // run ignored tests with cargo test -- --ignored
    fn it_works2() {
    }
}
```
#### Run Single Test
test single item by specifing its name like:
`cargo test test_a`

#### Find And Run Specific Test
or specify part of the name and it will match part of the test name
`cargo test test_`

#### Test Concurrency
Setting Testing Concurrency (defaults number of threads to number of coress)
`set RUST_TEST_THREADS=1`
`cargo test -- --test-threads=1 `


## Benchmarking
Benchmarks are similar to tests, but use the #[bench] attribute, and also used a Bencher which is used by calling an iterator on it:
```rust
#![feature(test)]

pub fn do_something() -> u32 {
    // ...
}
pub fn do_something_else() -> u64 {
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn bench_one(b: &mut Bencher) {
        b.iter(|| do_something())
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        b.iter(|| do_something_else())
    }
}
```

## Numbers
```rust
use std::f32;
use std::f64;

let largest: u64 = 0xfffffffffffff;
let zero: u64 = 0x10000000000000;
let num = 1_000_000u64;

let flt = 123.4f64;     // Double-like
let fp2 = 0.1f32;       // Float-like
let fp3 = 12E+99_f64;   //Exponents
let fp4 = 1e0;          // 1.0
let fp5 = 1.3e-1;       // 
let fp = 123e+10;
let fp = 1.0e-40_32;

let lowest = f32::MIN_POSITIVE;
let max = f32::MAX;

format!("", 1.0) == "";
// https://github.com/rust-lang/rust/issues/10843
format!("{:e}", 1.0) == "1e0";
format!("{:e}", 10.0) == "1e1";
format!("{:+e}", 10.0) == "+1e+1";
format!("{:e}", 1.1234567) == "1.123457e0";
format!("{:e}", 1.3e-1) == "1.3e-1";
format!("{:.2e}", 1.3e-1) == "1.30e-1";
format!("{:E}", 1.3e-1) == "1.3E-1";

```

#### Number String Conversions
```rust
let str_to_int = u64::from_str_radix(&"10", 10).expect("Not an integer");
let int_string = "1000000".to_string();
let parse_int: u64 = "1000000".parse().unwrap();
let parse_int: u64 = "1000000".parse::<u64>().unwrap(); // Turbo fish syntax ::<>()
let padded = format!("{:08.2}", 1000.1234); // 00001000.12
let pad_left = format!("{txt:=<width$}", txt="text", width=20); // left padded to a variable place with = as padding
let pad_right = format!("{:=>7}", "text"); // right padded to 7 places with = as padding
let justified = format!("{:=^width$}", "text", 12); // centered with = on both sides
```

#### Random Numbers
```rust
extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::range::SampleRange;
use num::{Num, Zero, One};
use std::ops::Add;

// Safely choose a random number
// Use the num crate to add one to the generic number
pub fn safe_range<N>(starting: N, ending: N) -> N where N: Num 6 PartialOrd + Copy + SampleRange {
    let end_plus = ending + N::one();
    if start < end {
        let mut rg = thread_rng();
        // gen_range is [start, end) meaning it returns numbers
        //      as low as start and up to one less than end
        rg.gen_range(starting, end_plus); 
    }
}

// Most basic random number using modulus operator
let rand_num: u8 = (rand::random() % 100) + 1 // random number 1-100
let rand_num = (rand::random::<u8>() % 100) + 1 // without +1 it would give 0-99

// Choose a random item
let mut rg = thread_rng();
let gcat = rg.choose(&items);

// Shuffling
let mut rg = thread_rng();
let mut items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
rg.shuffle(&mut items);
```

## Strings
#### Trim Strings
```rust
"  text  ".trim()
"    text".trim_left()
"text    ".trim_right()
```

#### Bytes And Raw Literals
```rust
// literals
let byte_data = b"This is a string of bytes";
let raw_string = r##"This "can" have weird \"characters" in it 'no' problem"##;
let raw_byte = br##"you can even mix and match"##;

// Convert to bytes
let data = "abcdefghijklmnopqrstuvwxyz";
let data_ref = data.as_bytes();
let data_owned = data.into_bytes();
```

## Environment
#### Program Arguments
```rust
use std::env;
let args: Vec<String> = env::args().collect();
```

#### Logging
```rust
// Run with RUST_LOG=info or RUST_LOG=traitstr=info or RUST_LOG=main=info
// Types are: trace, debug, info, warn, error
#[macro_use] extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init();
}
```

#### Argument Parsing
Using the argparse crate.  I find it to be very nice and easy to use.
```rust
extern crate argparse;
use argparse::{ArgumentParser, StoreFalse, StoreTrue, Store};
let mut vart = false;
let mut varf = true;
let mut vars = String::from("");
{
    let mut ap = ArgumentParser::new();
    ap.set_description("Program description");
    ap.refer(&mut vart).add_option(&["-t", "--true"], StoreTrue, "description");
    ap.refer(&mut varf).add_option(&["-f", "--false"], StoreFalse, "description");
    ap.refer(&mut vars).add_option(&["-s", "--store"], Store, "description");
    
    ap.parse_args_or_exit();
}
```

## Time
#### Timing/Benchmarking
```rust
extern crate time;
use std::time::Instant;

let start = Instant::now();

let end = start.elapsed();
println!("Processed in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
```

## Regular Expressions
```rust
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"^[A-Za-z0-9]$"#).unwrap();
}
```

## Serialization/Deserialization
#### Serialization
```rust
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, Read};
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};
use serde_json::Error;

// replace path & data with file and data structure to serialize
let mut f = File::create(path).expect("File create failed.");

// YAML
let ser = ::serde_yaml::to_string(data).expect("Could not serialize");
let ser = ::serde_yaml::to_vec(data).expect("Could not serialize");

// JSON
let ser = ::serde_json::to_string_pretty(data).expect("Could not serialize");
let ser = ::serde_json::to_string(data).expect("Could not serialize");
let ser = ::serde_json::to_vec_pretty(data).expect("Could not serialize");
let ser = ::serde_json::to_vec(data).expect("Could not serialize");

// MSGPACK
let mut ser: Vec<u8> = Vec::new();
data.serialize(&mut Serializer::new(&mut ser_buf)).expect("Could not serialize");
let mut ser: String = String::new();

// write string to file
let rst = f.write(ser.as_bytes());
// write vector to file
let rst = f.write(&ser);
// Result
if let Ok(res) = rst { if res != 0 { true } else { false } } else { false }
```

#### Deserialization
```rust
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rmp_serde as rmps;
extern crate serde_yaml;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, Read};
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};
use serde_json::Error;

let mut open = File::open(path).expect("Could not open file");

// Read as string
let mut des_buf: String = String::new();
f.read_to_string(&mut des_buf);
// Read as vector
let mut des_buf: Vec<u8> = Vec::new();
f.read_to_end(&mut des_buf);

// YAML
let des: Test = ::serde_yaml::from_str(&des_buf).expect("Could not deserialize");
let des: Test = ::serde_yaml::from_slice(&des_buf).expect("Could not deserialize");

// JSON
let des: Test = ::serde_json::from_slice(&mut des_buf).expect("Could not deserialize");
let des: Test = ::serde_json::from_str(&mut des_buf).expect("Could not deserialize");

// MSGPACK
// for sure works:
let mut de = Deserializer::new(&des_buf[..]);
let des: Test = Deserialize::deserialize(&mut de).expect("Could not deserialize");
// not tested:
let des: Test = Deserialize::deserialize(&mut Deserialize::deserialize(&des_buf[..])).expect("Could not deserialize");

```

## Files And IO
#### Writing Files
```rust
let mut f = BufWrtier::new(File::create(filename).expect("Could not create file"));

let text = include_bytes!("filename.txt");
f.write(text);

let text2 = include_str!("filename.txt");
f.write(text2.as_bytes());

let data = vec![b"this will be represented as a vector of bytes"];
f.write(&data[..]);
```

#### Reading Files
```rust
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::prelude::*;

// Buffered file reading
let mut f = BufReader::new(File::open("filename.txt").expect("Could not open file"));
for line in f.lines() {
    match file_line {
        Err(e) => println!("Error: {}", e);
        Ok(line) => {
            // ...
        }
    }
}
```

#### Console Input
```rust
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::prelude::*;
use std::io::{self, stdin, Read};

let sin = io::stdin();
let mut buf = String::new();
let mut name: String = String::new();
println!("Enter something");
match sin.read_line(&mut buf) {
    Ok(_) => {
        name = buf.trim().to_string();
        buf.clear();
    },
}
```

## Running Commands
```rust
use std::process::{Command, Output, ExitStatus};

let cmd = Command::new("passgen")
    .args(&["-e", "-c", "-l 10", "-r 20"])
    .output()
    .expect("Failed to execute command");
```


## Attributes
```rust
if cfg!(target_os = "windows") { /* ... */ }
#[cfg(target_os = "linux")] // can be linux android windows macos ios
#[cfg(target_pointer_width = 64)] // target 64 bit systems
#[cfg(target_pointer_width = 32)] // target 32 bit systems

// Compiler features
#![feature(feature1, feature2, feature3)]
#[cfg(feature = "foo")]

// combining multiple conditions
#[cfg(any(unix, windows))]
#[cfg(not(macos))]
#[cfg(not(unix), all(target_os="macos", target_arch = "powerpc"))]

#![plugin(foo, bar)]

// Load a module from a specified file
#[path = "foo.rs"]
mod bar;

// Set crate type to library
#![crate_type = "lib"]

// You can set attributes based on a cfg variable
#[cfg_attr("is_cool", windows)] // sets the is_cool attribute if compiling for windows
```

## Function Pointers
```rust
fn call_fn<F>(func: F, data: usize) -> bool where F: Fn(usize) -> bool {
    func(data)
}
let is_even = |n| n%2==0;
assert!(call_fn(is_even, 10));
```

## Mutable Global Variables
> Warning: this is bad.  I only am showing this as an example of what is possible, not what should be used.  I found it to be a neat example of unsafe.  But it will crash your program is you're not careful.  You are warned.

```rust
use std::mem;

// Put these in main.rs/lib.rs inside the main() to ensure they exist for entire program
static mut MY_GLOBAL: *const Vec<String> = 0 as *const Vec<String>;
fn main() {
    let mut initialized_my_global: Vec<String> = Vec::new();
    unsafe {
        MY_GLOBAL = mem::transmute(&initialized_my_global);
    }
}

// Elsewhere in the program
let mut my_global: &mut Vec<String>;
unsafe {
    my_global = mem::transmute(MY_GLOBAL);
}
```

