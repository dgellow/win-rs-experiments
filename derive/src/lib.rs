extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MarginBuilder)]
pub fn derive_margin_builder(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;

	let expanded = quote! {
		impl MarginBuilder for #name {
			fn get_margin(&self) -> Margin {
				self.margin
			}
			fn set_margin(&mut self, m: Margin) {
				self.margin = m
			}
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro_derive(PaddingBuilder)]
pub fn derive_padding_builder(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;

	let expanded = quote! {
		impl PaddingBuilder for #name {
			fn get_padding(&self) -> Padding {
				self.padding
			}
			fn set_padding(&mut self, m: Padding) {
				self.padding = m
			}
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro_derive(DimensionBuilder)]
pub fn derive_dimension_builder(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;

	let expanded = quote! {
		impl DimensionBuilder for #name {
			fn set_height(&mut self, v: i32) {
				self.dimension.height = v;
			}
			fn set_width(&mut self, v: i32) {
				self.dimension.width = v;
			}
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro_derive(WindowBase)]
pub fn derive_window_base(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;

	let expanded = quote! {
		impl WindowBase for #name {
			fn init_state(h_instance: HINSTANCE) -> Self {
				Self { h_instance, ..Default::default()}
			}
			fn h_instance(&self) -> HINSTANCE {
				self.h_instance
			}
			fn set_h_window(&mut self, h_window: HWND) {
				self.h_window = h_window;
			}
			fn h_window(&self) -> HWND {
				self.h_window
			}
		}
	};

	TokenStream::from(expanded)
}

// struct EmbedArgs {
// 	pub vars: HashSet<Ident>,
// }

// impl Parse for EmbedArgs {
// 	fn parse(input: syn::parse::ParseStream) -> Result<Self> {
// 		let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
// 		Ok(EmbedArgs {
// 			vars: vars.into_iter().collect(),
// 		})
// 	}
// }

// #[proc_macro_attribute]
// pub fn embed(args: TokenStream, tokens: TokenStream) -> TokenStream {
// 	let args = parse_macro_input!(args as EmbedArgs);
// 	// let input_derive = parse_macro_input!(tokens as DeriveInput);
// 	let input_struct = parse_macro_input!(tokens as ItemStruct);
// 	let ident = input_struct.ident;

// 	let mut fields: Punctuated<_, _> = match input_struct.fields {
// 		Fields::Named(fields) => fields.named,
// 		_ => panic!("expected a struct with named fields"),
// 	};
// 	for arg in args.vars {
// 		let temp_struct: ItemStruct = parse_quote! {
// 			struct T{ #arg: #arg }
// 		};
// 		let field = match temp_struct.fields {
// 			Fields::Named(fields) => fields.named[0].clone(),
// 			_ => panic!("expected a struct with named fields"),
// 		};
// 		fields.push(field);
// 	}

// 	// let output = args.fold_item_fn(input);
// 	TokenStream::from(quote! {
// 		#[derive(Default, Debug)]
// 		struct #ident {
// 			#fields
// 		}
// 	})
// }
