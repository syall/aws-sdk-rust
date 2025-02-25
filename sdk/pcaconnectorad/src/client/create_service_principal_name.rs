// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateServicePrincipalName`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_registration_arn(impl Into<String>)`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::directory_registration_arn) / [`set_directory_registration_arn(Option<String>)`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::set_directory_registration_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) that was returned when you called <a href="https://docs.aws.amazon.com/pca-connector-ad/latest/APIReference/API_CreateDirectoryRegistration.html">CreateDirectoryRegistration</a>.</p><br>
    ///   - [`connector_arn(impl Into<String>)`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::connector_arn) / [`set_connector_arn(Option<String>)`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::set_connector_arn):<br>required: **true**<br><p> The Amazon Resource Name (ARN) that was returned when you called <a href="https://docs.aws.amazon.com/pca-connector-ad/latest/APIReference/API_CreateConnector.html">CreateConnector</a>.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::set_client_token):<br>required: **false**<br><p>Idempotency token.</p><br>
    /// - On success, responds with [`CreateServicePrincipalNameOutput`](crate::operation::create_service_principal_name::CreateServicePrincipalNameOutput)
    /// - On failure, responds with [`SdkError<CreateServicePrincipalNameError>`](crate::operation::create_service_principal_name::CreateServicePrincipalNameError)
    pub fn create_service_principal_name(
        &self,
    ) -> crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder {
        crate::operation::create_service_principal_name::builders::CreateServicePrincipalNameFluentBuilder::new(self.handle.clone())
    }
}
