// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListTags`](crate::operation::list_tags::ListTags)
pub struct ListTagsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::operation::list_tags::builders::ListTagsInputBuilder,
    stop_on_duplicate_token: bool,
}

impl ListTagsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::operation::list_tags::builders::ListTagsInputBuilder) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `resource_tag_list`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::operation::list_tags::paginator::ListTagsPaginatorItems {
        crate::operation::list_tags::paginator::ListTagsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used
    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).
    pub fn send(
        self,
    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<
        ::std::result::Result<
            crate::operation::list_tags::ListTagsOutput,
            ::aws_smithy_runtime_api::client::result::SdkError<
                crate::operation::list_tags::ListTagsError,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >,
        >,
    > {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        let runtime_plugins = crate::operation::list_tags::ListTags::operation_runtime_plugins(
            handle.runtime_plugins.clone(),
            &handle.conf,
            ::std::option::Option::None,
        );
        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
            move |tx| {
                ::std::boxed::Box::pin(async move {
                    // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                    let mut input = match builder
                        .build()
                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
                    {
                        ::std::result::Result::Ok(input) => input,
                        ::std::result::Result::Err(e) => {
                            let _ = tx.send(::std::result::Result::Err(e)).await;
                            return;
                        }
                    };
                    loop {
                        let resp = crate::operation::list_tags::ListTags::orchestrate(&runtime_plugins, input.clone()).await;
                        // If the input member is None or it was an error
                        let done = match resp {
                            ::std::result::Result::Ok(ref resp) => {
                                let new_token = crate::lens::reflens_list_tags_output_next_token(resp);
                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                    true
                                } else {
                                    input.next_token = new_token.cloned();
                                    is_empty
                                }
                            }
                            ::std::result::Result::Err(_) => true,
                        };
                        if tx.send(resp).await.is_err() {
                            // receiving end was dropped
                            return;
                        }
                        if done {
                            return;
                        }
                    }
                })
            },
        ))
    }
}

/// Flattened paginator for `ListTagsPaginator`
///
/// This is created with [`.items()`](ListTagsPaginator::items)
pub struct ListTagsPaginatorItems(ListTagsPaginator);

impl ListTagsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note_: No requests will be dispatched until the stream is used
    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](aws_smithy_async::future::pagination_stream::PaginationStream::collect).
    pub fn send(
        self,
    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<
        ::std::result::Result<
            crate::types::ResourceTag,
            ::aws_smithy_runtime_api::client::result::SdkError<
                crate::operation::list_tags::ListTagsError,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >,
        >,
    > {
        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send())
            .flat_map(|page| crate::lens::lens_list_tags_output_resource_tag_list(page).unwrap_or_default().into_iter())
    }
}
