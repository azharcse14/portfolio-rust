//! Composition root — wires repositories into use-cases and bundles them
//! into a single `AppState` provided via Leptos context to server functions.

#[cfg(feature = "ssr")]
mod ssr {
    use std::sync::Arc;

    use application::{
        contact::SubmitMessage,
        posts::{GetPost, ListPublishedPosts},
        profile::GetProfile,
        projects::{GetProject, ListProjects},
        skills::ListSkills,
    };
    use infrastructure::{
        sqlite::{
            SqliteMessageRepository, SqlitePostRepository, SqliteProfileRepository,
            SqliteProjectRepository, SqliteSkillRepository,
        },
        Database,
    };

    #[derive(Clone)]
    pub struct AppState {
        pub list_projects: Arc<ListProjects>,
        pub get_project: Arc<GetProject>,
        pub list_posts: Arc<ListPublishedPosts>,
        pub get_post: Arc<GetPost>,
        pub list_skills: Arc<ListSkills>,
        pub get_profile: Arc<GetProfile>,
        pub submit_message: Arc<SubmitMessage>,
    }

    impl AppState {
        pub fn build(pool: Database) -> Self {
            let project_repo = Arc::new(SqliteProjectRepository::new(pool.clone()));
            let post_repo = Arc::new(SqlitePostRepository::new(pool.clone()));
            let skill_repo = Arc::new(SqliteSkillRepository::new(pool.clone()));
            let profile_repo = Arc::new(SqliteProfileRepository::new(pool.clone()));
            let message_repo = Arc::new(SqliteMessageRepository::new(pool));

            Self {
                list_projects: Arc::new(ListProjects::new(project_repo.clone())),
                get_project: Arc::new(GetProject::new(project_repo)),
                list_posts: Arc::new(ListPublishedPosts::new(post_repo.clone())),
                get_post: Arc::new(GetPost::new(post_repo)),
                list_skills: Arc::new(ListSkills::new(skill_repo)),
                get_profile: Arc::new(GetProfile::new(profile_repo)),
                submit_message: Arc::new(SubmitMessage::new(message_repo)),
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub use ssr::AppState;
