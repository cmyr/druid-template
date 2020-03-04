use druid::widget::{Button, Checkbox, Flex, Label, List, Scroll, TextBox, WidgetExt};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, PlatformError, Widget, WindowDesc,
};
use std::mem;
use std::sync::Arc;

const COUNT_BG: Color = Color::grey8(0x77);

#[derive(Debug, Default, Clone, Data, Lens)]
struct TodoItem {
    text: String,
    complete: bool,
}

#[derive(Debug, Clone, Data, Default, Lens)]
struct TodoList {
    items: Arc<Vec<TodoItem>>,
    new_item: String,
}

impl TodoList {
    fn add_item(&mut self, text: impl Into<String>) {
        let text = text.into();
        let item = TodoItem {
            text,
            complete: false,
        };
        Arc::make_mut(&mut self.items).push(item);
    }
}

fn dummy_data() -> TodoList {
    let mut list = TodoList::default();
    list.add_item("buy eggs");
    list.add_item("buy shoes");
    list.add_item("never die");
    list
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("").with_placeholder("Hello World!"));
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(dummy_data())?;

    Ok(())
}

fn ui_builder() -> impl Widget<TodoList> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(TextBox::raw().lens(TodoList::new_item), 1.0)
                .with_child(
                    Button::new("Add", |_ctx, data: &mut TodoList, _| {
                        let item_text = mem::take(&mut data.new_item);
                        data.add_item(item_text);
                    })
                    .fix_width(60.0),
                    0.,
                ),
            0.,
        )
        .with_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(
                            Checkbox::new().lens(TodoItem::complete).padding((10., 5.)),
                            0.0,
                        )
                        .with_child(
                            Label::new(|data: &TodoItem, _env: &Env| data.text.clone()).env_scope(
                                |env, data| {
                                    let text_color = if data.complete {
                                        Color::grey8(0x44)
                                    } else {
                                        Color::WHITE
                                    };

                                    env.set(druid::theme::LABEL_COLOR, text_color);
                                },
                            ),
                            0.0,
                        )
                })
                .lens(TodoList::items),
            ),
            1.0,
        )

    //Flex::column()
    //.with_child(
    //Label::new(|data: &MyState, _env: &Env| format!("Hello {}", data.text))
    //.padding(5.0)
    //.center(),
    //1.0,
    //)
    //.with_child(
    //Label::new(|data: &MyState, _env: &Env| format!("Count: {}", data.count))
    //.padding(5.0)
    //.background(COUNT_BG)
    //.center(),
    //1.0,
    //)
    //.with_child(TextBox::raw().lens(MyState::text).padding(5.0), 1.0)
    //.with_child(
    //Button::new("Click!", |_, count, _| *count += 1)
    //.lens(MyState::count)
    //.padding(5.0),
    //1.0,
    //)
}
