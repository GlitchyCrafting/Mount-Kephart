# WDHANY
A learning platform used to teach students HTML (and maybe CSS in the future).

## Binaries
Both Linux and Windows Binaries will be provided.

## Build
1. As it is written in Rust, just install rustup:
  - Linux/MacOS `curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`
  - Windows [Instructions](https://www.rust-lang.org/tools/install)
2. Then Build and Run
  ```sh
    git clone https://github.com/GlitchyCrafting/WDHANY.git
    cd WDHANY
    cargo build --release
    cd target/release
    ./wdhany
    # ./wdhany.exe on Windows
  ```

## The Platform
I created the platform to have the verbosity of W3Schools in the lessons, but in the format of FreeCodeCamp. The UI is built to be minimal using readable fonts, no more "Is this an l, or an I?", and a color scheme that's easy on the eyes. It has an online editor so nothing needs to be installed on the user's computer. It also has a live preview to allow for visualization of what code does what. No account creation is needed as everything is stored in a single cookie.

## Technical Information
The server is written in [Rust](https://www.rust-lang.org), all the crates used are in [Cargo.toml](Cargo.toml). The online editor is the [Ace Library](https://ace.c9.io/) written in Javascript. The UI uses the [Dracula Color Scheme](https://draculatheme.com/). There is a live preview that uses Javascript Blobs to get code from the editor and display it in an IFrame. Being written in Rust, it is fully cross-platform to Windows, Mac, and linux.
