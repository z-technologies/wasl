#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Attribute;
use syn::Ident;
use syn::Lit;
use syn::MetaItem;

#[proc_macro_derive(
    Repository,
    attributes(repo_model, repo_insert_model, repo_table_name)
)]
pub fn repo(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    let gen = repo_impl(&ast);

    gen.parse().unwrap()
}

fn repo_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let attrs = &ast.attrs;

    let table_name = get_attribute_value(&attrs, "repo_table_name");
    let model_name = get_attribute_value(&attrs, "repo_model");
    let insert_model_name = get_attribute_value(&attrs, "repo_insert_model");

    quote! {
        impl crate::repos::Repo for #name {
            type Model = #model_name;
            type InsertModel = #insert_model_name;

            fn get_all(&self) -> Result<Vec<Self::Model>> {
                use crate::schema::#table_name::dsl::*;
                Ok(#table_name.load::<Self::Model>(&self.get_connection()?)?)
            }

            fn get(&self, key: crate::models::KeyType) -> Result<Self::Model> {
                use crate::schema::#table_name::dsl::*;
                Ok(#table_name.filter(id.eq(key)).get_result(&self.get_connection()?)?)
            }

            fn insert<'a>(&self, item: &'a Self::InsertModel) -> Result<Self::Model> {
                use crate::schema::#table_name::dsl::*;
                Ok(diesel::insert_into(#table_name)
                    .values(item)
                    .get_result::<Self::Model>(&self.get_connection()?)?)
            }

            fn update<'a>(&self, item: &'a Self::Model) -> Result<&'a Self::Model> {
                use crate::schema::#table_name::dsl::*;
                diesel::update(#table_name.filter(id.eq(item.id)))
                    .set(item).execute(&self.get_connection()?)?;
                Ok(item)
            }

            fn delete(&self, item: &Self::Model) -> Result<()> {
                use crate::schema::#table_name::dsl::*;
                match diesel::delete(#table_name.filter(id.eq(item.id)))
                                           .execute(&self.get_connection()?) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(DataError::from(err)),
                }
            }

            fn get_connection(&self) -> Result<crate::repos::DbPooledConnection> {
                match self.pool.get() {
                    Ok(conn) => Ok(conn),
                    Err(err) => Err(DataError::ConnectionPoolError(format!("{}", err))),
                }
            }
        }
    }
}

fn get_attribute_value<'a>(attrs: &'a Vec<Attribute>, name: &'a str) -> Ident {
    for attr in attrs {
        if let MetaItem::NameValue(ident, lit) = &attr.value {
            if ident != name {
                continue;
            }

            if let Lit::Str(value, _) = lit {
                return Ident::from(value.clone());
            }

            panic!("only string values are accepted");
        }
    }

    panic!("attribute `{}' must be set", name);
}
