#[derive(Debug)]
/// Parses an `Ingot` from a reader.
pub struct IngotParser {}

fn split_chars(mut chars: Vec<char>, pos: usize) -> (Vec<char>, Vec<char>) {
	let after = chars.split_off(pos);
	(chars, after)
}

enum NewLineType {
	Cr,
	Lf,
	Crlf,
}

impl NewLineType {
	fn len(&self) -> usize {
		match self {
			NewLineType::Cr => 1,
			NewLineType::Lf => 1,
			NewLineType::Crlf => 2,
		}
	}
}

fn seek_next_nl(chars: &[char]) -> Option<(usize, NewLineType)> {
	let mut pos = 0;
	while let Some(c) = chars.get(pos) {
		match c {
			'\n' => return Some((pos, NewLineType::Lf)),
			'\r' => {
				if let Some('\n') = chars.get(pos + 1) {
					return Some((pos, NewLineType::Crlf));
				}
				return Some((pos, NewLineType::Cr));
			}
			_ => pos += 1,
		}
	}
	None
}

fn is_empty_chars(chars: &[char]) -> bool {
	chars.is_empty() || chars.iter().all(|c| c.is_whitespace())
}

impl IngotParser {}
