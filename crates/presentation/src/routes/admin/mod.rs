pub mod auth;
pub mod dashboard;
pub mod layout;
pub mod messages;
pub mod posts;
pub mod profile;
pub mod projects;
pub mod skills;

pub use auth::{LoginPage, SetupPage};
pub use dashboard::AdminDashboardPage;
pub use messages::AdminMessagesPage;
pub use posts::{AdminPostEditPage, AdminPostsPage};
pub use profile::AdminProfilePage;
pub use projects::{AdminProjectEditPage, AdminProjectsPage};
pub use skills::AdminSkillsPage;
