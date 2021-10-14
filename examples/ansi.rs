use kos::*;

fn main() {
    println!(
        "display: Here is a '{}' example",
        ITALIC.to_ansi("hello world (italic)")
    );

    println!(
        "debug: Here is a '{:?}' example",
        ITALIC.bold().to_ansi("hello world (italic and bold)")
    );

    println!(
        "display: Here is a '{}' example",
        BLUE.to_ansi("Hello World! (blue)")
    );

    println!(
        "debug: Here is a '{:?}' example",
        BLUE.bold().to_ansi("Hello World! (blue and bold)")
    );
}
