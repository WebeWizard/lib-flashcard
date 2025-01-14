use std::sync::Arc;

use crate::FlashManager;
use serde::Deserialize;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use webe_auth::session::Session;
use webe_web::request::Request;
use webe_web::responders::Responder;
use webe_web::responders::static_message::StaticResponder;
use webe_web::response::Response;
use webe_web::validation::Validation;

use async_trait::async_trait;

// Form for targeting a single card
#[derive(Deserialize)]
pub struct UpdateScoreForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    card_id: u64,
    score: u8,
}

pub struct UpdateScoreResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl UpdateScoreResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> UpdateScoreResponder {
        UpdateScoreResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for UpdateScoreResponder {
    async fn build_response(
        &self,
        request: &mut Request,
        _params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
            // TODO: maybe create some convenience function for unwrapping validation and parsing form from reader
            Some(dyn_box) => match dyn_box.downcast::<Session>() {
                Ok(session_box) => match &mut request.message_body {
                    Some(body_reader) => {
                        let mut body = Vec::<u8>::new();
                        // read the entire body or error.
                        // TODO: improve workaround for serde not being able to handle async
                        body_reader
                            .read_to_end(&mut body)
                            .await
                            .map_err(|_e| 400u16)?;
                        match serde_json::from_reader::<_, UpdateScoreForm>(body.as_slice()) {
                            Ok(form) => {
                                match self.flash_manager.lock().await.update_score(
                                    session_box.as_ref(),
                                    form.card_id,
                                    form.score,
                                ) {
                                    Ok(()) => {
                                        let responder = StaticResponder::from_standard_code(200);
                                        return Ok(responder.quick_response());
                                    }
                                    Err(_err) => {
                                        // TODO: Handle session errors / database errors
                                        return Err(500);
                                    }
                                }
                            }
                            Err(_err) => return Err(400), // bad request
                        }
                    }
                    None => return Err(400),
                },
                Err(_err) => return Err(500),
            },
            None => return Err(400),
        }
    }
}

// Deck Scores Responder
pub struct DeckScoresResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
    deck_id_param: String,
}

impl DeckScoresResponder {
    pub fn new(
        flash_manager: Arc<Mutex<FlashManager>>,
        deck_id_param: String,
    ) -> DeckScoresResponder {
        DeckScoresResponder {
            flash_manager: flash_manager,
            deck_id_param: deck_id_param,
        }
    }
}

#[async_trait]
impl Responder for DeckScoresResponder {
    async fn build_response(
        &self,
        _request: &mut Request,
        params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
            // TODO: maybe create some convenience function for unwrapping validation and parsing form from reader
            Some(dyn_box) => match dyn_box.downcast::<Session>() {
                Ok(session_box) => match params
                    .into_iter()
                    .find(|(key, _value)| key == &self.deck_id_param)
                {
                    Some((_key, deck_id_string)) => {
                        match deck_id_string.parse::<u64>() {
                            Ok(deck_id) => {
                                match self
                                    .flash_manager
                                    .lock()
                                    .await
                                    .get_deck_scores(session_box.as_ref(), deck_id)
                                {
                                    Ok(scores) => match serde_json::to_string(&scores) {
                                        Ok(scores_text) => {
                                            let responder = StaticResponder::new(200, scores_text);
                                            return Ok(responder.quick_response());
                                        }
                                        Err(_err) => return Err(500),
                                    },
                                    Err(_err) => {
                                        // TODO: Handle session errors / database errors
                                        return Err(500);
                                    }
                                }
                            }
                            Err(_err) => {
                                dbg!(_err);
                                return Err(400);
                            } // bad request, failed to parse deck id
                        }
                    }
                    None => {
                        dbg!("reached None");
                        return Err(400);
                    }
                },
                Err(_err) => return Err(500),
            },
            None => return Err(400),
        }
    }
}
