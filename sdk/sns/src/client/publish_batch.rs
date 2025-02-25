// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PublishBatch`](crate::operation::publish_batch::builders::PublishBatchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`topic_arn(impl Into<String>)`](crate::operation::publish_batch::builders::PublishBatchFluentBuilder::topic_arn) / [`set_topic_arn(Option<String>)`](crate::operation::publish_batch::builders::PublishBatchFluentBuilder::set_topic_arn):<br>required: **true**<br><p>The Amazon resource name (ARN) of the topic you want to batch publish to.</p><br>
    ///   - [`publish_batch_request_entries(PublishBatchRequestEntry)`](crate::operation::publish_batch::builders::PublishBatchFluentBuilder::publish_batch_request_entries) / [`set_publish_batch_request_entries(Option<Vec::<PublishBatchRequestEntry>>)`](crate::operation::publish_batch::builders::PublishBatchFluentBuilder::set_publish_batch_request_entries):<br>required: **true**<br><p>A list of <code>PublishBatch</code> request entries to be sent to the SNS topic.</p><br>
    /// - On success, responds with [`PublishBatchOutput`](crate::operation::publish_batch::PublishBatchOutput) with field(s):
    ///   - [`successful(Option<Vec::<PublishBatchResultEntry>>)`](crate::operation::publish_batch::PublishBatchOutput::successful): <p>A list of successful <code>PublishBatch</code> responses.</p>
    ///   - [`failed(Option<Vec::<BatchResultErrorEntry>>)`](crate::operation::publish_batch::PublishBatchOutput::failed): <p>A list of failed <code>PublishBatch</code> responses. </p>
    /// - On failure, responds with [`SdkError<PublishBatchError>`](crate::operation::publish_batch::PublishBatchError)
    pub fn publish_batch(&self) -> crate::operation::publish_batch::builders::PublishBatchFluentBuilder {
        crate::operation::publish_batch::builders::PublishBatchFluentBuilder::new(self.handle.clone())
    }
}
