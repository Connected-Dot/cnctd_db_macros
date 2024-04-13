extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(SqlInsertable)]
pub fn sql_insertable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;  // The struct name

    // Generate snake_case table name from struct name
    let table_name = camel_to_snake(&name.to_string());

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => fields.named,
            _ => unimplemented!(), // Handle other cases or report an error
        },
        _ => unimplemented!(), // Structs only; again, handle other cases or report an error
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let placeholders: Vec<_> = field_names.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect();

    let gen = quote! {
        impl #name {
            pub fn insert_query(&self) -> String {
                let fields = [#(stringify!(#field_names)),*].join(", ");
                let placeholders = [#(#placeholders),*].join(", ");
                format!("INSERT INTO {} ({}) VALUES ({})", #table_name, fields, placeholders)
            }
        }

        // Implement the SqlInsertable trait
        impl SqlInsertable for #name {
            fn insert_query(&self) -> String {
                Self::insert_query(self)
            }
        }
    };

    gen.into()
}


fn camel_to_snake(name: &str) -> String {
    let mut snake_case = String::new();
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !snake_case.is_empty() {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}