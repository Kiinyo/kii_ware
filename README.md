## kii_ware()
Welcome to kii_ware()! At its heart, this is a simple toy open-source hardware emulator. Inspired by games like Shenzhen I/O and the experience of breadboarding contraptions, the goal of kii_ware() is to create a similar experience without the entry cost of traditional hardware tinkering and a simple core that makes designing and getting new parts as simple as a quick copy and paste!
### How it works
Every bit of kiiware is simply a collection of "components" that are connected via a "board". Just like their real life counterpart, a component knows nothing about the state of the system as a whole and only knows the state of its pins. A board, similar to a breadboard, is responsible for connecting the components together!

For a more technical description we use Rust's Cargo to compile our kiit and leverage Rust's amazing ability to work with multiple threads to simulate each bit of hardware in its own thread and allow them to communicate over pins using `Arc<Mutex<T>>`. This was an intentional design choice so just like real hardware, you'll have to design your own syncronization solutions! If none of that made any sense, don't worry
### Installation
First thing's first, you'll need to have Rust installed in order for any of this to work! If you don't have Rust installed already, let me be the first to welcome you! The community is amazing and the official Discord will help you with anything you could possibly need! `https://www.rust-lang.org/tools/install` will walk you through everything you need to install Rust. With Rust installed there are two main ways to install kii_ware():
#### git:
  The simplest way to get started assuming you have `git` installed, run `git clone https://github.com/kiinyo/kii_ware` in your terminal. 
#### Manual Download:
  If you don't have `git` installed, you can always click the green `Code` button on the Github page, choose the Download ZIP button option and unzip it whever you'd like!
### Getting started
kii_ware() comes bundled with a few kiits to get you started, to run them simply open a terminal emulator of your choice (I recommend Alacritty) and navigate to your `/kii_ware` folder. Once there, enter `cargo run`, select a board, and watch it go!
### Adding and removing components and boards
Since all components are functions written in Rust, we'll need to actually edit some code to get it all to run natively!
   - Step 1: Place the `component_name.rs` that you'd like to add into the `kiiware/components` folders.
   - Step 2: Open the `kiiware/components/mod.rs` file in a text editor of your choice.
   - Step 3: At the top of the file add the line `pub mod component_name` with all the others.
   - Step 4: `Ctrl + F` or `/` depending on your editor to search for `// USER COMPONENTS: Add your component here`.
   - Step 5: Make a new line underneath the comment and write the following: `"Component Name" => component_name::run,` with `"Component Name"` being the name you wish to refer to the component when connecting them via boards and `component_name` being the same as the `component_name.rs` file.
   - Step 6: Save the file and you're done!
 
 To remove components, simply erase the lines you just added. To add/remove boards you just need to do the same in the `boards` folder but under `// USER BOARDS: Add your board here`, add `"Board Name" => board_name::create`.
### Creating your own components and boards!
If you want to move away from pre-packaged `kiits` and make your own components and boards, there are tutorials in the `boards` and `components` folders for each. Both require a bit of programming knowledge but I've tried my best to write the `Board Tutorial.md` and structure boards in such a way so that if all you want to do is connect components, it should be possible without really any experience in Rust or programming.

As for creating your own components, since they require all their logic to be written in Rust, some familiarity with programming in Rust will be required, but again I've done my best to make it as simple as possible to follow!
