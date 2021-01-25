use super::auth::{Auth, Create, Login, Session, Verify};

use rocket::{Rocket, State};
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};

#[post("/create", data = "<data>")]
async fn create(auth: State<'_, Auth>, data: Json<Create>) -> super::util::Result<JsonValue> {
    Ok(json!({
        "user_id": auth.inner().create_account(data.into_inner()).await?
    }))
}

#[get("/verify/<code>")]
async fn verify(auth: State<'_, Auth>, code: String) -> super::util::Result<Redirect> {
    auth.inner().verify_account(Verify { code }).await?;
    Ok(Redirect::to(auth.options.verified_uri.clone()))
}

#[post("/login", data = "<data>")]
async fn login(auth: State<'_, Auth>, data: Json<Login>) -> super::util::Result<JsonValue> {
    Ok(json!(auth.inner().login(data.into_inner()).await?))
}

#[get("/check")]
async fn check(_session: Session) -> super::util::Result<()> {
    Ok(())
}

#[get("/user")]
async fn get_account(auth: State<'_, Auth>, session: Session) -> super::util::Result<JsonValue> {
    Ok(json!(auth.get_account(session).await?))
}

#[get("/sessions")]
async fn fetch_sessions(auth: State<'_, Auth>, session: Session) -> super::util::Result<JsonValue> {
    Ok(json!(auth.fetch_all_sessions(session).await?))
}

#[delete("/sessions/<id>")]
async fn deauth_session(
    auth: State<'_, Auth>,
    session: Session,
    id: String,
) -> super::util::Result<()> {
    auth.deauth_session(session, id).await
}

#[get("/logout")]
async fn logout(auth: State<'_, Auth>, session: Session) -> super::util::Result<()> {
    let id = session.id.clone().unwrap();
    auth.deauth_session(session, id).await
}

pub fn mount(rocket: Rocket, path: &str, auth: Auth) -> Rocket {
    rocket.manage(auth).mount(
        path,
        routes![
            create,
            verify,
            login,
            get_account,
            check,
            fetch_sessions,
            deauth_session,
            logout
        ],
    )
}
