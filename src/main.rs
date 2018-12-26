extern crate sciter;

use sciter::HELEMENT;
use sciter::dom::event::BEHAVIOR_EVENTS;
use sciter::dom::event::PHASE_MASK;
use sciter::dom::EventReason;
use sciter::dom::Element;
struct EventRouter;

impl sciter::EventHandler for EventRouter {
    fn on_event(&mut self, _: HELEMENT, source: HELEMENT, _: HELEMENT, code: BEHAVIOR_EVENTS, _: PHASE_MASK, _: EventReason) -> bool {
        if code == BEHAVIOR_EVENTS::BUTTON_CLICK {
            let mut caller = Element::from(source);
            let caller_id = caller.get_attribute("id");
            if caller_id.is_some() {
                let caller_id = caller_id.unwrap();
                if caller_id == "helloButton" {
                    println!("Hello from RUST!");
                    let result = caller.set_text("Hello from RUST!");
                    if result.is_err() {
                        println!("Failed to change button text {:?}",result.err().unwrap()) //f
                    }
                }
            }
        }
        false
    }
}

fn main() {
    create_window();
}

fn create_window() {
    let mut frame = sciter::Window::new();
    frame.load_html(include_bytes!("main.htm"),None);
    frame.event_handler(EventRouter);
    frame.run_app();
}
