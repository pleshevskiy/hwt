use crate::cmd;
use crate::comp;
use crate::env;
use crate::state;
use druid::widget::{Button, Flex};
use druid::{MenuDesc, Target, Widget, WidgetExt, WidgetId, WindowDesc};

pub fn create(parent_widget_id: WidgetId) -> WindowDesc<state::App> {
    let win_width = 200.0;
    let win_height = 100.0;

    let rect = druid::Screen::get_display_rect();
    let x = (rect.width() - win_width) / 2.0;
    let y = 0.0;

    return WindowDesc::new(move || build(parent_widget_id))
        .show_titlebar(false)
        .menu(MenuDesc::empty())
        .set_position((x, y))
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height));
}

fn build(parent_widget_id: WidgetId) -> impl Widget<state::App> {
    Flex::column()
        .with_child(build_notifier_timer(parent_widget_id).lens(state::App::notifier))
        .with_default_spacer()
        .with_child(build_postpone_btn(parent_widget_id))
        .padding((8.0, 8.0))
}

fn build_notifier_timer(parent_widget_id: WidgetId) -> impl Widget<state::Timer> {
    comp::timer::build()
        .controller(comp::timer::TimerController::new(move |ctx| {
            ctx.submit_command(cmd::DEINIT_COMP.to(Target::Widget(ctx.widget_id())));
            ctx.submit_command(
                cmd::OPEN_IDLE_WINDOW
                    .with((parent_widget_id, 30.0))
                    .to(Target::Global),
            );
            ctx.submit_command(druid::commands::CLOSE_WINDOW);
        }))
        .controller(comp::deinit::DeinitController::default())
        .env_scope(move |env, _| {
            env.set(
                env::TIMER_DURATION,
                env.get(env::BREAK_NOTIFIER_TIMER_DURATION),
            );
        })
}

fn build_postpone_btn<D: druid::Data>(parent_widget_id: WidgetId) -> impl Widget<D> {
    Button::new("Postpone").on_click(move |ctx, _data, _env| {
        ctx.submit_command(cmd::POSTPONE_TIMER_COMP.to(Target::Widget(parent_widget_id)));
        ctx.submit_command(cmd::UNPAUSE_ALL_TIMER_COMP.with(true).to(Target::Global));
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
    })
}