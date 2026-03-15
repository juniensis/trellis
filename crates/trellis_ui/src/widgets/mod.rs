use crate::{
    render::{self, Renderer},
    widgets::id::Id,
};

pub mod id;

pub trait Widget<Message, Renderer>
where
    Renderer: render::Renderer,
{
    fn update(&mut self, message: Message) -> Option<Message>;
    fn draw(&self, renderer: &mut Renderer);
    fn id(&self) -> &Id;
    fn map<B>(&self, func: impl Fn(Message) -> B);
}
