use crate::cmd;
use crate::comp;
use crate::env;
use crate::sound;
use crate::state;
use druid::widget::Button;
use druid::{MenuDesc, Target, Widget, WidgetExt, WidgetId, WindowDesc};
use std::rc::Rc;

pub fn create(
    parent_widget_id: WidgetId,
    rest_duration_secs: f64,
    sound_sender: Rc<sound::Sender>,
) -> WindowDesc<state::App> {
    let win_width = 450.0;
    let win_height = 200.0;

    let rect = druid::Screen::get_display_rect();
    let x = (rect.width() - win_width) / 2.0;
    let y = (rect.height() - win_height) / 2.0;

    WindowDesc::new(move || build(parent_widget_id, rest_duration_secs, sound_sender))
        .show_titlebar(false)
        .menu(MenuDesc::empty())
        .set_position((x, y))
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height))
}

fn build(
    parent_widget_id: WidgetId,
    rest_duration_secs: f64,
    sound_sender: Rc<sound::Sender>,
) -> impl Widget<state::App> {
    comp::flex::col_cen_cen()
        .with_child(
            comp::flex::col_sta_end()
                .with_child(
                    build_idle_timer(parent_widget_id, rest_duration_secs, sound_sender)
                        .lens(state::App::notifier),
                )
                .with_default_spacer()
                .with_child(build_finish_btn(parent_widget_id)),
        )
        .padding((8.0, 8.0))
}

fn build_idle_timer(
    parent_widget_id: WidgetId,
    rest_duration_secs: f64,
    sound_sender: Rc<sound::Sender>,
) -> impl Widget<state::Timer> {
    comp::timer::build()
        .controller(
            comp::timer::Controller::new(move |ctx, env, _rest_duration| {
                sound_sender.send(sound::Type::EndRest).ok();

                ctx.submit_command(cmd::DEINIT_COMP.to(Target::Widget(ctx.widget_id())));

                if env.get(env::WIN_REST_AUTO_RESTART_BREAK_TIMERS) {
                    ctx.submit_command(cmd::UNPAUSE_ALL_TIMER_COMP.with(false).to(Target::Global));
                }

                ctx.submit_command(cmd::RESET_TIMER_COMP.to(Target::Widget(parent_widget_id)));
                ctx.submit_command(druid::commands::CLOSE_WINDOW);
            })
            .with_duration(rest_duration_secs)
            .with_init_duration(env::BREAK_NOTIFIER_TIMER_DURATION),
        )
        .controller(comp::deinit::Controller::default())
}

fn build_finish_btn(parent_widget_id: WidgetId) -> impl Widget<state::App> {
    Button::new("Finish").on_click(move |ctx, _data, _env| {
        ctx.submit_command(cmd::UNPAUSE_ALL_TIMER_COMP.with(false).to(Target::Global));
        ctx.submit_command(cmd::RESET_TIMER_COMP.to(Target::Widget(parent_widget_id)));
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
    })
}
