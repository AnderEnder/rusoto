#![cfg(feature = "lex-models")]

extern crate rusoto_core;
extern crate rusoto_lex_models;

use rusoto_core::Region;
use rusoto_lex_models::{GetBotsRequest, LexModels, LexModelsClient};

#[test]
fn should_get_bots() {
    let client = LexModelsClient::new(Region::UsEast1);
    let request = GetBotsRequest::default();

    let result = client.get_bots(request).sync().unwrap();
    println!("{:#?}", result);
}
