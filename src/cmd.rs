use druid::{Selector, WidgetId};

pub const OPEN_BREAK_WINDOW: Selector<WidgetId> = Selector::new("open_break_window");

pub const DEINIT_COMP: Selector = Selector::new("hwt.cmd.comp.deinit");
pub const PAUSE_ALL_TIMER_COMP: Selector = Selector::new("hwt.cmd.comp.timer.pause.all");
pub const UNPAUSE_ALL_TIMER_COMP: Selector<bool> = Selector::new("hwt.cmd.comp.timer.unpause.all");
pub const POSTPONE_TIMER_COMP: Selector = Selector::new("hwt.cmd.comp.timer.postpone");
pub const RESTART_TIMER_COMP: Selector = Selector::new("hwt.cmd.comp.timer.restart");

// pub const CYCLE_NOTIFICATION: Selector<usize> =
//     Selector::new("hwt.comp.timer.cycle_notification");
// pub const ACCEPT_TEMP_TIMER: Selector<WidgetId> =
//     Selector::new("hwt.comp.timer.accept_temp_timer");
