
// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {

    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('w');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("web event".to_owned());
    let click   = WebEvent::Click { x: 5, y: 6 };
    

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(click);
   
}

