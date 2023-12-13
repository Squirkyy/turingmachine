mod turing_machine;
use turing_machine::TuringMachine;

fn main() {
    println!("Hello, world!");
    let mut _tm = TuringMachine::new(Some('c'));
    println!("The blank symbol is {}", _tm.blank);
}
