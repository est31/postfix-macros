extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree as Tt, Punct, Group, Spacing};

#[proc_macro]
pub fn postfix_macros(stream :TokenStream) -> TokenStream {
	let mut vis = Visitor;
	let res = vis.visit_stream(stream);
	println!("{}", res);
	res
}

struct Visitor;

impl Visitor {
	fn visit_stream(&mut self, stream :TokenStream) -> TokenStream {
		let mut res = Vec::new();
		let mut stream_iter = stream.into_iter();
		while let Some(tt) = stream_iter.next() {
			match tt {
				Tt::Group(group) => {
					let mut postfix_macro = false;
					{
						let last_three = res.rchunks(3).next();
						if let Some(&[Tt::Punct(ref p1), Tt::Ident(_), Tt::Punct(ref p2)]) = last_three {
							if (p1.as_char(), p1.spacing(), p2.as_char(), p2.spacing()) == ('.', Spacing::Alone, '!', Spacing::Alone) {
								postfix_macro = true;
							}
						}
					}
					let group = if postfix_macro {
						// Remove the ! and macro ident
						let mac_bang = res.pop().unwrap();
						let mac = res.pop().unwrap();
						// Remove the . before the macro
						res.pop().unwrap();

						// The expression to pop
						// TODO support entire chains
						let expr = res.pop().unwrap();

						// Add back the macro ident and bang
						res.push(mac);
						res.push(mac_bang);

						let gr = self.visit_group(group);
						add_prefix_expr_to_group(expr, gr)
					} else {
						group
					};
					let tt = Tt::Group(self.visit_group(group));
					res.push(tt);
				},
				Tt::Ident(id) => {
					res.push(Tt::Ident(id));
				},
				Tt::Punct(p) => {
					res.push(Tt::Punct(p));
				},
				Tt::Literal(lit) => {
					res.push(Tt::Literal(lit));
				},
			}
		}
		res.into_iter().collect()
	}
	fn visit_group(&mut self, group :Group) -> Group {
		let delim = group.delimiter();
		let stream = self.visit_stream(group.stream());
		Group::new(delim, stream)
	}
}

fn add_prefix_expr_to_group(tokens :Tt, gr :Group) -> Group {
	let delim = gr.delimiter();
	let mut stream = TokenStream::from(tokens);
	stream.extend(std::iter::once(Tt::Punct(Punct::new(',', Spacing::Alone))));
	stream.extend(gr.stream());
	Group::new(delim, stream)
}
