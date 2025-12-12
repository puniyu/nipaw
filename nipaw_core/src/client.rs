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

use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Client: Send + Sync {
	/// 设置访问令牌
	///
	/// # 参数
	///
	/// * `token` - token
	///
	/// # 示例
	///
	/// ```ignore
	/// client.set_token("token").unwrap();
	/// ```
	fn set_token(&mut self, token: &str) -> Result<()>;

	/// 设置代理
	///
	/// # 参数
	///
	/// * `proxy` - 代理字符串
	///
	/// # 示例
	///
	/// ```ignore
	/// client.set_proxy("http://127.0.0.1:7890").unwrap();
	/// ```
	fn set_proxy(&mut self, proxy: &str) -> Result<()>;

	/// 获取用户实例
	fn user(&self) -> Box<dyn User>;

	/// 获取组织实例
	fn org(&self) -> Box<dyn Org>;

	/// 获取仓库实例
	fn repo(&self) -> Box<dyn Repo>;

	/// 获取提交实例
	fn commit(&self) -> Box<dyn Commit>;

	/// 获取议题实例
	fn issue(&self) -> Box<dyn Issue>;
}
