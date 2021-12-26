use crate::data::connection::*;
use crate::data::models::{
    KeyType, NewProduct, NewProductOrder, Product, ProductOrder,
    ProductOrderState, Transaction, TransactionConfirmation,
    TransactionConfirmationOutcome, User,
};
use crate::result::{Result, UserError};
use crate::services::{FinanceService, UsersService};

use bigdecimal::BigDecimal;
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

    pub fn is_available(&self, product: &Product, count: i64) -> Result<bool> {
        use crate::data::schema::product_orders::dsl::*;

        let total_ordered: i64 = ProductOrder::belonging_to(product)
            .filter(state.eq(ProductOrderState::Accepted))
            .select(quantity)
            .load(&self.conn.get()?)?
            .iter()
            .sum::<i64>();

        Ok(product.available_quantity - total_ordered + count > 0)
    }

    pub fn purchase(
        &self,
        product: &mut Product,
        count: i64,
        customer: &User,
        private_key: &[u8],
    ) -> Result<(ProductOrder, Transaction)> {
        use crate::data::schema::product_orders::dsl::*;

        Ok(self
            .conn
            .get()?
            .build_transaction()
            .run::<_, UserError, _>(|| {
                if !self.is_available(product, count)? {
                    return Err(UserError::OutOfStock);
                }

                let transaction = self.finance_svc.transfer(
                    customer,
                    &self.users_svc.get_by_id(product.user_id)?,
                    product.price.clone() * BigDecimal::from(count),
                    private_key,
                )?;

                Ok((
                    diesel::insert_into(product_orders)
                        .values(&NewProductOrder::new(
                            product,
                            count,
                            &transaction,
                        ))
                        .get_result(&self.conn.get()?)?,
                    transaction,
                ))
            })?)
    }

    pub fn confirm_order(
        &self,
        order: &ProductOrder,
        public_key: &[u8],
    ) -> Result<TransactionConfirmation> {
        self.finalize_order(
            order,
            TransactionConfirmationOutcome::Confirmed,
            public_key,
        )
    }

    pub fn decline_reservation(
        &self,
        order: &ProductOrder,
        public_key: &[u8],
    ) -> Result<TransactionConfirmation> {
        self.finalize_order(
            order,
            TransactionConfirmationOutcome::Declined,
            public_key,
        )
    }

    #[inline]
    fn finalize_order(
        &self,
        order: &ProductOrder,
        outcome: TransactionConfirmationOutcome,
        public_key: &[u8],
    ) -> Result<TransactionConfirmation> {
        use crate::data::schema::transactions::dsl::*;

        let transaction = transactions
            .find(order.transaction_id)
            .get_result(&self.conn.get()?)?;

        self.finance_svc.transactions_service().confirm(
            &transaction,
            outcome,
            public_key,
        )
    }
}
