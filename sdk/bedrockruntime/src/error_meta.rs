// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The request is denied because of missing access permissions.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The request failed due to an error while processing the model.</p>
    ModelErrorException(crate::types::error::ModelErrorException),
    /// <p>The model specified in the request is not ready to serve inference requests.</p>
    ModelNotReadyException(crate::types::error::ModelNotReadyException),
    /// <p>An error occurred while streaming the response.</p>
    ModelStreamErrorException(crate::types::error::ModelStreamErrorException),
    /// <p>The request took too long to process. Processing time exceeded the model timeout length.</p>
    ModelTimeoutException(crate::types::error::ModelTimeoutException),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The number of requests exceeds the service quota. Resubmit your request later.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>Input validation failed. Check your request parameters and retry the request.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ModelErrorException(inner) => inner.fmt(f),
            Error::ModelNotReadyException(inner) => inner.fmt(f),
            Error::ModelStreamErrorException(inner) => inner.fmt(f),
            Error::ModelTimeoutException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::invoke_model::InvokeModelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::invoke_model::InvokeModelError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::invoke_model::InvokeModelError> for Error {
    fn from(err: crate::operation::invoke_model::InvokeModelError) -> Self {
        match err {
            crate::operation::invoke_model::InvokeModelError::ModelTimeoutException(inner) => Error::ModelTimeoutException(inner),
            crate::operation::invoke_model::InvokeModelError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::invoke_model::InvokeModelError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::invoke_model::InvokeModelError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::invoke_model::InvokeModelError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::invoke_model::InvokeModelError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::invoke_model::InvokeModelError::ModelNotReadyException(inner) => Error::ModelNotReadyException(inner),
            crate::operation::invoke_model::InvokeModelError::ModelErrorException(inner) => Error::ModelErrorException(inner),
            crate::operation::invoke_model::InvokeModelError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::invoke_model::InvokeModelError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError> for Error {
    fn from(err: crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError) -> Self {
        match err {
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ModelTimeoutException(inner) => {
                Error::ModelTimeoutException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ModelStreamErrorException(inner) => {
                Error::ModelStreamErrorException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ModelNotReadyException(inner) => {
                Error::ModelNotReadyException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ModelErrorException(inner) => {
                Error::ModelErrorException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::ResponseStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::ResponseStreamError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::types::error::ResponseStreamError> for Error {
    fn from(err: crate::types::error::ResponseStreamError) -> Self {
        match err {
            crate::types::error::ResponseStreamError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::types::error::ResponseStreamError::ModelStreamErrorException(inner) => Error::ModelStreamErrorException(inner),
            crate::types::error::ResponseStreamError::ValidationException(inner) => Error::ValidationException(inner),
            crate::types::error::ResponseStreamError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::types::error::ResponseStreamError::ModelTimeoutException(inner) => Error::ModelTimeoutException(inner),
            crate::types::error::ResponseStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ModelErrorException(inner) => inner.source(),
            Error::ModelNotReadyException(inner) => inner.source(),
            Error::ModelStreamErrorException(inner) => inner.source(),
            Error::ModelTimeoutException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ModelErrorException(e) => e.request_id(),
            Self::ModelNotReadyException(e) => e.request_id(),
            Self::ModelStreamErrorException(e) => e.request_id(),
            Self::ModelTimeoutException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
