pub fn function() {
    println!("called `bar::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `bar::nested::private_function()`");
}
