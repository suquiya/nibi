use std::{io, path::PathBuf};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize, ser::SerializeSeq};

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Default,
	strum::EnumString,
	strum::Display,
	strum::VariantNames,
	strum::EnumIter,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum FileType {
	#[default]
	Ron,
	Toml,
	Json,
	Xml,
	Hcl,
}

impl FileType {
	pub fn append_ext(&self, path: &mut PathBuf) {
		path.set_extension(self.to_string());
	}
}

pub fn get_extended_path<T: Into<PathBuf>>(path: T, file_type: FileType) -> PathBuf {
	let mut path = path.into();
	file_type.append_ext(&mut path);
	path
}

#[derive(Debug, strum::Display)]
pub enum SerError {
	Ron(ron::Error),
	Json(serde_json::Error),
	Toml(toml::ser::Error),
	IO(io::Error),
	Xml(quick_xml::SeError),
	Hcl(hcl::Error),
}

pub type SerResult<T> = Result<T, SerError>;

pub fn write_serialized_string<T: Serialize, W: std::fmt::Write + std::io::Write>(
	mut writer: W,
	value: &T,
	file_type: FileType,
) -> SerResult<()> {
	match file_type {
		FileType::Ron => {
			ron::ser::to_writer_pretty(writer, value, PrettyConfig::default().struct_names(true))
				.map_err(SerError::Ron)
		}
		FileType::Json => serde_json::to_writer_pretty(writer, value).map_err(SerError::Json),
		FileType::Toml => {
			let str = toml::to_string_pretty(value).map_err(SerError::Toml)?;
			writer.write_all(str.as_bytes()).map_err(SerError::IO)
		}
		FileType::Xml => quick_xml::se::to_writer(writer, value)
			.map_err(SerError::Xml)
			.map(|_| ()),
		FileType::Hcl => hcl::ser::to_writer(writer, value).map_err(SerError::Hcl),
	}
}

pub fn get_serialized_string<T: Serialize>(value: &T, file_type: FileType) -> SerResult<String> {
	match file_type {
		FileType::Ron => {
			ron::ser::to_string_pretty(value, PrettyConfig::default()).map_err(SerError::Ron)
		}
		FileType::Json => serde_json::to_string_pretty(value).map_err(SerError::Json),
		FileType::Toml => toml::to_string_pretty(value).map_err(SerError::Toml),
		FileType::Xml => quick_xml::se::to_string(value).map_err(SerError::Xml),
		FileType::Hcl => hcl::ser::to_string(value).map_err(SerError::Hcl),
	}
}

pub fn write_serialized_string_all<T: Serialize, W: std::io::Write>(
	mut writer: W,
	value: &T,
	file_type: FileType,
) -> SerResult<()> {
	let str = get_serialized_string(value, file_type)?;
	writer.write_all(str.as_bytes()).map_err(SerError::IO)
}

#[derive(Debug, strum::Display)]
pub enum DeError {
	Ron(ron::de::SpannedError),
	Json(serde_json::Error),
	Toml(toml::de::Error),
	IO(io::Error),
	Xml(quick_xml::DeError),
	Hcl(hcl::Error),
}

pub type DeResult<T> = Result<T, DeError>;

pub fn get_deselialized_value<T: for<'de> serde::de::Deserialize<'de>>(
	str: &str,
	file_type: FileType,
) -> DeResult<T> {
	match file_type {
		FileType::Ron => ron::de::from_str(str).map_err(DeError::Ron),
		FileType::Json => serde_json::from_str(str).map_err(DeError::Json),
		FileType::Toml => toml::from_str(str).map_err(DeError::Toml),
		FileType::Xml => quick_xml::de::from_str(str).map_err(DeError::Xml),
		FileType::Hcl => hcl::de::from_str(str).map_err(DeError::Hcl),
	}
}

pub fn read_deserialized_value<T: for<'de> serde::de::Deserialize<'de>, R: std::io::Read>(
	read: R,
	file_type: FileType,
) -> DeResult<T> {
	let content = io::read_to_string(read).map_err(DeError::IO)?;
	get_deselialized_value(&content, file_type)
}

#[derive(Debug, Default)]
pub struct StrValOrArray(pub Vec<String>);

impl StrValOrArray {
	pub fn inner(&self) -> &Vec<String> {
		let StrValOrArray(inner) = self;
		inner
	}

	pub fn take_inner(self) -> Vec<String> {
		let StrValOrArray(inner) = self;
		inner
	}
}

impl Serialize for StrValOrArray {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let StrValOrArray(values) = self;
		match values.len() {
			0 => serializer.serialize_str(""),
			1 => serializer.serialize_str(&values[0]),
			_ => {
				let mut seq = serializer.serialize_seq(Some(values.len()))?;
				for value in values {
					seq.serialize_element(value)?;
				}
				seq.end()
			}
		}
	}
}

impl<'de> Deserialize<'de> for StrValOrArray {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_any(StrValOrArrayVisitor)
	}
}

struct StrValOrArrayVisitor;

impl<'de> serde::de::Visitor<'de> for StrValOrArrayVisitor {
	type Value = StrValOrArray;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("string or array of strings")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(StrValOrArray(v.split(',').map(|s| s.to_owned()).collect()))
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::SeqAccess<'de>,
	{
		let mut values = Vec::new();
		while let Some(value) = seq.next_element()? {
			values.push(value);
		}
		Ok(StrValOrArray(values))
	}
}

#[cfg(test)]
mod test {
	use strum::VariantNames;

	use super::FileType;
	use std::str::FromStr;

	#[test]
	fn test() {
		assert_eq!(FileType::default(), FileType::Ron);
	}

	#[test]
	fn test_from_str() {
		let strs: [String; 5] = [
			"ron".into(),
			"toml".into(),
			"json".into(),
			"xml".into(),
			"hcl".into(),
		];
		let enums = [
			FileType::Ron,
			FileType::Toml,
			FileType::Json,
			FileType::Xml,
			FileType::Hcl,
		];

		for (s, e) in strs.iter().zip(enums.iter()) {
			let chars = s.chars().collect::<Vec<char>>();
			let len = 2_u32.pow(chars.len().try_into().unwrap());
			for i in 0..len {
				let s = chars
					.iter()
					.enumerate()
					.fold(String::new(), |mut s, (pos, c)| {
						let mask = 1 << pos;
						if i & mask != 0 {
							s.push(c.to_ascii_uppercase());
						} else {
							s.push(*c);
						}
						s
					});
				println!("{s}");
				assert_eq!(e, &FileType::from_str(&s).unwrap());
			}
		}
	}

	#[test]
	fn test_display() {
		let enums = [
			FileType::Ron,
			FileType::Json,
			FileType::Toml,
			FileType::Xml,
			FileType::Hcl,
		];
		let strs = ["ron", "json", "toml", "xml", "hcl"];
		for (s, e) in strs.iter().zip(enums.iter()) {
			assert_eq!(s.to_owned().to_owned(), e.to_string());
		}
	}

	#[test]
	fn test_variants() {
		assert_eq!(FileType::VARIANTS, ["ron", "toml", "json", "xml", "hcl"]);
	}
}
