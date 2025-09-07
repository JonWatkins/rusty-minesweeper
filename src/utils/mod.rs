pub mod debug;
pub mod time;

pub use debug::{
    is_logging_enabled, is_show_mines_enabled, set_debug_enabled, set_show_mines_enabled,
};
pub use time::format_time;
