#[derive(Clone, Debug)]
pub enum RuleContent<'a> {
    Cons(Vec<Rule<'a>>),
    Literal(&'a str),
}

#[derive(Clone, Debug)]
pub struct Rule<'a> {
    name: &'a str,
    content: RuleContent<'a>,
}