use serde::Deserialize;
use webe_auth::WebeAuth;
use webe_web::request::Request;
use webe_web::responders::static_message::StaticResponder;
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

pub struct CreateCardResponder<'w> {
  auth_manager: &'w WebeAuth<'w>,
}

impl<'w> CreateCardResponder<'w> {
  pub fn new(auth_manager: &'w WebeAuth) -> CreateCardResponder<'w> {
    CreateCardResponder {
      auth_manager: auth_manager,
    }
  }
}

impl<'w> Responder for CreateCardResponder<'w> {
  fn validate(&self, _request: &Request, _params: &HashMap<String, String>) -> ValidationResult {
    // make sure session header belongs to a valid session
    unimplemented!()
  }

  fn build_response(
    &self,
    request: &mut Request,
    _params: &HashMap<String, String>,
    _validation: Validation,
  ) -> Result<Response, u16> {
    // get the session from the validation
    // create the card with owner set to the account on the session
    unimplemented!()
  }
}

pub struct UpdateCardForm {
  deck_id: u64,
  deck_pos: Option<u64>,
  question: Option<String>,
  answer: Option<String>,
}

pub struct UpdateDeckResponder<'w> {
  auth_manager: &'w WebeAuth<'w>,
}

impl<'w> UpdateDeckResponder<'w> {
  pub fn new(auth_manager: &'w WebeAuth) -> UpdateDeckResponder<'w> {
    UpdateDeckResponder {
      auth_manager: auth_manager,
    }
  }
}

impl<'w> Responder for UpdateDeckResponder<'w> {
  fn validate(&self, _request: &Request, _params: &HashMap<String, String>) -> ValidationResult {
    // make sure session header belongs to a valid session
    unimplemented!()
  }

  fn build_response(
    &self,
    request: &mut Request,
    _params: &HashMap<String, String>,
    _validation: Validation,
  ) -> Result<Response, u16> {
    // get the session from the validation
    // make sure the account on the session matches the account on the deck
    unimplemented!()
  }
}
