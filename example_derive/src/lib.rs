extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DataStruct, DeriveInput, Fields, Ident};

#[proc_macro_derive(Clauseable, attributes(search_field))]
pub fn clausable_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    match data {
        syn::Data::Struct(DataStruct { fields, .. }) => {
            let struct_name = &ident;
            impl_to_clauses(struct_name, fields)
        }
        _ => unimplemented!(),
    }
}

fn impl_to_clauses(struct_name: &Ident, fields: Fields) -> TokenStream {
    const FIELD_ATTR_NAME: &str = "search_field";

    let mut vec_push_expr = vec![];

    for field in fields {
        if let Some(search_field_attr) = field
            .attrs
            .iter()
            .find(|a| a.path.is_ident(FIELD_ATTR_NAME))
        {
            let search_field_value: syn::LitStr = search_field_attr.parse_args().unwrap();
            let field_ident = &field.ident.unwrap();
            vec_push_expr.push(quote! {

                if let Some(ref criteria_value) = self.#field_ident {
                    clauses.push(criteria_value.to_clause(#search_field_value.into()));
                }

            })
        }
    }

    let impl_block = quote! {
        impl #struct_name {
            pub fn to_clauses(&self) -> Vec<ElasticsearchQueryClause> {
                let mut clauses = vec![];
                #(#vec_push_expr)*
                clauses
            }
        }
    };
    impl_block.into()
}
