// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteGroupPolicy`](crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_name(impl Into<String>)`](crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder::set_group_name):<br>required: **true**<br><p>The name (friendly name, not ARN) identifying the group that the policy is embedded in.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p><br>
    ///   - [`policy_name(impl Into<String>)`](crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder::set_policy_name):<br>required: **true**<br><p>The name identifying the policy document to delete.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p><br>
    /// - On success, responds with [`DeleteGroupPolicyOutput`](crate::operation::delete_group_policy::DeleteGroupPolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteGroupPolicyError>`](crate::operation::delete_group_policy::DeleteGroupPolicyError)
    pub fn delete_group_policy(&self) -> crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder {
        crate::operation::delete_group_policy::builders::DeleteGroupPolicyFluentBuilder::new(self.handle.clone())
    }
}
