mod user;
#[doc(inline)]
pub use user::User;
mod repo;
#[doc(inline)]
pub use repo::Repo;
mod commit;
#[doc(inline)]
pub use commit::Commit;
mod org;
#[doc(inline)]
pub use org::Org;
mod issue;
#[doc(inline)]
pub use issue::Issue;
mod release;
#[doc(inline)]
pub use release::Release;

use std::error::Error;

pub trait Provider: Send + Sync {
	type Err: Error;
	type User: User<Err = Self::Err>;
	type Org: Org<Err = Self::Err>;
	type Repo: Repo<Err = Self::Err>;
	type Commit: Commit<Err = Self::Err>;
	type Issue: Issue<Err = Self::Err>;
	type Release: Release<Err = Self::Err>;

	fn user(&self) -> &Self::User;
	fn org(&self) -> &Self::Org;
	fn repo(&self) -> &Self::Repo;
	fn commit(&self) -> &Self::Commit;
	fn issue(&self) -> &Self::Issue;
	fn release(&self) -> &Self::Release;
}
