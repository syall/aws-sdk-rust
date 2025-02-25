// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_import_labels_task_run::_start_import_labels_task_run_output::StartImportLabelsTaskRunOutputBuilder;

pub use crate::operation::start_import_labels_task_run::_start_import_labels_task_run_input::StartImportLabelsTaskRunInputBuilder;

impl StartImportLabelsTaskRunInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_import_labels_task_run();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartImportLabelsTaskRun`.
///
/// <p>Enables you to provide additional labels (examples of truth) to be used to teach the machine learning transform and improve its quality. This API operation is generally used as part of the active learning workflow that starts with the <code>StartMLLabelingSetGenerationTaskRun</code> call and that ultimately results in improving the quality of your machine learning transform. </p>
/// <p>After the <code>StartMLLabelingSetGenerationTaskRun</code> finishes, Glue machine learning will have generated a series of questions for humans to answer. (Answering these questions is often called 'labeling' in the machine learning workflows). In the case of the <code>FindMatches</code> transform, these questions are of the form, “What is the correct way to group these rows together into groups composed entirely of matching records?” After the labeling process is finished, users upload their answers/labels with a call to <code>StartImportLabelsTaskRun</code>. After <code>StartImportLabelsTaskRun</code> finishes, all future runs of the machine learning transform use the new and improved labels and perform a higher-quality transformation.</p>
/// <p>By default, <code>StartMLLabelingSetGenerationTaskRun</code> continually learns from and combines all labels that you upload unless you set <code>Replace</code> to true. If you set <code>Replace</code> to true, <code>StartImportLabelsTaskRun</code> deletes and forgets all previously uploaded labels and learns only from the exact set that you upload. Replacing labels can be helpful if you realize that you previously uploaded incorrect labels, and you believe that they are having a negative effect on your transform quality.</p>
/// <p>You can check on the status of your task run by calling the <code>GetMLTaskRun</code> operation. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartImportLabelsTaskRunFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_import_labels_task_run::builders::StartImportLabelsTaskRunInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunOutput,
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunError,
    > for StartImportLabelsTaskRunFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunOutput,
            crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartImportLabelsTaskRunFluentBuilder {
    /// Creates a new `StartImportLabelsTaskRun`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartImportLabelsTaskRun as a reference.
    pub fn as_input(&self) -> &crate::operation::start_import_labels_task_run::builders::StartImportLabelsTaskRunInputBuilder {
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
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_import_labels_task_run::StartImportLabelsTaskRun::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRun::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunOutput,
        crate::operation::start_import_labels_task_run::StartImportLabelsTaskRunError,
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
    /// <p>The unique identifier of the machine learning transform.</p>
    pub fn transform_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transform_id(input.into());
        self
    }
    /// <p>The unique identifier of the machine learning transform.</p>
    pub fn set_transform_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transform_id(input);
        self
    }
    /// <p>The unique identifier of the machine learning transform.</p>
    pub fn get_transform_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transform_id()
    }
    /// <p>The Amazon Simple Storage Service (Amazon S3) path from where you import the labels.</p>
    pub fn input_s3_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_s3_path(input.into());
        self
    }
    /// <p>The Amazon Simple Storage Service (Amazon S3) path from where you import the labels.</p>
    pub fn set_input_s3_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_s3_path(input);
        self
    }
    /// <p>The Amazon Simple Storage Service (Amazon S3) path from where you import the labels.</p>
    pub fn get_input_s3_path(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_input_s3_path()
    }
    /// <p>Indicates whether to overwrite your existing labels.</p>
    pub fn replace_all_labels(mut self, input: bool) -> Self {
        self.inner = self.inner.replace_all_labels(input);
        self
    }
    /// <p>Indicates whether to overwrite your existing labels.</p>
    pub fn set_replace_all_labels(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_replace_all_labels(input);
        self
    }
    /// <p>Indicates whether to overwrite your existing labels.</p>
    pub fn get_replace_all_labels(&self) -> &::std::option::Option<bool> {
        self.inner.get_replace_all_labels()
    }
}
