//! Composition root — wires repositories + crypto into use-cases and bundles
//! them into a single `AppState` provided via Leptos context to server functions.

#[cfg(feature = "ssr")]
mod ssr {
    use std::sync::Arc;

    use application::{
        auth::{IsSetupComplete, Login, RegisterAdmin},
        contact::{DeleteMessage, ListMessages, MarkMessageRead, SubmitMessage},
        posts::{CreatePost, DeletePost, GetPost, ListAllPosts, ListPublishedPosts, UpdatePost},
        profile::{GetProfile, UpdateProfile},
        projects::{CreateProject, DeleteProject, GetProject, ListProjects, UpdateProject},
        skills::{CreateSkill, DeleteSkill, ListSkills},
    };
    use domain::ports::TokenIssuer;
    use infrastructure::{
        auth::{Argon2Hasher, JwtIssuer},
        sqlite::{
            SqliteMessageRepository, SqlitePostRepository, SqliteProfileRepository,
            SqliteProjectRepository, SqliteSkillRepository, SqliteUserRepository,
        },
        AppConfig, Database,
    };

    #[derive(Clone)]
    pub struct AppState {
        // public reads
        pub list_projects: Arc<ListProjects>,
        pub get_project: Arc<GetProject>,
        pub list_posts: Arc<ListPublishedPosts>,
        pub get_post: Arc<GetPost>,
        pub list_skills: Arc<ListSkills>,
        pub get_profile: Arc<GetProfile>,
        pub submit_message: Arc<SubmitMessage>,

        // admin: projects
        pub create_project: Arc<CreateProject>,
        pub update_project: Arc<UpdateProject>,
        pub delete_project: Arc<DeleteProject>,

        // admin: posts
        pub list_all_posts: Arc<ListAllPosts>,
        pub create_post: Arc<CreatePost>,
        pub update_post: Arc<UpdatePost>,
        pub delete_post: Arc<DeletePost>,

        // admin: skills
        pub create_skill: Arc<CreateSkill>,
        pub delete_skill: Arc<DeleteSkill>,

        // admin: profile
        pub update_profile: Arc<UpdateProfile>,

        // admin: messages
        pub list_messages: Arc<ListMessages>,
        pub mark_message_read: Arc<MarkMessageRead>,
        pub delete_message: Arc<DeleteMessage>,

        // auth
        pub login: Arc<Login>,
        pub register_admin: Arc<RegisterAdmin>,
        pub is_setup_complete: Arc<IsSetupComplete>,
        pub tokens: Arc<dyn TokenIssuer>,
    }

    impl AppState {
        pub fn build(pool: Database, cfg: &AppConfig) -> Self {
            let project_repo = Arc::new(SqliteProjectRepository::new(pool.clone()));
            let post_repo = Arc::new(SqlitePostRepository::new(pool.clone()));
            let skill_repo = Arc::new(SqliteSkillRepository::new(pool.clone()));
            let profile_repo = Arc::new(SqliteProfileRepository::new(pool.clone()));
            let message_repo = Arc::new(SqliteMessageRepository::new(pool.clone()));
            let user_repo = Arc::new(SqliteUserRepository::new(pool));

            let hasher = Arc::new(Argon2Hasher::new());
            let tokens: Arc<dyn TokenIssuer> = Arc::new(JwtIssuer::new(&cfg.jwt_secret));

            Self {
                list_projects: Arc::new(ListProjects::new(project_repo.clone())),
                get_project: Arc::new(GetProject::new(project_repo.clone())),
                create_project: Arc::new(CreateProject::new(project_repo.clone())),
                update_project: Arc::new(UpdateProject::new(project_repo.clone())),
                delete_project: Arc::new(DeleteProject::new(project_repo)),

                list_posts: Arc::new(ListPublishedPosts::new(post_repo.clone())),
                get_post: Arc::new(GetPost::new(post_repo.clone())),
                list_all_posts: Arc::new(ListAllPosts::new(post_repo.clone())),
                create_post: Arc::new(CreatePost::new(post_repo.clone())),
                update_post: Arc::new(UpdatePost::new(post_repo.clone())),
                delete_post: Arc::new(DeletePost::new(post_repo)),

                list_skills: Arc::new(ListSkills::new(skill_repo.clone())),
                create_skill: Arc::new(CreateSkill::new(skill_repo.clone())),
                delete_skill: Arc::new(DeleteSkill::new(skill_repo)),

                get_profile: Arc::new(GetProfile::new(profile_repo.clone())),
                update_profile: Arc::new(UpdateProfile::new(profile_repo)),

                submit_message: Arc::new(SubmitMessage::new(message_repo.clone())),
                list_messages: Arc::new(ListMessages::new(message_repo.clone())),
                mark_message_read: Arc::new(MarkMessageRead::new(message_repo.clone())),
                delete_message: Arc::new(DeleteMessage::new(message_repo)),

                login: Arc::new(Login::new(
                    user_repo.clone(),
                    hasher.clone(),
                    tokens.clone(),
                )),
                register_admin: Arc::new(RegisterAdmin::new(user_repo.clone(), hasher)),
                is_setup_complete: Arc::new(IsSetupComplete::new(user_repo)),
                tokens,
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub use ssr::AppState;
