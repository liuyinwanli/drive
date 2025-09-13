use std::collections::HashMap;
use hex;
use serde::{
	Deserialize,
	Serialize,
	de::DeserializeOwned,
};
use crate::xapp::traits::XCell;

use crate::constants;
use super::get_access_token::Meta;
use super::traits;
use secp256k1::{rand};
use secp256k1::{Secp256k1, Message};
use secp256k1::hashes::{sha256, Hash};

#[derive(Debug,Serialize,Deserialize)]
pub struct Container{
	access_token_type: String,
	access_token: String,
	refresh_token: String,
	expires_in: i64,
	expire_time: String,
	app_id: String,
	device_id: String,
	user_id: String,
	nonce: i64,
	device_name: String,
	model_name: String,
	secret_key:[u8;32],
	public_key:String,
	signature_data: String,
}
impl traits::XCell for Container {
		const FILE:&str = constants::CONTAINER_JSON;
}
impl Container {
	pub fn update(mut self,refresh:String,access:String,time:String)->Self{
		self.refresh_token = refresh;
		self.access_token = access;
		self.expire_time = time;
		self
	}
	pub fn get_sig_header(&self)->HashMap<String,String>{
		let mut ret = HashMap::new();
		ret.insert("x-device-id".to_string(), self.device_id.clone());
		ret.insert("x-signature".to_string(), self.signature_data.clone());
		ret.insert("authorization".to_string(), format!("{} {}",self.access_token_type,self.access_token));
		ret
	}
	pub fn get_auth_header(&self)->HashMap<String,String>{
		let mut ret = HashMap::new();
		ret.insert("authorization".to_string(), format!("{} {}",self.access_token_type,self.access_token));
		ret
	}

	pub fn gen_and_save(meta:&Meta){
		let secp = Secp256k1::new();
		let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
		let data = meta.take_sig_data();
		let cell = format!("{}:{}:{}:{}","25dzX3vbYqktVxyX",data.0,data.1,0);
		let digest = sha256::Hash::hash(cell.as_bytes());
		let message = Message::from_digest(digest.to_byte_array());

		let sig = secp.sign_ecdsa(message, &secret_key).serialize_compact();
		let mut sd = hex::encode(sig);
		sd.push_str("01");

		let mut pk = "04".to_string();
		pk.push_str(&public_key.to_string());
		let data = meta.take_container_data();
		let ret = Self {
			access_token_type: data.0,
			access_token: data.1,
			refresh_token: data.2,
			expires_in: data.3,
			expire_time: data.4,
			device_id: data.5,
			user_id: data.6,
			app_id: "25dzX3vbYqktVxyX".to_string(),
			nonce: 0,
			device_name: "Chrome浏览器".to_string(),
			model_name: "Windows网页版".to_string(),
			secret_key: secret_key.secret_bytes(),
			public_key: pk,
			signature_data: sd,
		};
		ret.save();
	}
}
