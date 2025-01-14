use std::sync::Arc;
use tokio::sync::Mutex;

use crate::FlashManager;
use serde::Deserialize;
use webe_auth::session::Session;
use webe_web::request::Request;
use webe_web::responders::Responder;
use webe_web::responders::static_message::StaticResponder;
use webe_web::response::Response;
use webe_web::validation::Validation;

use async_trait::async_trait;

use tokio::io::AsyncReadExt;

// Form for targeting a single deck
#[derive(Deserialize)]
pub struct DeckIdForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    pub deck_id: u64,
}

// FETCH DECKS FOR ACCOUNT
pub struct DecksResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl DecksResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> DecksResponder {
        DecksResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for DecksResponder {
    async fn build_response(
        &self,
        _request: &mut Request,
        _params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
            // TODO: maybe create some convenience function for unwrapping validation and parsing form from reader
            Some(dyn_box) => match dyn_box.downcast::<Session>() {
                Ok(session_box) => match self
                    .flash_manager
                    .lock()
                    .await
                    .get_decks_for_session(session_box.as_ref())
                {
                    Ok(decks) => match serde_json::to_string(&decks) {
                        Ok(deck_text) => {
                            let responder = StaticResponder::new(200, deck_text);
                            return Ok(responder.quick_response());
                        }
                        Err(_err) => {
                            dbg!(_err);
                            return Err(500);
                        } // TODO: parse session error, not found error, etc
                    },
                    Err(_err) => {
                        println!("manager error");
                        dbg!(session_box);
                        return Err(500);
                    }
                },
                Err(_err) => {
                    println!("session error");
                    dbg!(_err);
                    return Err(500);
                }
            },
            None => return Err(400),
        }
    }
}

// FETCH SINGLE DECK WITH CARDS

pub struct DeckDetailsResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
    deck_id_param: String,
}

impl DeckDetailsResponder {
    pub fn new(
        flash_manager: Arc<Mutex<FlashManager>>,
        deck_id_param: String,
    ) -> DeckDetailsResponder {
        DeckDetailsResponder {
            flash_manager: flash_manager,
            deck_id_param: deck_id_param,
        }
    }
}

#[async_trait]
impl Responder for DeckDetailsResponder {
    async fn build_response(
        &self,
        _request: &mut Request,
        params: &Vec<(String, String)>,
        validation: Validation,
    ) -> Result<Response, u16> {
        // Expecting session from an outer SecureResponder
        match validation {
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
                                    .get_deck_details(session_box.as_ref(), &deck_id)
                                {
                                    Ok(details) => match serde_json::to_string(&details) {
                                        Ok(details_text) => {
                                            let responder = StaticResponder::new(200, details_text);
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
                            Err(_err) => return Err(400),
                        }
                    }
                    None => {
                        println!("made it here");
                        return Err(400);
                    }
                },
                Err(_err) => return Err(500),
            },
            None => return Err(400),
        }
    }
}

// CREATE DECK
#[derive(Deserialize)]
pub struct CreateDeckForm {
    pub name: String,
}

pub struct CreateDeckResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl CreateDeckResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> CreateDeckResponder {
        CreateDeckResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for CreateDeckResponder {
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
                        match serde_json::from_reader::<_, CreateDeckForm>(body.as_slice()) {
                            Ok(form) => {
                                match self
                                    .flash_manager
                                    .lock()
                                    .await
                                    .create_deck(session_box.as_ref(), form.name)
                                {
                                    Ok(deck) => match serde_json::to_string(&deck) {
                                        Ok(deck_text) => {
                                            let responder = StaticResponder::new(200, deck_text);
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

// RENAME DECK
#[derive(Deserialize)]
pub struct RenameDeckForm {
    #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
    deck_id: u64,
    name: String,
}

pub struct UpdateDeckResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl UpdateDeckResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> UpdateDeckResponder {
        UpdateDeckResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for UpdateDeckResponder {
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
                        match serde_json::from_reader::<_, RenameDeckForm>(body.as_slice()) {
                            Ok(form) => {
                                match self.flash_manager.lock().await.rename_deck(
                                    session_box.as_ref(),
                                    form.deck_id,
                                    form.name.as_str(),
                                ) {
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

// DELETE DECK

pub struct DeleteDeckResponder {
    flash_manager: Arc<Mutex<FlashManager>>,
}

impl DeleteDeckResponder {
    pub fn new(flash_manager: Arc<Mutex<FlashManager>>) -> DeleteDeckResponder {
        DeleteDeckResponder {
            flash_manager: flash_manager,
        }
    }
}

#[async_trait]
impl Responder for DeleteDeckResponder {
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
                        match serde_json::from_reader::<_, DeckIdForm>(body.as_slice()) {
                            Ok(form) => {
                                match self
                                    .flash_manager
                                    .lock()
                                    .await
                                    .delete_deck(session_box.as_ref(), form.deck_id)
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
