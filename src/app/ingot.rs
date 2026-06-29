#[allow(clippy::module_inception)]
/// ingot module
pub mod ingot;
pub use ingot::Ingot;
/// error module
pub mod error;
/// parser module
pub mod parser;
/// `syntax_node` module
pub mod syntax_node;
/// `syntax_node_type` module
pub mod syntax_node_type;
