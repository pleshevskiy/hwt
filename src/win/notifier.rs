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
    let win_width = 200.0;
    let win_height = 100.0;

    let rect = druid::Screen::get_display_rect();
    let x = (rect.width() - win_width) / 2.0;
    let y = 0.0;

    return WindowDesc::new(move || build(parent_widget_id, rest_duration_secs, sound_sender))
        .show_titlebar(false)
        .menu(MenuDesc::empty())
        .set_position((x, y))
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height));
}

fn build(
    parent_widget_id: WidgetId,
    rest_duration_secs: f64,
    sound_sender: Rc<sound::Sender>,
) -> impl Widget<state::App> {
    comp::flex::col_cen_cen()
        .with_child(
            build_notifier_timer(parent_widget_id, rest_duration_secs, sound_sender)
                .lens(state::App::notifier),
        )
        .with_default_spacer()
        .with_child(build_postpone_btn(parent_widget_id))
        .padding((8.0, 8.0))
}

fn build_notifier_timer(
    parent_widget_id: WidgetId,
    rest_duration_secs: f64,
    sound_sender: Rc<sound::Sender>,
) -> impl Widget<state::Timer> {
    comp::timer::build()
        .controller(
            comp::timer::TimerController::new(move |ctx, _env, _rest_duration| {
                sound_sender.send(sound::Type::EndNotifier).ok();

                ctx.submit_command(cmd::DEINIT_COMP.to(Target::Widget(ctx.widget_id())));
                ctx.submit_command(
                    cmd::OPEN_IDLE_WINDOW
                        .with((parent_widget_id, rest_duration_secs))
                        .to(Target::Global),
                );
                ctx.submit_command(druid::commands::CLOSE_WINDOW);
            })
            .with_duration(env::BREAK_NOTIFIER_TIMER_DURATION),
        )
        .controller(comp::deinit::DeinitController::default())
}

fn build_postpone_btn<D: druid::Data>(parent_widget_id: WidgetId) -> impl Widget<D> {
    Button::new("Postpone").on_click(move |ctx, _data, _env| {
        ctx.submit_command(cmd::POSTPONE_TIMER_COMP.to(Target::Widget(parent_widget_id)));
        ctx.submit_command(cmd::UNPAUSE_ALL_TIMER_COMP.with(true).to(Target::Global));
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
    })
}
