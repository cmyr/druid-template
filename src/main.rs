use druid::widget::{Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, PlatformError, Widget, WindowDesc};

#[derive(Debug, Clone, Data, Lens)]
struct MyState {
    text: String,
}

impl Default for MyState {
    fn default() -> Self {
        MyState {
            text: String::from("World!"),
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
            1.0,
        )
        .with_child(TextBox::raw().lens(MyState::text).padding(5.0), 1.0)
}
