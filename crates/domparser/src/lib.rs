use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;

pub mod node;
pub mod serializer;

pub use markup5ever_rcdom;
pub use node::DomNode;

/// Parse string input to a html tree, return the root node.
pub fn parse(html: String) -> DomNode {
  let mut parser = parse_document(RcDom::default(), Default::default());
  parser.process(html.into());
  let dom = parser.finish();

  DomNode(dom.document)
}
