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
//! <p>Amazon GuardDuty is a continuous security monitoring service that analyzes and processes
//! the following data sources: VPC Flow Logs, AWS CloudTrail management event logs, CloudTrail S3 data event
//! logs, EKS audit logs, and DNS logs.
//! It uses threat intelligence
//! feeds (such as lists of malicious IPs and domains) and machine learning to identify
//! unexpected, potentially unauthorized, and malicious activity within your Amazon Web Services environment.
//! This can include issues like escalations of privileges, uses of exposed credentials, or
//! communication with malicious IPs, URLs, or domains. For example, GuardDuty can detect
//! compromised EC2 instances that serve malware or mine bitcoin. </p>
//! <p>GuardDuty also monitors Amazon Web Services account access behavior for signs of compromise. Some examples
//! of this are unauthorized infrastructure deployments such as EC2 instances deployed in a Region
//! that has never been used, or unusual API calls like a password policy change to reduce
//! password strength. </p>
//! <p>GuardDuty informs you of the status of your Amazon Web Services environment by producing security findings
//! that you can view in the GuardDuty console or through Amazon CloudWatch events. For more
//! information, see the <i>
//! <a href="https://docs.aws.amazon.com/guardduty/latest/ug/what-is-guardduty.html">Amazon
//! GuardDuty User Guide</a>
//! </i>. </p>
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
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("guardduty", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
