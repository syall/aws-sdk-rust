// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePresignedDomainUrl`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl Into<String>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::set_domain_id):<br>required: **true**<br><p>The domain ID.</p><br>
    ///   - [`user_profile_name(impl Into<String>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::user_profile_name) / [`set_user_profile_name(Option<String>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::set_user_profile_name):<br>required: **true**<br><p>The name of the UserProfile to sign-in as.</p><br>
    ///   - [`session_expiration_duration_in_seconds(i32)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::session_expiration_duration_in_seconds) / [`set_session_expiration_duration_in_seconds(Option<i32>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::set_session_expiration_duration_in_seconds):<br>required: **false**<br><p>The session expiration duration in seconds. This value defaults to 43200.</p><br>
    ///   - [`expires_in_seconds(i32)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::expires_in_seconds) / [`set_expires_in_seconds(Option<i32>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::set_expires_in_seconds):<br>required: **false**<br><p>The number of seconds until the pre-signed URL expires. This value defaults to 300.</p><br>
    ///   - [`space_name(impl Into<String>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::space_name) / [`set_space_name(Option<String>)`](crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::set_space_name):<br>required: **false**<br><p>The name of the space.</p><br>
    /// - On success, responds with [`CreatePresignedDomainUrlOutput`](crate::operation::create_presigned_domain_url::CreatePresignedDomainUrlOutput) with field(s):
    ///   - [`authorized_url(Option<String>)`](crate::operation::create_presigned_domain_url::CreatePresignedDomainUrlOutput::authorized_url): <p>The presigned URL.</p>
    /// - On failure, responds with [`SdkError<CreatePresignedDomainUrlError>`](crate::operation::create_presigned_domain_url::CreatePresignedDomainUrlError)
    pub fn create_presigned_domain_url(&self) -> crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder {
        crate::operation::create_presigned_domain_url::builders::CreatePresignedDomainUrlFluentBuilder::new(self.handle.clone())
    }
}
