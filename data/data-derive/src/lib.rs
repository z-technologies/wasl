#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Ident;

#[proc_macro_derive(Repository)]
pub fn repo(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    let gen = repo_impl(&ast);

    gen.parse().unwrap()
}

fn repo_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let table_name = Ident::from(get_table_name(&name));

    quote! {
        impl Repo for #name {
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
                diesel::update(#table_name).set(item).execute(&self.get_connection()?)?;
                Ok(item)
            }

            fn delete(&self, item: &Self::Model) -> Result<()> {
                use crate::schema::#table_name::dsl::*;
                match diesel::delete(#table_name
                                        .filter(id.eq(item.id)))
                                        .execute(&self.get_connection()?) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(DataError::from(err)),
                }
            }

            fn get_connection(&self) -> Result<DbPooledConnection> {
                match self.pool.get() {
                    Ok(conn) => Ok(conn),
                    Err(err) => Err(DataError::ConnectionPoolError(format!("{}", err))),
                }
            }
        }
    }
}

fn get_table_name(name: &Ident) -> String {
    let name = format!("{}", &name);

    if name.ends_with("Repo") {
        name.strip_suffix("Repo").unwrap().to_owned()
    } else if name.ends_with("Repository") {
        name.strip_suffix("Repository").unwrap().to_owned()
    } else {
        name
    }
    .to_lowercase()
}
