use postfix_macros::{postfix_macros, unwrap_or};

fn main() {
	postfix_macros! {
		let urls = ["https://rust-lang.org", "http://github.com"];
		for url in urls.iter() {
			let mut url_splitter = url.splitn(2, ':');
			let scheme = url_splitter.next().unwrap();
			let _remainder = url_splitter.next().unwrap_or! {
				println!("Ignoring URL: No scheme found");
				continue;
			};
			println!("scheme is {}", scheme);
		}
	}
}
