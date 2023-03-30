// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Do not use this.
            ///
            /// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
            #[deprecated(note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).")]
            pub type UploadDocumentsErrorKind = UploadDocumentsError;
/// Error type for the `UploadDocumentsError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UploadDocumentsError {
    /// <p>Information about any problems encountered while processing an upload request.</p>
    DocumentServiceException(crate::error::DocumentServiceException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
                    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for UploadDocumentsError {
    
                    fn create_unhandled_error(
                        source: Box<dyn std::error::Error + Send + Sync + 'static>,
                        meta: Option<aws_smithy_types::error::ErrorMetadata>
                    ) -> Self
                     {
        Self::Unhandled({
                                let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
                                builder.set_meta(meta);
                                builder.build()
                            })
    }
}
impl std::fmt::Display for UploadDocumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DocumentServiceException(_inner) =>
            _inner.fmt(f)
            ,
            Self::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for UploadDocumentsError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DocumentServiceException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::error::UploadDocumentsError {
                            fn request_id(&self) -> Option<&str> {
                                self.meta().request_id()
                            }
                        }
impl aws_smithy_types::retry::ProvideErrorKind for UploadDocumentsError {
    fn code(&self) -> Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl UploadDocumentsError {
    /// Creates the `UploadDocumentsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err).build())
                    }
    
                    /// Creates the `UploadDocumentsError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
                    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
                    }
    /// 
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    /// 
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::DocumentServiceException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `UploadDocumentsError::DocumentServiceException`.
    pub fn is_document_service_exception(&self) -> bool {
        matches!(self, Self::DocumentServiceException(_))
    }
}
impl std::error::Error for UploadDocumentsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DocumentServiceException(_inner) =>
            Some(_inner)
            ,
            Self::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>Information about any problems encountered while processing an upload request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DocumentServiceException  {
    /// <p>The return status of a document upload request, <code>error</code> or <code>success</code>.</p>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
    /// <p>The description of the errors returned by the document service.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl DocumentServiceException {
    /// <p>The return status of a document upload request, <code>error</code> or <code>success</code>.</p>
    pub fn status(&self) -> std::option::Option<& str> {
        self.status.as_deref()
    }
}
impl DocumentServiceException {
    /// Creates a new builder-style object to manufacture [`DocumentServiceException`](crate::error::DocumentServiceException).
    pub fn builder() -> crate::error::document_service_exception::Builder {
        crate::error::document_service_exception::Builder::default()
    }
}
/// See [`DocumentServiceException`](crate::error::DocumentServiceException).
pub mod document_service_exception {
    
    /// A builder for [`DocumentServiceException`](crate::error::DocumentServiceException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        /// <p>The return status of a document upload request, <code>error</code> or <code>success</code>.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p>The return status of a document upload request, <code>error</code> or <code>success</code>.</p>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input; self
        }
        /// <p>The description of the errors returned by the document service.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>The description of the errors returned by the document service.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                            pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                self.meta = Some(meta);
                                                self
                                            }
        
                                            /// Sets error metadata
                                            pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                self.meta = meta;
                                                self
                                            }
        /// Consumes the builder and constructs a [`DocumentServiceException`](crate::error::DocumentServiceException).
        pub fn build(self) -> crate::error::DocumentServiceException {
            crate::error::DocumentServiceException {
                status: self.status
                ,
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}
impl DocumentServiceException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for DocumentServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DocumentServiceException")?;
        if let Some(inner_1) = &self.message {
             {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for DocumentServiceException {}
impl aws_http::request_id::RequestId for crate::error::DocumentServiceException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for DocumentServiceException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}

/// Do not use this.
            ///
            /// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
            #[deprecated(note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).")]
            pub type SuggestErrorKind = SuggestError;
/// Error type for the `SuggestError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SuggestError {
    /// <p>Information about any problems encountered while processing a search request.</p>
    SearchException(crate::error::SearchException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
                    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for SuggestError {
    
                    fn create_unhandled_error(
                        source: Box<dyn std::error::Error + Send + Sync + 'static>,
                        meta: Option<aws_smithy_types::error::ErrorMetadata>
                    ) -> Self
                     {
        Self::Unhandled({
                                let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
                                builder.set_meta(meta);
                                builder.build()
                            })
    }
}
impl std::fmt::Display for SuggestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SearchException(_inner) =>
            _inner.fmt(f)
            ,
            Self::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for SuggestError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::SearchException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::error::SuggestError {
                            fn request_id(&self) -> Option<&str> {
                                self.meta().request_id()
                            }
                        }
impl aws_smithy_types::retry::ProvideErrorKind for SuggestError {
    fn code(&self) -> Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl SuggestError {
    /// Creates the `SuggestError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err).build())
                    }
    
                    /// Creates the `SuggestError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
                    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
                    }
    /// 
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    /// 
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::SearchException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `SuggestError::SearchException`.
    pub fn is_search_exception(&self) -> bool {
        matches!(self, Self::SearchException(_))
    }
}
impl std::error::Error for SuggestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SearchException(_inner) =>
            Some(_inner)
            ,
            Self::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>Information about any problems encountered while processing a search request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SearchException  {
    /// <p>A description of the error returned by the search service.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl SearchException {
    /// Creates a new builder-style object to manufacture [`SearchException`](crate::error::SearchException).
    pub fn builder() -> crate::error::search_exception::Builder {
        crate::error::search_exception::Builder::default()
    }
}
/// See [`SearchException`](crate::error::SearchException).
pub mod search_exception {
    
    /// A builder for [`SearchException`](crate::error::SearchException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        meta: Option<aws_smithy_types::error::ErrorMetadata>,
    }
    impl Builder {
        /// <p>A description of the error returned by the search service.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>A description of the error returned by the search service.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Sets error metadata
                                            pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
                                                self.meta = Some(meta);
                                                self
                                            }
        
                                            /// Sets error metadata
                                            pub fn set_meta(&mut self, meta: Option<aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                self.meta = meta;
                                                self
                                            }
        /// Consumes the builder and constructs a [`SearchException`](crate::error::SearchException).
        pub fn build(self) -> crate::error::SearchException {
            crate::error::SearchException {
                message: self.message
                ,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
    
    
}
impl SearchException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for SearchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SearchException")?;
        if let Some(inner_2) = &self.message {
             {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for SearchException {}
impl aws_http::request_id::RequestId for crate::error::SearchException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for SearchException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata { &self.meta }
}

/// Do not use this.
            ///
            /// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
            #[deprecated(note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).")]
            pub type SearchErrorKind = SearchError;
/// Error type for the `SearchError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SearchError {
    /// <p>Information about any problems encountered while processing a search request.</p>
    SearchException(crate::error::SearchException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
                    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for SearchError {
    
                    fn create_unhandled_error(
                        source: Box<dyn std::error::Error + Send + Sync + 'static>,
                        meta: Option<aws_smithy_types::error::ErrorMetadata>
                    ) -> Self
                     {
        Self::Unhandled({
                                let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
                                builder.set_meta(meta);
                                builder.build()
                            })
    }
}
impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SearchException(_inner) =>
            _inner.fmt(f)
            ,
            Self::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for SearchError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::SearchException(_inner) =>
            aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            ,
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::error::SearchError {
                            fn request_id(&self) -> Option<&str> {
                                self.meta().request_id()
                            }
                        }
impl aws_smithy_types::retry::ProvideErrorKind for SearchError {
    fn code(&self) -> Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl SearchError {
    /// Creates the `SearchError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err).build())
                    }
    
                    /// Creates the `SearchError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
                    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
                        Self::Unhandled(aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
                    }
    /// 
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    /// 
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::SearchException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `SearchError::SearchException`.
    pub fn is_search_exception(&self) -> bool {
        matches!(self, Self::SearchException(_))
    }
}
impl std::error::Error for SearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SearchException(_inner) =>
            Some(_inner)
            ,
            Self::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

