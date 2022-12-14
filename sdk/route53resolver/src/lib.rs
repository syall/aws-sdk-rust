#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>When you create a VPC using Amazon VPC, you automatically get DNS resolution within the VPC
//! from Route 53 Resolver. By default, Resolver answers DNS queries for VPC domain names
//! such as domain names for EC2 instances or Elastic Load Balancing load balancers.
//! Resolver performs recursive lookups against public name servers for all other domain
//! names.</p>
//!
//! <p>You can also configure DNS resolution between your VPC and your network over a Direct Connect or VPN connection:</p>
//!
//! <p>
//! <b>Forward DNS queries from resolvers on your network to Route 53 Resolver</b>
//! </p>
//!
//! <p>DNS resolvers on your network can forward DNS queries to Resolver in a specified VPC. This allows your DNS resolvers
//! to easily resolve domain names for Amazon Web Services resources such as EC2 instances or records in a Route 53 private hosted zone.
//! For more information, see
//! <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/resolver.html#resolver-overview-forward-network-to-vpc">How DNS Resolvers
//! on Your Network Forward DNS Queries to Route 53 Resolver</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
//!
//! <p>
//! <b>Conditionally forward queries from a VPC to resolvers on your network</b>
//! </p>
//!
//! <p>You can configure Resolver to forward queries that it receives from EC2 instances in your VPCs to DNS resolvers on your network.
//! To forward selected queries, you create Resolver rules that specify the domain names for the DNS queries that you want to forward
//! (such as example.com), and the IP addresses of the DNS resolvers on your network that you want to forward the queries to.
//! If a query matches multiple rules (example.com, acme.example.com), Resolver chooses the rule with the most specific match
//! (acme.example.com) and forwards the query to the IP addresses that you specified in that rule. For more information, see
//! <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/resolver.html#resolver-overview-forward-vpc-to-network">How Route 53 Resolver
//! Forwards DNS Queries from Your VPCs to Your Network</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
//!
//! <p>Like Amazon VPC, Resolver is Regional. In each Region where you have VPCs, you can choose
//! whether to forward queries from your VPCs to your network (outbound queries), from your
//! network to your VPCs (inbound queries), or both.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// All error types that operations can return.
pub mod error;
mod error_meta;
mod idempotency_token;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::error::display::DisplayErrorContext;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("route53resolver", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
