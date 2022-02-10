use druid::{Selector, WidgetId};

pub const OPEN_BREAK_WINDOW: Selector<WidgetId> = Selector::new("open_break_window");
pub const POSTPONE_BREAK: Selector<WidgetId> = Selector::new("postpone_break");

pub const PAUSE_ALL_TIMER_COMPONENT: Selector = Selector::new("hwt.cmd.components.timer.pause.all");
pub const UNPAUSE_ALL_TIMER_COMPONENT: Selector =
    Selector::new("hwt.cmd.components.timer.unpause.all");
pub const RESTART_TIMER_COMPONENT: Selector = Selector::new("hwt.cmd.components.timer.restart");
pub const REQUEST_FINAL_TIMER: Selector<u32> =
    Selector::new("hwt.cmd.components.timer.request_final_timer");

// pub const CYCLE_NOTIFICATION: Selector<usize> =
//     Selector::new("hwt.components.timer.cycle_notification");
// pub const ACCEPT_TEMP_TIMER: Selector<WidgetId> =
//     Selector::new("hwt.components.timer.accept_temp_timer");
