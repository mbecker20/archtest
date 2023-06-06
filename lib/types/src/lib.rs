pub mod api;
pub mod entity;

#[macro_export]
macro_rules! impl_has_req_type {
    ($t:ty) => {
        impl $crate::api::HasReqType for $t {
            fn req_type() -> &'static str {
                stringify!($t)
            }
        }
    };
}

// #[proc_macro_derive(HasReqType)]
// pub fn derive_has_req_type() {

// }
