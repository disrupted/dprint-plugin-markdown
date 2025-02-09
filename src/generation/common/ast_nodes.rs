use crate::generation::gen_types::Context;

pub type Range = std::ops::Range<usize>;

pub trait Ranged {
  fn range(&self) -> &Range;
  fn text<'a>(&self, context: &Context<'a>) -> &'a str;
}

pub struct SourceFile {
  pub range: Range,
  pub children: Vec<Node>,
  pub yaml_header: Option<YamlHeader>,
}

pub struct YamlHeader {
  pub range: Range,
  pub text: String,
}

pub struct Heading {
  pub range: Range,
  pub level: u32,
  pub children: Vec<Node>,
}

pub struct Paragraph {
  pub range: Range,
  pub children: Vec<Node>,
}

pub struct BlockQuote {
  pub range: Range,
  pub children: Vec<Node>,
}

pub struct Text {
  pub range: Range,
  pub text: String,
}

pub enum TextDecorationKind {
  Emphasis,
  Strong,
  Strikethrough,
}

pub struct TextDecoration {
  pub range: Range,
  pub kind: TextDecorationKind,
  pub children: Vec<Node>,
}

pub struct Html {
  pub range: Range,
  pub text: String,
}

pub struct FootnoteReference {
  pub range: Range,
  pub name: String,
}

pub struct FootnoteDefinition {
  pub range: Range,
  pub name: String,
  pub children: Vec<Node>,
}

pub struct InlineLink {
  pub range: Range,
  pub children: Vec<Node>,
  pub url: String,
  pub title: Option<String>,
}

pub struct ReferenceLink {
  pub range: Range,
  pub children: Vec<Node>,
  pub reference: String,
}

pub struct ShortcutLink {
  pub range: Range,
  pub children: Vec<Node>,
}

pub struct AutoLink {
  pub range: Range,
  pub children: Vec<Node>,
}

pub struct LinkReference {
  pub range: Range,
  pub name: String,
  pub link: String,
  pub title: Option<String>,
}

pub struct InlineImage {
  pub range: Range,
  pub text: String,
  pub url: String,
  pub title: Option<String>,
}

pub struct ReferenceImage {
  pub range: Range,
  pub text: String,
  pub reference: String,
}

impl Text {
  pub fn starts_with_list_word(&self) -> bool {
    return crate::generation::utils::is_list_word(&get_first_word(&self.text));

    fn get_first_word(text: &str) -> String {
      let mut result = String::new();
      for c in text.chars() {
        if c.is_whitespace() {
          break;
        }
        result.push(c);
      }
      result
    }
  }
}

pub struct SoftBreak {
  pub range: Range,
}

pub struct HardBreak {
  pub range: Range,
}

pub struct List {
  pub range: Range,
  pub start_index: Option<u64>,
  pub children: Vec<Node>,
}

pub struct Item {
  pub range: Range,
  pub marker: Option<TaskListMarker>,
  pub children: Vec<Node>,
  pub sub_lists: Vec<Node>,
}

pub struct TaskListMarker {
  pub range: Range,
  pub is_checked: bool,
}

/// Inline code.
pub struct Code {
  pub range: Range,
  pub code: String,
}

pub struct CodeBlock {
  pub range: Range,
  pub tag: Option<String>,
  pub is_fenced: bool,
  pub code: String,
}

pub struct HorizontalRule {
  pub range: Range,
}

pub struct Table {
  pub range: Range,
  pub header: TableHead,
  pub column_alignment: Vec<ColumnAlignment>,
  pub rows: Vec<TableRow>,
}

pub struct TableHead {
  pub range: Range,
  pub cells: Vec<TableCell>,
}

pub struct TableRow {
  pub range: Range,
  pub cells: Vec<TableCell>,
}

pub struct TableCell {
  pub range: Range,
  pub children: Vec<Node>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ColumnAlignment {
  None,
  Left,
  Center,
  Right,
}

pub struct NotImplemented {
  pub range: Range,
}

macro_rules! generate_node {
    ($($node_name:ident),*) => {
        pub enum Node {
            $($node_name($node_name)),*,
        }

        #[cfg(debug_assertions)]
        #[derive(Debug)]
        pub enum NodeKind {
            $($node_name),*,
        }

        #[cfg(debug_assertions)]
        impl Node {
            #[allow(dead_code)]
            pub fn kind(&self) -> NodeKind {
                match self {
                    $(Node::$node_name(_) => NodeKind::$node_name),*
                }
            }
        }

        impl Ranged for Node {
            fn range<'a>(&'a self) -> &'a Range {
                match self {
                    $(Node::$node_name(node) => node.range()),*
                }
            }

            fn text<'a>(&self, context: &Context<'a>) -> &'a str {
                match self {
                    $(Node::$node_name(node) => node.text(context)),*
                }
            }
        }

        $(
        impl Ranged for $node_name {
            fn range<'a>(&'a self) -> &'a Range {
                &self.range
            }

            fn text<'a>(&self, context: &Context<'a>) -> &'a str {
                &context.file_text[self.range.start..self.range.end]
            }
        }

        impl Into<Node> for $node_name {
            fn into(self) -> Node {
                Node::$node_name(self)
            }
        }
        )*
    };
}

impl Node {
  pub fn starts_with_list_word(&self) -> bool {
    if let Node::Text(text) = self {
      text.starts_with_list_word()
    } else {
      false
    }
  }

  pub fn has_preceeding_space(&self, file_text: &str) -> bool {
    let range = self.range();
    if range.start == 0 {
      false
    } else {
      file_text.as_bytes().get(range.start - 1) == " ".as_bytes().get(0)
    }
  }

  pub fn starts_with_punctuation(&self, file_text: &str) -> bool {
    let range = self.range();
    let text = &file_text[range.start..range.end];
    if let Some(first_char) = text.chars().next() {
      first_char.is_ascii_punctuation()
    } else {
      false
    }
  }

  pub fn ends_with_punctuation(&self, file_text: &str) -> bool {
    let range = self.range();
    let text = &file_text[range.start..range.end];
    if let Some(last_char) = text.chars().last() {
      last_char.is_ascii_punctuation()
    } else {
      false
    }
  }
}

generate_node![
  NotImplemented,
  SourceFile,
  Heading,
  Paragraph,
  BlockQuote,
  Text,
  TextDecoration,
  Html,
  FootnoteReference,
  FootnoteDefinition,
  InlineLink,
  ReferenceLink,
  ShortcutLink,
  AutoLink,
  LinkReference,
  InlineImage,
  ReferenceImage,
  List,
  Item,
  TaskListMarker,
  SoftBreak,
  HardBreak,
  Code,
  CodeBlock,
  HorizontalRule,
  Table,
  TableHead,
  TableRow,
  TableCell
];
