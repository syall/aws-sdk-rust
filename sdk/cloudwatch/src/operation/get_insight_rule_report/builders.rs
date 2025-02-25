// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_insight_rule_report::_get_insight_rule_report_output::GetInsightRuleReportOutputBuilder;

pub use crate::operation::get_insight_rule_report::_get_insight_rule_report_input::GetInsightRuleReportInputBuilder;

impl GetInsightRuleReportInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_insight_rule_report::GetInsightRuleReportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_insight_rule_report::GetInsightRuleReportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_insight_rule_report();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetInsightRuleReport`.
///
/// <p>This operation returns the time series data collected by a Contributor Insights rule. The data includes the identity and number of contributors to the log group.</p>
/// <p>You can also optionally return one or more statistics about each data point in the time series. These statistics can include the following:</p>
/// <ul>
/// <li> <p> <code>UniqueContributors</code> -- the number of unique contributors for each data point.</p> </li>
/// <li> <p> <code>MaxContributorValue</code> -- the value of the top contributor for each data point. The identity of the contributor might change for each data point in the graph.</p> <p>If this rule aggregates by COUNT, the top contributor for each data point is the contributor with the most occurrences in that period. If the rule aggregates by SUM, the top contributor is the contributor with the highest sum in the log field specified by the rule's <code>Value</code>, during that period.</p> </li>
/// <li> <p> <code>SampleCount</code> -- the number of data points matched by the rule.</p> </li>
/// <li> <p> <code>Sum</code> -- the sum of the values from all contributors during the time period represented by that data point.</p> </li>
/// <li> <p> <code>Minimum</code> -- the minimum value from a single observation during the time period represented by that data point.</p> </li>
/// <li> <p> <code>Maximum</code> -- the maximum value from a single observation during the time period represented by that data point.</p> </li>
/// <li> <p> <code>Average</code> -- the average value from all contributors during the time period represented by that data point.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetInsightRuleReportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_insight_rule_report::builders::GetInsightRuleReportInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_insight_rule_report::GetInsightRuleReportOutput,
        crate::operation::get_insight_rule_report::GetInsightRuleReportError,
    > for GetInsightRuleReportFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_insight_rule_report::GetInsightRuleReportOutput,
            crate::operation::get_insight_rule_report::GetInsightRuleReportError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetInsightRuleReportFluentBuilder {
    /// Creates a new `GetInsightRuleReport`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetInsightRuleReport as a reference.
    pub fn as_input(&self) -> &crate::operation::get_insight_rule_report::builders::GetInsightRuleReportInputBuilder {
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
        crate::operation::get_insight_rule_report::GetInsightRuleReportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_insight_rule_report::GetInsightRuleReportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_insight_rule_report::GetInsightRuleReport::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_insight_rule_report::GetInsightRuleReport::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_insight_rule_report::GetInsightRuleReportOutput,
        crate::operation::get_insight_rule_report::GetInsightRuleReportError,
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
    /// <p>The name of the rule that you want to see data from.</p>
    pub fn rule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_name(input.into());
        self
    }
    /// <p>The name of the rule that you want to see data from.</p>
    pub fn set_rule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_name(input);
        self
    }
    /// <p>The name of the rule that you want to see data from.</p>
    pub fn get_rule_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_name()
    }
    /// <p>The start time of the data to use in the report. When used in a raw HTTP Query API, it is formatted as <code>yyyy-MM-dd'T'HH:mm:ss</code>. For example, <code>2019-07-01T23:59:59</code>.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start time of the data to use in the report. When used in a raw HTTP Query API, it is formatted as <code>yyyy-MM-dd'T'HH:mm:ss</code>. For example, <code>2019-07-01T23:59:59</code>.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The start time of the data to use in the report. When used in a raw HTTP Query API, it is formatted as <code>yyyy-MM-dd'T'HH:mm:ss</code>. For example, <code>2019-07-01T23:59:59</code>.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The end time of the data to use in the report. When used in a raw HTTP Query API, it is formatted as <code>yyyy-MM-dd'T'HH:mm:ss</code>. For example, <code>2019-07-01T23:59:59</code>.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end time of the data to use in the report. When used in a raw HTTP Query API, it is formatted as <code>yyyy-MM-dd'T'HH:mm:ss</code>. For example, <code>2019-07-01T23:59:59</code>.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The end time of the data to use in the report. When used in a raw HTTP Query API, it is formatted as <code>yyyy-MM-dd'T'HH:mm:ss</code>. For example, <code>2019-07-01T23:59:59</code>.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>The period, in seconds, to use for the statistics in the <code>InsightRuleMetricDatapoint</code> results.</p>
    pub fn period(mut self, input: i32) -> Self {
        self.inner = self.inner.period(input);
        self
    }
    /// <p>The period, in seconds, to use for the statistics in the <code>InsightRuleMetricDatapoint</code> results.</p>
    pub fn set_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_period(input);
        self
    }
    /// <p>The period, in seconds, to use for the statistics in the <code>InsightRuleMetricDatapoint</code> results.</p>
    pub fn get_period(&self) -> &::std::option::Option<i32> {
        self.inner.get_period()
    }
    /// <p>The maximum number of contributors to include in the report. The range is 1 to 100. If you omit this, the default of 10 is used.</p>
    pub fn max_contributor_count(mut self, input: i32) -> Self {
        self.inner = self.inner.max_contributor_count(input);
        self
    }
    /// <p>The maximum number of contributors to include in the report. The range is 1 to 100. If you omit this, the default of 10 is used.</p>
    pub fn set_max_contributor_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_contributor_count(input);
        self
    }
    /// <p>The maximum number of contributors to include in the report. The range is 1 to 100. If you omit this, the default of 10 is used.</p>
    pub fn get_max_contributor_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_contributor_count()
    }
    /// Appends an item to `Metrics`.
    ///
    /// To override the contents of this collection use [`set_metrics`](Self::set_metrics).
    ///
    /// <p>Specifies which metrics to use for aggregation of contributor values for the report. You can specify one or more of the following metrics:</p>
    /// <ul>
    /// <li> <p> <code>UniqueContributors</code> -- the number of unique contributors for each data point.</p> </li>
    /// <li> <p> <code>MaxContributorValue</code> -- the value of the top contributor for each data point. The identity of the contributor might change for each data point in the graph.</p> <p>If this rule aggregates by COUNT, the top contributor for each data point is the contributor with the most occurrences in that period. If the rule aggregates by SUM, the top contributor is the contributor with the highest sum in the log field specified by the rule's <code>Value</code>, during that period.</p> </li>
    /// <li> <p> <code>SampleCount</code> -- the number of data points matched by the rule.</p> </li>
    /// <li> <p> <code>Sum</code> -- the sum of the values from all contributors during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Minimum</code> -- the minimum value from a single observation during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Maximum</code> -- the maximum value from a single observation during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Average</code> -- the average value from all contributors during the time period represented by that data point.</p> </li>
    /// </ul>
    pub fn metrics(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metrics(input.into());
        self
    }
    /// <p>Specifies which metrics to use for aggregation of contributor values for the report. You can specify one or more of the following metrics:</p>
    /// <ul>
    /// <li> <p> <code>UniqueContributors</code> -- the number of unique contributors for each data point.</p> </li>
    /// <li> <p> <code>MaxContributorValue</code> -- the value of the top contributor for each data point. The identity of the contributor might change for each data point in the graph.</p> <p>If this rule aggregates by COUNT, the top contributor for each data point is the contributor with the most occurrences in that period. If the rule aggregates by SUM, the top contributor is the contributor with the highest sum in the log field specified by the rule's <code>Value</code>, during that period.</p> </li>
    /// <li> <p> <code>SampleCount</code> -- the number of data points matched by the rule.</p> </li>
    /// <li> <p> <code>Sum</code> -- the sum of the values from all contributors during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Minimum</code> -- the minimum value from a single observation during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Maximum</code> -- the maximum value from a single observation during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Average</code> -- the average value from all contributors during the time period represented by that data point.</p> </li>
    /// </ul>
    pub fn set_metrics(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_metrics(input);
        self
    }
    /// <p>Specifies which metrics to use for aggregation of contributor values for the report. You can specify one or more of the following metrics:</p>
    /// <ul>
    /// <li> <p> <code>UniqueContributors</code> -- the number of unique contributors for each data point.</p> </li>
    /// <li> <p> <code>MaxContributorValue</code> -- the value of the top contributor for each data point. The identity of the contributor might change for each data point in the graph.</p> <p>If this rule aggregates by COUNT, the top contributor for each data point is the contributor with the most occurrences in that period. If the rule aggregates by SUM, the top contributor is the contributor with the highest sum in the log field specified by the rule's <code>Value</code>, during that period.</p> </li>
    /// <li> <p> <code>SampleCount</code> -- the number of data points matched by the rule.</p> </li>
    /// <li> <p> <code>Sum</code> -- the sum of the values from all contributors during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Minimum</code> -- the minimum value from a single observation during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Maximum</code> -- the maximum value from a single observation during the time period represented by that data point.</p> </li>
    /// <li> <p> <code>Average</code> -- the average value from all contributors during the time period represented by that data point.</p> </li>
    /// </ul>
    pub fn get_metrics(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_metrics()
    }
    /// <p>Determines what statistic to use to rank the contributors. Valid values are <code>Sum</code> and <code>Maximum</code>.</p>
    pub fn order_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.order_by(input.into());
        self
    }
    /// <p>Determines what statistic to use to rank the contributors. Valid values are <code>Sum</code> and <code>Maximum</code>.</p>
    pub fn set_order_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_order_by(input);
        self
    }
    /// <p>Determines what statistic to use to rank the contributors. Valid values are <code>Sum</code> and <code>Maximum</code>.</p>
    pub fn get_order_by(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_order_by()
    }
}
