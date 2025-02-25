// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `GetChunk`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct GetChunk;
impl GetChunk {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::get_chunk::GetChunkInput,
    ) -> ::std::result::Result<
        crate::operation::get_chunk::GetChunkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_chunk::GetChunkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err =
            |err: ::aws_smithy_runtime_api::client::result::SdkError<
                ::aws_smithy_runtime_api::client::interceptors::context::Error,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >| { err.map_service_error(|err| err.downcast::<crate::operation::get_chunk::GetChunkError>().expect("correct error type")) };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::get_chunk::GetChunkOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::get_chunk::GetChunkInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("backupstorage", "GetChunk", input, runtime_plugins, stop_point).await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
        runtime_plugins = runtime_plugins.with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![
            ::aws_runtime::auth::sigv4::SCHEME_ID,
        ]));
        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetChunk {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetChunk");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            GetChunkRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            GetChunkResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new("GetChunk", "backupstorage"));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        ::std::borrow::Cow::Owned(
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetChunk")
                .with_interceptor(GetChunkEndpointParamsInterceptor)
                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                    crate::operation::get_chunk::GetChunkError,
                >::new())
                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                    crate::operation::get_chunk::GetChunkError,
                >::new())
                .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                    crate::operation::get_chunk::GetChunkError,
                >::new()),
        )
    }
}

#[derive(Debug)]
struct GetChunkResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetChunkResponseDeserializer {
    fn deserialize_streaming(
        &self,
        response: &mut ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));

        // If this is an error, defer to the non-streaming parser
        if (!response.status().is_success() && response.status().as_u16() != 200) || force_error {
            return ::std::option::Option::None;
        }
        ::std::option::Option::Some(crate::protocol_serde::type_erase_result(
            crate::protocol_serde::shape_get_chunk::de_get_chunk_http_response(response),
        ))
    }

    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        // For streaming operations, we only hit this case if its an error
        let body = response.body().bytes().expect("body loaded");
        crate::protocol_serde::type_erase_result(crate::protocol_serde::shape_get_chunk::de_get_chunk_http_error(
            response.status().as_u16(),
            response.headers(),
            body,
        ))
    }
}
#[derive(Debug)]
struct GetChunkRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetChunkRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input.downcast::<crate::operation::get_chunk::GetChunkInput>().expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::get_chunk::GetChunkInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                let input_1 = &_input.storage_job_id;
                let input_1 = input_1
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("storage_job_id", "cannot be empty or unset"))?;
                let storage_job_id = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                if storage_job_id.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                        "storage_job_id",
                        "cannot be empty or unset",
                    ));
                }
                let input_2 = &_input.chunk_token;
                let input_2 = input_2
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("chunk_token", "cannot be empty or unset"))?;
                let chunk_token = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                if chunk_token.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
                        "chunk_token",
                        "cannot be empty or unset",
                    ));
                }
                ::std::write!(
                    output,
                    "/restore-jobs/{StorageJobId}/chunk/{ChunkToken}",
                    StorageJobId = storage_job_id,
                    ChunkToken = chunk_token
                )
                .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::get_chunk::GetChunkInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from("");

        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct GetChunkEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetChunkEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "GetChunkEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<GetChunkInput>()
            .ok_or("failed to downcast to GetChunkInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type GetChunkErrorKind = GetChunkError;
/// Error type for the `GetChunkError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GetChunkError {
    #[allow(missing_docs)] // documentation missing in model
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// Non-retryable exception, indicates client error (wrong argument passed to API). See exception message for details.
    IllegalArgumentException(crate::types::error::IllegalArgumentException),
    /// Non-retryable exception. Indicates the KMS key usage is incorrect. See exception message for details.
    KmsInvalidKeyUsageException(crate::types::error::KmsInvalidKeyUsageException),
    /// Non-retryable exception. Attempted to make an operation on non-existing or expired resource.
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// Retryable exception. In general indicates internal failure that can be fixed by retry.
    RetryableException(crate::types::error::RetryableException),
    /// Deprecated. To be removed from the model.
    ServiceInternalException(crate::types::error::ServiceInternalException),
    /// Increased rate over throttling limits. Can be retried with exponential backoff.
    ThrottlingException(crate::types::error::ThrottlingException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for GetChunkError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for GetChunkError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AccessDeniedException(_inner) => _inner.fmt(f),
            Self::IllegalArgumentException(_inner) => _inner.fmt(f),
            Self::KmsInvalidKeyUsageException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::RetryableException(_inner) => _inner.fmt(f),
            Self::ServiceInternalException(_inner) => _inner.fmt(f),
            Self::ThrottlingException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetChunkError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessDeniedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::IllegalArgumentException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::KmsInvalidKeyUsageException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::RetryableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ServiceInternalException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ThrottlingException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::get_chunk::GetChunkError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GetChunkError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl GetChunkError {
    /// Creates the `GetChunkError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err).build())
    }

    /// Creates the `GetChunkError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::AccessDeniedException(e) => e.meta(),
            Self::IllegalArgumentException(e) => e.meta(),
            Self::KmsInvalidKeyUsageException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::RetryableException(e) => e.meta(),
            Self::ServiceInternalException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GetChunkError::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(self, Self::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `GetChunkError::IllegalArgumentException`.
    pub fn is_illegal_argument_exception(&self) -> bool {
        matches!(self, Self::IllegalArgumentException(_))
    }
    /// Returns `true` if the error kind is `GetChunkError::KmsInvalidKeyUsageException`.
    pub fn is_kms_invalid_key_usage_exception(&self) -> bool {
        matches!(self, Self::KmsInvalidKeyUsageException(_))
    }
    /// Returns `true` if the error kind is `GetChunkError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `GetChunkError::RetryableException`.
    pub fn is_retryable_exception(&self) -> bool {
        matches!(self, Self::RetryableException(_))
    }
    /// Returns `true` if the error kind is `GetChunkError::ServiceInternalException`.
    pub fn is_service_internal_exception(&self) -> bool {
        matches!(self, Self::ServiceInternalException(_))
    }
    /// Returns `true` if the error kind is `GetChunkError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
}
impl ::std::error::Error for GetChunkError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AccessDeniedException(_inner) => ::std::option::Option::Some(_inner),
            Self::IllegalArgumentException(_inner) => ::std::option::Option::Some(_inner),
            Self::KmsInvalidKeyUsageException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::RetryableException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceInternalException(_inner) => ::std::option::Option::Some(_inner),
            Self::ThrottlingException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::get_chunk::_get_chunk_output::GetChunkOutput;

pub use crate::operation::get_chunk::_get_chunk_input::GetChunkInput;

mod _get_chunk_input;

mod _get_chunk_output;

/// Builders
pub mod builders;
