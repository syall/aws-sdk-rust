// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_association::_add_association_output::AddAssociationOutputBuilder;

pub use crate::operation::add_association::_add_association_input::AddAssociationInputBuilder;

impl AddAssociationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_association::AddAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_association::AddAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_association();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddAssociation`.
///
/// <p>Creates an <i>association</i> between the source and the destination. A source can be associated with multiple destinations, and a destination can be associated with multiple sources. An association is a lineage tracking entity. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/lineage-tracking.html">Amazon SageMaker ML Lineage Tracking</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_association::builders::AddAssociationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_association::AddAssociationOutput,
        crate::operation::add_association::AddAssociationError,
    > for AddAssociationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_association::AddAssociationOutput,
            crate::operation::add_association::AddAssociationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddAssociationFluentBuilder {
    /// Creates a new `AddAssociation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddAssociation as a reference.
    pub fn as_input(&self) -> &crate::operation::add_association::builders::AddAssociationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::add_association::AddAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_association::AddAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_association::AddAssociation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_association::AddAssociation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_association::AddAssociationOutput,
        crate::operation::add_association::AddAssociationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ARN of the source.</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_arn(input.into());
        self
    }
    /// <p>The ARN of the source.</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_arn(input);
        self
    }
    /// <p>The ARN of the source.</p>
    pub fn get_source_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn destination_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn set_destination_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn get_destination_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_arn()
    }
    /// <p>The type of association. The following are suggested uses for each type. Amazon SageMaker places no restrictions on their use.</p>
    /// <ul>
    /// <li> <p>ContributedTo - The source contributed to the destination or had a part in enabling the destination. For example, the training data contributed to the training job.</p> </li>
    /// <li> <p>AssociatedWith - The source is connected to the destination. For example, an approval workflow is associated with a model deployment.</p> </li>
    /// <li> <p>DerivedFrom - The destination is a modification of the source. For example, a digest output of a channel input for a processing job is derived from the original inputs.</p> </li>
    /// <li> <p>Produced - The source generated the destination. For example, a training job produced a model artifact.</p> </li>
    /// </ul>
    pub fn association_type(mut self, input: crate::types::AssociationEdgeType) -> Self {
        self.inner = self.inner.association_type(input);
        self
    }
    /// <p>The type of association. The following are suggested uses for each type. Amazon SageMaker places no restrictions on their use.</p>
    /// <ul>
    /// <li> <p>ContributedTo - The source contributed to the destination or had a part in enabling the destination. For example, the training data contributed to the training job.</p> </li>
    /// <li> <p>AssociatedWith - The source is connected to the destination. For example, an approval workflow is associated with a model deployment.</p> </li>
    /// <li> <p>DerivedFrom - The destination is a modification of the source. For example, a digest output of a channel input for a processing job is derived from the original inputs.</p> </li>
    /// <li> <p>Produced - The source generated the destination. For example, a training job produced a model artifact.</p> </li>
    /// </ul>
    pub fn set_association_type(mut self, input: ::std::option::Option<crate::types::AssociationEdgeType>) -> Self {
        self.inner = self.inner.set_association_type(input);
        self
    }
    /// <p>The type of association. The following are suggested uses for each type. Amazon SageMaker places no restrictions on their use.</p>
    /// <ul>
    /// <li> <p>ContributedTo - The source contributed to the destination or had a part in enabling the destination. For example, the training data contributed to the training job.</p> </li>
    /// <li> <p>AssociatedWith - The source is connected to the destination. For example, an approval workflow is associated with a model deployment.</p> </li>
    /// <li> <p>DerivedFrom - The destination is a modification of the source. For example, a digest output of a channel input for a processing job is derived from the original inputs.</p> </li>
    /// <li> <p>Produced - The source generated the destination. For example, a training job produced a model artifact.</p> </li>
    /// </ul>
    pub fn get_association_type(&self) -> &::std::option::Option<crate::types::AssociationEdgeType> {
        self.inner.get_association_type()
    }
}
