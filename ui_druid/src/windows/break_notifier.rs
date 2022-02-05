use crate::commands;
use crate::components;
use crate::state;
use druid::widget::{Button, Flex};
use druid::{Widget, WidgetExt, WidgetId, WindowDesc};

pub fn create(parent_widget_id: WidgetId) -> WindowDesc<state::App> {
    let win_width = 200.0;
    let win_height = 100.0;

    let rect = druid::Screen::get_display_rect();
    let x = (rect.width() - win_width) / 2.0;
    let y = 0.0;

    return WindowDesc::new(move || build(parent_widget_id))
        .show_titlebar(false)
        .set_position((x, y))
        .window_size((win_width, win_height));
}

fn build(parent_widget_id: WidgetId) -> impl Widget<state::App> {
    Flex::column()
        .with_child(
            components::break_timer::build("Break", |ctx| {
                ctx.submit_command(druid::commands::CLOSE_WINDOW)
            })
            .lens(state::App::notifier),
        )
        .with_default_spacer()
        .with_child(Button::new("Postpone").on_click(move |ctx, _data, _env| {
            ctx.submit_command(commands::POSTPONE_BREAK.with(parent_widget_id));
            ctx.submit_command(druid::commands::CLOSE_WINDOW);
        }))
        .padding((8.0, 8.0))
}
