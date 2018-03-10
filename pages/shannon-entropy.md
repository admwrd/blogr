# Shannon Entropy

## What Is Entropy
In computer science terminology entropy measures the randomness of data.  Higher entropy means greater randomness or disorder.  A file with a high entropy might contain, for example, all of the possible combinations of a byte (0-255), and each symbol would be repeat a relatively equal number of times.

In contrast an example of a file with low entropy might contain only English text (or for very low entropy just numbers), with some of those numbers being repeated a disproportional number of times (maybe the numbers 1, 2, and 3 are repeated much more often than the others).

Essentially the more predictable the data is the lower the entropy, and the less predictable (more random) the higher the entropy.

## Entropy And Compression
Finding the entropy of a file can be very useful for determining whether or not to cmopress files.  If you have a bunch of files with high entropy you may not want to compress them, maybe just an uncompressed .tar archive would suffice.

Finding the entropy of a file will not tell you exactly how well a particular comrpession algorithm will perform on a given dataset but instead tell you how well the average compression alogirthm may perform.

## Shannon Entropy Formula
<!-- Original Markdown Forumula  -\sum_{k=1}^{n} p_k log_2 p_k  -->
<!-- jqMath formula  $$∑↙{k=0}↖n p_k log_2 p_k$$  -->

$$∑↙{k=0}↖n p_k log_2 p_k$$

Where _**n**_ is the number of symbols (or length of the file), _**p**_ is the probability of the symbol _**k**_.

Oh you don't know how to speak greek?  The algorithm is actually fairly simple.  Find the probability of each symbol, for each unique symbol multiply the probability by log2( probability ) and subtract all of the the multiplied values.

## Algorithm
In a computer program the process would look something like:
1. ##### Count Symbol Occurences
    1. Initialize a mutable array named `histogram` with 256 unsigned integer elements with each element having a value of 0.
    2. Create a mutable variable, `t` with a value of 0
    2. Iterate each byte of the file, counting the number of times that particular byte appears.  Also increment `t` each iteration.
        - 0 maps to the first array element while 255 is the last element in the array
        - At the end of the file (or dataset) `t` should equal the total size of the file, since it is the total number of 

2. ##### Calculate The Entropy
    3. Create a mutable floating point variable `entropy`
    4. For each element `i` of the `histogram` array (0-255):
        1. Create a variable `probability`
            1. The value for `probability` is found by dividing the value of `histogram[i]` by `t` (the total size of the file or length of data)
        2. Subtract the result of the following from `entropy`:
            1. `probability` &times; log2(`probability`)
    3. The above steps will yield a result that is a deciamal number between 0 and 8.  This is because there are 8 bits in each symbol.  To display the entropy as a decimal between 0 and 1 (my preference) divide the result by 8.


## Shannon Entropy In Rust
Now onto some code.  The following program was one of my first programs in Rust and as such it is not overly sophisticated, but it is fairly simple to understand.

#### Rust Code

```rust

use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

// Allow the precision to be easily changed
type EntropyType = f32;

pub fn find_entropy(path: &Path) -> io::Result<EntropyType> {
    // I'm not sure if BufReader actually 
    // buffers anything for the bytes() method
    // but it can't hurt to use it in case
    let mut f = BufReader::new(File::open(path)?);
    
    // Number of bytes procsesed, should end up
    // with the size of the file for its value
    let mut total = 0u32;
    // Histogram tracking the frequency of each byte/symbol
    let mut hist = [0; 256];
    
    for byte in f.bytes() {
        if let Ok(b) = byte {
             // Get a mutable reference to the
             // corresponding histogram element
             // then increment the element by one.
             // The get_mut(idx) method take an index
             // parameter and returns mutable
             // reference to the index element
            hist.get_mut(b as usize).map(|v| *v += 1);
            // Increment number of bytes processed
            total += 1;
        }
    }
    // Convert total to a float
    let tot: EntropyType = total as EntropyType;
    
    // Iterate through all possible combinations of a byte
    let result = (0..256).into_iter()
        // Use fold's accumulator to subtact all values from 0
        .fold(0.0, |acc, f| 
        // Don't waste time processing 0's
            if hist[f] > 0 { 
                // Find probability of symbol by
                // dividing symbol frequency by total
                let p = (hist[f] as EntropyType)/tot;
                // Calculate entropy by multiplying
                // probability by log2(probability).
                acc - ( p * p.log2() ) 
            } else { acc });
    Ok(result / 8.0)
}


fn main() {
    let ebotd: PathBuf = r#"D:\code\data\Datasets\Egyptian Book Of The Dead.doc"#.into();
    let ebotde = find_entropy(&ebotd);
    println!("Analyzing {}\n\tFound an entropy of: {:.3}", ebotd.display(), ebotde.unwrap());
}

```
