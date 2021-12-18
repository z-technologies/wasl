use crate::data::connection::*;
use crate::data::models::{KeyType, NewProduct, Product, Transaction, User};
use crate::result::{Result, UserError};
use crate::services::{FinanceService, UsersService};

use diesel::prelude::*;

use std::sync::Arc;

pub struct ProductsService {
    conn: PostgresConnection,
    users_svc: Arc<UsersService>,
    finance_svc: Arc<FinanceService>,
}

impl ProductsService {
    pub fn new(
        conn: PostgresConnection,
        users_svc: Arc<UsersService>,
        finance_svc: Arc<FinanceService>,
    ) -> ProductsService {
        ProductsService {
            conn,
            users_svc,
            finance_svc,
        }
    }

    pub fn get_product_by_id(&self, key: KeyType) -> Result<Product> {
        use crate::data::schema::products::dsl::*;

        Ok(products.find(key).get_result(&self.conn.get()?)?)
    }

    pub fn create(&self, new_product: &NewProduct) -> Result<Product> {
        use crate::data::schema::products::dsl::*;

        Ok(diesel::insert_into(products)
            .values(new_product)
            .get_result(&self.conn.get()?)?)
    }

    pub fn delete(&self, product: Product, for_user: &User) -> Result<usize> {
        if product.user_id != for_user.id {
            return Err(UserError::PermissionDenied);
        }

        Ok(diesel::delete(&product).execute(&self.conn.get()?)?)
    }

    pub fn is_available(&self, product: &Product) -> Result<bool> {
        Ok(product.available_quantity > 0)
    }

    pub fn purchase(
        &self,
        product: &mut Product,
        customer: &User,
    ) -> Result<Transaction> {
        Ok(self
            .conn
            .get()?
            .build_transaction()
            .run::<_, UserError, _>(|| {
                if !self.is_available(product)? {
                    return Err(UserError::OutOfStock);
                }

                // TODO: Fix this
                product.available_quantity -= 1;

                let transactions = self.finance_svc.transfer_pending(
                    customer,
                    &self.users_svc.get_by_id(product.user_id)?,
                    product.price.clone(),
                )?;

                Ok(transactions)
            })?)
    }
}
