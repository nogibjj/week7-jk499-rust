[![Clippy](https://github.com/nogibjj/week7-jk499-rust/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week7-jk499-rust/actions/workflows/lint.yml)
[![Build binary release](https://github.com/nogibjj/week7-jk499-rust/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week7-jk499-rust/actions/workflows/release.yml)

# Count number of vowles and consonents.

**Developer:** Jaya Khan 

## Abstract
We first initialise two variables, vowels and consonants, to 0. We then use a loop to repeatedly read input from the user until a non-alphabetic character is encountered. For each character in the input, we check if it is alphabetic. If it is not, we break out of the loop. If it is, we check if it is a vowel or a consonant, and increment the appropriate counter. Finally, we print out the number of vowels and consonants that were counted.

## Requirements
The code was tested on:
- rust edition = 2021


## Project Structure
                                                                               
    vowel-consonent
    ├── Cargo.toml
    ├── README.md   
    ├── src
        ├── main.rs
    ├── .github
        ├── workflows
            ├── lint.yml
            ├── release.yml
            ├── rustfmt.yml
            ├── tests.yml
    ├── Makefile


## Commands to install Rust
1. sudo apt install curl
2. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3. source $HOME/.cargo/env

    ### Check installation:
    rustc --version

    ### Command to uninstall rust
    rustup self uninstall

## Command to build project (CI/CD): 
`make all`