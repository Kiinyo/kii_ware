pub mod components;
pub mod boards;

use std::sync::{Arc, Mutex};

fn quiz() -> String {
    String::from("u8")
}

fn main () { 
    // Intro
    println!("Hello and welcome to kii_ware!");
    // Determine the type
    let result = quiz();
    // Get the board
    let board = match result.as_str() {
        "u8" => boards::get_u8(),
        _ => panic! ("What do?")
    };
    // Create the actual bus
    let mut bus = Vec::new();
    for _ in 0..board.bus.len() {
        bus.push(Arc::new(Mutex::new(board.default_value)));
    }
    // Create the components and connect them to the bus
    // as well as a container to hold all the threads.
    let mut threads = Vec::new();
    for c in 0..board.components.len() {
        // Create the component
        let mut component = match result.as_str() {
            "u8" => components::create_u8(board.components[c].clone()),
            _ => panic! ("What do?")
        };
        // Go through all the component's pins
        'pin_assignment: for p in 0..component.pin_number {
            // For each component's pin go through all of the
            // bus connections
            for bc in 0..bus.len() {
                // And in each one of the connections
                for y in 0..board.bus[bc].len() {
                    // Check to see if it's the component's pin
                    if board.bus[bc][y][0] == c 
                    && board.bus[bc][y][1] == p {
                        // If it is we assign it a clone of the
                        // bus to it.
                        component.pins.push(Arc::clone(&bus[bc]));
                        // And we can skip to the next pin!
                        continue 'pin_assignment;
                    }
                }
            }
            // If it's not connected to a bus we simply make a new
            // one to be used instead!
            component.pins.push(
                Arc::new(
                    Mutex::new(
                        component.default_value
                    )
                )
            );
        }
        // Once all the pins have been assigned, create the
        // registers the component will be using
        for _ in 0..component.reg_number {
            component.registers.push(component.default_value);
        }
        // Finally run the component
        let cmp = std::thread::spawn( move || {
            let run = component.logic;
            loop {
                run ( 
                    &mut component.pins, 
                    &mut component.registers
                );
            }
        });
        threads.push(cmp);
    }
    // And we're done we're done!
    
    // Wait until either all the components shut down or
    // the user ends the process!
    for cmp in threads {
        cmp.join().unwrap();
    }
    println! ( "Thanks for using kii_ware~! <3" );
}