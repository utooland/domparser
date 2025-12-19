#[macro_use]
extern crate napi_derive;

use html5ever::{parse_document, QualName, ns, local_name, namespace_url};
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{RcDom, NodeData, Node};
use node_repr::NodeRepr;
use std::rc::Rc;
use std::cell::RefCell;

mod serializer;

mod node_repr;

/// Parse string input to a html tree, return the root node.
///
#[napi]
pub fn parse(html: String) -> NodeRepr {
  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();
    
  {
      let mut children = dom.document.children.borrow_mut();
      let has_html = children.iter().any(|c| {
          if let NodeData::Element { name, .. } = &c.data {
              name.local.as_ref() == "html"
          } else {
              false
          }
      });
      
      if !has_html {
          let html_name = QualName::new(None, ns!(html), local_name!("html"));
          let html_node = Node::new(NodeData::Element {
              name: html_name,
              attrs: RefCell::new(vec![]),
              template_contents: RefCell::new(None),
              mathml_annotation_xml_integration_point: false,
          });
          
          for child in children.drain(..) {
              html_node.children.borrow_mut().push(child.clone());
              child.parent.set(Some(Rc::downgrade(&html_node)));
          }
          
          children.push(html_node.clone());
          html_node.parent.set(Some(Rc::downgrade(&dom.document)));
      }
  }
  
  NodeRepr(dom.document)
}
