// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketMetricsConfiguration`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the bucket for which the metrics configuration is set.</p><br>
    ///   - [`id(impl Into<String>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::set_id):<br>required: **true**<br><p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p><br>
    ///   - [`metrics_configuration(MetricsConfiguration)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::metrics_configuration) / [`set_metrics_configuration(Option<MetricsConfiguration>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::set_metrics_configuration):<br>required: **true**<br><p>Specifies the metrics configuration.</p><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`PutBucketMetricsConfigurationOutput`](crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutBucketMetricsConfigurationError>`](crate::operation::put_bucket_metrics_configuration::PutBucketMetricsConfigurationError)
    pub fn put_bucket_metrics_configuration(
        &self,
    ) -> crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder {
        crate::operation::put_bucket_metrics_configuration::builders::PutBucketMetricsConfigurationFluentBuilder::new(self.handle.clone())
    }
}
