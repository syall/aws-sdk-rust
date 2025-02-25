// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPricingPlans`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`billing_period(impl Into<String>)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::billing_period) / [`set_billing_period(Option<String>)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::set_billing_period):<br>required: **false**<br><p>The preferred billing period to get pricing plan. </p><br>
    ///   - [`filters(ListPricingPlansFilter)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::filters) / [`set_filters(Option<ListPricingPlansFilter>)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::set_filters):<br>required: **false**<br><p>A <code>ListPricingPlansFilter</code> that specifies the Amazon Resource Name (ARNs) of pricing plans to retrieve pricing plans information.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of pricing plans to retrieve.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token that's used on subsequent call to get pricing plans. </p><br>
    /// - On success, responds with [`ListPricingPlansOutput`](crate::operation::list_pricing_plans::ListPricingPlansOutput) with field(s):
    ///   - [`billing_period(Option<String>)`](crate::operation::list_pricing_plans::ListPricingPlansOutput::billing_period): <p> The billing period for which the described pricing plans are applicable. </p>
    ///   - [`pricing_plans(Option<Vec::<PricingPlanListElement>>)`](crate::operation::list_pricing_plans::ListPricingPlansOutput::pricing_plans): <p>A list of <code>PricingPlanListElement</code> retrieved. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_pricing_plans::ListPricingPlansOutput::next_token): <p>The pagination token that's used on subsequent calls to get pricing plans. </p>
    /// - On failure, responds with [`SdkError<ListPricingPlansError>`](crate::operation::list_pricing_plans::ListPricingPlansError)
    pub fn list_pricing_plans(&self) -> crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder {
        crate::operation::list_pricing_plans::builders::ListPricingPlansFluentBuilder::new(self.handle.clone())
    }
}
