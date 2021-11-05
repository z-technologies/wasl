pub fn model_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let id = if let syn::Body::Struct(ss) = &ast.body {
        ss.fields()
            .iter()
            .filter(|f| match &f.ident {
                Some(name) => name == "id",
                None => false,
            })
            .next()
            .unwrap()
    } else {
        panic!("Model cannot be implemented on non-structs");
    };

    let id_name = &id.ident;
    let id_type = &id.ty;

    quote! {
        impl Model<#id_type> for #name {
            type KeyType = #id_type;

            fn get_id(&self) -> #id_type {
                self.#id_name
            }

            fn set_id(&mut self, id: #id_type) {
                self.#id_name = id
            }
        }
    }
}
