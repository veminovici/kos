use kos::*;

fn main() {
    println!(
        "display: Here is a '{}' example",
        ITALIC.to_ansi("hello world (italic)")
    );

    println!(
        "debug: Here is a '{:?}' example",
        ITALIC
            .bold()
            .underline()
            .to_ansi("hello world (italic, bold, underline)")
    );

    println!(
        "display: Here is a '{}' example",
        BLUE.to_ansi("Hello World! (blue)")
    );

    println!(
        "debug: Here is a '{:?}' example",
        BLUE.bold()
            .blink()
            .to_ansi("Hello World! (blue, bold, and blink)")
    );

    println!(
        "display: Here is a '{}' example",
        BLUE.bg(WHITE)
            .hidden()
            .to_ansi("Hello World! (blue, white, hidden)")
    );

    println!(
        "debug: Here is a '{}' example",
        RED.bg(YELLOW)
            .strikethrough()
            .to_ansi("Hello world! (red, yellow, and strikethrough)")
    );
}
