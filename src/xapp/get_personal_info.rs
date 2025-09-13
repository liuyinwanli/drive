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

*/
#[derive(Debug,Serialize,Deserialize)]
pub struct PersonalInfo_S{}
impl XSend for PersonalInfo_S  {
  const DOMAIN:&str = constants::API_DOMAIN;
  const URI:&str = constants::API_PERSONAL_INFO;
  type RECE = PersonalInfo_R;
}
impl PersonalInfo_S {
    pub fn new()->Self{
      Self{ }
    }
}






/*

*/
#[derive(Debug,Serialize,Deserialize)]
struct Data {
	driveId: String,
	driveName: String,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct PersonalInfo_R{
	code: String,
	message: String,
	resultCode: String,
  data: Data,
}
impl XCell for PersonalInfo_R  {
  const FILE:&str = constants::PERSONAL_INFO_JSON;
}