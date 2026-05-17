use leptos::prelude::*;
use leptos::server_fn::ServerFnError;

#[server]
pub async fn submit_contact_message(
    name: String,
    email: String,
    subject: String,
    body: String,
) -> Result<(), ServerFnError> {
    use application::dto::ContactMessageDto;

    let state = expect_context::<crate::state::AppState>();
    let dto = ContactMessageDto {
        name,
        email,
        subject: if subject.trim().is_empty() {
            None
        } else {
            Some(subject)
        },
        body,
    };
    state.submit_message.execute(dto).await?;
    Ok(())
}
