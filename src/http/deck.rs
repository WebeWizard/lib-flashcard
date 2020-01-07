use crate::FlashManager;
use serde::Deserialize;
use webe_auth::session::Session;
use webe_web::request::Request;
use webe_web::responders::static_message::StaticResponder;
use webe_web::responders::Responder;
use webe_web::response::Response;
use webe_web::validation::Validation;

use std::collections::HashMap;

// FETCH DECKS FOR ACCOUNT
pub struct DecksResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> DecksResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager<'f>) -> DecksResponder<'f> {
    DecksResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for DecksResponder<'f> {
  fn build_response(
    &self,
    request: &mut Request,
    params: &HashMap<String, String>,
    validation: Validation,
  ) -> Result<Response, u16> {
    // Expecting session from an outer SecureResponder
    match validation {
      // TODO: maybe create some convenience function for unwrapping validation and parsing form from reader
      Some(dyn_box) => match dyn_box.downcast::<Session>() {
        Ok(session_box) => match self
          .flash_manager
          .get_decks_for_session(session_box.as_ref())
        {
          Ok(decks) => match serde_json::to_string(&decks) {
            Ok(deck_text) => {
              let responder = StaticResponder::new(200, deck_text);
              return responder.build_response(request, params, None);
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

// CREATE DECK
#[derive(Deserialize)]
pub struct CreateDeckForm {
  pub name: String,
}

pub struct CreateDeckResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> CreateDeckResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager<'f>) -> CreateDeckResponder<'f> {
    CreateDeckResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for CreateDeckResponder<'f> {
  fn build_response(
    &self,
    request: &mut Request,
    params: &HashMap<String, String>,
    validation: Validation,
  ) -> Result<Response, u16> {
    // Expecting session from an outer SecureResponder
    match validation {
      // TODO: maybe create some convenience function for unwrapping validation and parsing form from reader
      Some(dyn_box) => match dyn_box.downcast::<Session>() {
        Ok(session_box) => match &mut request.message_body {
          Some(body_reader) => match serde_json::from_reader::<_, CreateDeckForm>(body_reader) {
            Ok(form) => {
              match self
                .flash_manager
                .create_deck(session_box.as_ref(), form.name)
              {
                Ok(deck) => match serde_json::to_string(&deck) {
                  Ok(deck_text) => {
                    let responder = StaticResponder::new(200, deck_text);
                    return responder.build_response(request, params, None);
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
          },
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

pub struct UpdateDeckResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> UpdateDeckResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager) -> UpdateDeckResponder<'f> {
    UpdateDeckResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for UpdateDeckResponder<'f> {
  fn build_response(
    &self,
    request: &mut Request,
    params: &HashMap<String, String>,
    validation: Validation,
  ) -> Result<Response, u16> {
    // Expecting session from an outer SecureResponder
    match validation {
      Some(dyn_box) => match dyn_box.downcast::<Session>() {
        Ok(session_box) => match &mut request.message_body {
          Some(body_reader) => match serde_json::from_reader::<_, RenameDeckForm>(body_reader) {
            Ok(form) => {
              match self.flash_manager.rename_deck(
                session_box.as_ref(),
                form.deck_id,
                form.name.as_str(),
              ) {
                Ok(()) => {
                  let responder = StaticResponder::from_standard_code(200);
                  return responder.build_response(request, params, None);
                }
                Err(_err) => {
                  // TODO: Handle session errors / database errors                {
                  return Err(500);
                }
              }
            }
            Err(_err) => return Err(400), // bad request
          },
          None => return Err(400),
        },
        Err(_err) => return Err(500),
      },
      None => return Err(400),
    }
  }
}

// DELETE DECK
#[derive(Deserialize)]
pub struct DeleteDeckForm {
  #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
  deck_id: u64,
  name: String,
}

pub struct DeleteDeckResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> DeleteDeckResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager) -> DeleteDeckResponder<'f> {
    DeleteDeckResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for DeleteDeckResponder<'f> {
  fn build_response(
    &self,
    request: &mut Request,
    params: &HashMap<String, String>,
    validation: Validation,
  ) -> Result<Response, u16> {
    // Expecting session from an outer SecureResponder
    match validation {
      Some(dyn_box) => match dyn_box.downcast::<Session>() {
        Ok(session_box) => match &mut request.message_body {
          Some(body_reader) => match serde_json::from_reader::<_, DeleteDeckForm>(body_reader) {
            Ok(form) => {
              match self
                .flash_manager
                .delete_deck(session_box.as_ref(), form.deck_id)
              {
                Ok(()) => {
                  let responder = StaticResponder::from_standard_code(200);
                  return responder.build_response(request, params, None);
                }
                Err(_err) => {
                  // TODO: Handle session errors / database errors                {
                  return Err(500);
                }
              }
            }
            Err(_err) => return Err(400), // bad request
          },
          None => return Err(400),
        },
        Err(_err) => return Err(500),
      },
      None => return Err(400),
    }
  }
}
