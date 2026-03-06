use trellis_ui::render::region::Region;

pub trait Widget<Message> {
    fn update(&mut self, message: &Message) -> Option<Message> {
        None
    }
    fn render(&self, region: Region) -> Region;
}
