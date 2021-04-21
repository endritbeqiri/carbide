use crate::prelude::*;

use lyon::tessellation::{VertexBuffers, FillTessellator, FillOptions, BuffersBuilder, FillVertex};
use lyon::tessellation::path::{Path, Winding};
use lyon::tessellation::path::traits::PathBuilder;
use lyon::tessellation::math::rect;
use lyon::tessellation::path::builder::BorderRadii;
use crate::widget::types::triangle_store::TriangleStore;
use crate::render::primitive_kind::PrimitiveKind;
use crate::draw::shape::triangle::Triangle;
use crate::color::Rgba;
use crate::state::environment_color::EnvironmentColor;
use crate::widget::primitive::shape::{tessellate, Shape};
use crate::widget::types::shape_style::ShapeStyle;
use crate::widget::types::stroke_style::StrokeStyle;

/// A basic, non-interactive rectangle shape widget.
#[derive(Debug, Clone, Widget)]
pub struct RoundedRectangle<GS> where GS: GlobalState {
    id: Uuid,
    position: Point,
    dimension: Dimensions,
    corner_radius: f64,
    #[state] stroke_color: ColorState<GS>,
    #[state] fill_color: ColorState<GS>,
    style: ShapeStyle,
    stroke_style: StrokeStyle,
    triangle_store: TriangleStore,
}

impl<GS: GlobalState> RoundedRectangle<GS> {

    pub fn fill(mut self, color: ColorState<GS>) -> Box<Self> {
        self.fill_color = color;
        self.style += ShapeStyle::Fill;
        Box::new(self)
    }

    pub fn stroke(mut self, color: ColorState<GS>) -> Box<Self> {
        self.stroke_color = color;
        self.style += ShapeStyle::Stroke;
        Box::new(self)
    }

    pub fn stroke_style(mut self, line_width: f64) -> Box<Self> {
        self.stroke_style = StrokeStyle::Solid {line_width};
        self.style += ShapeStyle::Stroke;
        Box::new(self)
    }

    pub fn initialize(corner_radius: f64) -> Box<RoundedRectangle<GS>> {
        Box::new(RoundedRectangle {
            id: Uuid::new_v4(),
            position: [0.0,0.0],
            dimension: [100.0,100.0],
            corner_radius,
            stroke_color: EnvironmentColor::Blue.into(),
            fill_color: EnvironmentColor::Blue.into(),
            style: ShapeStyle::Default,
            stroke_style: StrokeStyle::Solid {line_width: 2.0},
            triangle_store: TriangleStore::new()
        })
    }
}

impl<S: GlobalState> Layout<S> for RoundedRectangle<S> {
    fn flexibility(&self) -> u32 {
        0
    }

    fn calculate_size(&mut self, requested_size: Dimensions, env: &Environment<S>) -> Dimensions {
        self.dimension = requested_size;
        requested_size
    }

    fn position_children(&mut self) {
    }
}

impl<S: GlobalState> CommonWidget<S> for RoundedRectangle<S> {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn get_flag(&self) -> Flags {
        Flags::EMPTY
    }

    fn get_children(&self) -> WidgetIter<S> {
        WidgetIter::Empty
    }

    fn get_children_mut(&mut self) -> WidgetIterMut<S> {
        WidgetIterMut::Empty
    }

    fn get_proxied_children(&mut self) -> WidgetIterMut<S> {
        WidgetIterMut::Empty
    }

    fn get_proxied_children_rev(&mut self) -> WidgetIterMut<S> {
        WidgetIterMut::Empty
    }


    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, position: Dimensions) {
        self.position = position;
    }

    fn get_dimension(&self) -> Dimensions {
        self.dimension
    }

    fn set_dimension(&mut self, dimensions: Dimensions) {
        self.dimension = dimensions
    }
}

impl<GS: GlobalState> Shape<GS> for RoundedRectangle<GS> {
    fn get_triangle_store_mut(&mut self) -> &mut TriangleStore {
        &mut self.triangle_store
    }

    fn get_stroke_style(&self) -> StrokeStyle {
        self.stroke_style.clone()
    }

    fn get_shape_style(&self) -> ShapeStyle {
        self.style.clone()
    }
}

impl<S: GlobalState> Render<S> for RoundedRectangle<S> {

    fn get_primitives(&mut self, fonts: &text::font::Map) -> Vec<Primitive> {

        let rectangle = rect(self.get_x() as f32, self.get_y() as f32, self.get_width() as f32, self.get_height() as f32);

        let corner_radius = self.corner_radius as f32;

        tessellate(self, &rectangle, &|builder, rect| {
            builder.add_rounded_rectangle(
                rect,
                &BorderRadii {
                    top_left: corner_radius,
                    top_right: corner_radius,
                    bottom_left: corner_radius,
                    bottom_right: corner_radius,
                },
                Winding::Positive
            );
        });

        let mut prims = self.triangle_store.get_primitives(*self.fill_color.get_latest_value(), *self.stroke_color.get_latest_value());

        prims.extend(Rectangle::<S>::debug_outline(Rect::new(self.position, self.dimension), 1.0));

        return prims;
    }
}

impl<GS: GlobalState> WidgetExt<GS> for RoundedRectangle<GS> {}
