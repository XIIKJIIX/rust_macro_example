use example_derive::Clauseable;

use crate::query::ElasticsearchQueryClause;

#[derive(Clauseable)]
pub struct SearchInput {
    #[search_field("name")]
    pub name_like: Option<MatchInput>,
    #[search_field("weight")]
    pub weight_range: Option<RangeInput>,
    #[search_field("breed.keyword")]
    pub breed: Option<MatchInput>,
}

pub struct RangeInput {
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
}

impl RangeInput {
    fn to_clause(&self, field_name: String) -> ElasticsearchQueryClause {
        ElasticsearchQueryClause::Range {
            field: field_name,
            gte: self.lower_bound,
            lte: self.upper_bound,
        }
    }
}

pub struct MatchInput {
    value: String,
}

impl MatchInput {
    fn to_clause(&self, field_name: String) -> ElasticsearchQueryClause {
        ElasticsearchQueryClause::Match {
            field: field_name,
            search_val: self.value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::query::ElasticsearchQueryClause;

    use super::{MatchInput, RangeInput, SearchInput};

    #[test]
    fn to_clauses_of_search_input_should_return_a_correct_result() {
        let input = SearchInput {
            name_like: Some(MatchInput {
                value: "Leo".into(),
            }),
            weight_range: Some(RangeInput {
                lower_bound: Some(3),
                upper_bound: Some(10),
            }),
            breed: Some(MatchInput {
                value: "PERSIAN".into(),
            }),
        };

        let clauses = input.to_clauses();

        let expected = vec![
            ElasticsearchQueryClause::Match {
                field: "name".into(),
                search_val: "Leo".into(),
            },
            ElasticsearchQueryClause::Range {
                field: "weight".into(),
                gte: Some(3),
                lte: Some(10),
            },
            ElasticsearchQueryClause::Match {
                field: "breed.keyword".into(),
                search_val: "PERSIAN".into(),
            },
        ];

        assert_eq!(clauses, expected);
    }
}
