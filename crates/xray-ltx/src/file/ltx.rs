use crate::file::include::LtxIncludeConvertor;
use crate::file::inherit::LtxInheritConvertor;
use crate::file::section::section_entry::SectionEntry;
use crate::file::section::section_setter::SectionSetter;
use crate::file::types::{LtxIncluded, LtxSections};
use crate::{LtxResult, Section, ROOT_SECTION};
use std::ops::{Index, IndexMut};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ltx {
  pub(crate) includes: LtxIncluded,
  pub directory: Option<PathBuf>,
  pub path: Option<PathBuf>,
  pub sections: LtxSections,
}

impl Ltx {
  /// Create an instance.
  pub fn new() -> Self {
    Self::default()
  }

  /// Convert current instance of ltx file into full parsed one.
  pub fn into_included(self) -> LtxResult<Self> {
    LtxIncludeConvertor::convert(self)
  }

  /// Convert current instance of ltx file into full parsed one.
  pub fn into_inherited(self) -> LtxResult<Self> {
    LtxInheritConvertor::convert(self)
  }

  /// Get parent directory of LTX file.
  pub fn get_directory(&self) -> Option<&PathBuf> {
    self.directory.as_ref()
  }

  /// Set with a specified section, `None` is for the general section
  pub fn with_section<S>(&mut self, section: S) -> SectionSetter
  where
    S: Into<String>,
  {
    SectionSetter::new(self, section.into())
  }

  /// Set with general section, a simple wrapper of `with_section(ROOT_SECTION)`
  pub fn with_root_section(&mut self) -> SectionSetter {
    self.with_section(ROOT_SECTION)
  }

  /// Get the immutable general section
  pub fn root_section(&mut self) -> &Section {
    self
      .entry(ROOT_SECTION.into())
      .or_insert_with(Default::default)
  }

  /// Get the mutable general section
  pub fn root_section_mut(&mut self) -> &mut Section {
    self
      .section_mut(ROOT_SECTION)
      .expect("There is no root section in this Ltx")
  }

  /// Get a immutable section
  pub fn section<S>(&self, name: S) -> Option<&Section>
  where
    S: Into<String>,
  {
    self.sections.get(&name.into())
  }

  /// Check whether ltx has section with name.
  pub fn has_section<S>(&self, name: S) -> bool
  where
    S: Into<String>,
  {
    self.sections.contains_key(&name.into())
  }

  /// Get a mutable section
  pub fn section_mut<S>(&mut self, name: S) -> Option<&mut Section>
  where
    S: Into<String>,
  {
    self.sections.get_mut(&name.into())
  }

  pub fn entry(&mut self, name: String) -> SectionEntry<'_> {
    SectionEntry::from(self.sections.entry(name))
  }

  pub fn include(&mut self, file: String) {
    self.includes.push(file);
  }

  pub fn includes(&self, file: &String) -> bool {
    self.includes.contains(file)
  }

  pub fn get_included(&self) -> &Vec<String> {
    &self.includes
  }

  /// Clear all entries
  pub fn clear(&mut self) {
    self.sections.clear()
  }

  /// Iterate with sections
  pub fn sections(&self) -> impl DoubleEndedIterator<Item = &str> {
    self.sections.keys().map(|s| s.as_str())
  }

  /// Set key-value to a section
  pub fn set_to<S>(&mut self, section: S, key: String, value: String)
  where
    S: Into<String>,
  {
    self.with_section(section).set(key, value);
  }

  /// Get the first value from the sections with key
  pub fn get_from<S>(&self, section: S, key: &str) -> Option<&str>
  where
    S: Into<String>,
  {
    self
      .sections
      .get(&section.into())
      .and_then(|props| props.get(key))
  }

  /// Get the first value from the sections with key, return the default value if it does not exist
  pub fn get_from_or<'a, S>(&'a self, section: S, key: &str, default: &'a str) -> &'a str
  where
    S: Into<String>,
  {
    self.get_from(section, key).unwrap_or(default)
  }

  /// Get the first mutable value from the sections with key
  pub fn get_from_mut<S>(&mut self, section: S, key: &str) -> Option<&mut str>
  where
    S: Into<String>,
  {
    self
      .sections
      .get_mut(&section.into())
      .and_then(|prop| prop.get_mut(key).map(|it| it.as_mut_str()))
  }

  /// Delete the first section with key, return the properties if it exists
  pub fn delete<S>(&mut self, section: S) -> Option<Section>
  where
    S: Into<String>,
  {
    self.sections.shift_remove(&section.into())
  }

  /// Delete the key from the section, return the value if key exists or None
  pub fn delete_from<S>(&mut self, section: S, key: &str) -> Option<String>
  where
    S: Into<String>,
  {
    self.section_mut(section).and_then(|prop| prop.remove(key))
  }

  /// Total sections count
  pub fn len(&self) -> usize {
    self.sections.len()
  }

  /// Check if object contains no section
  pub fn is_empty(&self) -> bool {
    self.sections.is_empty()
  }
}

impl<'q> Index<&'q str> for Ltx {
  type Output = Section;

  fn index<'a>(&'a self, index: &'q str) -> &'a Section {
    match self.section(index) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

impl<'q> IndexMut<&'q str> for Ltx {
  fn index_mut<'a>(&'a mut self, index: &'q str) -> &'a mut Section {
    match self.section_mut(index) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::file::ltx::Ltx;
  use crate::{LtxError, LtxParseError, LtxResult, Section, ROOT_SECTION};

  #[test]
  fn load_from_str_with_empty_general_section() {
    let input = "[sec1]\nkey1=val1\n";
    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_ok());

    let mut output: Ltx = ltx.unwrap();
    assert_eq!(output.len(), 1);

    assert!(output.root_section().is_empty());
    assert!(output.root_section_mut().is_empty());

    let props1 = output.section(ROOT_SECTION).unwrap();
    assert!(props1.is_empty());
    let props2 = output.section("sec1").unwrap();
    assert_eq!(props2.len(), 1);
    assert_eq!(props2.get("key1"), Some("val1"));

    // Root section added.
    assert_eq!(output.len(), 2);
  }

  #[test]
  fn load_from_str_with_empty_input() {
    let input: &str = "";
    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_ok());

    let mut output: Ltx = ltx.unwrap();
    assert!(output.root_section().is_empty());
    assert!(output.root_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_empty_lines() {
    let input: &str = "\n\n\n";
    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_ok());

    let mut output: Ltx = ltx.unwrap();
    assert!(output.root_section().is_empty());
    assert!(output.root_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_valid_input() {
    let input: &str = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar\n";
    let opt: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(opt.is_ok());

    let output = opt.unwrap();
    // there is always a general section
    assert_eq!(output.len(), 2);
    assert!(output.section("sec1").is_some());

    let sec1 = output.section("sec1").unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "val1".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "377".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn load_from_str_without_ending_newline() {
    let input: &str = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar";
    let opt: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(opt.is_ok());
  }

  #[test]
  fn parse_error_numbers() {
    let invalid_input: &str = "\n\n[not_closed";
    let ltx: LtxResult<Ltx> = Ltx::read_from_str(invalid_input);

    assert!(ltx.is_err());

    match ltx.unwrap_err() {
      LtxError::Parse(error) => {
        assert_eq!(error.line, 3);
        assert_eq!(error.col, 12);
      }
      _ => {
        panic!("Unexpected error received");
      }
    }
  }

  #[test]
  fn parse_comment() {
    let input: &str = "; abcdefghijklmn\n";
    let opt = Ltx::read_from_str(input);
    assert!(opt.is_ok());
  }

  #[test]
  fn iter() {
    let input = "
[section name]
name = hello
gender = mail ; abdddd
";

    let mut ltx: Ltx = Ltx::read_from_str(input).unwrap();

    for _ in &mut ltx {}
    for _ in &ltx {}
    // for _ in ini {}
  }

  #[test]
  fn inherited() {
    let input = "
[section_name]: base1, base2, base3
name = hello
key = value ; comment
";

    let ltx: Ltx = Ltx::read_from_str(input).unwrap();

    assert_eq!(ltx.get_from("section_name", "name").unwrap(), "hello");
    assert_eq!(ltx.get_from("section_name", "key").unwrap(), "value");

    let properties = ltx.section("section_name").expect("Existing section");

    assert_eq!(properties.inherited.len(), 3);
    assert!(!properties.inherits_section("base0"));
    assert!(properties.inherits_section("base1"));
    assert!(properties.inherits_section("base2"));
    assert!(properties.inherits_section("base3"));
    assert!(!properties.inherits_section("base4"));
  }

  #[test]
  fn inherited_empty() {
    let input = "
[section_name]: ,,
name = hello
";

    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let properties: &Section = ltx.section("section_name").expect("Existing section");

    assert_eq!(properties.inherited.len(), 0);
  }

  #[test]
  fn includes() {
    let input = "
; comment line 1 before
; comment line 2 before
#include \"file1.ltx\"
#include \"file2.ltx\"
; comment line between
#include \"file3.ltx\"

[section_name]: base1, base2
name = hello
key = value ; comment
";

    let ltx: Ltx = Ltx::read_from_str(input).unwrap();

    assert_eq!(ltx.get_from("section_name", "name").unwrap(), "hello");
    assert_eq!(ltx.get_from("section_name", "key").unwrap(), "value");

    assert_eq!(ltx.get_included().len(), 3);
    assert!(ltx.includes(&String::from("file1.ltx")));
    assert!(ltx.includes(&String::from("file2.ltx")));
    assert!(ltx.includes(&String::from("file3.ltx")));
  }

  #[test]
  fn includes_no_duplicates() -> LtxResult {
    let input = "
#include \"file1.ltx\"
#include \"file1.ltx\"

[section_name]: base1, base2
name = hello
";

    let ltx = Ltx::read_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      TryInto::<LtxParseError>::try_into(ltx.unwrap_err())?.message,
      "Failed to parse include statement in ltx file, including 'file1.ltx' more than once"
    );

    Ok(())
  }

  #[test]
  fn includes_valid() -> LtxResult {
    let input = "
#include

[section_name]: base1, base2
name = hello
";

    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      TryInto::<LtxParseError>::try_into(ltx.unwrap_err())?.message,
      "Expected correct '#include \"config.ltx\"' statement, got '#include'"
    );

    Ok(())
  }

  #[test]
  fn includes_only_ltx() -> LtxResult {
    let input = "
#include \"file1.ini\"

[section_name]: base1, base2
name = hello
";

    let ltx = Ltx::read_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      TryInto::<LtxParseError>::try_into(ltx.unwrap_err())?.message,
      "Included file should have .ltx extension, got 'file1.ini'"
    );

    Ok(())
  }

  #[test]
  fn includes_empty() -> LtxResult {
    let input = "
#include \"\"

[section_name]: base1, base2
name = hello
";

    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      TryInto::<LtxParseError>::try_into(ltx.unwrap_err())?.message,
      "Expected valid file name in include statement, got empty file name"
    );

    Ok(())
  }

  #[test]
  fn string() {
    let input: &str = "
[section name]
; This is a comment
Key = \"Value\"
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    assert_eq!(ltx.get_from("section name", "Key").unwrap(), "\"Value\"");
  }

  #[test]
  fn string_comment() {
    let input: &str = "
[section name]
; This is a comment
Key = \"Value   # This is not a comment ; at all\"
Stuff = Other
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from("section name", "Key").unwrap(),
      "\"Value   # This is not a comment"
    );
  }

  #[test]
  fn string_single() {
    let input: &str = "
[section name]
; This is a comment
Key = 'Value'
Stuff = Other
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    assert_eq!(ltx.get_from("section name", "Key").unwrap(), "'Value'");
  }

  #[test]
  fn string_includes_quote() {
    let input: &str = "
[test]
Comment[tr]=İnternet'e erişin
Comment[uk]=Доступ до Інтернету
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from("test", "Comment[tr]").unwrap(),
      "İnternet'e erişin"
    );
  }

  #[test]
  fn string_single_comment() {
    let input: &str = "
[section name]
; This is a comment
Key = 'Value   # This is not a comment ; at all'
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from("section name", "Key").unwrap(),
      "'Value   # This is not a comment"
    );
  }

  #[test]
  fn load_from_str_with_valid_empty_input() {
    let input: &str = "key1=\nkey2=val2\n";
    let opt = Ltx::read_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    assert_eq!(output.len(), 1);
    assert!(output.section(ROOT_SECTION).is_some());

    let sec1 = output.section(ROOT_SECTION).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "val2".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn load_from_str_with_crlf() {
    let input: &str = "key1=val1\r\nkey2=val2\r\n";
    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_ok());

    let ltx: Ltx = ltx.unwrap();
    assert_eq!(ltx.len(), 1);
    assert!(ltx.section(ROOT_SECTION).is_some());

    let sec1: &Section = ltx.section(ROOT_SECTION).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "val1".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "val2".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn load_from_str_with_cr() {
    let input: &str = "key1=val1\rkey2=val2\r";
    let opt = Ltx::read_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    assert_eq!(output.len(), 1);
    assert!(output.section(ROOT_SECTION).is_some());
    let sec1 = output.section(ROOT_SECTION).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "val1".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "val2".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn get_with_non_static_key() {
    let input: &str = "key1=val1\nkey2=val2\n";
    let opt = Ltx::read_from_str(input).unwrap();

    let sec1 = opt.section(ROOT_SECTION).unwrap();

    let key = "key1".to_owned();
    sec1.get(&key).unwrap();
  }

  #[test]
  fn parse_without_quote() {
    let input = "
[desktop_entry]
Exec = \"/path/to/exe with space\" arg
";

    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let sec = ltx.section("desktop_entry").unwrap();
    assert_eq!(&sec["Exec"], "\"/path/to/exe with space\" arg");
  }

  #[test]
  fn preserve_order_section() {
    let input: &str = r"
none2 = n2
[sb]
p2 = 2
[sa]
x2 = 2
[sc]
cd1 = x
[xc]
xd = x
        ";

    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let keys: Vec<&str> = ltx.iter().map(|(k, _)| k).collect();

    assert_eq!(keys.len(), 5);
    assert_eq!(keys[0], ROOT_SECTION);
    assert_eq!(keys[1], "sb");
    assert_eq!(keys[2], "sa");
    assert_eq!(keys[3], "sc");
    assert_eq!(keys[4], "xc");
  }

  #[test]
  fn preserve_order_property() {
    let input = r"
x2 = n2
x1 = n2
x3 = n2
";
    let mut ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let section: &Section = ltx.root_section();
    let keys: Vec<&str> = section.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["x2", "x1", "x3"]);
  }

  #[test]
  fn preserve_order_property_in_section() {
    let input = r"
[s]
x2 = n2
xb = n2
a3 = n3
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let section: &Section = ltx.section("s").unwrap();
    let keys: Vec<&str> = section.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["x2", "xb", "a3"])
  }

  #[test]
  fn duplicate_sections() -> LtxResult {
    // https://github.com/zonyitoo/rust-ini/issues/49

    let input = r"
[peer]
foo = a

[peer]
foo = c
";

    let ltx: LtxResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      TryInto::<LtxParseError>::try_into(ltx.unwrap_err())?.message,
      "Duplicate sections are not allowed, looks like 'peer' is declared twice"
    );

    Ok(())
  }

  #[test]
  fn new_has_empty_general_section() {
    let mut ltx: Ltx = Ltx::new();

    assert!(ltx.root_section().is_empty());
    assert!(ltx.root_section_mut().is_empty());
    assert_eq!(ltx.len(), 1);
  }

  #[test]
  fn fix_issue63() {
    let section = "PHP";
    let key = "engine";
    let value = "On";
    let new_value = "Off";

    // create a new configuration
    let mut conf = Ltx::new();
    conf.with_section(section).set(key, value);

    // assert the value is the one expected
    let v = conf.get_from(section, key).unwrap();
    assert_eq!(v, value);

    // update the section/key with a new value
    conf.set_to(section, key.to_string(), new_value.to_string());

    // assert the new value was set
    let v = conf.get_from(section, key).unwrap();
    assert_eq!(v, new_value);
  }

  #[test]
  fn iter_mut_preserve_order_in_section() {
    let input: &str = r"
x2 = nc
x1 = na
x3 = nb
";

    let mut str: Ltx = Ltx::read_from_str(input).unwrap();
    let section: &mut Section = str.root_section_mut();
    section.iter_mut().enumerate().for_each(|(i, (_, v))| {
      v.push_str(&i.to_string());
    });
    let props: Vec<_> = section.iter().collect();
    assert_eq!(props, vec![("x2", "nc0"), ("x1", "na1"), ("x3", "nb2")]);
  }

  #[test]
  fn preserve_order_properties_into_iter() {
    let input: &str = r"
x2 = nc
x1 = na
x3 = nb
";

    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let (_, section) = ltx.into_iter().next().unwrap();
    let props: Vec<_> = section.into_iter().collect();
    assert_eq!(
      props,
      vec![
        ("x2".to_owned(), "nc".to_owned()),
        ("x1".to_owned(), "na".to_owned()),
        ("x3".to_owned(), "nb".to_owned())
      ]
    );
  }
}
