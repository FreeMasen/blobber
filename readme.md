# Blobber

Create an arbitrary length `String` or `Vec<u8>`.

## Documentation

[Generated API Documentation](https://docs.rs/blobber/0.1.5/blobber/)

## Usage

Adding to your project
```toml
[dependencies]
blobber = "0.1"
```

Using the crate
```rust
extern crate blobber;
use blobber::*;
use std::thread::sleep;

fn generate_strings() {
    let lorem = get_lorem(100, false);
    assert!(lorem == "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore");
    let prints = get_string(25, "print", false);
    assert!(prints = "printprintprintprintprint");
    let numbered_prints = get_string(40, "print", true);
    assert!(numbered_prints = "1. print2. print3. print4. print5. print");
}

fn generate_vecs() {
    let first = get_rng_blob(100);
    //this uses the unix epoch for a seed
    //we need to wait to get a new seed
    sleep(1000);
    let second = get_rng_blob(100);
    assert!(first != second);
    //This will always generate the same vec
    let seeded = get_seeded_rng_blob(100, 125);
    let again = get_seeded_rng_blob(100, 125);
    assert!(seeded == again);
    let custom = get_blob(25, &[1,2,3,4,5]);
    assert!(custom = vec![1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5]);
}
```

## Why
I am working on a longer term project to duplicate the functionality of the unix command `dd` that will display a progress string. This crate was developed to generate the file contents for testing. 

You might be interested in using it if you need to generate random data that is inspectable and/or reproducable.

## Contributing

If you are using this crate and have found a bug or want to see a feature, feel free to open an issue or PR. 