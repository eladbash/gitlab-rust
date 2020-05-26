// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashSet;

use derive_builder::Builder;

use crate::api::common::{self, AccessLevel, SortOrder};
use crate::api::endpoint_prelude::*;

/// Keys group results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupOrderBy {
    /// Order by the name of the group.
    Name,
    /// Order by the full path of the group.
    Path,
    /// Order by the group ID.
    Id,
}

impl Default for GroupOrderBy {
    fn default() -> Self {
        GroupOrderBy::Name
    }
}

impl GroupOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            GroupOrderBy::Name => "name",
            GroupOrderBy::Path => "path",
            GroupOrderBy::Id => "id",
        }
    }
}

impl ParamValue<'static> for GroupOrderBy {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for groups on an instance.
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Groups<'a> {
    /// Search for groups using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Skip groups with the given IDs.
    #[builder(setter(name = "_skip_groups"), default, private)]
    skip_groups: HashSet<u64>,
    /// Show all groups with access.
    ///
    /// Note that the default for this endpoint differs based on whether the API caller has
    /// administrator privileges or not.
    #[builder(default)]
    all_available: Option<bool>,
    /// Filter owned by those owned by the API caller.
    #[builder(default)]
    owned: Option<bool>,
    /// Filter groups by those where the API caller has a minimum access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,

    /// Include project statistics in the results.
    #[builder(default)]
    statistics: Option<bool>,
    /// Include custom attributes in th response.
    #[builder(default)]
    with_custom_attributes: Option<bool>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<GroupOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> Groups<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupsBuilder<'a> {
        GroupsBuilder::default()
    }
}

impl<'a> GroupsBuilder<'a> {
    /// Skip the given group ID.
    pub fn skip_group(&mut self, group: u64) -> &mut Self {
        self.skip_groups
            .get_or_insert_with(HashSet::new)
            .insert(group);
        self
    }

    /// Skip the given group IDs.
    pub fn skip_groups<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.skip_groups
            .get_or_insert_with(HashSet::new)
            .extend(iter);
        self
    }
}

impl<'a> Endpoint for Groups<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "groups".into()
    }

    fn add_parameters(&self, mut pairs: Pairs) {
        self.search
            .as_ref()
            .map(|value| pairs.append_pair("search", value));
        self.skip_groups.iter().for_each(|value| {
            pairs.append_pair("skip_groups[]", &format!("{}", value));
        });
        self.all_available
            .map(|value| pairs.append_pair("all_available", common::bool_str(value)));
        self.owned
            .map(|value| pairs.append_pair("owned", common::bool_str(value)));
        self.min_access_level
            .map(|value| pairs.append_pair("min_access_level", value.as_str()));
        self.statistics
            .map(|value| pairs.append_pair("statistics", common::bool_str(value)));
        self.with_custom_attributes
            .map(|value| pairs.append_pair("with_custom_attributes", common::bool_str(value)));

        self.order_by
            .map(|value| pairs.append_pair("order_by", value.as_str()));
        self.sort
            .map(|value| pairs.append_pair("sort", value.as_str()));
    }
}

impl<'a> Pageable for Groups<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::groups::{GroupOrderBy, Groups};

    #[test]
    fn order_by_default() {
        assert_eq!(GroupOrderBy::default(), GroupOrderBy::Name);
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (GroupOrderBy::Name, "name"),
            (GroupOrderBy::Path, "path"),
            (GroupOrderBy::Id, "id"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn defaults_are_sufficient() {
        Groups::builder().build().unwrap();
    }
}
