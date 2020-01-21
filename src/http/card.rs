use crate::FlashManager;
use serde::Deserialize;
use webe_auth::session::Session;
use webe_web::request::Request;
use webe_web::responders::static_message::StaticResponder;
use webe_web::responders::Responder;
use webe_web::response::Response;
use webe_web::validation::Validation;

use std::collections::HashMap;

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

pub struct CreateCardResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> CreateCardResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager) -> CreateCardResponder<'f> {
    CreateCardResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for CreateCardResponder<'f> {
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
          Some(body_reader) => match serde_json::from_reader::<_, CreateCardForm>(body_reader) {
            Ok(form) => {
              match self.flash_manager.create_card(
                session_box.as_ref(),
                form.deck_id,
                form.deck_pos,
                form.question,
                form.answer,
              ) {
                Ok(card) => match serde_json::to_string(&card) {
                  Ok(card_text) => {
                    let responder = StaticResponder::new(200, card_text);
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

#[derive(Deserialize)]
pub struct UpdateCardForm {
  #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
  id: u64,
  deck_pos: Option<u16>,
  question: Option<String>,
  answer: Option<String>,
}

pub struct UpdateCardResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> UpdateCardResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager) -> UpdateCardResponder<'f> {
    UpdateCardResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for UpdateCardResponder<'f> {
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
          Some(body_reader) => match serde_json::from_reader::<_, UpdateCardForm>(body_reader) {
            Ok(form) => {
              match self.flash_manager.update_card(
                session_box.as_ref(),
                form.id,
                form.deck_pos,
                form.question,
                form.answer,
              ) {
                Ok(()) => {
                  let responder = StaticResponder::from_standard_code(200);
                  return responder.build_response(request, params, None);
                }
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

// DELETE CARD

pub struct DeleteCardResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> DeleteCardResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager) -> DeleteCardResponder<'f> {
    DeleteCardResponder {
      flash_manager: flash_manager,
    }
  }
}
impl<'f> Responder for DeleteCardResponder<'f> {
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
          Some(body_reader) => match serde_json::from_reader::<_, CardIdForm>(body_reader) {
            Ok(form) => {
              match self
                .flash_manager
                .delete_card(session_box.as_ref(), form.card_id)
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
