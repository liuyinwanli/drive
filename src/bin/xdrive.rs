use std::{collections::HashMap, io::{self, Read, Write}};

use clap::{self, crate_name, Command};
use drive::xapp::{
		container::Container, create_session, get_access_token::{Meta, Init}, traits::{XCell, XSend},
		get_album_info::{AlbumInfo,Album_S},
	};
use drive::file::{
	file_get_list::FileList_S,
};
use tokio;
#[tokio::main]
async fn main(){
	let app = clap::Command::new(crate_name!())
	.subcommand(
		Command::new("init")
	)
	.subcommand(
		Command::new("refresh")
	)
	.subcommand(
		Command::new("album-info")
	)
	.subcommand(
		Command::new("file-list")
	)
	.subcommand(
		Command::new("exit")
	)
	;
	loop {
		print!(">>> ");
		io::stdout().flush();
		let mut cmd = String::new();
		match std::io::stdin().read_line(&mut cmd) {
			Ok(n) => {
				if n < 2 {
					continue;
				}
				let mut vx = shlex::split(&cmd).unwrap();
				vx.insert(0, "xdrive".to_string());
				dbg!(&vx);
				let matches = app.clone().get_matches_from(vx);
				match matches.subcommand(){
					Some(("init",x)) => {
						let init = Init::from_file();
						init.web_work(HashMap::new()).await;

						let meta = Meta::from_file();
						Container::gen_and_save(&meta);

						let con = Container::from_file();
						let session = create_session::For_Create_Session::new_from_container(&con);
						session.web_work(con.get_sig_header()).await;
						continue;
					},
					Some(("refresh",x)) => {
						let init = Init::from_file();
						init.web_work(HashMap::new()).await;

						let meta = Meta::from_file();
						init.update(meta.take_refresh()).save();

						let mut con = Container::from_file();
						con.update(meta.take_refresh(), meta.take_access(),meta.take_time()).save();

						continue;
					},
					Some(("exit",_)) =>{
						break;
					},
					_ => {

					},
				}
				let mut con = Container::from_file();
				match matches.subcommand(){
					Some(("album-info",x)) => {
						let agent = Album_S::new();
						agent.web_work(con.get_auth_header()).await;
					},
					Some(("file-list",x)) => {
						let agent = FileList_S::new();
						agent.web_work(con.get_auth_header()).await;
					},
					_ => {

					},
				}
			},
			Err(e) => {
				dbg!(e);
			},
		}
	}


}