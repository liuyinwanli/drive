use std::collections::HashMap;
use clap::builder::Str;
use serde::{
	Serialize,
	Deserialize,
	de::DeserializeOwned,
};
use serde_json::{
	Value,
};
use super::traits::{
	XCell,
	XSend,
};
use crate::constants;
/*
*/
#[derive(Debug,Serialize,Deserialize)]
pub struct UserInfo_S {
}
impl XSend for UserInfo_S  {
  const DOMAIN:&str = constants::USER_DOMAIN;
  const URI:&str = constants::USER_INFO;
  type RECE = UserInfo_R;
}
impl UserInfo_S {
    pub fn new()->Self{
      Self{ }
    }
}
/*
*/
#[derive(Debug,Serialize,Deserialize)]
pub struct UserInfo_R  {
	domain_id: String,
	user_id: String,
	avatar: String,
	created_at: i64,
	updated_at: i64,
	email: String,
	nick_name: String,
	phone: String,
	role: String,
	status: String,
	user_name: String,
	description: String,
	default_drive_id: String,
  backup_drive_id: String,
  resource_drive_id: String,
}
impl XCell for UserInfo_R  {
  const FILE:&str = constants::USER_INFO_JSON;
}
/*
*/