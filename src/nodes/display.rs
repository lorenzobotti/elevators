use std::fmt;

use super::node::Node;
use super::node::NodeContent;

impl<'grammar, 'input> fmt::Display for Node<'grammar, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name {
            Some(name) => write!(f, "{{\"name\": \"{}\", \"content\": [", name)?,
            None => write!(f, "[")?,
        }

        write!(f, "{}", self.content)?;

        match self.name {
            Some(_) => write!(f, "]}}")?,
            None => write!(f, "]")?,
        }

        Ok(())
    }
}

impl<'grammar, 'input> fmt::Display for NodeContent<'grammar, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(string) => write!(f, "\"{}\"", string)?,
            Self::Cons(list) => {
                write!(f, "{}", list[0])?;
                for node in list.iter().skip(1) {
                    write!(f, ", ")?;
                    write!(f, "{}", node)?;
                }
            }
        }

        Ok(())
    }
}

impl<'grammar, 'input> NodeContent<'grammar, 'input> {
    // fn primitive(&self) -> Option<&'input str> {
    //     match self {
    //         Self::Literal(string) => Some(string),
    //         Self::Cons(_) => None,
    //     }
    // }
}
