### Introduction
Welcome to kii_ware()! At its heart, this is a simple toy open-source hardware emulator. Inspired by games like Shenzhen I/O and the experience of breadboarding contraptions, the goal of kii_ware() is to create a similar experience without the entry cost of traditional hardware tinkering and a simple core that makes designing and getting new parts as simple as a quick copy and paste!
### How it works
Every bit of kiiware is simply a collection of "components" that are connected via a "board". Just like their real life counterpart, a component knows nothing about the state of the system as a whole and only knows the state of its pins. A board, similar to a breadboard, is the way in which you connect components together!
### Installation
First thing's first, you'll need to have Rust installed in order for any of this to work! If you don't have Rust installed already `https://www.rust-lang.org/tools/install` will walk you through everything! With Rust installed there are two main ways to install kii_ware():
git:
  - The simplest way to get started assuming you have `git` installed, run `git clone https://github.com/kiinyo/kii_ware` in your terminal. 
Manual Download:
  - If you don't have `git` installed, you can always click the green `Code` button, choose the Download ZIP button option and unzip it whever you'd like!
### Getting started
As mentioned earlier, to get started assembling kiiware all you need to do is gather components and connect them with a board! Adding these is as simple as dragging new `component_name.rs` and `board_name.rs` files to the `components` and `boards` folders respectively! Then open a terminal of your choice, navigate (`cd`) to your `/kiiware` folder and run `cargo run -- install`. To remove anything, simply remove them from the folder and run `cargo run -- update` again!

Finally to run your kiiware simply run `cargo run`, select the board, and watch it go!
