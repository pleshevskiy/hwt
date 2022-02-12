use crate::cmd;
use crate::comp;
use crate::env;
use crate::state;
use druid::widget::Flex;
use druid::{MenuDesc, Target, Widget, WidgetExt, WidgetId, WindowDesc};

pub fn create(parent_widget_id: WidgetId, rest_duration: f64) -> WindowDesc<state::App> {
    let win_width = 450.0;
    let win_height = 200.0;

    let rect = druid::Screen::get_display_rect();
    let x = (rect.width() - win_width) / 2.0;
    let y = (rect.height() - win_height) / 2.0;

    return WindowDesc::new(move || build(parent_widget_id, rest_duration))
        .show_titlebar(false)
        .menu(MenuDesc::empty())
        .set_position((x, y))
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height));
}

fn build(parent_widget_id: WidgetId, rest_duration: f64) -> impl Widget<state::App> {
    Flex::column()
        .with_child(build_idle_timer(parent_widget_id, rest_duration).lens(state::App::notifier))
        .padding((8.0, 8.0))
}

fn build_idle_timer(parent_widget_id: WidgetId, rest_duration: f64) -> impl Widget<state::Timer> {
    comp::timer::build()
        .controller(comp::timer::TimerController::new(move |ctx| {
            ctx.submit_command(cmd::DEINIT_COMP.to(Target::Widget(ctx.widget_id())));
            ctx.submit_command(cmd::UNPAUSE_ALL_TIMER_COMP.with(false).to(Target::Global));
            ctx.submit_command(cmd::RESTART_TIMER_COMP.to(Target::Widget(parent_widget_id)));
            ctx.submit_command(druid::commands::CLOSE_WINDOW);
        }))
        .controller(comp::deinit::DeinitController::default())
        .env_scope(move |env, _| {
            env.set(
                env::TIMER_INIT_DURATION,
                env.get(env::BREAK_NOTIFIER_TIMER_DURATION),
            );
            env.set(env::TIMER_DURATION, rest_duration)
        })
}
