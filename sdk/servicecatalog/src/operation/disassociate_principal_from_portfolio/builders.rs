// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_principal_from_portfolio::_disassociate_principal_from_portfolio_output::DisassociatePrincipalFromPortfolioOutputBuilder;

pub use crate::operation::disassociate_principal_from_portfolio::_disassociate_principal_from_portfolio_input::DisassociatePrincipalFromPortfolioInputBuilder;

impl DisassociatePrincipalFromPortfolioInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_principal_from_portfolio();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociatePrincipalFromPortfolio`.
///
/// <p>Disassociates a previously associated principal ARN from a specified portfolio.</p>
/// <p>The <code>PrincipalType</code> and <code>PrincipalARN</code> must match the <code>AssociatePrincipalWithPortfolio</code> call request details. For example, to disassociate an association created with a <code>PrincipalARN</code> of <code>PrincipalType</code> IAM you must use the <code>PrincipalType</code> IAM when calling <code>DisassociatePrincipalFromPortfolio</code>. </p>
/// <p>For portfolios that have been shared with principal name sharing enabled: after disassociating a principal, share recipient accounts will no longer be able to provision products in this portfolio using a role matching the name of the associated principal. </p>
/// <p>For more information, review <a href="https://docs.aws.amazon.com/cli/latest/reference/servicecatalog/associate-principal-with-portfolio.html#options">associate-principal-with-portfolio</a> in the Amazon Web Services CLI Command Reference. </p> <note>
/// <p>If you disassociate a principal from a portfolio, with PrincipalType as <code>IAM</code>, the same principal will still have access to the portfolio if it matches one of the associated principals of type <code>IAM_PATTERN</code>. To fully remove access for a principal, verify all the associated Principals of type <code>IAM_PATTERN</code>, and then ensure you disassociate any <code>IAM_PATTERN</code> principals that match the principal whose access you are removing.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociatePrincipalFromPortfolioFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_principal_from_portfolio::builders::DisassociatePrincipalFromPortfolioInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioOutput,
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioError,
    > for DisassociatePrincipalFromPortfolioFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioOutput,
            crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociatePrincipalFromPortfolioFluentBuilder {
    /// Creates a new `DisassociatePrincipalFromPortfolio`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociatePrincipalFromPortfolio as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_principal_from_portfolio::builders::DisassociatePrincipalFromPortfolioInputBuilder {
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
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolio::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolio::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioOutput,
        crate::operation::disassociate_principal_from_portfolio::DisassociatePrincipalFromPortfolioError,
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
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn set_accept_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn get_accept_language(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_accept_language()
    }
    /// <p>The portfolio identifier.</p>
    pub fn portfolio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.portfolio_id(input.into());
        self
    }
    /// <p>The portfolio identifier.</p>
    pub fn set_portfolio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_portfolio_id(input);
        self
    }
    /// <p>The portfolio identifier.</p>
    pub fn get_portfolio_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_portfolio_id()
    }
    /// <p>The ARN of the principal (user, role, or group). This field allows an ARN with no <code>accountID</code> with or without wildcard characters if <code>PrincipalType</code> is <code>IAM_PATTERN</code>.</p>
    pub fn principal_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal_arn(input.into());
        self
    }
    /// <p>The ARN of the principal (user, role, or group). This field allows an ARN with no <code>accountID</code> with or without wildcard characters if <code>PrincipalType</code> is <code>IAM_PATTERN</code>.</p>
    pub fn set_principal_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal_arn(input);
        self
    }
    /// <p>The ARN of the principal (user, role, or group). This field allows an ARN with no <code>accountID</code> with or without wildcard characters if <code>PrincipalType</code> is <code>IAM_PATTERN</code>.</p>
    pub fn get_principal_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal_arn()
    }
    /// <p>The supported value is <code>IAM</code> if you use a fully defined ARN, or <code>IAM_PATTERN</code> if you specify an <code>IAM</code> ARN with no AccountId, with or without wildcard characters. </p>
    pub fn principal_type(mut self, input: crate::types::PrincipalType) -> Self {
        self.inner = self.inner.principal_type(input);
        self
    }
    /// <p>The supported value is <code>IAM</code> if you use a fully defined ARN, or <code>IAM_PATTERN</code> if you specify an <code>IAM</code> ARN with no AccountId, with or without wildcard characters. </p>
    pub fn set_principal_type(mut self, input: ::std::option::Option<crate::types::PrincipalType>) -> Self {
        self.inner = self.inner.set_principal_type(input);
        self
    }
    /// <p>The supported value is <code>IAM</code> if you use a fully defined ARN, or <code>IAM_PATTERN</code> if you specify an <code>IAM</code> ARN with no AccountId, with or without wildcard characters. </p>
    pub fn get_principal_type(&self) -> &::std::option::Option<crate::types::PrincipalType> {
        self.inner.get_principal_type()
    }
}
