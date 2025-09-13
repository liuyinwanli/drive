use std::collections::HashMap;

use serde::{
	Serialize,
	Deserialize,
	de::DeserializeOwned,

};
use serde_json::{
	Value,
};

use crate::xapp::traits::{
	XCell,
	XSend,
};
use crate::{constants, xapp::container::Container};
/*

	}
*/
#[derive(Debug,Serialize,Deserialize)]
pub struct FileList_S{
  drive_id: String,
  parent_file_id: String,
  limit: i64,
  all: bool,
  url_expire_sec: i64,
  image_thumbnail_process: String,
  image_url_process: String,
  video_thumbnail_process: String,
  fields: String,
  order_by: String,
  order_direction: String,
}
impl FileList_S {
    pub fn new()->Self{
      Self {
        // drive_id: "786479".to_string(),
        drive_id: "363443872".to_string(),
        //363443872
        parent_file_id: "root".to_string(),
        limit: 11,
        all: false,
        url_expire_sec: 1600,
        image_thumbnail_process: "image/resize,w_400/format,jpeg".to_string(),
        image_url_process: "image/resize,w_1920/format,jpeg".to_string(),
        video_thumbnail_process: "video/snapshot,t_0,f_jpg,ar_auto,w_800".to_string(),
        fields: "*".to_string(),
        order_by: "name".to_string(),
        order_direction: "ASC".to_string(),
      }
    }
}
impl XSend for FileList_S  {
  const DOMAIN:&str = constants::API_DOMAIN;
  const URI:&str = constants::API_FILE_LIST;
  type RECE = FileList_R;
}
/*

*/
#[derive(Debug,Serialize,Deserialize,Default)]
#[serde(default)]
pub struct FileEntity_R{
  drive_id: String,
  domain_id: String,
  file_id: String,
  name: String,
  #[serde(alias = "type")]
  xtype: String,
  content_type: String,
  created_at: String,
  updated_at: String,
  file_extension: String,
  mime_type: String,
  mime_extension: String,
  size: i64,
  starred: bool,
  hidden: bool,
  status: String,
  upload_id: String,
  parent_file_id: String,
  crc64_hash: String,
  content_hash: String,
  content_hash_name: String,
  download_Url: String,
  url: String,
  category: String,
  encrypt_mode: String,
  punish_flag: i64,
  sync_flag: bool,
  sync_meta: String,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct FileList_R  {
  items: Vec<FileEntity_R>,
  next_marker: String,
}
impl XCell for FileList_R  {
  const FILE:&str = constants::FILE_LIST_JSON;
}
/*

*/