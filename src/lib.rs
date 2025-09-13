pub mod xapp;
pub mod file;
pub mod constants;
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, io::{Read, Write}};
    use shlex;
    use tokio::io::{stdout, AsyncWriteExt};
    fn dd(){

      for _ in 0..3 {
				print!(">>>");
				stdout().flush();
				let mut cmd = String::new();
				match std::io::stdin().read_line(&mut cmd) {
					Ok(x) => {
						println!("==={}",cmd);
						stdout().flush();
						// let y = shlex::split(&cmd).unwrap();
						// dbg!(y);
					},
					Err(e) => {
						dbg!(e);
					},
				}
			}
		}
    #[test]
    fn it_works() {
        dd();
    }
}