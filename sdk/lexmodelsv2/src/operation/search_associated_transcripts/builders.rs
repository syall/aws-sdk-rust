// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_associated_transcripts::_search_associated_transcripts_output::SearchAssociatedTranscriptsOutputBuilder;

pub use crate::operation::search_associated_transcripts::_search_associated_transcripts_input::SearchAssociatedTranscriptsInputBuilder;

impl SearchAssociatedTranscriptsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_associated_transcripts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchAssociatedTranscripts`.
///
/// <p>Search for associated transcripts that meet the specified criteria.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchAssociatedTranscriptsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_associated_transcripts::builders::SearchAssociatedTranscriptsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsOutput,
        crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsError,
    > for SearchAssociatedTranscriptsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsOutput,
            crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchAssociatedTranscriptsFluentBuilder {
    /// Creates a new `SearchAssociatedTranscripts`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchAssociatedTranscripts as a reference.
    pub fn as_input(&self) -> &crate::operation::search_associated_transcripts::builders::SearchAssociatedTranscriptsInputBuilder {
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
        crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_associated_transcripts::SearchAssociatedTranscripts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_associated_transcripts::SearchAssociatedTranscripts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsOutput,
        crate::operation::search_associated_transcripts::SearchAssociatedTranscriptsError,
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
    /// <p>The unique identifier of the bot associated with the transcripts that you are searching.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot associated with the transcripts that you are searching.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The unique identifier of the bot associated with the transcripts that you are searching.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The version of the bot containing the transcripts that you are searching.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot containing the transcripts that you are searching.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The version of the bot containing the transcripts that you are searching.</p>
    pub fn get_bot_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_version()
    }
    /// <p>The identifier of the language and locale of the transcripts to search. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a> </p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale of the transcripts to search. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a> </p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The identifier of the language and locale of the transcripts to search. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a> </p>
    pub fn get_locale_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale_id()
    }
    /// <p>The unique identifier of the bot recommendation associated with the transcripts to search.</p>
    pub fn bot_recommendation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_recommendation_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot recommendation associated with the transcripts to search.</p>
    pub fn set_bot_recommendation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_recommendation_id(input);
        self
    }
    /// <p>The unique identifier of the bot recommendation associated with the transcripts to search.</p>
    pub fn get_bot_recommendation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_recommendation_id()
    }
    /// <p>How SearchResults are ordered. Valid values are Ascending or Descending. The default is Descending.</p>
    pub fn search_order(mut self, input: crate::types::SearchOrder) -> Self {
        self.inner = self.inner.search_order(input);
        self
    }
    /// <p>How SearchResults are ordered. Valid values are Ascending or Descending. The default is Descending.</p>
    pub fn set_search_order(mut self, input: ::std::option::Option<crate::types::SearchOrder>) -> Self {
        self.inner = self.inner.set_search_order(input);
        self
    }
    /// <p>How SearchResults are ordered. Valid values are Ascending or Descending. The default is Descending.</p>
    pub fn get_search_order(&self) -> &::std::option::Option<crate::types::SearchOrder> {
        self.inner.get_search_order()
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A list of filter objects.</p>
    pub fn filters(mut self, input: crate::types::AssociatedTranscriptFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A list of filter objects.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AssociatedTranscriptFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A list of filter objects.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AssociatedTranscriptFilter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of bot recommendations to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of bot recommendations to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of bot recommendations to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If the response from the SearchAssociatedTranscriptsRequest operation contains more results than specified in the maxResults parameter, an index is returned in the response. Use that index in the nextIndex parameter to return the next page of results.</p>
    pub fn next_index(mut self, input: i32) -> Self {
        self.inner = self.inner.next_index(input);
        self
    }
    /// <p>If the response from the SearchAssociatedTranscriptsRequest operation contains more results than specified in the maxResults parameter, an index is returned in the response. Use that index in the nextIndex parameter to return the next page of results.</p>
    pub fn set_next_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_next_index(input);
        self
    }
    /// <p>If the response from the SearchAssociatedTranscriptsRequest operation contains more results than specified in the maxResults parameter, an index is returned in the response. Use that index in the nextIndex parameter to return the next page of results.</p>
    pub fn get_next_index(&self) -> &::std::option::Option<i32> {
        self.inner.get_next_index()
    }
}
