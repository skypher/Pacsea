//! Core non-UI logic split into modular submodules.

pub mod deps;
pub mod distro;
pub mod files;
pub mod filter;
pub mod gating;
pub mod lists;
pub mod prefetch;
pub mod preflight;
pub mod query;
pub mod sandbox;
pub mod selection;
pub mod services;
pub mod sort;
pub mod summary;

// Re-export public APIs to preserve existing import paths (crate::logic::...)
pub use filter::apply_filters_and_sort_preserve_selection;
pub use gating::{is_allowed, set_allowed_only_selected, set_allowed_ring};
pub use lists::{add_to_downgrade_list, add_to_install_list, add_to_remove_list};
pub use prefetch::ring_prefetch_from_selected;
pub use query::send_query;
pub use selection::move_sel_cached;
pub use services::resolve_service_impacts;
pub use sort::sort_results_preserve_selection;

#[cfg(test)]
static TEST_MUTEX: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();

#[cfg(test)]
pub(crate) fn test_mutex() -> &'static std::sync::Mutex<()> {
    TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(()))
}

#[cfg(test)]
/// What: Acquire test mutex lock with automatic poison recovery.
pub(crate) fn lock_test_mutex() -> std::sync::MutexGuard<'static, ()> {
    test_mutex().lock().unwrap_or_else(|e| e.into_inner())
}
pub use summary::compute_post_summary;
