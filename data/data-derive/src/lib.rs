mod model;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Model)]
pub fn model(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    model::model_impl(&get_ast_generator(ts)).parse().unwrap()
}

fn get_ast_generator(ts: proc_macro::TokenStream) -> syn::DeriveInput {
    syn::parse_derive_input(&ts.to_string()).unwrap()
}
