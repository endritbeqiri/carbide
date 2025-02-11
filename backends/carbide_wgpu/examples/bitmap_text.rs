use carbide_core::environment::EnvironmentColor;
use carbide_core::text::{FontFamily, FontStyle, FontWeight, PolarBearMarkup};
use carbide_core::widget::*;
use carbide_wgpu::window::*;

fn main() {
    carbide_wgpu::init_logger();

    let icon_path = Window::relative_path_to_assets("images/rust_press.png");

    let mut window = Window::new(
        "Bitmap text example".to_string(),
        800,
        1200,
        Some(icon_path),
    );

    let mut noto_family = FontFamily::new_from_paths("NotoSans", vec![
        "fonts/NotoSans/NotoSans-Regular.ttf",
        "fonts/NotoSans/NotoSans-Italic.ttf",
        "fonts/NotoSans/NotoSans-Bold.ttf",
    ]);
    window.add_font_family(noto_family);

    let mut family = FontFamily::new("Apple Color Emoji");
    family.add_bitmap_font_with_hints(
        "/System/Library/Fonts/Apple Color Emoji.ttc",
        FontWeight::Normal,
        FontStyle::Normal,
    );
    window.add_font_family(family);

    window.set_widgets(
        Text::new_with_generator("# Rich text\nHello *world*, this is /italic/, _underlined_ and -striked-. We can even show 😀, and we support a list of fallback fonts!", PolarBearMarkup::new())
            .border()
            .border_width(1)
            .color(EnvironmentColor::Green)
            .padding(EdgeInsets::all(40.0))
    );

    window.launch();
}
