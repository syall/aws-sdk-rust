// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_job::_create_job_output::CreateJobOutputBuilder;

pub use crate::operation::create_job::_create_job_input::CreateJobInputBuilder;

impl CreateJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_job::CreateJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_job::CreateJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateJob`.
///
/// <p>Creates a job to import or export data between Amazon S3 and your on-premises data center. Your Amazon Web Services account must have the right trust policies and permissions in place to create a job for a Snow device. If you're creating a job for a node in a cluster, you only need to provide the <code>clusterId</code> value; the other job attributes are inherited from the cluster. </p> <note>
/// <p>Only the Snowball; Edge device type is supported when ordering clustered jobs.</p>
/// <p>The device capacity is optional.</p>
/// <p>Availability of device types differ by Amazon Web Services Region. For more information about Region availability, see <a href="https://aws.amazon.com/about-aws/global-infrastructure/regional-product-services/?p=ngi&amp;loc=4">Amazon Web Services Regional Services</a>.</p>
/// </note>
/// <p></p>
/// <p class="title"> <b>Snow Family devices and their capacities.</b> </p>
/// <ul>
/// <li> <p>Device type: <b>SNC1_SSD</b> </p>
/// <ul>
/// <li> <p>Capacity: T14</p> </li>
/// <li> <p>Description: Snowcone </p> </li>
/// </ul> <p></p> </li>
/// <li> <p>Device type: <b>SNC1_HDD</b> </p>
/// <ul>
/// <li> <p>Capacity: T8</p> </li>
/// <li> <p>Description: Snowcone </p> </li>
/// </ul> <p></p> </li>
/// <li> <p>Device type: <b>EDGE_S</b> </p>
/// <ul>
/// <li> <p>Capacity: T98</p> </li>
/// <li> <p>Description: Snowball Edge Storage Optimized for data transfer only </p> </li>
/// </ul> <p></p> </li>
/// <li> <p>Device type: <b>EDGE_CG</b> </p>
/// <ul>
/// <li> <p>Capacity: T42</p> </li>
/// <li> <p>Description: Snowball Edge Compute Optimized with GPU</p> </li>
/// </ul> <p></p> </li>
/// <li> <p>Device type: <b>EDGE_C</b> </p>
/// <ul>
/// <li> <p>Capacity: T42</p> </li>
/// <li> <p>Description: Snowball Edge Compute Optimized without GPU</p> </li>
/// </ul> <p></p> </li>
/// <li> <p>Device type: <b>EDGE</b> </p>
/// <ul>
/// <li> <p>Capacity: T100</p> </li>
/// <li> <p>Description: Snowball Edge Storage Optimized with EC2 Compute</p> </li>
/// </ul> <note>
/// <p>This device is replaced with T98.</p>
/// </note> <p></p> </li>
/// <li> <p>Device type: <b>STANDARD</b> </p>
/// <ul>
/// <li> <p>Capacity: T50</p> </li>
/// <li> <p>Description: Original Snowball device</p> <note>
/// <p>This device is only available in the Ningxia, Beijing, and Singapore Amazon Web Services Region </p>
/// </note> </li>
/// </ul> <p></p> </li>
/// <li> <p>Device type: <b>STANDARD</b> </p>
/// <ul>
/// <li> <p>Capacity: T80</p> </li>
/// <li> <p>Description: Original Snowball device</p> <note>
/// <p>This device is only available in the Ningxia, Beijing, and Singapore Amazon Web Services Region. </p>
/// </note> </li>
/// </ul> <p></p> </li>
/// <li> <p>Snow Family device type: <b>RACK_5U_C</b> </p>
/// <ul>
/// <li> <p>Capacity: T13 </p> </li>
/// <li> <p>Description: Snowblade.</p> </li>
/// </ul> </li>
/// <li> <p>Device type: <b>V3_5S</b> </p>
/// <ul>
/// <li> <p>Capacity: T240</p> </li>
/// <li> <p>Description: Snowball Edge Storage Optimized 210TB</p> </li>
/// </ul> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_job::builders::CreateJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::create_job::CreateJobOutput, crate::operation::create_job::CreateJobError>
    for CreateJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::create_job::CreateJobOutput, crate::operation::create_job::CreateJobError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateJobFluentBuilder {
    /// Creates a new `CreateJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateJob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_job::builders::CreateJobInputBuilder {
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
        crate::operation::create_job::CreateJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_job::CreateJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_job::CreateJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_job::CreateJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_job::CreateJobOutput,
        crate::operation::create_job::CreateJobError,
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
    /// <p>Defines the type of job that you're creating. </p>
    pub fn job_type(mut self, input: crate::types::JobType) -> Self {
        self.inner = self.inner.job_type(input);
        self
    }
    /// <p>Defines the type of job that you're creating. </p>
    pub fn set_job_type(mut self, input: ::std::option::Option<crate::types::JobType>) -> Self {
        self.inner = self.inner.set_job_type(input);
        self
    }
    /// <p>Defines the type of job that you're creating. </p>
    pub fn get_job_type(&self) -> &::std::option::Option<crate::types::JobType> {
        self.inner.get_job_type()
    }
    /// <p>Defines the Amazon S3 buckets associated with this job.</p>
    /// <p>With <code>IMPORT</code> jobs, you specify the bucket or buckets that your transferred data will be imported into.</p>
    /// <p>With <code>EXPORT</code> jobs, you specify the bucket or buckets that your transferred data will be exported from. Optionally, you can also specify a <code>KeyRange</code> value. If you choose to export a range, you define the length of the range by providing either an inclusive <code>BeginMarker</code> value, an inclusive <code>EndMarker</code> value, or both. Ranges are UTF-8 binary sorted.</p>
    pub fn resources(mut self, input: crate::types::JobResource) -> Self {
        self.inner = self.inner.resources(input);
        self
    }
    /// <p>Defines the Amazon S3 buckets associated with this job.</p>
    /// <p>With <code>IMPORT</code> jobs, you specify the bucket or buckets that your transferred data will be imported into.</p>
    /// <p>With <code>EXPORT</code> jobs, you specify the bucket or buckets that your transferred data will be exported from. Optionally, you can also specify a <code>KeyRange</code> value. If you choose to export a range, you define the length of the range by providing either an inclusive <code>BeginMarker</code> value, an inclusive <code>EndMarker</code> value, or both. Ranges are UTF-8 binary sorted.</p>
    pub fn set_resources(mut self, input: ::std::option::Option<crate::types::JobResource>) -> Self {
        self.inner = self.inner.set_resources(input);
        self
    }
    /// <p>Defines the Amazon S3 buckets associated with this job.</p>
    /// <p>With <code>IMPORT</code> jobs, you specify the bucket or buckets that your transferred data will be imported into.</p>
    /// <p>With <code>EXPORT</code> jobs, you specify the bucket or buckets that your transferred data will be exported from. Optionally, you can also specify a <code>KeyRange</code> value. If you choose to export a range, you define the length of the range by providing either an inclusive <code>BeginMarker</code> value, an inclusive <code>EndMarker</code> value, or both. Ranges are UTF-8 binary sorted.</p>
    pub fn get_resources(&self) -> &::std::option::Option<crate::types::JobResource> {
        self.inner.get_resources()
    }
    /// <p>Specifies the service or services on the Snow Family device that your transferred data will be exported from or imported into. Amazon Web Services Snow Family supports Amazon S3 and NFS (Network File System) and the Amazon Web Services Storage Gateway service Tape Gateway type.</p>
    pub fn on_device_service_configuration(mut self, input: crate::types::OnDeviceServiceConfiguration) -> Self {
        self.inner = self.inner.on_device_service_configuration(input);
        self
    }
    /// <p>Specifies the service or services on the Snow Family device that your transferred data will be exported from or imported into. Amazon Web Services Snow Family supports Amazon S3 and NFS (Network File System) and the Amazon Web Services Storage Gateway service Tape Gateway type.</p>
    pub fn set_on_device_service_configuration(mut self, input: ::std::option::Option<crate::types::OnDeviceServiceConfiguration>) -> Self {
        self.inner = self.inner.set_on_device_service_configuration(input);
        self
    }
    /// <p>Specifies the service or services on the Snow Family device that your transferred data will be exported from or imported into. Amazon Web Services Snow Family supports Amazon S3 and NFS (Network File System) and the Amazon Web Services Storage Gateway service Tape Gateway type.</p>
    pub fn get_on_device_service_configuration(&self) -> &::std::option::Option<crate::types::OnDeviceServiceConfiguration> {
        self.inner.get_on_device_service_configuration()
    }
    /// <p>Defines an optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Defines an optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Defines an optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The ID for the address that you want the Snow device shipped to.</p>
    pub fn address_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.address_id(input.into());
        self
    }
    /// <p>The ID for the address that you want the Snow device shipped to.</p>
    pub fn set_address_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_address_id(input);
        self
    }
    /// <p>The ID for the address that you want the Snow device shipped to.</p>
    pub fn get_address_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_address_id()
    }
    /// <p>The <code>KmsKeyARN</code> that you want to associate with this job. <code>KmsKeyARN</code>s are created using the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> Key Management Service (KMS) API action.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_arn(input.into());
        self
    }
    /// <p>The <code>KmsKeyARN</code> that you want to associate with this job. <code>KmsKeyARN</code>s are created using the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> Key Management Service (KMS) API action.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_arn(input);
        self
    }
    /// <p>The <code>KmsKeyARN</code> that you want to associate with this job. <code>KmsKeyARN</code>s are created using the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> Key Management Service (KMS) API action.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_arn()
    }
    /// <p>The <code>RoleARN</code> that you want to associate with this job. <code>RoleArn</code>s are created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> Identity and Access Management (IAM) API action.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The <code>RoleARN</code> that you want to associate with this job. <code>RoleArn</code>s are created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> Identity and Access Management (IAM) API action.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The <code>RoleARN</code> that you want to associate with this job. <code>RoleArn</code>s are created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> Identity and Access Management (IAM) API action.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>If your job is being created in one of the US regions, you have the option of specifying what size Snow device you'd like for this job. In all other regions, Snowballs come with 80 TB in storage capacity.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn snowball_capacity_preference(mut self, input: crate::types::SnowballCapacity) -> Self {
        self.inner = self.inner.snowball_capacity_preference(input);
        self
    }
    /// <p>If your job is being created in one of the US regions, you have the option of specifying what size Snow device you'd like for this job. In all other regions, Snowballs come with 80 TB in storage capacity.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn set_snowball_capacity_preference(mut self, input: ::std::option::Option<crate::types::SnowballCapacity>) -> Self {
        self.inner = self.inner.set_snowball_capacity_preference(input);
        self
    }
    /// <p>If your job is being created in one of the US regions, you have the option of specifying what size Snow device you'd like for this job. In all other regions, Snowballs come with 80 TB in storage capacity.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn get_snowball_capacity_preference(&self) -> &::std::option::Option<crate::types::SnowballCapacity> {
        self.inner.get_snowball_capacity_preference()
    }
    /// <p>The shipping speed for this job. This speed doesn't dictate how soon you'll get the Snow device, rather it represents how quickly the Snow device moves to its destination while in transit. Regional shipping speeds are as follows:</p>
    /// <ul>
    /// <li> <p>In Australia, you have access to express shipping. Typically, Snow devices shipped express are delivered in about a day.</p> </li>
    /// <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snow devices shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li>
    /// <li> <p>In India, Snow devices are delivered in one to seven days.</p> </li>
    /// <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li>
    /// </ul>
    pub fn shipping_option(mut self, input: crate::types::ShippingOption) -> Self {
        self.inner = self.inner.shipping_option(input);
        self
    }
    /// <p>The shipping speed for this job. This speed doesn't dictate how soon you'll get the Snow device, rather it represents how quickly the Snow device moves to its destination while in transit. Regional shipping speeds are as follows:</p>
    /// <ul>
    /// <li> <p>In Australia, you have access to express shipping. Typically, Snow devices shipped express are delivered in about a day.</p> </li>
    /// <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snow devices shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li>
    /// <li> <p>In India, Snow devices are delivered in one to seven days.</p> </li>
    /// <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li>
    /// </ul>
    pub fn set_shipping_option(mut self, input: ::std::option::Option<crate::types::ShippingOption>) -> Self {
        self.inner = self.inner.set_shipping_option(input);
        self
    }
    /// <p>The shipping speed for this job. This speed doesn't dictate how soon you'll get the Snow device, rather it represents how quickly the Snow device moves to its destination while in transit. Regional shipping speeds are as follows:</p>
    /// <ul>
    /// <li> <p>In Australia, you have access to express shipping. Typically, Snow devices shipped express are delivered in about a day.</p> </li>
    /// <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snow devices shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li>
    /// <li> <p>In India, Snow devices are delivered in one to seven days.</p> </li>
    /// <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li>
    /// </ul>
    pub fn get_shipping_option(&self) -> &::std::option::Option<crate::types::ShippingOption> {
        self.inner.get_shipping_option()
    }
    /// <p>Defines the Amazon Simple Notification Service (Amazon SNS) notification settings for this job.</p>
    pub fn notification(mut self, input: crate::types::Notification) -> Self {
        self.inner = self.inner.notification(input);
        self
    }
    /// <p>Defines the Amazon Simple Notification Service (Amazon SNS) notification settings for this job.</p>
    pub fn set_notification(mut self, input: ::std::option::Option<crate::types::Notification>) -> Self {
        self.inner = self.inner.set_notification(input);
        self
    }
    /// <p>Defines the Amazon Simple Notification Service (Amazon SNS) notification settings for this job.</p>
    pub fn get_notification(&self) -> &::std::option::Option<crate::types::Notification> {
        self.inner.get_notification()
    }
    /// <p>The ID of a cluster. If you're creating a job for a node in a cluster, you need to provide only this <code>clusterId</code> value. The other job attributes are inherited from the cluster.</p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_id(input.into());
        self
    }
    /// <p>The ID of a cluster. If you're creating a job for a node in a cluster, you need to provide only this <code>clusterId</code> value. The other job attributes are inherited from the cluster.</p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_id(input);
        self
    }
    /// <p>The ID of a cluster. If you're creating a job for a node in a cluster, you need to provide only this <code>clusterId</code> value. The other job attributes are inherited from the cluster.</p>
    pub fn get_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_id()
    }
    /// <p>The type of Snow Family devices to use for this job. </p> <note>
    /// <p>For cluster jobs, Amazon Web Services Snow Family currently supports only the <code>EDGE</code> device type.</p>
    /// </note>
    /// <p>The type of Amazon Web Services Snow device to use for this job. Currently, the only supported device type for cluster jobs is <code>EDGE</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/snowball/latest/developer-guide/device-differences.html">Snowball Edge Device Options</a> in the Snowball Edge Developer Guide.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn snowball_type(mut self, input: crate::types::SnowballType) -> Self {
        self.inner = self.inner.snowball_type(input);
        self
    }
    /// <p>The type of Snow Family devices to use for this job. </p> <note>
    /// <p>For cluster jobs, Amazon Web Services Snow Family currently supports only the <code>EDGE</code> device type.</p>
    /// </note>
    /// <p>The type of Amazon Web Services Snow device to use for this job. Currently, the only supported device type for cluster jobs is <code>EDGE</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/snowball/latest/developer-guide/device-differences.html">Snowball Edge Device Options</a> in the Snowball Edge Developer Guide.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn set_snowball_type(mut self, input: ::std::option::Option<crate::types::SnowballType>) -> Self {
        self.inner = self.inner.set_snowball_type(input);
        self
    }
    /// <p>The type of Snow Family devices to use for this job. </p> <note>
    /// <p>For cluster jobs, Amazon Web Services Snow Family currently supports only the <code>EDGE</code> device type.</p>
    /// </note>
    /// <p>The type of Amazon Web Services Snow device to use for this job. Currently, the only supported device type for cluster jobs is <code>EDGE</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/snowball/latest/developer-guide/device-differences.html">Snowball Edge Device Options</a> in the Snowball Edge Developer Guide.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn get_snowball_type(&self) -> &::std::option::Option<crate::types::SnowballType> {
        self.inner.get_snowball_type()
    }
    /// <p>The forwarding address ID for a job. This field is not supported in most Regions.</p>
    pub fn forwarding_address_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.forwarding_address_id(input.into());
        self
    }
    /// <p>The forwarding address ID for a job. This field is not supported in most Regions.</p>
    pub fn set_forwarding_address_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_forwarding_address_id(input);
        self
    }
    /// <p>The forwarding address ID for a job. This field is not supported in most Regions.</p>
    pub fn get_forwarding_address_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_forwarding_address_id()
    }
    /// <p>The tax documents required in your Amazon Web Services Region.</p>
    pub fn tax_documents(mut self, input: crate::types::TaxDocuments) -> Self {
        self.inner = self.inner.tax_documents(input);
        self
    }
    /// <p>The tax documents required in your Amazon Web Services Region.</p>
    pub fn set_tax_documents(mut self, input: ::std::option::Option<crate::types::TaxDocuments>) -> Self {
        self.inner = self.inner.set_tax_documents(input);
        self
    }
    /// <p>The tax documents required in your Amazon Web Services Region.</p>
    pub fn get_tax_documents(&self) -> &::std::option::Option<crate::types::TaxDocuments> {
        self.inner.get_tax_documents()
    }
    /// <p>Defines the device configuration for an Snowcone job.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn device_configuration(mut self, input: crate::types::DeviceConfiguration) -> Self {
        self.inner = self.inner.device_configuration(input);
        self
    }
    /// <p>Defines the device configuration for an Snowcone job.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn set_device_configuration(mut self, input: ::std::option::Option<crate::types::DeviceConfiguration>) -> Self {
        self.inner = self.inner.set_device_configuration(input);
        self
    }
    /// <p>Defines the device configuration for an Snowcone job.</p>
    /// <p>For more information, see "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i> or "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    pub fn get_device_configuration(&self) -> &::std::option::Option<crate::types::DeviceConfiguration> {
        self.inner.get_device_configuration()
    }
    /// <p>Allows you to securely operate and manage Snowcone devices remotely from outside of your internal network. When set to <code>INSTALLED_AUTOSTART</code>, remote management will automatically be available when the device arrives at your location. Otherwise, you need to use the Snowball Edge client to manage the device. When set to <code>NOT_INSTALLED</code>, remote management will not be available on the device. </p>
    pub fn remote_management(mut self, input: crate::types::RemoteManagement) -> Self {
        self.inner = self.inner.remote_management(input);
        self
    }
    /// <p>Allows you to securely operate and manage Snowcone devices remotely from outside of your internal network. When set to <code>INSTALLED_AUTOSTART</code>, remote management will automatically be available when the device arrives at your location. Otherwise, you need to use the Snowball Edge client to manage the device. When set to <code>NOT_INSTALLED</code>, remote management will not be available on the device. </p>
    pub fn set_remote_management(mut self, input: ::std::option::Option<crate::types::RemoteManagement>) -> Self {
        self.inner = self.inner.set_remote_management(input);
        self
    }
    /// <p>Allows you to securely operate and manage Snowcone devices remotely from outside of your internal network. When set to <code>INSTALLED_AUTOSTART</code>, remote management will automatically be available when the device arrives at your location. Otherwise, you need to use the Snowball Edge client to manage the device. When set to <code>NOT_INSTALLED</code>, remote management will not be available on the device. </p>
    pub fn get_remote_management(&self) -> &::std::option::Option<crate::types::RemoteManagement> {
        self.inner.get_remote_management()
    }
    /// <p>The ID of the long-term pricing type for the device.</p>
    pub fn long_term_pricing_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.long_term_pricing_id(input.into());
        self
    }
    /// <p>The ID of the long-term pricing type for the device.</p>
    pub fn set_long_term_pricing_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_long_term_pricing_id(input);
        self
    }
    /// <p>The ID of the long-term pricing type for the device.</p>
    pub fn get_long_term_pricing_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_long_term_pricing_id()
    }
    /// <p>The highest impact level of data that will be stored or processed on the device, provided at job creation.</p>
    pub fn impact_level(mut self, input: crate::types::ImpactLevel) -> Self {
        self.inner = self.inner.impact_level(input);
        self
    }
    /// <p>The highest impact level of data that will be stored or processed on the device, provided at job creation.</p>
    pub fn set_impact_level(mut self, input: ::std::option::Option<crate::types::ImpactLevel>) -> Self {
        self.inner = self.inner.set_impact_level(input);
        self
    }
    /// <p>The highest impact level of data that will be stored or processed on the device, provided at job creation.</p>
    pub fn get_impact_level(&self) -> &::std::option::Option<crate::types::ImpactLevel> {
        self.inner.get_impact_level()
    }
    /// <p>Information identifying the person picking up the device.</p>
    pub fn pickup_details(mut self, input: crate::types::PickupDetails) -> Self {
        self.inner = self.inner.pickup_details(input);
        self
    }
    /// <p>Information identifying the person picking up the device.</p>
    pub fn set_pickup_details(mut self, input: ::std::option::Option<crate::types::PickupDetails>) -> Self {
        self.inner = self.inner.set_pickup_details(input);
        self
    }
    /// <p>Information identifying the person picking up the device.</p>
    pub fn get_pickup_details(&self) -> &::std::option::Option<crate::types::PickupDetails> {
        self.inner.get_pickup_details()
    }
}
