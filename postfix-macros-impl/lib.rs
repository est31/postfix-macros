/*!
Postfix macros on stable Rust, today.

This is the crate containing the proc macro implementation
of the [`postfix_macros!`] macro.

The `postfix-macros` crate reexports the macro
defined by this crate, and adds some macros of its
own that are helpful in postfix macro context.
If you don't need these extra macros,
you can use this crate instead and save
the extra dependency.

```
# use postfix_macros_impl::postfix_macros;
# #[derive(Debug, Clone, Copy)] enum Custom { Enum(()), EnumOther}
# let val = [((),Custom::EnumOther,)];
postfix_macros! {
	"hello".assert_ne!("world");

	val.iter()
		.map(|v| v.1)
		.find(|z| z.matches!(Custom::Enum(_) | Custom::EnumOther))
		.dbg!();
}
```

*/
#![forbid(unsafe_code)]

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree as Tt, Punct, Group, Spacing,
	Delimiter};

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
						// form the expression we want to feed to the macro.
						let expr_len = expression_length(&res);

						if expr_len == 0 {
							panic!("expected something before the postfix macro invocation");
						}
						//println!("  -> built");

						// Build the group
						let gr = self.visit_group(group);
						let arg_tokens = &res[(res.len() - expr_len)..];
						let gr = prepend_macro_arg_to_group(arg_tokens, gr);
						res.truncate(res.len() - expr_len);

						// Add back the macro ident and bang
						res.push(mac);
						res.push(mac_bang);

						/*println!("res so far: {}",
							res.iter().cloned().collect::<TokenStream>());*/

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
		let span = group.span();
		let stream = self.visit_stream(group.stream());
		let mut gr = Group::new(delim, stream);
		gr.set_span(span);
		gr
	}
}


/// Walk the entire chain of tt's that
/// form an expression that a postfix macro call
/// would be part of.
///
/// Returns the number of token tree items that
/// belong to the expression.
fn expression_length(tts :&[Tt]) -> usize {
	let mut expr_len = 0;
	let mut last_was_punctuation = true;
	let mut last_was_group = true;
	'outer: while expr_len < tts.len() {
		let tt = &tts[tts.len() - 1 - expr_len];
		let mut is_punctuation = false;
		let mut is_group = false;
		//println!("    {} {}", tt, last_was_punctuation);
		match tt {
			Tt::Group(group) => {
				is_group = true;
				// If the group wasn't terminated by a punctuation,
				// it belongs to e.g. a function body, if clause, etc,
				// but not to our expression
				if !last_was_punctuation {
					break;
				}

				// If the group was terminated by a punctuation,
				// it belongs to the postfix macro chain.
				// If it's delimitered by braces, so is { ... },
				// we need to check whether the group was an if,
				// match, else, or else if block, and add stuff accordingly.

				// If we have {}. it might be an if, match or else block.
				if group.delimiter() == Delimiter::Brace {
					loop {
						println!("HELLO IM HERE {}", group);
						// We are at the end, it was a {} block.
						if expr_len + 1 >= tts.len() {
							expr_len += 1;
							break 'outer;
						}
						let tt_before = &tts[tts.len() - 2 - expr_len];
						match tt_before {
							Tt::Group(_group) => {
								// e.g. `if foo() {}`, `if { true } {}`, `if if {true } else { false } {}`,
								// `if bools[..] {}`.
								// Basically, just start the expression search and hope for the best :)
							},
							Tt::Ident(id) => {
								let id_str = id.to_string();
								if id_str == "else" {
									expr_len += 2;
									// Continue the chain search
									continue;
								} else {
									// Any other ident: must be part of an expression like if something.expr {}.foo().
									// Start the full expression search
								}
							},
							Tt::Punct(p) => match p.as_char() {
								// These indicate the end of the expression
								';' | ',' => {
									expr_len += 1;
									break 'outer;
								},
								// This indicates the group was part of something else,
								// like a prior macro foo! {} . bar!().
								// Just continue the outer search normally
								'!' => break,
								// Unsupported stuff
								// TODO support closures
								'|' => panic!("Closures not supported yet"),
								p => panic!("Group expr search encountered unsupported punctuation {}", p),
							},
							Tt::Literal(_lit) => {
								// Start the expression search
							},
						}
						// Perform the expression search
						let sub_expr_len = expression_length(&tts[..tts.len() - 1 - expr_len]);
						expr_len += sub_expr_len;
						// Now check what's beyond the expression
						let tt_before = tts.get(tts.len() - 1 - expr_len);
						let tt_before_that = if tts.len() < 2 + expr_len {
							None
						} else {
							tts.get(tts.len() - 2 - expr_len)
						};
						match (tt_before_that, tt_before) {
							(Some(Tt::Ident(id_t)), Some(Tt::Ident(id))) => {
								let id_t = id_t.to_string();
								let id = id.to_string();
								if id_t == "else" && id == "if" {
									// Else if clause.
									expr_len += 3;
									// Continue the chain search.
								} else if id == "match" {
									// Done! Match terminates everything.
									expr_len += 1;
									break 'outer;
								}
							},
							(_, Some(Tt::Ident(id))) => {
								let id = id.to_string();
								if id == "if" || id == "match" {
									// Done!
									expr_len += 1;
									break 'outer;
								} else {
									// IDK something failed
								}
							},
							(_, Some(Tt::Punct(p))) => {
								match p.as_char() {
									// This can be either == or if let Foo() =
									'=' => {
										if let Some(Tt::Punct(p_t)) = tt_before_that {
											if p_t.as_char() == '=' {
												// Parse another expr
												// TODO
												// TODO maybe instead of calling expression_length above,
												// create a new function that calls expression_length internally and
												// handles this case, calling expression_length again if needed?
												// Or pass some kind of precedence setting to expression_length?
												panic!("== in if clause not supported yet");
											}
										}
										panic!("if let not supported");
									},
									_ => panic!("{} in if not supported yet", p),
								}
							},
							_ => {
								panic!("Hit unsupported case: {:?} {:?}", tt_before_that.map(|v| v.to_string()),
									tt_before.map(|v| v.to_string()));
							},
						}
					}
				}
			},
			Tt::Ident(id) => {
				if !last_was_punctuation && !last_was_group {
					// two idents following another... must be `if <something>.foo!() { <stuff> }`
					// or something like it.
					// We need to special case the keyword mut though because `&mut` is usually
					// prefixed to an expression.
					let id_str = id.to_string();
					if id_str != "mut" {
						break;
					}
				}
			},
			Tt::Punct(p) => {
				is_punctuation = true;
				match p.as_char() {
					// No expression termination
					'.' if p.spacing() == Spacing::Alone => (),
					':' | '?' | '!' => (),
					// These all terminate expressions
					'.' | '&' if p.spacing() == Spacing::Joint => break,
					',' | ';' | '+' | '/' | '%' | '=' | '<' | '>' | '|' | '^' => break,
					// All of & * and - can be safely prepended to expressions in any number,
					// However the symbols also exist in a binop context.
					// Only the leading symbol can be a binop, but what makes matters a bit
					// more complicated is that `&&` is a valid binop as well.
					'&' | '*' | '-' => {
						// First, we find the end of our binop partner
						let mut offs_until_binop_partner = 0;
						for tt in tts[.. tts.len() - expr_len - 1].iter().rev() {
							match tt {
								Tt::Group(gr) => {
									match gr.delimiter() {
										// `{0} & 7;` is invalid and `{}` is.
										// => all &*- are unary ops.
										Delimiter::Brace => {
											expr_len += offs_until_binop_partner + 1;
											break 'outer;
										}
										// Both [] and () are other-parties/partners of binops
										// e.g. `(4) & 7` is valid while `(()) & 7` isn't
										// => this group belongs to our binop partner
										Delimiter::Parenthesis | Delimiter::Bracket => {
											break;
										},

										// IDK what to do here, let's just error
										Delimiter::None => {
											panic!("We don't support groups delimitered by none yet: {}", gr);
										},
									}
								},

								Tt::Ident(id) => {
									let id_str = id.to_string();
									if id_str != "mut" {
										// If we encounter any ident other than the `mut` keyword,
										// it's part of the binop partner.
										break;
									}
								},
								Tt::Punct(p) => {
									match p.as_char() {
										// ; either stands for the separator in array types/definitions,
										// or it stands for a new statement. In both cases, unary op.
										';' |
										// , is used in tuples, argument lists, etc. Implies an unary op
										',' |
										// If we encounter =, it means an assignment OR comparison,
										// both implying that all leading &*- were unary ops.
										// (even though == is a binop, but that would be a binop at a higher level)
										'=' => {
											expr_len += offs_until_binop_partner + 1;
											break 'outer;
										},
										// Continue the search
										'&' | '*' | '-' => (),

										// We don't support special symbols yet
										// TODO support more
										_ => panic!("Binop partner search encountered punct '{}'", p),
									}
								},
								Tt::Literal(_lit) => {
									// Literals are binop partners
									break;
								},
							}
							offs_until_binop_partner += 1;
						}
						// If there is nothing beyond the one unary op in tts,
						// no binop partner could be found,
						// and we know that the sole punctuation
						// was an unary op.
						if offs_until_binop_partner == 0 {
							break;
						}
						let first = &tts[tts.len() - (expr_len + 1) - offs_until_binop_partner];
						let second = &tts[tts.len() - (expr_len + 1) - offs_until_binop_partner + 1];
						let mut binop_tts = 1;
						match first {
							Tt::Group(_gr) => unreachable!(),
							// This can occur, as of current code only when we have code like `(mut hello.foo!())`,
							// which would indicate a pattern context I guess... but for now we don't support
							// our macro to be called in pattern contexts.
							Tt::Ident(id) => panic!("Can't start a binop chain with ident '{}'", id),
							Tt::Punct(p1) => {
								if let Tt::Punct(p2) = second {
									let is_binop_and_and = p1.spacing() == Spacing::Joint &&
										p1.as_char() == '&' && p2.as_char() == '&';
									if is_binop_and_and {
										binop_tts = 2;
									}
								}
							},
							Tt::Literal(_lit) => unreachable!(),
						}
						// We finally know how many tt's the binop operator takes up (1 or 2).
						// Set the length of the expression and emit the expression.
						expr_len += 1 + offs_until_binop_partner - binop_tts;
						break;
					},
					c => panic!("Encountered unsupported punctuation {}", c),
				}
			},
			Tt::Literal(_lit) => {
			},
		}
		expr_len += 1;
		last_was_punctuation = is_punctuation;
		last_was_group = is_group;
	}
	expr_len
}

fn prepend_macro_arg_to_group(tokens :&[Tt], gr :Group) -> Group {
	// Build the expr's tt.
	// If there is only one token and it's
	// a variable/constant/static name, or a literal,
	// we pass it directly, otherwise we wrap it in {}
	// to make it safer.
	let expr = match &tokens {
		&[tt] if matches!(tt, Tt::Literal(_) | Tt::Ident(_)) => {
			tt.clone()
		},
		_ => {
			let expr_stream = tokens.iter().cloned().collect();
			let expr_gr = Group::new(Delimiter::Brace, expr_stream);
			Tt::Group(expr_gr)
		},
	};

	let stream = gr.stream();
	let delim = gr.delimiter();
	let mut res_stream = TokenStream::from(expr);
	if !stream.is_empty() {
		res_stream.extend(std::iter::once(Tt::Punct(Punct::new(',', Spacing::Alone))));
		res_stream.extend(stream);
	}
	Group::new(delim, res_stream)
}
