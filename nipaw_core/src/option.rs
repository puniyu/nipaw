pub mod commit;
pub mod issue;
pub mod release;
pub mod repo;
pub(super) const fn default_per_page() -> Option<u32> {
	Some(30)
}

pub(super) const fn default_page() -> Option<u32> {
	Some(1)
}
