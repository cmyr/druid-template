use druid::widget::{Button, Flex, Label, TextBox, WidgetExt};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, PlatformError, Widget, WindowDesc,
};

const COUNT_BG: Color = Color::grey8(0x77);

#[derive(Debug, Clone, Data, Lens)]
struct MyState {
    text: String,
    count: usize,
}

impl Default for MyState {
    fn default() -> Self {
        MyState {
            text: String::from("World!"),
            count: 0,
        }
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("").with_placeholder("Hello World!"));
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(MyState::default())?;

    Ok(())
}

fn ui_builder() -> impl Widget<MyState> {
    Flex::column()
        .with_child(
            Label::new(|data: &MyState, _env: &Env| format!("Hello {}", data.text))
                .padding(5.0)
                .center(),
        )
        .with_child(
            Label::new(|data: &MyState, _env: &Env| format!("Count: {}", data.count))
                .padding(5.0)
                .background(COUNT_BG)
                .center(),
        )
        .with_child(TextBox::new().lens(MyState::text).padding(5.0))
        .with_child(
            Button::new("Click!")
                .on_click(|_, count, _| *count += 1)
                .lens(MyState::count)
                .padding(5.0),
        )
        .center()
        .debug_paint_layout()
}
