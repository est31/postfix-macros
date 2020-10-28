extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree as Tt, Punct, Group, Spacing, Delimiter};

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

						// Walk the entire chain of tt's that
						// form an expression.
						let mut res_iter = res.iter().rev();
						let mut expr_len = 0;
						let mut last_was_punctuation = true;
						for tt in res_iter {
							let mut is_punctuation = false;
							//println!("    {} {}", tt, last_was_punctuation);
							match tt {
								Tt::Group(_group) => {
									is_punctuation = true;
								},
								Tt::Ident(_id) => {
									if !last_was_punctuation {
										// two idents following another... must be `if <something>.foo!()`
										// or something like it
										break;
									}
								},
								Tt::Punct(p) => {
									is_punctuation = true;
									match p.as_char() {
										// No expression termination
										'.' if p.spacing() == Spacing::Alone => (),
										'?' | '!' => (),
										// These all terminate expressions
										'.' | '&' if p.spacing() == Spacing::Joint => break,
										'&' if p.spacing() == Spacing::Joint => (),
										',' | ';' | '+' | '/' | '%' | '=' | '<' | '>' | '|' | '^' => break,
										// TODO figure out what to do about & and * and - as they can be prepended to expressions.
										// For safety reasons (to not accidentially change meaning of code),
										// we error here
										'&' | '*' | '-' => panic!("We currently don't know whether an expression ends \
											in punctuation {} and error about it to not change meaning", p.as_char()),
										c => panic!("Encountered unsupported punctuation {}", c),
									}
								},
								Tt::Literal(_lit) => {
								},
							}
							expr_len += 1;
							last_was_punctuation = is_punctuation;
						}
						if expr_len == 0 {
							panic!("expected something before the postfix macro invocation");
						}
						//println!("  -> built");
						// Build the expr's tt
						let expr_stream = res[(res.len() - expr_len)..].iter().cloned().collect();
						res.truncate(res.len() - expr_len);
						let expr_gr = Group::new(Delimiter::Brace, expr_stream);
						let expr = Tt::Group(expr_gr);

						let gr = self.visit_group(group);
						let gr = add_prefix_expr_to_group(expr, gr);

						// Add back the macro ident and bang
						res.push(mac);
						res.push(mac_bang);

						gr
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
