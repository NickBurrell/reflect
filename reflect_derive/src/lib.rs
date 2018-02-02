#![feature(const_fn)]
#[cfg(test)]

pub mod reflect_macro;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

pub trait Reflect {
    pub fn get_members(self) -> String;
}

fn get_field_types_iter<'a>(fields: &'a Vec<syn::Field>) -> Box<Iterator<Item = &'a Ty> + 'a> {
    Box::new(fields.iter().map(|f| &f.ty))
}

fn get_field_types<'a>(fields: &'a Vec<syn::Field>) -> <&'a Ty> {
    get_field_types_iter(fields).collect()
}

fn get_field_names_iter<'a>(fields: &'a Vec<syn::Field>) -> Box<Iterator<Item = &'a Ident> + 'a> {
    Box::new(fields.iter().map(|f| &f.ident.unwrap()))
}

fn get_field_names<'a>(fields: &'a Vec<syn::Field>) -> <&'a Ident> {
    get_field_names_iter(fields).collect()
}

fn reflect_impl_struct<T: quote::ToTokens>(ast: &syn::DeriveInput, fields: &Vec<syn::Field>, body: T) -> quote::Tokens {
    let input_type = &ast.ident;
    let struct_field_names = &get_field_names(fields);
    let struct_field_types = &get_field_types(fields);
    quote! {

        impl #input_type {
            const fn get_members_impl() -> &'static str {
                concat!("struct ", #input_type, "{\n",
                        (#(stringify!(#struct_field_names), ": ", stringify!(#struct_field_types), ",\n"),*,
                        "}")
                )
            }
        }

        impl ::Reflect for #input_type {
            pub fn get_members(self) -> String {
                #input_type::get_members_impl().to_owned();
            }
        }
    }
}


mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}  
