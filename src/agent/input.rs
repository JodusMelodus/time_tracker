use rdev::{Event, listen};

pub fn start_input_listener() {
    std::thread::spawn(|| {
        if let Err(error) = listen(|event: Event| {
            // println!("Got event: {:?}", event);
        }) {
            println!("Error: {:?}", error);
        }
    });
}
