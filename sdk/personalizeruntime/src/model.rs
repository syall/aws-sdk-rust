// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that identifies an item.</p>
/// <p>The and APIs return a list of <code>PredictedItem</code>s.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PredictedItem {
    /// <p>The recommended item ID.</p>
    #[doc(hidden)]
    pub item_id: std::option::Option<std::string::String>,
    /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
    #[doc(hidden)]
    pub score: std::option::Option<f64>,
    /// <p>The name of the promotion that included the predicted item.</p>
    #[doc(hidden)]
    pub promotion_name: std::option::Option<std::string::String>,
}
impl PredictedItem {
    /// <p>The recommended item ID.</p>
    pub fn item_id(&self) -> std::option::Option<&str> {
        self.item_id.as_deref()
    }
    /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
    pub fn score(&self) -> std::option::Option<f64> {
        self.score
    }
    /// <p>The name of the promotion that included the predicted item.</p>
    pub fn promotion_name(&self) -> std::option::Option<&str> {
        self.promotion_name.as_deref()
    }
}
/// See [`PredictedItem`](crate::model::PredictedItem).
pub mod predicted_item {

    /// A builder for [`PredictedItem`](crate::model::PredictedItem).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) item_id: std::option::Option<std::string::String>,
        pub(crate) score: std::option::Option<f64>,
        pub(crate) promotion_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The recommended item ID.</p>
        pub fn item_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.item_id = Some(input.into());
            self
        }
        /// <p>The recommended item ID.</p>
        pub fn set_item_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.item_id = input;
            self
        }
        /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
        pub fn score(mut self, input: f64) -> Self {
            self.score = Some(input);
            self
        }
        /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
        pub fn set_score(mut self, input: std::option::Option<f64>) -> Self {
            self.score = input;
            self
        }
        /// <p>The name of the promotion that included the predicted item.</p>
        pub fn promotion_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.promotion_name = Some(input.into());
            self
        }
        /// <p>The name of the promotion that included the predicted item.</p>
        pub fn set_promotion_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.promotion_name = input;
            self
        }
        /// Consumes the builder and constructs a [`PredictedItem`](crate::model::PredictedItem).
        pub fn build(self) -> crate::model::PredictedItem {
            crate::model::PredictedItem {
                item_id: self.item_id,
                score: self.score,
                promotion_name: self.promotion_name,
            }
        }
    }
}
impl PredictedItem {
    /// Creates a new builder-style object to manufacture [`PredictedItem`](crate::model::PredictedItem).
    pub fn builder() -> crate::model::predicted_item::Builder {
        crate::model::predicted_item::Builder::default()
    }
}

/// <p>Contains information on a promotion. A promotion defines additional business rules that apply to a configurable subset of recommended items.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Promotion {
    /// <p>The name of the promotion.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The percentage of recommended items to apply the promotion to.</p>
    #[doc(hidden)]
    pub percent_promoted_items: i32,
    /// <p>The Amazon Resource Name (ARN) of the filter used by the promotion. This filter defines the criteria for promoted items. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/promoting-items.html#promotion-filters">Promotion filters</a>.</p>
    #[doc(hidden)]
    pub filter_arn: std::option::Option<std::string::String>,
    /// <p>The values to use when promoting items. For each placeholder parameter in your promotion's filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
    /// <p>For more information on creating filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
    #[doc(hidden)]
    pub filter_values:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl Promotion {
    /// <p>The name of the promotion.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The percentage of recommended items to apply the promotion to.</p>
    pub fn percent_promoted_items(&self) -> i32 {
        self.percent_promoted_items
    }
    /// <p>The Amazon Resource Name (ARN) of the filter used by the promotion. This filter defines the criteria for promoted items. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/promoting-items.html#promotion-filters">Promotion filters</a>.</p>
    pub fn filter_arn(&self) -> std::option::Option<&str> {
        self.filter_arn.as_deref()
    }
    /// <p>The values to use when promoting items. For each placeholder parameter in your promotion's filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
    /// <p>For more information on creating filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
    pub fn filter_values(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.filter_values.as_ref()
    }
}
/// See [`Promotion`](crate::model::Promotion).
pub mod promotion {

    /// A builder for [`Promotion`](crate::model::Promotion).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) percent_promoted_items: std::option::Option<i32>,
        pub(crate) filter_arn: std::option::Option<std::string::String>,
        pub(crate) filter_values: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The name of the promotion.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the promotion.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The percentage of recommended items to apply the promotion to.</p>
        pub fn percent_promoted_items(mut self, input: i32) -> Self {
            self.percent_promoted_items = Some(input);
            self
        }
        /// <p>The percentage of recommended items to apply the promotion to.</p>
        pub fn set_percent_promoted_items(mut self, input: std::option::Option<i32>) -> Self {
            self.percent_promoted_items = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the filter used by the promotion. This filter defines the criteria for promoted items. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/promoting-items.html#promotion-filters">Promotion filters</a>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.filter_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the filter used by the promotion. This filter defines the criteria for promoted items. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/promoting-items.html#promotion-filters">Promotion filters</a>.</p>
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.filter_arn = input;
            self
        }
        /// Adds a key-value pair to `filter_values`.
        ///
        /// To override the contents of this collection use [`set_filter_values`](Self::set_filter_values).
        ///
        /// <p>The values to use when promoting items. For each placeholder parameter in your promotion's filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
        /// <p>For more information on creating filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
        pub fn filter_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.filter_values.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.filter_values = Some(hash_map);
            self
        }
        /// <p>The values to use when promoting items. For each placeholder parameter in your promotion's filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
        /// <p>For more information on creating filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
        pub fn set_filter_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.filter_values = input;
            self
        }
        /// Consumes the builder and constructs a [`Promotion`](crate::model::Promotion).
        pub fn build(self) -> crate::model::Promotion {
            crate::model::Promotion {
                name: self.name,
                percent_promoted_items: self.percent_promoted_items.unwrap_or_default(),
                filter_arn: self.filter_arn,
                filter_values: self.filter_values,
            }
        }
    }
}
impl Promotion {
    /// Creates a new builder-style object to manufacture [`Promotion`](crate::model::Promotion).
    pub fn builder() -> crate::model::promotion::Builder {
        crate::model::promotion::Builder::default()
    }
}
