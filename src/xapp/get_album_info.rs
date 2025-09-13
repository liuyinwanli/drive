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
pub struct Album_S{}
impl XSend for Album_S  {
  const DOMAIN:&str = constants::API_DOMAIN;
  const URI:&str = constants::API_ALBUM_INFO;
  type RECE = AlbumInfo;
}
impl Album_S {
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
pub struct AlbumInfo{
	code: String,
	message: String,
	resultCode: String,
  data: Data,
}
impl XCell for AlbumInfo  {
  const FILE:&str = constants::ALBUM_INFO_JSON;
}