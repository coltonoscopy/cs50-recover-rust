# CS50's Recover in Rust

Created a version of CS50's `recover` problem set (normally in C) in Rust as an exercise
in learning the language.

## Project Specification
https://cs50.harvard.edu/x/2020/psets/4/recover/

## Installing Rust
https://www.rust-lang.org/tools/install

## Project Installation
```
git clone https://github.com/coltonoscopy/cs50-recover-rust.git
cd cs50-recover-rust
wget https://cdn.cs50.net/2019/fall/psets/4/recover/recover.zip
unzip recover.zip
cargo run -- recover/card.raw
```

If you don't have `wget` installed, simply downloading and unzipping the distribution of `card.raw` to
the project (or elsewhere) and specifying the path correctly to `cargo run` should suffice :)