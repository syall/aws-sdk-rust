// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The health details of an Elastic Inference Accelerator. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ElasticInferenceAcceleratorHealth {
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub status: ::std::option::Option<::std::string::String>,
}
impl ElasticInferenceAcceleratorHealth {
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl ElasticInferenceAcceleratorHealth {
    /// Creates a new builder-style object to manufacture [`ElasticInferenceAcceleratorHealth`](crate::types::ElasticInferenceAcceleratorHealth).
    pub fn builder() -> crate::types::builders::ElasticInferenceAcceleratorHealthBuilder {
        crate::types::builders::ElasticInferenceAcceleratorHealthBuilder::default()
    }
}

/// A builder for [`ElasticInferenceAcceleratorHealth`](crate::types::ElasticInferenceAcceleratorHealth).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ElasticInferenceAcceleratorHealthBuilder {
    pub(crate) status: ::std::option::Option<::std::string::String>,
}
impl ElasticInferenceAcceleratorHealthBuilder {
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }
    /// Consumes the builder and constructs a [`ElasticInferenceAcceleratorHealth`](crate::types::ElasticInferenceAcceleratorHealth).
    pub fn build(self) -> crate::types::ElasticInferenceAcceleratorHealth {
        crate::types::ElasticInferenceAcceleratorHealth { status: self.status }
    }
}
