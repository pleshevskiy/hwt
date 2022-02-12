use druid::widget::{Axis, CrossAxisAlignment, Flex, MainAxisAlignment};
use druid::Data;

macro_rules! flex {
    ($($name:ident: $axis:ident $main:ident $cross:ident)+) => {
        $(pub fn $name<D: Data>() -> Flex<D> {
            flex_layout(Axis::$axis, MainAxisAlignment::$main, CrossAxisAlignment::$cross)
        })+
    };
}

flex! {
    col_sta_sta: Vertical   Start  Start
    col_sta_end: Vertical   Start  End
    col_cen_cen: Vertical   Center Center
    row_sta_sta: Horizontal Start  Start
}

fn flex_layout<D>(axis: Axis, main: MainAxisAlignment, cross: CrossAxisAlignment) -> Flex<D>
where
    D: Data,
{
    Flex::for_axis(axis)
        .main_axis_alignment(main)
        .cross_axis_alignment(cross)
}
