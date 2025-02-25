// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RevokeSecurityGroupIngressOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    pub r#return: ::std::option::Option<bool>,
    /// <p>The inbound rules that were unknown to the service. In some cases, <code>unknownIpPermissionSet</code> might be in a different format from the request parameter. </p>
    pub unknown_ip_permissions: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    _request_id: Option<String>,
}
impl RevokeSecurityGroupIngressOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    pub fn r#return(&self) -> ::std::option::Option<bool> {
        self.r#return
    }
    /// <p>The inbound rules that were unknown to the service. In some cases, <code>unknownIpPermissionSet</code> might be in a different format from the request parameter. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.unknown_ip_permissions.is_none()`.
    pub fn unknown_ip_permissions(&self) -> &[crate::types::IpPermission] {
        self.unknown_ip_permissions.as_deref().unwrap_or_default()
    }
}
impl ::aws_http::request_id::RequestId for RevokeSecurityGroupIngressOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RevokeSecurityGroupIngressOutput {
    /// Creates a new builder-style object to manufacture [`RevokeSecurityGroupIngressOutput`](crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressOutput).
    pub fn builder() -> crate::operation::revoke_security_group_ingress::builders::RevokeSecurityGroupIngressOutputBuilder {
        crate::operation::revoke_security_group_ingress::builders::RevokeSecurityGroupIngressOutputBuilder::default()
    }
}

/// A builder for [`RevokeSecurityGroupIngressOutput`](crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RevokeSecurityGroupIngressOutputBuilder {
    pub(crate) r#return: ::std::option::Option<bool>,
    pub(crate) unknown_ip_permissions: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    _request_id: Option<String>,
}
impl RevokeSecurityGroupIngressOutputBuilder {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    pub fn r#return(mut self, input: bool) -> Self {
        self.r#return = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    pub fn set_return(mut self, input: ::std::option::Option<bool>) -> Self {
        self.r#return = input;
        self
    }
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    pub fn get_return(&self) -> &::std::option::Option<bool> {
        &self.r#return
    }
    /// Appends an item to `unknown_ip_permissions`.
    ///
    /// To override the contents of this collection use [`set_unknown_ip_permissions`](Self::set_unknown_ip_permissions).
    ///
    /// <p>The inbound rules that were unknown to the service. In some cases, <code>unknownIpPermissionSet</code> might be in a different format from the request parameter. </p>
    pub fn unknown_ip_permissions(mut self, input: crate::types::IpPermission) -> Self {
        let mut v = self.unknown_ip_permissions.unwrap_or_default();
        v.push(input);
        self.unknown_ip_permissions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The inbound rules that were unknown to the service. In some cases, <code>unknownIpPermissionSet</code> might be in a different format from the request parameter. </p>
    pub fn set_unknown_ip_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>) -> Self {
        self.unknown_ip_permissions = input;
        self
    }
    /// <p>The inbound rules that were unknown to the service. In some cases, <code>unknownIpPermissionSet</code> might be in a different format from the request parameter. </p>
    pub fn get_unknown_ip_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IpPermission>> {
        &self.unknown_ip_permissions
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RevokeSecurityGroupIngressOutput`](crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressOutput).
    pub fn build(self) -> crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressOutput {
        crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressOutput {
            r#return: self.r#return,
            unknown_ip_permissions: self.unknown_ip_permissions,
            _request_id: self._request_id,
        }
    }
}
