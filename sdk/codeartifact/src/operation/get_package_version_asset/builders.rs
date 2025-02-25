// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_package_version_asset::_get_package_version_asset_output::GetPackageVersionAssetOutputBuilder;

pub use crate::operation::get_package_version_asset::_get_package_version_asset_input::GetPackageVersionAssetInputBuilder;

impl GetPackageVersionAssetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_package_version_asset::GetPackageVersionAssetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_package_version_asset::GetPackageVersionAssetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_package_version_asset();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPackageVersionAsset`.
///
/// <p> Returns an asset (or file) that is in a package. For example, for a Maven package version, use <code>GetPackageVersionAsset</code> to download a <code>JAR</code> file, a <code>POM</code> file, or any other assets in the package version. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPackageVersionAssetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_package_version_asset::builders::GetPackageVersionAssetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_package_version_asset::GetPackageVersionAssetOutput,
        crate::operation::get_package_version_asset::GetPackageVersionAssetError,
    > for GetPackageVersionAssetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_package_version_asset::GetPackageVersionAssetOutput,
            crate::operation::get_package_version_asset::GetPackageVersionAssetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPackageVersionAssetFluentBuilder {
    /// Creates a new `GetPackageVersionAsset`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPackageVersionAsset as a reference.
    pub fn as_input(&self) -> &crate::operation::get_package_version_asset::builders::GetPackageVersionAssetInputBuilder {
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
        crate::operation::get_package_version_asset::GetPackageVersionAssetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_package_version_asset::GetPackageVersionAssetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_package_version_asset::GetPackageVersionAsset::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_package_version_asset::GetPackageVersionAsset::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_package_version_asset::GetPackageVersionAssetOutput,
        crate::operation::get_package_version_asset::GetPackageVersionAssetError,
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
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p> The name of the domain that contains the repository that contains the package version with the requested asset. </p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain()
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn domain_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_owner(input.into());
        self
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn set_domain_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_owner(input);
        self
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn get_domain_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_owner()
    }
    /// <p> The repository that contains the package version with the requested asset. </p>
    pub fn repository(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository(input.into());
        self
    }
    /// <p> The repository that contains the package version with the requested asset. </p>
    pub fn set_repository(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository(input);
        self
    }
    /// <p> The repository that contains the package version with the requested asset. </p>
    pub fn get_repository(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository()
    }
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    pub fn format(mut self, input: crate::types::PackageFormat) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::PackageFormat>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p> A format that specifies the type of the package version with the requested asset file. </p>
    pub fn get_format(&self) -> &::std::option::Option<crate::types::PackageFormat> {
        self.inner.get_format()
    }
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The namespace of the package version with the requested asset file. The package version component that specifies its namespace depends on its type. For example:</p>
    /// <ul>
    /// <li> <p> The namespace of a Maven package version is its <code>groupId</code>. </p> </li>
    /// <li> <p> The namespace of an npm package version is its <code>scope</code>. </p> </li>
    /// <li> <p> Python and NuGet package versions do not contain a corresponding component, package versions of those formats do not have a namespace. </p> </li>
    /// <li> <p> The namespace of a generic package is its <code>namespace</code>. </p> </li>
    /// </ul>
    pub fn get_namespace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace()
    }
    /// <p> The name of the package that contains the requested asset. </p>
    pub fn package(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package(input.into());
        self
    }
    /// <p> The name of the package that contains the requested asset. </p>
    pub fn set_package(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package(input);
        self
    }
    /// <p> The name of the package that contains the requested asset. </p>
    pub fn get_package(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package()
    }
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    pub fn package_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_version(input.into());
        self
    }
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    pub fn set_package_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_version(input);
        self
    }
    /// <p> A string that contains the package version (for example, <code>3.5.2</code>). </p>
    pub fn get_package_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package_version()
    }
    /// <p> The name of the requested asset. </p>
    pub fn asset(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset(input.into());
        self
    }
    /// <p> The name of the requested asset. </p>
    pub fn set_asset(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asset(input);
        self
    }
    /// <p> The name of the requested asset. </p>
    pub fn get_asset(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asset()
    }
    /// <p> The name of the package version revision that contains the requested asset. </p>
    pub fn package_version_revision(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_version_revision(input.into());
        self
    }
    /// <p> The name of the package version revision that contains the requested asset. </p>
    pub fn set_package_version_revision(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_version_revision(input);
        self
    }
    /// <p> The name of the package version revision that contains the requested asset. </p>
    pub fn get_package_version_revision(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package_version_revision()
    }
}
