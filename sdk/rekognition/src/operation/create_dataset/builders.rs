// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dataset::_create_dataset_output::CreateDatasetOutputBuilder;

pub use crate::operation::create_dataset::_create_dataset_input::CreateDatasetInputBuilder;

impl CreateDatasetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dataset::CreateDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_dataset();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDataset`.
///
/// <note>
/// <p>This operation applies only to Amazon Rekognition Custom Labels.</p>
/// </note>
/// <p>Creates a new Amazon Rekognition Custom Labels dataset. You can create a dataset by using an Amazon Sagemaker format manifest file or by copying an existing Amazon Rekognition Custom Labels dataset.</p>
/// <p>To create a training dataset for a project, specify <code>TRAIN</code> for the value of <code>DatasetType</code>. To create the test dataset for a project, specify <code>TEST</code> for the value of <code>DatasetType</code>. </p>
/// <p>The response from <code>CreateDataset</code> is the Amazon Resource Name (ARN) for the dataset. Creating a dataset takes a while to complete. Use <code>DescribeDataset</code> to check the current status. The dataset created successfully if the value of <code>Status</code> is <code>CREATE_COMPLETE</code>. </p>
/// <p>To check if any non-terminal errors occurred, call <code>ListDatasetEntries</code> and check for the presence of <code>errors</code> lists in the JSON Lines.</p>
/// <p>Dataset creation fails if a terminal error occurs (<code>Status</code> = <code>CREATE_FAILED</code>). Currently, you can't access the terminal error information. </p>
/// <p>For more information, see Creating dataset in the <i>Amazon Rekognition Custom Labels Developer Guide</i>.</p>
/// <p>This operation requires permissions to perform the <code>rekognition:CreateDataset</code> action. If you want to copy an existing dataset, you also require permission to perform the <code>rekognition:ListDatasetEntries</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDatasetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dataset::builders::CreateDatasetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_dataset::CreateDatasetOutput,
        crate::operation::create_dataset::CreateDatasetError,
    > for CreateDatasetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_dataset::CreateDatasetOutput,
            crate::operation::create_dataset::CreateDatasetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDatasetFluentBuilder {
    /// Creates a new `CreateDataset`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDataset as a reference.
    pub fn as_input(&self) -> &crate::operation::create_dataset::builders::CreateDatasetInputBuilder {
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
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dataset::CreateDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_dataset::CreateDataset::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_dataset::CreateDataset::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_dataset::CreateDatasetOutput,
        crate::operation::create_dataset::CreateDatasetError,
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
    /// <p> The source files for the dataset. You can specify the ARN of an existing dataset or specify the Amazon S3 bucket location of an Amazon Sagemaker format manifest file. If you don't specify <code>datasetSource</code>, an empty dataset is created. To add labeled images to the dataset, You can use the console or call <code>UpdateDatasetEntries</code>. </p>
    pub fn dataset_source(mut self, input: crate::types::DatasetSource) -> Self {
        self.inner = self.inner.dataset_source(input);
        self
    }
    /// <p> The source files for the dataset. You can specify the ARN of an existing dataset or specify the Amazon S3 bucket location of an Amazon Sagemaker format manifest file. If you don't specify <code>datasetSource</code>, an empty dataset is created. To add labeled images to the dataset, You can use the console or call <code>UpdateDatasetEntries</code>. </p>
    pub fn set_dataset_source(mut self, input: ::std::option::Option<crate::types::DatasetSource>) -> Self {
        self.inner = self.inner.set_dataset_source(input);
        self
    }
    /// <p> The source files for the dataset. You can specify the ARN of an existing dataset or specify the Amazon S3 bucket location of an Amazon Sagemaker format manifest file. If you don't specify <code>datasetSource</code>, an empty dataset is created. To add labeled images to the dataset, You can use the console or call <code>UpdateDatasetEntries</code>. </p>
    pub fn get_dataset_source(&self) -> &::std::option::Option<crate::types::DatasetSource> {
        self.inner.get_dataset_source()
    }
    /// <p> The type of the dataset. Specify <code>TRAIN</code> to create a training dataset. Specify <code>TEST</code> to create a test dataset. </p>
    pub fn dataset_type(mut self, input: crate::types::DatasetType) -> Self {
        self.inner = self.inner.dataset_type(input);
        self
    }
    /// <p> The type of the dataset. Specify <code>TRAIN</code> to create a training dataset. Specify <code>TEST</code> to create a test dataset. </p>
    pub fn set_dataset_type(mut self, input: ::std::option::Option<crate::types::DatasetType>) -> Self {
        self.inner = self.inner.set_dataset_type(input);
        self
    }
    /// <p> The type of the dataset. Specify <code>TRAIN</code> to create a training dataset. Specify <code>TEST</code> to create a test dataset. </p>
    pub fn get_dataset_type(&self) -> &::std::option::Option<crate::types::DatasetType> {
        self.inner.get_dataset_type()
    }
    /// <p> The ARN of the Amazon Rekognition Custom Labels project to which you want to asssign the dataset. </p>
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_arn(input.into());
        self
    }
    /// <p> The ARN of the Amazon Rekognition Custom Labels project to which you want to asssign the dataset. </p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_arn(input);
        self
    }
    /// <p> The ARN of the Amazon Rekognition Custom Labels project to which you want to asssign the dataset. </p>
    pub fn get_project_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_arn()
    }
}
