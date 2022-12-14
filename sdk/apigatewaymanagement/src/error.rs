// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The data has exceeded the maximum size allowed.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PayloadTooLargeException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl PayloadTooLargeException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for PayloadTooLargeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PayloadTooLargeException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for PayloadTooLargeException {}
/// See [`PayloadTooLargeException`](crate::error::PayloadTooLargeException).
pub mod payload_too_large_exception {

    /// A builder for [`PayloadTooLargeException`](crate::error::PayloadTooLargeException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`PayloadTooLargeException`](crate::error::PayloadTooLargeException).
        pub fn build(self) -> crate::error::PayloadTooLargeException {
            crate::error::PayloadTooLargeException {
                message: self.message,
            }
        }
    }
}
impl PayloadTooLargeException {
    /// Creates a new builder-style object to manufacture [`PayloadTooLargeException`](crate::error::PayloadTooLargeException).
    pub fn builder() -> crate::error::payload_too_large_exception::Builder {
        crate::error::payload_too_large_exception::Builder::default()
    }
}

/// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct LimitExceededException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl LimitExceededException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LimitExceededException")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for LimitExceededException {}
/// See [`LimitExceededException`](crate::error::LimitExceededException).
pub mod limit_exceeded_exception {

    /// A builder for [`LimitExceededException`](crate::error::LimitExceededException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`LimitExceededException`](crate::error::LimitExceededException).
        pub fn build(self) -> crate::error::LimitExceededException {
            crate::error::LimitExceededException {
                message: self.message,
            }
        }
    }
}
impl LimitExceededException {
    /// Creates a new builder-style object to manufacture [`LimitExceededException`](crate::error::LimitExceededException).
    pub fn builder() -> crate::error::limit_exceeded_exception::Builder {
        crate::error::limit_exceeded_exception::Builder::default()
    }
}

/// <p>The connection with the provided id no longer exists.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GoneException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl GoneException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for GoneException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoneException")?;
        if let Some(inner_3) = &self.message {
            {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for GoneException {}
/// See [`GoneException`](crate::error::GoneException).
pub mod gone_exception {

    /// A builder for [`GoneException`](crate::error::GoneException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`GoneException`](crate::error::GoneException).
        pub fn build(self) -> crate::error::GoneException {
            crate::error::GoneException {
                message: self.message,
            }
        }
    }
}
impl GoneException {
    /// Creates a new builder-style object to manufacture [`GoneException`](crate::error::GoneException).
    pub fn builder() -> crate::error::gone_exception::Builder {
        crate::error::gone_exception::Builder::default()
    }
}

/// <p>The caller is not authorized to invoke this operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ForbiddenException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ForbiddenException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ForbiddenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForbiddenException")?;
        if let Some(inner_4) = &self.message {
            {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ForbiddenException {}
/// See [`ForbiddenException`](crate::error::ForbiddenException).
pub mod forbidden_exception {

    /// A builder for [`ForbiddenException`](crate::error::ForbiddenException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ForbiddenException`](crate::error::ForbiddenException).
        pub fn build(self) -> crate::error::ForbiddenException {
            crate::error::ForbiddenException {
                message: self.message,
            }
        }
    }
}
impl ForbiddenException {
    /// Creates a new builder-style object to manufacture [`ForbiddenException`](crate::error::ForbiddenException).
    pub fn builder() -> crate::error::forbidden_exception::Builder {
        crate::error::forbidden_exception::Builder::default()
    }
}

/// Error type for the `DeleteConnection` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteConnectionError {
    /// Kind of error that occurred.
    pub kind: DeleteConnectionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for DeleteConnectionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: DeleteConnectionErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `DeleteConnection` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteConnectionErrorKind {
    /// <p>The caller is not authorized to invoke this operation.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>The connection with the provided id no longer exists.</p>
    GoneException(crate::error::GoneException),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceededException(crate::error::LimitExceededException),
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
impl std::fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteConnectionErrorKind::ForbiddenException(_inner) => _inner.fmt(f),
            DeleteConnectionErrorKind::GoneException(_inner) => _inner.fmt(f),
            DeleteConnectionErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            DeleteConnectionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteConnectionError {
    fn code(&self) -> Option<&str> {
        DeleteConnectionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteConnectionError {
    /// Creates a new `DeleteConnectionError`.
    pub fn new(kind: DeleteConnectionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DeleteConnectionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteConnectionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `DeleteConnectionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteConnectionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `DeleteConnectionErrorKind::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(&self.kind, DeleteConnectionErrorKind::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `DeleteConnectionErrorKind::GoneException`.
    pub fn is_gone_exception(&self) -> bool {
        matches!(&self.kind, DeleteConnectionErrorKind::GoneException(_))
    }
    /// Returns `true` if the error kind is `DeleteConnectionErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteConnectionErrorKind::LimitExceededException(_)
        )
    }
}
impl std::error::Error for DeleteConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteConnectionErrorKind::ForbiddenException(_inner) => Some(_inner),
            DeleteConnectionErrorKind::GoneException(_inner) => Some(_inner),
            DeleteConnectionErrorKind::LimitExceededException(_inner) => Some(_inner),
            DeleteConnectionErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `GetConnection` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetConnectionError {
    /// Kind of error that occurred.
    pub kind: GetConnectionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetConnectionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetConnectionErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetConnection` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetConnectionErrorKind {
    /// <p>The caller is not authorized to invoke this operation.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>The connection with the provided id no longer exists.</p>
    GoneException(crate::error::GoneException),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceededException(crate::error::LimitExceededException),
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
impl std::fmt::Display for GetConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetConnectionErrorKind::ForbiddenException(_inner) => _inner.fmt(f),
            GetConnectionErrorKind::GoneException(_inner) => _inner.fmt(f),
            GetConnectionErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            GetConnectionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetConnectionError {
    fn code(&self) -> Option<&str> {
        GetConnectionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetConnectionError {
    /// Creates a new `GetConnectionError`.
    pub fn new(kind: GetConnectionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetConnectionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetConnectionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `GetConnectionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetConnectionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `GetConnectionErrorKind::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(&self.kind, GetConnectionErrorKind::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `GetConnectionErrorKind::GoneException`.
    pub fn is_gone_exception(&self) -> bool {
        matches!(&self.kind, GetConnectionErrorKind::GoneException(_))
    }
    /// Returns `true` if the error kind is `GetConnectionErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetConnectionErrorKind::LimitExceededException(_)
        )
    }
}
impl std::error::Error for GetConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetConnectionErrorKind::ForbiddenException(_inner) => Some(_inner),
            GetConnectionErrorKind::GoneException(_inner) => Some(_inner),
            GetConnectionErrorKind::LimitExceededException(_inner) => Some(_inner),
            GetConnectionErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `PostToConnection` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PostToConnectionError {
    /// Kind of error that occurred.
    pub kind: PostToConnectionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PostToConnectionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PostToConnectionErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PostToConnection` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PostToConnectionErrorKind {
    /// <p>The caller is not authorized to invoke this operation.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>The connection with the provided id no longer exists.</p>
    GoneException(crate::error::GoneException),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The data has exceeded the maximum size allowed.</p>
    PayloadTooLargeException(crate::error::PayloadTooLargeException),
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
impl std::fmt::Display for PostToConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PostToConnectionErrorKind::ForbiddenException(_inner) => _inner.fmt(f),
            PostToConnectionErrorKind::GoneException(_inner) => _inner.fmt(f),
            PostToConnectionErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            PostToConnectionErrorKind::PayloadTooLargeException(_inner) => _inner.fmt(f),
            PostToConnectionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PostToConnectionError {
    fn code(&self) -> Option<&str> {
        PostToConnectionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PostToConnectionError {
    /// Creates a new `PostToConnectionError`.
    pub fn new(kind: PostToConnectionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PostToConnectionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PostToConnectionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `PostToConnectionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PostToConnectionErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `PostToConnectionErrorKind::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(&self.kind, PostToConnectionErrorKind::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `PostToConnectionErrorKind::GoneException`.
    pub fn is_gone_exception(&self) -> bool {
        matches!(&self.kind, PostToConnectionErrorKind::GoneException(_))
    }
    /// Returns `true` if the error kind is `PostToConnectionErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            PostToConnectionErrorKind::LimitExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `PostToConnectionErrorKind::PayloadTooLargeException`.
    pub fn is_payload_too_large_exception(&self) -> bool {
        matches!(
            &self.kind,
            PostToConnectionErrorKind::PayloadTooLargeException(_)
        )
    }
}
impl std::error::Error for PostToConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PostToConnectionErrorKind::ForbiddenException(_inner) => Some(_inner),
            PostToConnectionErrorKind::GoneException(_inner) => Some(_inner),
            PostToConnectionErrorKind::LimitExceededException(_inner) => Some(_inner),
            PostToConnectionErrorKind::PayloadTooLargeException(_inner) => Some(_inner),
            PostToConnectionErrorKind::Unhandled(_inner) => Some(_inner),
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
