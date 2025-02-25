// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCacheSecurityGroup`](crate::operation::delete_cache_security_group::builders::DeleteCacheSecurityGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cache_security_group_name(impl Into<String>)`](crate::operation::delete_cache_security_group::builders::DeleteCacheSecurityGroupFluentBuilder::cache_security_group_name) / [`set_cache_security_group_name(Option<String>)`](crate::operation::delete_cache_security_group::builders::DeleteCacheSecurityGroupFluentBuilder::set_cache_security_group_name):<br>required: **true**<br><p>The name of the cache security group to delete.</p> <note>   <p>You cannot delete the default security group.</p>  </note><br>
    /// - On success, responds with [`DeleteCacheSecurityGroupOutput`](crate::operation::delete_cache_security_group::DeleteCacheSecurityGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteCacheSecurityGroupError>`](crate::operation::delete_cache_security_group::DeleteCacheSecurityGroupError)
    pub fn delete_cache_security_group(&self) -> crate::operation::delete_cache_security_group::builders::DeleteCacheSecurityGroupFluentBuilder {
        crate::operation::delete_cache_security_group::builders::DeleteCacheSecurityGroupFluentBuilder::new(self.handle.clone())
    }
}
