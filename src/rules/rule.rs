#[derive(PartialEq, Debug)]
pub struct Rule<'a> {
    pub name: &'a str,
    pub rule: RuleOrs<'a>,
}

pub type RuleRef = usize;

#[derive(PartialEq, Debug)]
pub enum RulePiece<'a> {
    Literal(&'a str),
    Rule(RuleRef),
}

#[derive(PartialEq, Debug)]
pub struct RuleList<'a>(pub Vec<RulePiece<'a>>);

#[derive(PartialEq, Debug)]
pub struct RuleOrs<'a>(pub Vec<RuleList<'a>>);

// impl<'a> RuleOrs<'a> {
//     fn parse(&self, input: &'a str) -> Option<(Node<'a>, &'a str)> {
//         for rule_list in &self.0 {
//             if let Some(node) = rule_list.parse(input) {
//                 return Some(node);
//             }
//         }

//         None
//     }
// }

// impl<'a> RuleList<'a> {
//     fn parse(&self, input: &str) -> Option<(Node<'a>, &'a str)> {
//         let mut left = input;
//         let mut nodes = Vec::new();

//         for rule in &self.0 {
//             if let Some((node, input_left)) = rule.parse(left) {
//                 left = input_left;
//                 nodes.push(node);
//             }
//         }

//         // let node = Node {
//         //     name:
//         // }

//         // Some((Node, left))
//         todo!()
//     }
// }

// impl<'a> RulePiece<'a> {
//     fn parse(&self, input: &str) -> Option<(Node<'a>, &'a str)> {
//         match self {
//             Self::Literal(_) => todo!(),
//             Self::Rule(_) => todo!(),
//         }
//     }
// }
