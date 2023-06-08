pub mod api;
pub mod entity;

#[macro_export]
macro_rules! impl_has_response {
    ($req:ty, $res:ty) => {
        impl $crate::api::HasResponse for $req {
            type Response = $res;
            fn req_type() -> &'static str {
                stringify!($req)
            }
        }
    };
}
