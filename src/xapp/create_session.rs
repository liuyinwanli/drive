use std::collections::HashMap;

use serde::{
	Deserialize,Serialize
};
/*
*/
use crate::constants;
use super::container::{
	Container,
};
use super::traits::{
	XCell,
	XSend,
};
#[derive(Debug,Serialize,Deserialize)]
pub struct For_Create_Session{
	#[serde(alias = "device_name")]
	deviceName: String,
	#[serde(alias = "model_name")]
	modelName: String,
	#[serde(alias = "public_key",default)]
	pubKey: String,
}
impl XSend for For_Create_Session  {
	const DOMAIN:&str = constants::API_DOMAIN;
	const URI:&str = constants::API_CREATE_SESSION;
	type RECE = Session_R;

}
/*
*/
#[derive(Debug,Serialize,Deserialize)]
pub struct Session_R{
	result:bool,
	success:bool,
}
impl XCell for Session_R  {
		const FILE:&str = constants::SESSION_R_JSON;
}