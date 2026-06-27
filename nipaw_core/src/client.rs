mod user;
pub use user::User;
mod repo;
pub use repo::Repo;
mod commit;
pub use commit::Commit;
mod org;
pub use org::Org;
mod issue;
pub use issue::Issue;
mod release;
pub use release::Release;

use crate::Result;

/// 访问令牌
pub trait Token: Send + Sync {
	/// 设置访问令牌
	fn set_token(&mut self, token: &str) -> Result<()>;
}

/// 代理
pub trait Proxy: Send + Sync {
	/// 设置代理
	fn set_proxy(&mut self, proxy: &str) -> Result<()>;
}


pub trait Config: Token + Proxy {}

impl<T> Config for T where T: Token + Proxy {}

pub trait Provider: Send + Sync {
	type User: User;
	type Org: Org;
	type Repo: Repo;
	type Commit: Commit;
	type Issue: Issue;
	type Release: Release;

	fn user(&self) -> Self::User;
	fn org(&self) -> Self::Org;
	fn repo(&self) -> Self::Repo;
	fn commit(&self) -> Self::Commit;
	fn issue(&self) -> Self::Issue;
	fn release(&self) -> Self::Release;
}

pub trait Client: Config + Provider {}

impl<T> Client for T where T: Config + Provider {}
