extern crate rand;

use rand::Rng;

mod my;
mod bar;

fn function() {
    println!("called `function()`");
}

fn main() {
    println!(
        "The secret number is: {}",
        rand::thread_rng().gen_range(1, 101)
    );

    my::function();

    function();

    my::indirect_access();
    
    bar::nested::function();

    another_function();
}

fn another_function() {
    println!("Another function.");
}
