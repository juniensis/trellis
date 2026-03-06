use crate::render::region::Region;

pub trait Element<Message> {
    fn update(&mut self, message: Message) -> Option<Message>;
    fn draw(&self) -> Region;
    fn map<B>(self, func: impl Fn(Message) -> B);
}
