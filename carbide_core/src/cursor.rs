//! Contains an extendable enum of supported mouse cursor render.
//!
//! Use this module to map from the carbide's mouse cursor render to the render known to the window
//! backend you are using. A lot of these are already implemented in `carbide::backend`. Unless you
//! are using custom mouse cursor render not provided here, then using one of the implementations in
//! `carbide::backend` should be sufficient.

/// This enum specifies cursor render used by internal widgets. For custom widgets using custom
/// cursor render, you can still use this enum by specifying a numbered custom variant.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseCursor {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    /// Custom cursor variant. Encode your favourite cursor with a u8.
    Custom(u8),
}
