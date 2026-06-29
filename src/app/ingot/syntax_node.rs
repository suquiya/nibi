use crate::app::{basic_token::token::BasicToken, ingot::syntax_node_type::SyntaxNodeType};

pub enum NodeChild {
	Node(SyntaxNode),
	BasicToken(BasicToken),
}

/// Syntax node for Ingot .
pub struct SyntaxNode {
	pub node_type: SyntaxNodeType,
	pub children: Vec<NodeChild>,
}

impl SyntaxNode {
	pub fn new(node_type: SyntaxNodeType, children: Vec<NodeChild>) -> Self {
		Self {
			node_type,
			children,
		}
	}
}
