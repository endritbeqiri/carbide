use std::borrow::Borrow;
use std::fmt::Debug;

use instant::Instant;

use crate::draw::{Dimension, Position, Rect};
use crate::prelude::*;
//use crate::render::text::Text as RenderText;
use crate::render::new_primitive;
use crate::render::PrimitiveKind;
use crate::text::{FontStyle, FontWeight, Glyph, NoStyleTextSpanGenerator, TextDecoration, TextSpanGenerator, TextStyle};
use crate::text::Text as InternalText;
//use crate::text_old::PositionedGlyph;
use crate::widget::types::Wrap;

/// Displays some given text centered within a rectangular area.
///
/// By default, the rectangular dimensions are fit to the area occupied by the text.
///
/// If some horizontal dimension is given, the text will automatically wrap to the width and align
/// in accordance with the produced **Alignment**.
#[derive(Debug, Clone, Widget)]
pub struct Text {
    id: Uuid,
    position: Position,
    dimension: Dimension,
    wrap_mode: Wrap,
    #[state] pub text: StringState,
    #[state] font_size: U32State,
    #[state] color: ColorState,
    font_family: String,
    font_style: FontStyle,
    font_weight: FontWeight,
    text_decoration: TextDecoration,
    internal_text: Option<InternalText>,
    text_span_generator: Box<dyn TextSpanGenerator>,
}

impl Text {
    pub fn new<K: Into<StringState>>(text: K) -> Box<Self> {
        let text = text.into();

        Box::new(Text {
            id: Uuid::new_v4(),
            text,
            font_size: EnvironmentFontSize::Body.into(),
            position: Position::new(0.0, 0.0),
            dimension: Dimension::new(100.0, 100.0),
            wrap_mode: Wrap::Whitespace,
            color: EnvironmentColor::Label.into(),
            font_family: "system-font".to_string(),
            font_style: FontStyle::Normal,
            font_weight: FontWeight::Normal,
            text_decoration: TextDecoration::None,
            internal_text: None,
            text_span_generator: Box::new(NoStyleTextSpanGenerator {}),
        })
    }

    pub fn new_with_generator<K: Into<StringState>, G: Into<Box<dyn TextSpanGenerator>>>(text: K, generator: G) -> Box<Self> {
        let text = text.into();

        Box::new(Text {
            id: Uuid::new_v4(),
            text,
            font_size: EnvironmentFontSize::Body.into(),
            position: Position::new(0.0, 0.0),
            dimension: Dimension::new(100.0, 100.0),
            wrap_mode: Wrap::Whitespace,
            color: EnvironmentColor::Label.into(),
            font_family: "system-font".to_string(),
            font_style: FontStyle::Normal,
            font_weight: FontWeight::Normal,
            text_decoration: TextDecoration::None,
            internal_text: None,
            text_span_generator: generator.into(),
        })
    }

    pub fn color<C: Into<ColorState>>(mut self, color: C) -> Box<Self> {
        self.color = color.into();
        Box::new(self)
    }

    pub fn font_size<K: Into<U32State>>(mut self, size: K) -> Box<Self> {
        self.font_size = size.into();
        Box::new(self)
    }

    pub fn wrap_mode(mut self, wrap: Wrap) -> Box<Self> {
        self.wrap_mode = wrap;
        Box::new(self)
    }

    /// Align the text to the left of its bounding **Rect**'s *x* axis range.
    pub fn left_justify(self) -> Self {
        self.justify(Justify::Left)
    }

    /// Align the text to the middle of its bounding **Rect**'s *x* axis range.
    pub fn center_justify(self) -> Self {
        self.justify(Justify::Center)
    }

    pub fn justify(self, _j: Justify) -> Self {
        self
    }

    /// Align the text to the right of its bounding **Rect**'s *x* axis range.
    pub fn right_justify(self) -> Self {
        self.justify(Justify::Right)
    }

    pub fn get_positioned_glyphs(&self, _: &Environment, scale_factor: f32) -> Vec<Glyph> {
        if let Some(internal) = &self.internal_text {
            internal.first_glyphs()
        } else {
            vec![]
        }
    }

    pub fn get_style(&self) -> TextStyle {
        TextStyle {
            font_family: self.font_family.clone(),
            font_size: *self.font_size.value(),
            font_style: self.font_style,
            font_weight: self.font_weight,
            text_decoration: self.text_decoration.clone(),
            color: Some(self.color.value().deref().clone()),
        }
    }
}

impl Layout for Text {
    fn flexibility(&self) -> u32 {
        2
    }

    fn calculate_size(&mut self, requested_size: Dimension, env: &mut Environment) -> Dimension {
        let now = Instant::now();
        let style = self.get_style();

        if let None = self.internal_text {
            let text = self.text.value().deref().clone();
            let style = self.get_style();
            self.internal_text = Some(InternalText::new(text, style, self.text_span_generator.borrow(), env))
        }

        if let Some(internal) = &mut self.internal_text {
            let text = self.text.value().deref().clone();
            if internal.string_that_generated_this() != &text {
                *internal = InternalText::new(text, style, self.text_span_generator.borrow(), env);
            }
            self.dimension = internal.calculate_size(requested_size, env);
        }

        println!("Time for calculate size: {}us", now.elapsed().as_micros());

        self.dimension
    }

    fn position_children(&mut self) {
        let position = Position::new(self.x(), self.y());
        if let Some(internal) = &mut self.internal_text {
            internal.position(position)
        }
    }
}

impl Render for Text {
    fn get_primitives(&mut self, env: &mut Environment) -> Vec<Primitive> {
        let mut prims: Vec<Primitive> = vec![];
        let default_color = *self.color.value();

        if let Some(internal) = &mut self.internal_text {
            internal.ensure_glyphs_added_to_atlas(env);

            for (glyphs, color, additional_rects) in internal.span_glyphs() {
                let color = if let Some(color) = color {
                    color
                } else {
                    default_color
                };
                let kind = PrimitiveKind::Text {
                    color,
                    text: glyphs,
                };
                prims.push(new_primitive(kind, Rect::new(self.position, self.dimension)));

                for additional_rect in additional_rects {
                    let position = Position::new(additional_rect.position.x, additional_rect.position.y);
                    let dimension = Dimension::new(additional_rect.dimension.width, additional_rect.dimension.height);
                    prims.push(Primitive {
                        kind: PrimitiveKind::Rectangle { color },
                        rect: Rect::new(position, dimension),
                    });
                }
            }
        }

        prims.extend(Rectangle::debug_outline(Rect::new(self.position, self.dimension), 1.0));

        return prims;
    }
}

impl CommonWidget for Text {
    fn id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }

    fn flag(&self) -> Flags {
        Flags::EMPTY
    }

    fn children(&self) -> WidgetIter {
        WidgetIter::Empty
    }

    fn children_mut(&mut self) -> WidgetIterMut {
        WidgetIterMut::Empty
    }

    fn proxied_children(&mut self) -> WidgetIterMut {
        WidgetIterMut::Empty
    }

    fn proxied_children_rev(&mut self) -> WidgetIterMut {
        WidgetIterMut::Empty
    }

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn dimension(&self) -> Dimension {
        self.dimension
    }

    fn set_dimension(&mut self, dimensions: Dimension) {
        self.dimension = dimensions
    }
}

impl WidgetExt for Text {}