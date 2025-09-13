use std::collections::HashMap;

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
发送
*/
#[derive(Debug,Serialize,Deserialize)]
pub struct Init{
	api_id:String,
	grant_type:String,
	refresh_token:String,
}
impl Init {
		pub fn update(mut self,refresh:String)->Self{
			self.refresh_token = refresh;
			self
		}
}

impl XSend for Init  {
	const DOMAIN:&str = constants::AUTH_DOMAIN;
	const URI:&str = constants::AUTH_URI;
	type RECE = Meta;
}
impl XCell for Init  {
		const FILE:&str = constants::INIT_JSON;
}
/*

*/
#[derive(Debug,Serialize,Deserialize)]

pub struct Meta {
	access_token: String,
	refresh_token: String,
	expires_in: i64,
	token_type: String,
	user_id: String,
	user_name: String,
	nick_name: String,
	default_drive_id: String,
	default_sbox_drive_id: String,
	role: String,
	status: String,
	expire_time: String,
	device_id: String,
}
impl Meta {
		pub fn take_refresh(&self)->String{
			self.refresh_token.clone()
		}
		pub fn take_access(&self)->String{
			self.access_token.clone()
		}
		pub fn take_time(&self)->String{
			self.expire_time.clone()
		}
		pub fn take_container_data(&self)->(String,String,String,i64,String,String,String){
			(
				self.token_type.clone(),
				self.access_token.clone(),
				self.refresh_token.clone(),
				self.expires_in,
				self.expire_time.clone(),
				self.device_id.clone(),
				self.user_id.clone(),
			)
		}
		pub fn take_sig_data(&self)->(String,String){
			(self.device_id.clone(),self.user_id.clone())
		}
}
impl XCell for Meta  {
	const FILE:&str = constants::META_JSON;
}
