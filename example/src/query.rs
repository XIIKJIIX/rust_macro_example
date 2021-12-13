#[derive(Debug, PartialEq, Eq)]
pub enum ElasticsearchQueryClause {
    Match {
        field: String,
        search_val: String,
    },
    Range {
        field: String,
        gte: Option<i32>,
        lte: Option<i32>,
    },
}
