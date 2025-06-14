#[derive(Debug)]
pub enum ParseError {
	Invalid,
	Empty,
	IO(std::io::Error),
}
