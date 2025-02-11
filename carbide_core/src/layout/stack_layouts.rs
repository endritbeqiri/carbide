use crate::draw::{Dimension, Position};
use crate::environment::Environment;
use crate::flags::Flags;
use crate::layout::Layout;
use crate::prelude::CrossAxisAlignment;
use crate::prelude::WidgetValMut;

pub(crate) fn calculate_size_vstack(widget: &mut dyn Layout, spacing: f64, requested_size: Dimension, env: &mut Environment) {
    calculate_size_stack(widget, height, width, height_width, spacing, requested_size, env);
}

pub(crate) fn position_children_vstack(widget: &mut dyn Layout, spacing: f64, cross_axis_alignment: CrossAxisAlignment) {
    position_children_stack(widget, y, height, x, width, y_x, cross_axis_alignment, spacing);
}

pub(crate) fn calculate_size_hstack(widget: &mut dyn Layout, spacing: f64, requested_size: Dimension, env: &mut Environment) {
    calculate_size_stack(widget, width, height, width_height, spacing, requested_size, env);
}

pub(crate) fn position_children_hstack(widget: &mut dyn Layout, spacing: f64, cross_axis_alignment: CrossAxisAlignment) {
    position_children_stack(widget, x, width, y, height, x_y, cross_axis_alignment, spacing);
}

fn x(position: Position) -> f64 {
    position.x
}

fn y(position: Position) -> f64 {
    position.y
}

fn x_y(main_axis: f64, cross_axis: f64) -> Position {
    Position::new(main_axis, cross_axis)
}

fn y_x(main_axis: f64, cross_axis: f64) -> Position {
    Position::new(cross_axis, main_axis)
}

fn height(dimension: Dimension) -> f64 {
    dimension.height
}

fn width(dimension: Dimension) -> f64 {
    dimension.width
}

fn height_width(main_axis: f64, cross_axis: f64) -> Dimension {
    Dimension::new(cross_axis, main_axis)
}

fn width_height(main_axis: f64, cross_axis: f64) -> Dimension {
    Dimension::new(main_axis, cross_axis)
}

/// *dimension*(main_axis, cross_axis)
fn calculate_size_stack(widget: &mut dyn Layout, main_axis: fn(Dimension) -> f64, cross_axis: fn(Dimension) -> f64, dimension: fn(f64, f64) -> Dimension, spacing: f64, requested_size: Dimension, env: &mut Environment) {
    let mut number_of_children_that_needs_sizing = widget
        .children()
        .filter(|m| m.flag() != Flags::SPACER)
        .count();

    let non_spacers_vec: Vec<bool> =
        widget.children().map(|n| n.flag() != Flags::SPACER).collect();

    let non_spacers_vec_length = non_spacers_vec.len();

    let number_of_spaces = non_spacers_vec
        .iter()
        .enumerate()
        .take(non_spacers_vec_length.max(1) - 1)
        .filter(|(n, b)| **b && non_spacers_vec[n + 1])
        .count() as f64;

    let spacing_total = number_of_spaces * spacing;

    let mut size_for_children =
        dimension(main_axis(requested_size) - spacing_total, cross_axis(requested_size));

    let mut children_flexibility: Vec<(u32, WidgetValMut)> = widget
        .children_mut()
        .filter(|m| m.flag() != Flags::SPACER)
        .map(|child| (child.flexibility(), child))
        .collect();

    children_flexibility.sort_by(|(a, _), (b, _)| b.cmp(&a));

    let mut max_cross_axis = 0.0;

    let mut total_main_axis = 0.0;

    for (_, mut child) in children_flexibility {
        let size_for_child = dimension(
            main_axis(size_for_children) / number_of_children_that_needs_sizing as f64,
            cross_axis(size_for_children),
        );

        let chosen_size = child.calculate_size(size_for_child, env);

        if cross_axis(chosen_size) > max_cross_axis {
            max_cross_axis = cross_axis(chosen_size);
        }

        size_for_children = dimension((main_axis(size_for_children) - main_axis(chosen_size)).max(0.0), cross_axis(size_for_children));

        number_of_children_that_needs_sizing -= 1;

        total_main_axis += main_axis(chosen_size);
    }

    let spacer_count = widget
        .children()
        .filter(|m| m.flag() == Flags::SPACER)
        .count() as f64;

    let rest_space = main_axis(requested_size) - total_main_axis - spacing_total;

    let request_dimension = dimension(rest_space / spacer_count, 0.0);

    for mut spacer in widget.children_mut().filter(|m| m.flag() == Flags::SPACER) {
        let chosen_size = spacer.calculate_size(
            request_dimension,
            env,
        );

        total_main_axis += main_axis(chosen_size);
    }

    widget.set_dimension(dimension(total_main_axis + spacing_total, max_cross_axis));
}

fn position_children_stack(widget: &mut dyn Layout, main_axis_position: fn(Position) -> f64, main_axis_dimension: fn(Dimension) -> f64, cross_axis_position: fn(Position) -> f64, cross_axis_dimension: fn(Dimension) -> f64, position_from_main_and_cross: fn(f64, f64) -> Position, cross_axis_alignment: CrossAxisAlignment, spacing: f64) {
    let alignment = cross_axis_alignment;
    let mut main_axis_offset = 0.0;

    let position = widget.position();
    let dimension = widget.dimension();

    let spacers: Vec<bool> = widget.children().map(|n| n.flag() == Flags::SPACER).collect();

    for (n, mut child) in widget.children_mut().enumerate() {
        let cross = match alignment {
            CrossAxisAlignment::Start => cross_axis_position(position),
            CrossAxisAlignment::Center => {
                cross_axis_position(position) + cross_axis_dimension(dimension) / 2.0 - cross_axis_dimension(child.dimension()) / 2.0
            }
            CrossAxisAlignment::End => {
                cross_axis_position(position) + cross_axis_dimension(dimension) - cross_axis_dimension(child.dimension())
            }
        };

        child.set_position(position_from_main_and_cross(main_axis_position(position) + main_axis_offset, cross));

        if child.flag() != Flags::SPACER && n < spacers.len() - 1 && !spacers[n + 1] {
            main_axis_offset += spacing;
        }
        main_axis_offset += main_axis_dimension(child.dimension());

        child.position_children();
    }
}
