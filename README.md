# Mount Kephart
A learning platform used to teach students HTML (and maybe CSS in the future).

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com) 
![open-source](https://user-images.githubusercontent.com/74939524/206866352-368c4845-8e07-41d9-b65e-e275cca17af2.svg)
![works-on-my-machine](https://user-images.githubusercontent.com/74939524/206866410-61df2174-7549-49d2-9c27-785710e36e23.svg)
![gpl-v3+](https://user-images.githubusercontent.com/74939524/206866473-d05866de-1fff-498c-866d-5373c2f23c8f.svg)

[![Clippy & Tests](https://github.com/GlitchyCrafting/Mount-Kephart/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/GlitchyCrafting/Mount-Kephart/actions/workflows/rust-clippy.yml)

## Binaries
Both Linux and Windows Binaries will be provided.

## Build
Note: I have optimized binary size at the cost of compile speed, but not runtime speed. For faster compile times at the cost of binary size, remove the last 4 lines in [Cargo.toml](Cargo.toml).
1. As it is written in Rust, you need rustup:
  - Linux/MacOS `curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`
  - Windows [Instructions](https://www.rust-lang.org/tools/install)
2. Run the script for your platform (Linux.sh or Windows.ps1).
3. Move the resulting `Mount Kephart` directory to where you want to install it.

## The Platform
I created the platform to have the verbosity of W3Schools in the lessons, but in the format of FreeCodeCamp. The UI is built to be minimal using readable fonts, no more "Is this an l, or an I?", and a color scheme that's easy on the eyes. It has an online editor so nothing needs to be installed on the user's computer. It also has a live preview to allow for visualization of what code does what. No account creation is needed as everything is stored in a single cookie.

## Technical Information
The server is written in [Rust](https://www.rust-lang.org), all the crates used are in [Cargo.toml](Cargo.toml). The online editor is the [Ace Library](https://ace.c9.io/) written in Javascript. The UI uses the [Dracula Color Scheme](https://draculatheme.com/). There is a live preview that uses Javascript Blobs to get code from the editor and display it in an IFrame. Being written in Rust, it is fully cross-platform to Windows, Mac, and linux.
