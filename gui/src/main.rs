use iced::window;
use iced::{Size, Subscription};

#[derive(Debug)]
enum Message {
    WindowResized(Size),
}

pub fn main() -> iced::Result {
    iced::application("A cool application", update, view)
        .subscription(subscription)
        .run()
}

fn subscription(state: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}