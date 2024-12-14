use std::sync::Arc;

use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
use futures_util::StreamExt;
use log::warn;
use tokio::task;
use usecases::AsyaResponse;

use crate::{requests::Requests, responses::Responses};

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            if let Ok(AggregatedMessage::Text(text)) = msg {
                handle_message(&mut session, text.to_string()).await;
            }
        }
    });

    Ok(res)
}

async fn handle_message(session: &mut Session, input: String) {
    let request = extract_request(input);

    let Requests::General { action } = request;
    const DEFAULT_EXPECT_MSG: &str = "The Responses enum should be able to be converted into JSON";
    usecases::subscribe_once({
        let session = session.clone(); // should be a clone of the session?
        move |event: Arc<AsyaResponse>| {
            let mut session = session.clone();
            task::spawn(async move {
                let response = Responses::Base {
                    is_err: false,
                    message: event.to_string(),
                };

                session
                    .text(
                        serde_json::to_string(&response)
                            .expect(DEFAULT_EXPECT_MSG)
                            .to_string(),
                    )
                    .await
                    .unwrap();
            })
        }
    })
    .await;
    usecases::dispatch_usecase(action, "".to_string()).await;
}

fn extract_request(input: String) -> Requests {
    serde_json::from_str::<Requests>(&input).unwrap_or_else(|err| {
        warn!("{:?}", err);
        Requests::General {
            action: "".to_string(),
        }
    })
}
