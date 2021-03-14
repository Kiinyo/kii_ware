### Introduction
Welcome to kii_ware()! At its heart, this is a simple toy open-source hardware emulator. Inspired by games like Shenzhen I/O and the experience of breadboarding contraptions, the goal of kii_ware() is to create a similar experience without the entry cost of traditional hardware tinkering and a simple core that makes designing and getting new parts as simple as a quick copy and paste!
### How it works
Every bit of kiiware is simply a collection of "components" that are connected via a "board". Just like their real life counterpart, a component knows nothing about the state of the system as a whole and only knows the state of its pins. A board, similar to a breadboard, is responsible for connecting the components together!
### Installation
First thing's first, you'll need to have Rust installed in order for any of this to work! If you don't have Rust installed already, let me be the first to welcome you! The community is amazing and the official Discord will help you with anything you could possibly need! `https://www.rust-lang.org/tools/install` will walk you through everything you need to install Rust. With Rust installed there are two main ways to install kii_ware():
#### git:
  The simplest way to get started assuming you have `git` installed, run `git clone https://github.com/kiinyo/kii_ware` in your terminal. 
#### Manual Download:
  If you don't have `git` installed, you can always click the green `Code` button, choose the Download ZIP button option and unzip it whever you'd like!
### Getting started
kii_ware() comes bundled with a few kiits to get you started, to run them simply open a terminal emulator of your choice (I recommend Alacritty) and navigate to your `/kii_ware` folder. Once there, enter `cargo run`, select a board, and watch it go!
### Adding and removing components and boards
Since all components are functions written in Rust, we'll need to actually edit some code to get it all to run natively!
   - Step 1: Place the `component_name.rs` that you'd like to add into the `kiiware/components` folders.
   - Step 2: Open the `kiiware/components/mod.rs` file in a text editor of your choice.
   - Step 3: At the top of the file add the line `pub mod component_name`.
   - Step 4: `Ctrl + F` or `/` depending on your editor to search for `// USER COMPONENTS: Add your component here`.
   - Step 5: Make a new line underneath and write the following `"Component Name" => component_name::run,` with `"Component Name"` being the name you wish to refer to the component when connecting them via boards and `component_name` being the same as the `component_name.rs` file.
   - Step 6: Save the file and you're done!
 
 To remove components, simply erase the lines you just added. To add/remove boards you just need to do the same in the `boards` folder!
### Creating boards
Boards are rather straight forward to create but do require a bit of of programming knowledge to do. The example boards are written to be pretty understandable so if you'd like to just figure out how it works from looking over the source code! However for the rest of us mortals, in my experience, the best way to learn is by doing so let's create a demo board!

To start off, we'll need need to create a new `demo_board.rs` file in the `board` folder. Inside that file we can write the following:
```rust
use Board;

pub fn create () -> Board {

  let components = vec![
    String::from("Countdown Timer"),
    String::from("Console Display"),
    String::from("Console Display"),
    String::from("Shutdown Trigger")
  ];
  
}
```
The first line of code tells Rust we'll be `use`ing `Board` from the `kii_ware/src/main.rs` file so that the bit of kii_ware() that emulates the hardware can be sure it has the right type!

We then declare our `create` function and tell Rust it's going to return `->` a `Board`

Next we define our components! We `let components =` a `vec!` of `String`s that include the names of the names we assigned in the `Adding components and boards` section. In this case we we have two `"Console Display"`s mainly just to show that we can.

That's great and all but the components have no way to communicate! Let's see about making a few `connections`! Let's look at the documentation for at the top of the `countdown_timer.rs` file!
```rust
// countdown_timer.rs

// Data Type: u8
// Pin(s): 2
// Registers: 1
// Description: When initialized, sets pin[0] to '10' and subtracts one from this value every second.
// At '3', sends '100' to pin[1];
// At '0', sends '255' to pin[1];

// console_display.rs

// Data Type: u8
// Pin(s): 1
// Registers: 1
// Description: When pin[0] is changed to 0..10 prints "Countdown is at: [number]"
// If pin[0] is set to 100, prints "The system will shut down in 3 seconds!"

// shutdown_trigger.rs

// Data Type: u8
// Pin(s): 1
// Registers: 1
// Description: When pin[0] is set to 255, sends a signal to kii_ware() to shut down.
```
