mod turing_machine;
use turing_machine::TuringMachine;
// https://studip.uni-hannover.de/sendfile.php?type=0&file_id=9f53f0ebc4afad25285363991c41721d&file_name=Skript.pdf
fn main() {
    println!("Hello, world!");
    let mut _tm = TuringMachine::new(Some('c'));
    println!("The blank symbol is {}", _tm.blank);
}
