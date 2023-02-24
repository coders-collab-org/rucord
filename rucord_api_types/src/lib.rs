#[macro_use]
mod macros;

pub type Snowflake = String;

pub mod gateway;
pub mod structures;

pub use gateway::*;
pub use structures::*;

macro_rules! create_routes {
    ($name:ident => $ret:literal $($tt:tt)*) => {
        create_routes! {
            $name() => $ret
            $($tt)*
        }
    };

    ($name:ident($($param_name:path: $param_ty:ty),* $(,)?) => $ret:literal $($tt:tt)*) => {
        #[inline(always)]
        pub fn $name($(param_name: $param_ty),*) -> String {
            format!($ret)
        }


        create_routes! {
            $($tt)*
        }


    };

    () => {}
}

pub mod routes {

    create_routes! {
        gateway => "/gateway"

        gateway_bot => "/gateway/bot"
    }
}
