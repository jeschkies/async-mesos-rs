//! Asynchronous Mesos Client
//!
//! This crate provides an asynchronous client for the Mesos
//! [Scheduler HTTP API](http://mesos.apache.org/documentation/latest/scheduler-http-api).
//!
//! ## Installation
//!
//! Simply add this to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! async_mesos = 0.1
//! ```
//!
//! ## Getting Started
//!
//! ```rust
//! # extern crate async_mesos;
//! # extern crate hyper;
//! # extern crate tokio_core;
//! # fn main() {
//! use async_mesos::mesos;
//! use async_mesos::client::Client;
//! use hyper::Uri;
//! use tokio_core::reactor::Core;
//!
//! // Create a Tokio core handle.
//! let mut core = Core::new().expect("Could not create core.");
//! let handle = core.handle();
//!
//! // Create the Mesos framework info to register a new framework.
//! let mut framework_info = mesos::FrameworkInfo::new();
//! framework_info.set_user(String::from("donnie"));
//! framework_info.set_name(String::from("Example FOO Framework"));
//!
//! // Connect to Mesos scheduler API.
//! let uri = "http://localhost:5050/api/v1/scheduler"
//!     .parse::<Uri>()
//!     .expect("Could not parse Uri.");
//! let client = Client::connect(&handle, uri, framework_info);
//! # }
//! ```

extern crate bytes;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mime;
extern crate protobuf;
extern crate spectral; // TODO: Move to test dependencies.
extern crate tokio_core;

pub mod client;
pub mod mesos;
pub mod model;
pub mod scheduler;
