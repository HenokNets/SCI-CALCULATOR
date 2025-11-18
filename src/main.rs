use std::io::{self, Write};
fn main() {
   loop {
    print! ("Enter something (type 'exit' to quit): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.eq_ignore_ascii_case("exit") {
        println!("Exiting the program. Goodbye!");
        break;
    } else {
        println!("You entered: {}", input);

   }
}
}
