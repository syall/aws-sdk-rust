// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>User does not have sufficient access to perform this action. </p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Unexpected error during processing of request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Request would cause a service quota to be exceeded. The limit is 10 concurrent operations. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p> Request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableControlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisableControlError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                                            aws_smithy_types::error::Unhandled::builder()
                                                .meta(aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                                                .source(err)
                                                .build()
                                        ),
        }
    }
}
impl From<crate::error::DisableControlError> for Error {
    fn from(err: crate::error::DisableControlError) -> Self {
        match err {
            crate::error::DisableControlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisableControlError::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DisableControlError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DisableControlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DisableControlError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::DisableControlError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DisableControlError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DisableControlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableControlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::EnableControlError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                                            aws_smithy_types::error::Unhandled::builder()
                                                .meta(aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                                                .source(err)
                                                .build()
                                        ),
        }
    }
}
impl From<crate::error::EnableControlError> for Error {
    fn from(err: crate::error::EnableControlError) -> Self {
        match err {
            crate::error::EnableControlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::EnableControlError::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::EnableControlError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::EnableControlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::EnableControlError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::EnableControlError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::EnableControlError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::EnableControlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetControlOperationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetControlOperationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                                            aws_smithy_types::error::Unhandled::builder()
                                                .meta(aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                                                .source(err)
                                                .build()
                                        ),
        }
    }
}
impl From<crate::error::GetControlOperationError> for Error {
    fn from(err: crate::error::GetControlOperationError) -> Self {
        match err {
            crate::error::GetControlOperationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetControlOperationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetControlOperationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetControlOperationError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::GetControlOperationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetControlOperationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEnabledControlsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEnabledControlsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                                            aws_smithy_types::error::Unhandled::builder()
                                                .meta(aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                                                .source(err)
                                                .build()
                                        ),
        }
    }
}
impl From<crate::error::ListEnabledControlsError> for Error {
    fn from(err: crate::error::ListEnabledControlsError) -> Self {
        match err {
            crate::error::ListEnabledControlsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListEnabledControlsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListEnabledControlsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListEnabledControlsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListEnabledControlsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListEnabledControlsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}

