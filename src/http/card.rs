use std::sync::Arc;

use crate::FlashManager;
use serde::Deserialize;
use tokio::sync::Mutex;
use webe_auth::session::Session;
use webe_web::request::Request;
use webe_web::responders::Responder;
use webe_web::responders::static_message::StaticResponder;
use webe_web::response::Response;
use webe_web::validation::Validation;

use async_trait::async_trait;

use tokio::io::AsyncReadExt;

// Form for targeting a single card
#[derive(Deserialize)]
pub struct CardIdForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    card_id: u64,
}

#[derive(Deserialize)]
pub struct CreateCardForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    deck_id: u64,
    deck_pos: u16,
    question: String,
    answer: String,
}

pub struct CreateCardResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl CreateCardResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> CreateCardResponder {
        CreateCardResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for CreateCardResponder {
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
                        match serde_json::from_reader::<_, CreateCardForm>(body.as_slice()) {
                            Ok(form) => {
                                match self.flash_manager.lock().await.create_card(
                                    session_box.as_ref(),
                                    form.deck_id,
                                    form.deck_pos,
                                    form.question,
                                    form.answer,
                                ) {
                                    Ok(card) => match serde_json::to_string(&card) {
                                        Ok(card_text) => {
                                            let responder = StaticResponder::new(200, card_text);
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

#[derive(Deserialize)]
pub struct UpdateCardForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    id: u64,
    question: Option<String>,
    answer: Option<String>,
}

pub struct UpdateCardResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl UpdateCardResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> UpdateCardResponder {
        UpdateCardResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for UpdateCardResponder {
    async fn build_response(
        &self,
        request: &mut Request,
        _params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
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
                        match serde_json::from_reader::<_, UpdateCardForm>(body.as_slice()) {
                            Ok(form) => {
                                match self.flash_manager.lock().await.update_card(
                                    session_box.as_ref(),
                                    form.id,
                                    form.question,
                                    form.answer,
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

// UPDATE CARD POSITION

#[derive(Deserialize)]
pub struct UpdateCardPositionForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    deck_id: u64,
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    id: u64,
    orig_pos: u16,
    new_pos: u16,
}

pub struct UpdateCardPositionResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl UpdateCardPositionResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> UpdateCardPositionResponder {
        UpdateCardPositionResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for UpdateCardPositionResponder {
    async fn build_response(
        &self,
        request: &mut Request,
        _params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
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
                        match serde_json::from_reader::<_, UpdateCardPositionForm>(body.as_slice())
                        {
                            Ok(form) => {
                                match self.flash_manager.lock().await.update_card_position(
                                    session_box.as_ref(),
                                    form.id,
                                    form.deck_id,
                                    form.orig_pos,
                                    form.new_pos,
                                ) {
                                    Ok(()) => {
                                        let responder = StaticResponder::from_standard_code(200);
                                        return Ok(responder.quick_response());
                                    }
                                    Err(_err) => {
                                        dbg!(_err);
                                        // TODO: Handle session errors / database errors
                                        return Err(500);
                                    }
                                }
                            }
                            Err(_err) => return Err(400),
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

// DELETE CARD

pub struct DeleteCardResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl DeleteCardResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> DeleteCardResponder {
        DeleteCardResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for DeleteCardResponder {
    async fn build_response(
        &self,
        request: &mut Request,
        _params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
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
                        match serde_json::from_reader::<_, CardIdForm>(body.as_slice()) {
                            Ok(form) => {
                                match self
                                    .flash_manager
                                    .lock()
                                    .await
                                    .delete_card(session_box.as_ref(), form.card_id)
                                {
                                    Ok(()) => {
                                        let responder = StaticResponder::from_standard_code(200);
                                        return Ok(responder.quick_response());
                                    }
                                    Err(_err) => {
                                        // TODO: Handle session errors / database errors                {
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
