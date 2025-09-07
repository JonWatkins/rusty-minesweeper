use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);
static SHOW_MINES_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn is_logging_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

pub fn set_debug_enabled(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn is_show_mines_enabled() -> bool {
    SHOW_MINES_ENABLED.load(Ordering::Relaxed)
}

pub fn set_show_mines_enabled(enabled: bool) {
    SHOW_MINES_ENABLED.store(enabled, Ordering::Relaxed);
}
