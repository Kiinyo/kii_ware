### Creating boards
Boards are rather straight forward to create but do require a bit of of Rust/Programming knowledge to make. The example boards are designed to be really readable and understandable so if you'd like to learn by using the source code you totally can! However for the rest of us mortals, in my experience, the best way to learn is by doing so let's create a demo board!

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
The first line of code tells Rust we'll be `use`ing `Board` from the `kii_ware/src/main.rs` file so kii_ware() knows we're giving it a properly formatted board!

We then declare our public function (`pub fn`_ called `create` that doesn't accept any arguments `()` and returns `->` a `Board`. Public function just means that bits of Rust outside of this file can also use this function.

Next we define our components! Inside the function `{...}`, we `let components =` a `vec![...]` of `String`s that we got `from` the names of the components we assigned in the `Adding components and boards` section. A `vec![...]` is simply an array with a length that might change, in Rust we call these `Vector`s. 

Also important to note is that in this case we have two `"Console Display"`s! If we've installed a component previously, we can use as many copies as we want! How's that for convenient?

Unfortunately as things stand right now, the components have no way to communicate! Let's see about making a `bus` with a few `connection`s! Let's look at the documentation for at the top of the corresponding `component.rs` file!
```rust
// countdown_timer.rs

// Data Type: u8
// Pin(s): 2
// Registers: 1
// Description: When initialized, sets pin[0] to '10' and subtracts one from this value every second.
//  At '3', sends '100' to pin[1];
//  At '0', sends '255' to pin[1];

// console_display.rs

// Data Type: u8
// Pin(s): 1
// Registers: 1
// Description: When pin[0] is changed to a value between `0` and `10` prints "Countdown is at: [number]"
//  If pin[0] is set to 100, prints "The system will shut down in 3 seconds!"

// shutdown_trigger.rs

// Data Type: u8
// Pin(s): 1
// Registers: 1
// Description: When pin[0] is set to 255, sends a signal to kii_ware() to shut down.
```
One thing that's important to note is that lists in /most programming languages/ start with the number 0. This means that for a list of 5 things, the index positions would look like `[0, 1, 2, 3, 4]`. With that in mind, looks like if we connect `pin[0]` of the `"Countdown Timer"` to `pin[0]` of our first `"Console Display"` we should get a countdown timer. Then if we connect `pin[1]` to the pins for the other `"Console Display"` and `"Shutdown Trigger"` we should get a 3 second warning before the kii_ware() ends its simulation so let's do that! A connection is a `vec`tor holding a list of `[component_index, pin_number]`s it's connecting. The `bus` is simply a `vec` that holds all of the connections!
```rust
let bus = vec![
  // First connection
  vec![
    [0,0], [1,0]
  ],
  // Second Connection
  vec![
    [0,1], [2,0], [3,0]
  ]
];
```
One thing to keep in mind is that your board can only support one `Data Type`. In this case all the components were `u8` but this might not always be the case! Make sure all the components you want to use have the same `Data Type` or the program won't compile!

With that disclaimer out of the way we're almost done, all we need to do is actually return the values and we'll be done, all together your board should look like:
```rust
use Board;

pub fn create () -> Board {

  let components = vec![
    String::from("Countdown Timer"),
    String::from("Console Display"),
    String::from("Console Display"),
    String::from("Shutdown Trigger")
  ];
  
  let (cdtm, dis1, dis2, sdtg) = (0, 1, 2, 3);
	
  let bus = vec![
    vec![
      [cdtm,0], [dis1,0]
    ],
    vec![
      [cdtm,1], [dis2,0], [sdtg,0]
    ]
  ];
  
  return (components, bus);
}
```
Well almost like this! While it isn't that bad for 3 or 4 components, having to use the index number for every component can become a nightmare after the 10th or 20th one so I've gone ahead and assigned the index numbers names to make them easier to keep track of! It's all personal perference in the end though!

With that our board is done! Make sure to save your `board_name.rs`, add it to the `mod.rs` file as explained in `Adding and removing components and boards` and enjoy!
