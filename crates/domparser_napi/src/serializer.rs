use html5ever::{
  serialize::{
    Serialize, Serializer,
    TraversalScope::{self, ChildrenOnly, IncludeNode},
  },
  QualName,
};
use kuchikiki::{NodeData, NodeRef};
use std::io::{Result, Write};

use crate::node_repr::NodeRepr;

impl Serialize for NodeRepr {
  fn serialize<S: Serializer>(
    &self,
    serializer: &mut S,
    traversal_scope: TraversalScope,
  ) -> Result<()> {
    match (traversal_scope, self.0.data()) {
      (ref scope, NodeData::Element(element)) => {
        if *scope == IncludeNode {
          let attrs = element.attributes.borrow();

          let attrs = attrs
            .map
            .iter()
            .map(|(name, attr)| {
              (
                QualName::new(attr.prefix.clone(), name.ns.clone(), name.local.clone()),
                &attr.value,
              )
            })
            .collect::<Vec<_>>();

          serializer.start_elem(
            element.name.clone(),
            attrs.iter().map(|&(ref name, value)| (name, &**value)),
          )?
        }

        for child in self.0.children() {
          Serialize::serialize(&child, serializer, IncludeNode)?
        }

        if *scope == IncludeNode {
          serializer.end_elem(element.name.clone())?
        }
        Ok(())
      }

      (_, &NodeData::DocumentFragment) | (_, &NodeData::Document(_)) => {
        for child in self.0.children() {
          Serialize::serialize(&child, serializer, IncludeNode)?
        }
        Ok(())
      }

      (ChildrenOnly(_), _) => Ok(()),

      (IncludeNode, NodeData::Doctype(doctype)) => serializer.write_doctype(&doctype.name),
      (IncludeNode, NodeData::Text(text)) => serializer.write_text(&text.borrow()),
      (IncludeNode, NodeData::Comment(text)) => serializer.write_comment(&text.borrow()),
      (IncludeNode, NodeData::ProcessingInstruction(contents)) => {
        let contents = contents.borrow();
        serializer.write_processing_instruction(&contents.0, &contents.1)
      }
    }
  }
}

pub(crate) fn serialize_text_only<Wr: Write>(node_ref: &NodeRef, writer: &mut Wr) -> Result<()> {
  match node_ref.data() {
    NodeData::Text(text) => {
      writer.write_all(text.borrow().as_bytes())?;
      Ok(())
    }
    NodeData::Element(_) => {
      for child in node_ref.children() {
        serialize_text_only(&child, writer)?
      }
      Ok(())
    }
    _ => Ok(()),
  }
}
