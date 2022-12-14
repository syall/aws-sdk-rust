// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// This exception is thrown when an internal service error occurs.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct MarketplaceCommerceAnalyticsException {
    /// This message describes details of the error.
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl MarketplaceCommerceAnalyticsException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MarketplaceCommerceAnalyticsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MarketplaceCommerceAnalyticsException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for MarketplaceCommerceAnalyticsException {}
/// See [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException).
pub mod marketplace_commerce_analytics_exception {

    /// A builder for [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// This message describes details of the error.
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// This message describes details of the error.
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException).
        pub fn build(self) -> crate::error::MarketplaceCommerceAnalyticsException {
            crate::error::MarketplaceCommerceAnalyticsException {
                message: self.message,
            }
        }
    }
}
impl MarketplaceCommerceAnalyticsException {
    /// Creates a new builder-style object to manufacture [`MarketplaceCommerceAnalyticsException`](crate::error::MarketplaceCommerceAnalyticsException).
    pub fn builder() -> crate::error::marketplace_commerce_analytics_exception::Builder {
        crate::error::marketplace_commerce_analytics_exception::Builder::default()
    }
}

/// Error type for the `GenerateDataSet` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GenerateDataSetError {
    /// Kind of error that occurred.
    pub kind: GenerateDataSetErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GenerateDataSetError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GenerateDataSetErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GenerateDataSet` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GenerateDataSetErrorKind {
    /// This exception is thrown when an internal service error occurs.
    MarketplaceCommerceAnalyticsException(crate::error::MarketplaceCommerceAnalyticsException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for GenerateDataSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException(_inner) => {
                _inner.fmt(f)
            }
            GenerateDataSetErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GenerateDataSetError {
    fn code(&self) -> Option<&str> {
        GenerateDataSetError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GenerateDataSetError {
    /// Creates a new `GenerateDataSetError`.
    pub fn new(kind: GenerateDataSetErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GenerateDataSetError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GenerateDataSetErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `GenerateDataSetError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GenerateDataSetErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException`.
    pub fn is_marketplace_commerce_analytics_exception(&self) -> bool {
        matches!(
            &self.kind,
            GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException(_)
        )
    }
}
impl std::error::Error for GenerateDataSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GenerateDataSetErrorKind::MarketplaceCommerceAnalyticsException(_inner) => Some(_inner),
            GenerateDataSetErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `StartSupportDataExport` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartSupportDataExportError {
    /// Kind of error that occurred.
    pub kind: StartSupportDataExportErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for StartSupportDataExportError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: StartSupportDataExportErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `StartSupportDataExport` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartSupportDataExportErrorKind {
    /// This exception is thrown when an internal service error occurs.
    MarketplaceCommerceAnalyticsException(crate::error::MarketplaceCommerceAnalyticsException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for StartSupportDataExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException(_inner) => {
                _inner.fmt(f)
            }
            StartSupportDataExportErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for StartSupportDataExportError {
    fn code(&self) -> Option<&str> {
        StartSupportDataExportError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartSupportDataExportError {
    /// Creates a new `StartSupportDataExportError`.
    pub fn new(kind: StartSupportDataExportErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `StartSupportDataExportError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartSupportDataExportErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `StartSupportDataExportError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartSupportDataExportErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException`.
    pub fn is_marketplace_commerce_analytics_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException(_)
        )
    }
}
impl std::error::Error for StartSupportDataExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartSupportDataExportErrorKind::MarketplaceCommerceAnalyticsException(_inner) => {
                Some(_inner)
            }
            StartSupportDataExportErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
