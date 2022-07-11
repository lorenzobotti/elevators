use super::rule_line::RuleLine;
use super::rule_ors::RuleOrs;
use super::rule_piece::RulePiece;
use super::rule_series::RuleSeries;

pub trait IsPrimitive {
    fn is_primitive(&self) -> bool;
}

impl<'a> IsPrimitive for RulePiece<'a> {
    fn is_primitive(&self) -> bool {
        match self {
            Self::Double(_) => true,
            Self::Single(_) => true,
            Self::Ident(_) => false,
            Self::Range(_) => true,
        }
    }
}

impl<'a> IsPrimitive for RuleSeries<'a> {
    fn is_primitive(&self) -> bool {
        all_true(self.0.iter(), |piece| piece.is_primitive())
    }
}
impl<'a> IsPrimitive for RuleOrs<'a> {
    fn is_primitive(&self) -> bool {
        all_true(self.0.iter(), |series| series.is_primitive())
    }
}
impl<'a> IsPrimitive for RuleLine<'a> {
    fn is_primitive(&self) -> bool {
        self.rules.is_primitive()
    }
}

fn all_true<Iter, Func, T>(mut iter: Iter, func: Func) -> bool
where
    Iter: Iterator<Item = T>,
    Func: Fn(&T) -> bool,
{
    iter.find(func).is_some()
}
