use rustc_hash::FxHashMap;

use super::rule::Rule;
use super::rule::RuleRef;

#[derive(PartialEq, Debug)]
pub struct Grammar<'a> {
    // todo: questo potrebbe tranquillamente essere un Vec
    pub rules: FxHashMap<RuleRef, Rule<'a>>,
}

impl<'a> Grammar<'a> {
    pub fn main(&'a self) -> Option<&'a Rule<'a>> {
        self.rules.get(&0)
    }

    pub fn get(&'a self, rf: RuleRef) -> Option<&'a Rule<'a>> {
        self.rules.get(&rf)
    }
}
