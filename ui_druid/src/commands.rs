use druid::{Selector, WidgetId};

pub const OPEN_BREAK_WINDOW: Selector<WidgetId> = Selector::new("open_break_window");
pub const POSTPONE_BREAK: Selector<WidgetId> = Selector::new("postpone_break");

pub const CYCLE_NOTIFICATION: Selector<usize> =
    Selector::new("hwt.components.timer.cycle_notification");
pub const ACCEPT_TEMP_TIMER: Selector<WidgetId> =
    Selector::new("hwt.components.timer.accept_temp_timer");
