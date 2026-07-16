#[derive(Debug, Clone)]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
    EqualityMatch(String, String),
    Substring(String, SubstringFilter),
    GreaterOrEqual(String, String),
    LessOrEqual(String, String),
    Present(String),
    ApproximateMatch(String, String),
}

#[derive(Debug, Clone)]
pub struct SubstringFilter {
    pub initial: Option<String>,
    pub any: Vec<String>,
    pub final_: Option<String>,
}
