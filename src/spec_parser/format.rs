use std::fmt;

use crate::spec_parser::{
    grammar::Grammar, rule_line::RuleLine, rule_ors::RuleOrs, rule_piece::RulePiece,
    rule_series::RuleSeries,
};

impl<'a> fmt::Display for RulePiece<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(content) => write!(f, "{}", content.0),
            Self::Single(content) => write!(f, "{}", content.0),
            Self::Ident(content) => write!(f, "{}", content.0),
        }
    }
}

impl<'a> fmt::Display for RuleSeries<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rules = self.0.iter();

        match rules.next() {
            Some(rule) => rule.fmt(f)?,
            None => return Ok(()),
        }

        for rule in rules {
            write!(f, " ")?;
            rule.fmt(f)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for RuleOrs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rules = self.0.iter();

        match rules.next() {
            Some(rule) => rule.fmt(f)?,
            None => return Ok(()),
        }

        for rule in rules {
            write!(f, " | ")?;
            rule.fmt(f)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for RuleLine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.name)?;
        self.rules.fmt(f)?;
        Ok(())
    }
}

impl<'a> fmt::Display for Grammar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rules = self.rules.iter();
        write!(f, "{};", rules.next().unwrap().1)?;

        for (_, rule) in rules {
            write!(f, "\n{};", rule)?;
        }

        Ok(())
    }
}
