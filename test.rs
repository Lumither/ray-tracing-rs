fn main() {
    let red = "\x1b[31m";
    let bold = "\x1b[22";
    let reset = "\x1b[0m";
    println!("\x1b[31mhello world!\x1b[0m");
    println!("\x1b[1mhello world!\x1b[0m");
}