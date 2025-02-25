// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SearchSkillGroups`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::set_next_token):<br>required: **false**<br><p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p><br>
    ///   - [`max_results(i32)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p><br>
    ///   - [`filters(Filter)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::set_filters):<br>required: **false**<br><p>The filters to use to list a specified set of skill groups. The supported filter key is SkillGroupName. </p><br>
    ///   - [`sort_criteria(Sort)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::sort_criteria) / [`set_sort_criteria(Option<Vec::<Sort>>)`](crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::set_sort_criteria):<br>required: **false**<br><p>The sort order to use in listing the specified set of skill groups. The supported sort key is SkillGroupName. </p><br>
    /// - On success, responds with [`SearchSkillGroupsOutput`](crate::operation::search_skill_groups::SearchSkillGroupsOutput) with field(s):
    ///   - [`skill_groups(Option<Vec::<SkillGroupData>>)`](crate::operation::search_skill_groups::SearchSkillGroupsOutput::skill_groups): <p>The skill groups that meet the filter criteria, in sort order.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::search_skill_groups::SearchSkillGroupsOutput::next_token): <p>The token returned to indicate that there is more data available.</p>
    ///   - [`total_count(Option<i32>)`](crate::operation::search_skill_groups::SearchSkillGroupsOutput::total_count): <p>The total number of skill groups returned.</p>
    /// - On failure, responds with [`SdkError<SearchSkillGroupsError>`](crate::operation::search_skill_groups::SearchSkillGroupsError)
    #[deprecated(note = "Alexa For Business is no longer supported")]
    pub fn search_skill_groups(&self) -> crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder {
        crate::operation::search_skill_groups::builders::SearchSkillGroupsFluentBuilder::new(self.handle.clone())
    }
}
