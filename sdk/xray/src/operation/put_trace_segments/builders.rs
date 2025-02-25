// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_trace_segments::_put_trace_segments_output::PutTraceSegmentsOutputBuilder;

pub use crate::operation::put_trace_segments::_put_trace_segments_input::PutTraceSegmentsInputBuilder;

impl PutTraceSegmentsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_trace_segments::PutTraceSegmentsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_trace_segments::PutTraceSegmentsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_trace_segments();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutTraceSegments`.
///
/// <p>Uploads segment documents to Amazon Web Services X-Ray. The <a href="https://docs.aws.amazon.com/xray/index.html">X-Ray SDK</a> generates segment documents and sends them to the X-Ray daemon, which uploads them in batches. A segment document can be a completed segment, an in-progress segment, or an array of subsegments.</p>
/// <p>Segments must include the following fields. For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">Amazon Web Services X-Ray Segment Documents</a> in the <i>Amazon Web Services X-Ray Developer Guide</i>.</p>
/// <p class="title"> <b>Required segment document fields</b> </p>
/// <ul>
/// <li> <p> <code>name</code> - The name of the service that handled the request.</p> </li>
/// <li> <p> <code>id</code> - A 64-bit identifier for the segment, unique among segments in the same trace, in 16 hexadecimal digits.</p> </li>
/// <li> <p> <code>trace_id</code> - A unique identifier that connects all segments and subsegments originating from a single client request.</p> </li>
/// <li> <p> <code>start_time</code> - Time the segment or subsegment was created, in floating point seconds in epoch time, accurate to milliseconds. For example, <code>1480615200.010</code> or <code>1.480615200010E9</code>.</p> </li>
/// <li> <p> <code>end_time</code> - Time the segment or subsegment was closed. For example, <code>1480615200.090</code> or <code>1.480615200090E9</code>. Specify either an <code>end_time</code> or <code>in_progress</code>.</p> </li>
/// <li> <p> <code>in_progress</code> - Set to <code>true</code> instead of specifying an <code>end_time</code> to record that a segment has been started, but is not complete. Send an in-progress segment when your application receives a request that will take a long time to serve, to trace that the request was received. When the response is sent, send the complete segment to overwrite the in-progress segment.</p> </li>
/// </ul>
/// <p>A <code>trace_id</code> consists of three numbers separated by hyphens. For example, 1-58406520-a006649127e371903a2de979. This includes:</p>
/// <p class="title"> <b>Trace ID Format</b> </p>
/// <ul>
/// <li> <p>The version number, for instance, <code>1</code>.</p> </li>
/// <li> <p>The time of the original request, in Unix epoch time, in 8 hexadecimal digits. For example, 10:00AM December 2nd, 2016 PST in epoch time is <code>1480615200</code> seconds, or <code>58406520</code> in hexadecimal.</p> </li>
/// <li> <p>A 96-bit identifier for the trace, globally unique, in 24 hexadecimal digits.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutTraceSegmentsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_trace_segments::builders::PutTraceSegmentsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_trace_segments::PutTraceSegmentsOutput,
        crate::operation::put_trace_segments::PutTraceSegmentsError,
    > for PutTraceSegmentsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_trace_segments::PutTraceSegmentsOutput,
            crate::operation::put_trace_segments::PutTraceSegmentsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutTraceSegmentsFluentBuilder {
    /// Creates a new `PutTraceSegments`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutTraceSegments as a reference.
    pub fn as_input(&self) -> &crate::operation::put_trace_segments::builders::PutTraceSegmentsInputBuilder {
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
        crate::operation::put_trace_segments::PutTraceSegmentsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_trace_segments::PutTraceSegmentsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_trace_segments::PutTraceSegments::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_trace_segments::PutTraceSegments::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_trace_segments::PutTraceSegmentsOutput,
        crate::operation::put_trace_segments::PutTraceSegmentsError,
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
    /// Appends an item to `TraceSegmentDocuments`.
    ///
    /// To override the contents of this collection use [`set_trace_segment_documents`](Self::set_trace_segment_documents).
    ///
    /// <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    pub fn trace_segment_documents(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.trace_segment_documents(input.into());
        self
    }
    /// <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    pub fn set_trace_segment_documents(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_trace_segment_documents(input);
        self
    }
    /// <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    pub fn get_trace_segment_documents(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_trace_segment_documents()
    }
}
