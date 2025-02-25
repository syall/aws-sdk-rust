// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucket`](crate::operation::get_bucket::builders::GetBucketFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_account_id):<br>required: **true**<br><p>The Amazon Web Services account ID of the Outposts bucket.</p><br>
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_bucket):<br>required: **true**<br><p>Specifies the bucket.</p>  <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>  <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:   <region>    :    <account-id>     :outpost/     <outpost-id>      /bucket/      <my-bucket-name></my-bucket-name>     </outpost-id>    </account-id>   </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p><br>
    /// - On success, responds with [`GetBucketOutput`](crate::operation::get_bucket::GetBucketOutput) with field(s):
    ///   - [`bucket(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::bucket): <p>The Outposts bucket requested.</p>
    ///   - [`public_access_block_enabled(bool)`](crate::operation::get_bucket::GetBucketOutput::public_access_block_enabled): <p></p>
    ///   - [`creation_date(Option<DateTime>)`](crate::operation::get_bucket::GetBucketOutput::creation_date): <p>The creation date of the Outposts bucket.</p>
    /// - On failure, responds with [`SdkError<GetBucketError>`](crate::operation::get_bucket::GetBucketError)
    pub fn get_bucket(&self) -> crate::operation::get_bucket::builders::GetBucketFluentBuilder {
        crate::operation::get_bucket::builders::GetBucketFluentBuilder::new(self.handle.clone())
    }
}
