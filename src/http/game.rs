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
pub struct UpdateScoreForm {
  #[serde(deserialize_with = "webe_auth::utility::deserialize_from_string")]
  card_id: u64,
  score: u8,
}

pub struct UpdateScoreResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
}

impl<'f> UpdateScoreResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager) -> UpdateScoreResponder<'f> {
    UpdateScoreResponder {
      flash_manager: flash_manager,
    }
  }
}

impl<'f> Responder for UpdateScoreResponder<'f> {
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
          Some(body_reader) => match serde_json::from_reader::<_, UpdateScoreForm>(body_reader) {
            Ok(form) => {
              match self
                .flash_manager
                .update_score(session_box.as_ref(), form.card_id, form.score)
              {
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

// Deck Scores Responder
pub struct DeckScoresResponder<'f> {
  flash_manager: &'f FlashManager<'f>,
  deck_id_param: String,
}

impl<'f> DeckScoresResponder<'f> {
  pub fn new(flash_manager: &'f FlashManager, deck_id_param: String) -> DeckScoresResponder<'f> {
    DeckScoresResponder {
      flash_manager: flash_manager,
      deck_id_param: deck_id_param,
    }
  }
}

impl<'f> Responder for DeckScoresResponder<'f> {
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
        Ok(session_box) => match params.get(&self.deck_id_param) {
          Some(deck_id_string) => {
            match deck_id_string.parse::<u64>() {
              Ok(deck_id) => {
                match self
                  .flash_manager
                  .get_deck_scores(session_box.as_ref(), deck_id)
                {
                  Ok(scores) => match serde_json::to_string(&scores) {
                    Ok(scores_text) => {
                      let responder = StaticResponder::new(200, scores_text);
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
