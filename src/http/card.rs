use crate::{FlashError, FlashManager};
use serde::Deserialize;
use webe_web::request::Request;
use webe_web::responders::Responder;
use webe_web::response::Response;
use webe_web::validation::{Validation, ValidationResult};

use std::collections::HashMap;

pub struct CreateCardForm {
  deck_id: u64,
  deck_pos: u64,
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
    _params: &HashMap<String, String>,
    _validation: Validation,
  ) -> Result<Response, u16> {
    // get the session from the validation
    unimplemented!()
  }
}

pub struct UpdateCardForm {
  deck_id: u64,
  deck_pos: Option<u64>,
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
    _params: &HashMap<String, String>,
    _validation: Validation,
  ) -> Result<Response, u16> {
    // get the session from the validation
    // make sure the account on the session matches the account on the deck
    //  - logic for this should be handled in manager

    unimplemented!()
  }
}
