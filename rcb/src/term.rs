use ansi_term::Colour;

pub fn run() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
}
