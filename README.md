### Introduction
Welcome to kii_ware()! At its heart, this is a simple toy open-source hardware emulator. Inspired by games like Shenzhen I/O and the experience of breadboarding contraptions, the goal of kii_ware() is to create a similar experience without the entry cost of traditional hardware tinkering and a simple core that makes designing and getting new parts as simple as a quick copy and paste!
### How it works
Every bit of kiiware is simply a collection of "components" that are connected via a "board". Just like their real life counterpart, a component knows nothing about the state of the system as a whole and only knows the state of its pins. A board, similar to a breadboard, is responsible for connecting the components together!
### Installation
First thing's first, you'll need to have Rust installed in order for any of this to work! If you don't have Rust installed already `https://www.rust-lang.org/tools/install` will walk you through everything! With Rust installed there are two main ways to install kii_ware():
#### git:
  The simplest way to get started assuming you have `git` installed, run `git clone https://github.com/kiinyo/kii_ware` in your terminal. 
#### Manual Download:
  If you don't have `git` installed, you can always click the green `Code` button, choose the Download ZIP button option and unzip it whever you'd like!
#### Getting started
With your 

#### Adding components and boards

#### Creating boards
Boards are rather straight forward to create but do require a bit of of programming knowledge to do. The example boards are written to be pretty understandable so if you'd like to just figure out how it works from looking over the source code! However for the rest of us mortals, in my experience, the best way to learn is by doing so let's create a demo board!

To start off, we'll need need to create a new `board_name.rs` file in the `board` folder. Inside that file we can write
```rust
use crate::boards::{DataType, Connections, Components};

pub fn run () -> ( Components, Connections, DataType ) {
  let components = vec![];
  
  let connections = vec![];
  
  let data_type = DataType::U8;
  
  return ( components, connections, data_type );
}
```
