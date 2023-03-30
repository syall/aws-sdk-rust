// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Too many tags.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// <p>Validation exception error.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateProfileError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateProfileError, R>) -> Self {
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
impl From<crate::error::CreateProfileError> for Error {
    fn from(err: crate::error::CreateProfileError) -> Self {
        match err {
            crate::error::CreateProfileError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateProfileError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateProfileError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateTrustAnchorError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateTrustAnchorError, R>) -> Self {
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
impl From<crate::error::CreateTrustAnchorError> for Error {
    fn from(err: crate::error::CreateTrustAnchorError) -> Self {
        match err {
            crate::error::CreateTrustAnchorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateTrustAnchorError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateTrustAnchorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteCrlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteCrlError, R>) -> Self {
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
impl From<crate::error::DeleteCrlError> for Error {
    fn from(err: crate::error::DeleteCrlError) -> Self {
        match err {
            crate::error::DeleteCrlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteCrlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteCrlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteProfileError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteProfileError, R>) -> Self {
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
impl From<crate::error::DeleteProfileError> for Error {
    fn from(err: crate::error::DeleteProfileError) -> Self {
        match err {
            crate::error::DeleteProfileError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteProfileError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteProfileError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteTrustAnchorError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteTrustAnchorError, R>) -> Self {
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
impl From<crate::error::DeleteTrustAnchorError> for Error {
    fn from(err: crate::error::DeleteTrustAnchorError) -> Self {
        match err {
            crate::error::DeleteTrustAnchorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteTrustAnchorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteTrustAnchorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableCrlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisableCrlError, R>) -> Self {
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
impl From<crate::error::DisableCrlError> for Error {
    fn from(err: crate::error::DisableCrlError) -> Self {
        match err {
            crate::error::DisableCrlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisableCrlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DisableCrlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableProfileError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisableProfileError, R>) -> Self {
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
impl From<crate::error::DisableProfileError> for Error {
    fn from(err: crate::error::DisableProfileError) -> Self {
        match err {
            crate::error::DisableProfileError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisableProfileError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DisableProfileError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableTrustAnchorError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisableTrustAnchorError, R>) -> Self {
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
impl From<crate::error::DisableTrustAnchorError> for Error {
    fn from(err: crate::error::DisableTrustAnchorError) -> Self {
        match err {
            crate::error::DisableTrustAnchorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisableTrustAnchorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DisableTrustAnchorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableCrlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::EnableCrlError, R>) -> Self {
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
impl From<crate::error::EnableCrlError> for Error {
    fn from(err: crate::error::EnableCrlError) -> Self {
        match err {
            crate::error::EnableCrlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::EnableCrlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::EnableCrlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableProfileError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::EnableProfileError, R>) -> Self {
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
impl From<crate::error::EnableProfileError> for Error {
    fn from(err: crate::error::EnableProfileError) -> Self {
        match err {
            crate::error::EnableProfileError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::EnableProfileError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::EnableProfileError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableTrustAnchorError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::EnableTrustAnchorError, R>) -> Self {
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
impl From<crate::error::EnableTrustAnchorError> for Error {
    fn from(err: crate::error::EnableTrustAnchorError) -> Self {
        match err {
            crate::error::EnableTrustAnchorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::EnableTrustAnchorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::EnableTrustAnchorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCrlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetCrlError, R>) -> Self {
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
impl From<crate::error::GetCrlError> for Error {
    fn from(err: crate::error::GetCrlError) -> Self {
        match err {
            crate::error::GetCrlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetCrlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetProfileError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetProfileError, R>) -> Self {
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
impl From<crate::error::GetProfileError> for Error {
    fn from(err: crate::error::GetProfileError) -> Self {
        match err {
            crate::error::GetProfileError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetProfileError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetProfileError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSubjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSubjectError, R>) -> Self {
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
impl From<crate::error::GetSubjectError> for Error {
    fn from(err: crate::error::GetSubjectError) -> Self {
        match err {
            crate::error::GetSubjectError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetSubjectError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetSubjectError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTrustAnchorError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTrustAnchorError, R>) -> Self {
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
impl From<crate::error::GetTrustAnchorError> for Error {
    fn from(err: crate::error::GetTrustAnchorError) -> Self {
        match err {
            crate::error::GetTrustAnchorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetTrustAnchorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetTrustAnchorError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetTrustAnchorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ImportCrlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ImportCrlError, R>) -> Self {
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
impl From<crate::error::ImportCrlError> for Error {
    fn from(err: crate::error::ImportCrlError) -> Self {
        match err {
            crate::error::ImportCrlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ImportCrlError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ImportCrlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCrlsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListCrlsError, R>) -> Self {
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
impl From<crate::error::ListCrlsError> for Error {
    fn from(err: crate::error::ListCrlsError) -> Self {
        match err {
            crate::error::ListCrlsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListCrlsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListCrlsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProfilesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListProfilesError, R>) -> Self {
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
impl From<crate::error::ListProfilesError> for Error {
    fn from(err: crate::error::ListProfilesError) -> Self {
        match err {
            crate::error::ListProfilesError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListProfilesError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListProfilesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSubjectsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSubjectsError, R>) -> Self {
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
impl From<crate::error::ListSubjectsError> for Error {
    fn from(err: crate::error::ListSubjectsError) -> Self {
        match err {
            crate::error::ListSubjectsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListSubjectsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListSubjectsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
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
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err {
            crate::error::ListTagsForResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListTagsForResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTrustAnchorsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTrustAnchorsError, R>) -> Self {
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
impl From<crate::error::ListTrustAnchorsError> for Error {
    fn from(err: crate::error::ListTrustAnchorsError) -> Self {
        match err {
            crate::error::ListTrustAnchorsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListTrustAnchorsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListTrustAnchorsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
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
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err {
            crate::error::TagResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::TagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::TagResourceError::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
            crate::error::TagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::TagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
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
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err {
            crate::error::UntagResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UntagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UntagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateCrlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateCrlError, R>) -> Self {
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
impl From<crate::error::UpdateCrlError> for Error {
    fn from(err: crate::error::UpdateCrlError) -> Self {
        match err {
            crate::error::UpdateCrlError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateCrlError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateCrlError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateCrlError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateProfileError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateProfileError, R>) -> Self {
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
impl From<crate::error::UpdateProfileError> for Error {
    fn from(err: crate::error::UpdateProfileError) -> Self {
        match err {
            crate::error::UpdateProfileError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateProfileError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateProfileError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateProfileError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateTrustAnchorError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateTrustAnchorError, R>) -> Self {
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
impl From<crate::error::UpdateTrustAnchorError> for Error {
    fn from(err: crate::error::UpdateTrustAnchorError) -> Self {
        match err {
            crate::error::UpdateTrustAnchorError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateTrustAnchorError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateTrustAnchorError::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateTrustAnchorError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::TooManyTagsException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}

