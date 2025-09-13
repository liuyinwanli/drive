use std::{collections::HashMap, io::Write};

use reqwest::{
	Client,
	header,
};
use serde::de::value;
use serde::{
	de::DeserializeOwned, Serialize
};

use crate::constants;
use super::container::Container;
/*
给发送数据的类型
*/
pub trait XSend
where Self:
DeserializeOwned + Serialize
{
	const DOMAIN:&str;
	const URI:&str;
	type RECE:XCell;
	// fn deal_with(value:serde_json::Value){
	// 		if let Ok(x) = serde_json::from_value::<Self::RECE>(value.clone()){
	// 			x.save();
	// 		}else{
	// 			dbg!(value);
	// 		}
	// }
	fn to_data(&self)->serde_json::Value {
		serde_json::to_value(&self).unwrap()
	}
	fn new_from_container(source:&Container)->Self{
		let value = serde_json::to_value(source).unwrap();
		serde_json::from_value(value).unwrap()
	}
	async fn web_work(&self,headerx:HashMap<String,String>){
		let url = format!("{}{}",Self::DOMAIN,Self::URI);
		let mut headers = header::HeaderMap::new();
		headers.insert(header::ACCEPT, "application/json, text/plain, */*".parse().unwrap());
		headers.insert(header::REFERER, "https://www.aliyundrive.com/".parse().unwrap());
		headers.insert(header::ORIGIN, "https://www.aliyundrive.com/".parse().unwrap());
		headers.insert(header::CONTENT_TYPE, "application/json;charset=UTF-8".parse().unwrap());
		headers.insert(header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:142.0) Gecko/20100101 Firefox/142.0".parse().unwrap());

		let mut  req = Client::new()
			.post(url)
			.headers(headers)
			.json(&self.to_data());

		for (k,v) in headerx{
			req = req.header(k, v);
		}
		match req.send().await{
			Ok(resp) =>{
				match resp.json::<serde_json::Value>().await {
					Ok(value) => {
						if let Ok(x) = serde_json::from_value::<Self::RECE>(value.clone()){
							x.save();
						}else{
							dbg!(value);
						}
					},
					Err(e) => {
						dbg!(e);
					},
				}

			},
			Err(e) =>{
				dbg!(e);
			},
		}
	}

}
/*
给返回的数据用
*/
pub trait XCell
where Self:
DeserializeOwned + Serialize
{
	const FILE:&str;
	fn save(&self){
		let p = format!("{}{}",constants::CONFIG_PATH,Self::FILE);
		if let Ok(f) = std::fs::OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(p)
		{
			let writer = std::io::BufWriter::new(f);
			serde_json::to_writer_pretty(writer, self).unwrap();
		}
		// dbg!("--");

	}
	fn from_file()->Self
	{
		let f = std::fs::OpenOptions::new()
			.read(true)
			.open(format!("{}{}",constants::CONFIG_PATH,Self::FILE))
			.unwrap();

		let reader = std::io::BufReader::new(f);
		serde_json::from_reader(reader).unwrap()

	}
}
/*

*/
