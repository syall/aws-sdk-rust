// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_launch::_update_launch_output::UpdateLaunchOutputBuilder;

pub use crate::operation::update_launch::_update_launch_input::UpdateLaunchInputBuilder;

impl UpdateLaunchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_launch::UpdateLaunchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_launch::UpdateLaunchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_launch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLaunch`.
///
/// <p>Updates a launch of a given feature. </p>
/// <p>Don't use this operation to update the tags of an existing launch. Instead, use <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_TagResource.html">TagResource</a>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLaunchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_launch::builders::UpdateLaunchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_launch::UpdateLaunchOutput,
        crate::operation::update_launch::UpdateLaunchError,
    > for UpdateLaunchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_launch::UpdateLaunchOutput,
            crate::operation::update_launch::UpdateLaunchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateLaunchFluentBuilder {
    /// Creates a new `UpdateLaunch`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLaunch as a reference.
    pub fn as_input(&self) -> &crate::operation::update_launch::builders::UpdateLaunchInputBuilder {
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
        crate::operation::update_launch::UpdateLaunchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_launch::UpdateLaunchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_launch::UpdateLaunch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_launch::UpdateLaunch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_launch::UpdateLaunchOutput,
        crate::operation::update_launch::UpdateLaunchError,
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
    /// <p>The name or ARN of the project that contains the launch that you want to update.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project(input.into());
        self
    }
    /// <p>The name or ARN of the project that contains the launch that you want to update.</p>
    pub fn set_project(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project(input);
        self
    }
    /// <p>The name or ARN of the project that contains the launch that you want to update.</p>
    pub fn get_project(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project()
    }
    /// <p>The name of the launch that is to be updated.</p>
    pub fn launch(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.launch(input.into());
        self
    }
    /// <p>The name of the launch that is to be updated.</p>
    pub fn set_launch(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_launch(input);
        self
    }
    /// <p>The name of the launch that is to be updated.</p>
    pub fn get_launch(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_launch()
    }
    /// <p>An optional description for the launch.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description for the launch.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description for the launch.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>An array of structures that contains the feature and variations that are to be used for the launch.</p>
    pub fn groups(mut self, input: crate::types::LaunchGroupConfig) -> Self {
        self.inner = self.inner.groups(input);
        self
    }
    /// <p>An array of structures that contains the feature and variations that are to be used for the launch.</p>
    pub fn set_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LaunchGroupConfig>>) -> Self {
        self.inner = self.inner.set_groups(input);
        self
    }
    /// <p>An array of structures that contains the feature and variations that are to be used for the launch.</p>
    pub fn get_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LaunchGroupConfig>> {
        self.inner.get_groups()
    }
    /// Appends an item to `metricMonitors`.
    ///
    /// To override the contents of this collection use [`set_metric_monitors`](Self::set_metric_monitors).
    ///
    /// <p>An array of structures that define the metrics that will be used to monitor the launch performance.</p>
    pub fn metric_monitors(mut self, input: crate::types::MetricMonitorConfig) -> Self {
        self.inner = self.inner.metric_monitors(input);
        self
    }
    /// <p>An array of structures that define the metrics that will be used to monitor the launch performance.</p>
    pub fn set_metric_monitors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetricMonitorConfig>>) -> Self {
        self.inner = self.inner.set_metric_monitors(input);
        self
    }
    /// <p>An array of structures that define the metrics that will be used to monitor the launch performance.</p>
    pub fn get_metric_monitors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetricMonitorConfig>> {
        self.inner.get_metric_monitors()
    }
    /// <p>When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the launch name as the <code>randomizationSalt</code>.</p>
    pub fn randomization_salt(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.randomization_salt(input.into());
        self
    }
    /// <p>When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the launch name as the <code>randomizationSalt</code>.</p>
    pub fn set_randomization_salt(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_randomization_salt(input);
        self
    }
    /// <p>When Evidently assigns a particular user session to a launch, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the launch name as the <code>randomizationSalt</code>.</p>
    pub fn get_randomization_salt(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_randomization_salt()
    }
    /// <p>An array of structures that define the traffic allocation percentages among the feature variations during each step of the launch.</p>
    pub fn scheduled_splits_config(mut self, input: crate::types::ScheduledSplitsLaunchConfig) -> Self {
        self.inner = self.inner.scheduled_splits_config(input);
        self
    }
    /// <p>An array of structures that define the traffic allocation percentages among the feature variations during each step of the launch.</p>
    pub fn set_scheduled_splits_config(mut self, input: ::std::option::Option<crate::types::ScheduledSplitsLaunchConfig>) -> Self {
        self.inner = self.inner.set_scheduled_splits_config(input);
        self
    }
    /// <p>An array of structures that define the traffic allocation percentages among the feature variations during each step of the launch.</p>
    pub fn get_scheduled_splits_config(&self) -> &::std::option::Option<crate::types::ScheduledSplitsLaunchConfig> {
        self.inner.get_scheduled_splits_config()
    }
}
