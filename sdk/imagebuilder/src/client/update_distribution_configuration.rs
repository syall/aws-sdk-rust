// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDistributionConfiguration`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`distribution_configuration_arn(impl Into<String>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::distribution_configuration_arn) / [`set_distribution_configuration_arn(Option<String>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::set_distribution_configuration_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the distribution configuration that you want to update.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::set_description):<br>required: **false**<br><p>The description of the distribution configuration.</p><br>
    ///   - [`distributions(Distribution)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::distributions) / [`set_distributions(Option<Vec::<Distribution>>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::set_distributions):<br>required: **true**<br><p>The distributions of the distribution configuration.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::set_client_token):<br>required: **true**<br><p>The idempotency token of the distribution configuration.</p><br>
    /// - On success, responds with [`UpdateDistributionConfigurationOutput`](crate::operation::update_distribution_configuration::UpdateDistributionConfigurationOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::update_distribution_configuration::UpdateDistributionConfigurationOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`client_token(Option<String>)`](crate::operation::update_distribution_configuration::UpdateDistributionConfigurationOutput::client_token): <p>The idempotency token used to make this request idempotent.</p>
    ///   - [`distribution_configuration_arn(Option<String>)`](crate::operation::update_distribution_configuration::UpdateDistributionConfigurationOutput::distribution_configuration_arn): <p>The Amazon Resource Name (ARN) of the distribution configuration that was updated by this request.</p>
    /// - On failure, responds with [`SdkError<UpdateDistributionConfigurationError>`](crate::operation::update_distribution_configuration::UpdateDistributionConfigurationError)
    pub fn update_distribution_configuration(
        &self,
    ) -> crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder {
        crate::operation::update_distribution_configuration::builders::UpdateDistributionConfigurationFluentBuilder::new(self.handle.clone())
    }
}
