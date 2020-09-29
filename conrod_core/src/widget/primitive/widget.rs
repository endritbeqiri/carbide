use crate::widget::primitive::shape::rectangle::Rectangle;
use ::{Color, Rect};
use color::rgb;
use graph::Container;
use widget::{Id, Oval, Line, Text, Image};
use widget::render::Render;
use widget::primitive::shape::oval::Full;
use render::primitive_kind::PrimitiveKind;
use render::util::new_primitive;
use render::primitive::Primitive;
use render::owned_primitive::OwnedPrimitive;
use text;

#[derive(Clone, Debug)]
pub enum CWidget {
    Rectangle(Rectangle),
    Line(Line),
    Oval(Oval<Full>),
    Text(Text),
    Image(Image),
    Complex
}

impl Render for CWidget {
    fn render(self, id: Id, clip: Rect, container: &Container) -> Option<Primitive> {
        match self {
            CWidget::Rectangle(n) => n.render(id, clip, container),
            CWidget::Oval(n) => n.render(id, clip, container),
            CWidget::Complex => {
                let kind = PrimitiveKind::Rectangle { color: Color::random()};
                return Some(new_primitive(id, kind, clip, container.rect));
            },

            CWidget::Line(n) => {n.render(id, clip, container)}
            CWidget::Text(n) => {n.render(id, clip, container)}
            CWidget::Image(n) => {n.render(id, clip, container)}
        }
    }

    fn get_primitives(&self, fonts: &text::font::Map) -> Vec<Primitive> {
        match self {
            CWidget::Rectangle(n) => {n.get_primitives(fonts)},
            CWidget::Oval(n) => {n.get_primitives(fonts)},
            CWidget::Complex => {vec![]},
            CWidget::Line(n) => {n.get_primitives(fonts)}
            CWidget::Text(n) => {n.get_primitives(fonts)}
            CWidget::Image(n) => {n.get_primitives(fonts)}
        }
    }
}