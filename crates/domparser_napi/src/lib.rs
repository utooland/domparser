#[macro_use]
extern crate napi_derive;

use kuchikiki::{parse_html, traits::*};
use node_repr::NodeRepr;

mod serializer;

mod node_repr;

/// Parse string input to a html tree, return the root node.
///
#[napi]
pub fn parse(html: String) -> NodeRepr {
  let parser = parse_html();
  let document_node = parser.one(html);
  NodeRepr(document_node)
}
