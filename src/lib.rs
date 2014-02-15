#[crate_id = "rustful#0.1-pre"];

#[comment = "RESTful web framework"];
#[license = "MIT"];
#[crate_type = "rlib"];

extern mod extra;

extern mod http;

pub use router::Router;
pub use server::Server;

pub mod router;
pub mod server;