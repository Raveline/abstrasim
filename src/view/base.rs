use conrod;
use piston_window;
use piston_window::{PistonWindow, WindowSettings, EventLoop, UpdateEvent};
use find_folder;
use conrod::{color, Text, Canvas, Frameable, Colorable, Widget, Positionable};

type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

pub fn show_window() {
    let opengl = piston_window::OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new("Test", [800, 600])
            .opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();

    let mut ui =  {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("font/ModernM/modernM.ttf");
        let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.clone()).unwrap();
        let theme = conrod::Theme::default();
        Ui::new(glyph_cache, theme)
    };

    window.set_ups(60);

    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui)));
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }

}

fn set_widgets(ui: &mut UiCell) {
    widget_ids! {
        CANVAS,
        TEST_TEXT
    }
    Canvas::new()
        .frame(1.0)
        .pad(30.0)
        .color(color::BLACK)
        .scroll_kids()
        .set(CANVAS, ui);
    Text::new("Test")
        .top_left_with_margins_on(CANVAS, 0.0, 10.0)
        .color(color::LIGHT_RED)
        .align_text_left()
        .set(TEST_TEXT, ui)
}
