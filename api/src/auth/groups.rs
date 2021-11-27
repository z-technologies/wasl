use derive_more::Display;

#[derive(Display)]
pub enum AuthGroup {
    #[display(fmt = "customer")]
    Customer,

    #[display(fmt = "provider")]
    Provider,

    #[display(fmt = "admin")]
    Admin,
}
