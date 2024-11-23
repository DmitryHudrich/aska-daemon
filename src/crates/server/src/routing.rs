use actix_web::{dev::ServiceFactory, middleware, web, App, Error};
use actix_ws::Session;
use log::warn;

use crate::{
    requests::{MusicAction, Requests},
    ws_utils,
};

pub fn route_all() -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
        .route("/hey", web::get().to(|| async { "bebra" }))
        .route("/sex", web::get().to(|| async { "не было" }))
        .route("/ws", web::get().to(ws_utils::ws_handler))
}

pub async fn route_ws(session: &mut Session, input: String) {
    let request = extract_request(input);
    if let Requests::Music { action } = request {
        match action {
            MusicAction::PlayPause => {
                features::services::commands::music::play_pause();
            }
            MusicAction::GetStatus => {
                let status = features::services::commands::music::get_status();
                session.text(format!("{:?}", status)).await.unwrap();
            }
        }
    }
}

fn extract_request(input: String) -> Requests {
    serde_json::from_str::<Requests>(&input).unwrap_or_else(|err| {
        warn!("{:?}", err);
        Requests::Empty
    })
}
