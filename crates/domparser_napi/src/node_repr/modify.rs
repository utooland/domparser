use html5ever::{namespace_url, ns, tendril::StrTendril, LocalName, QualName};
use markup5ever_rcdom::{Handle, Node, NodeData};
use std::cell::RefCell;
use std::rc::Rc;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  fn detach_node(node: &Handle) {
    let parent = super::get_parent(node);
    if let Some(parent) = parent {
      let mut children = parent.children.borrow_mut();
      if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, node)) {
        children.remove(pos);
      }
    }
    node.parent.set(None);
  }

  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    Self::detach_node(&new_child.0);
    self.0.children.borrow_mut().push(new_child.0.clone());
    new_child.0.parent.set(Some(Rc::downgrade(&self.0)));
  }

  #[napi(js_name = "appendChild")]
  pub fn append_child(&self, new_child: &NodeRepr) -> NodeRepr {
    self.append(new_child);
    NodeRepr(new_child.0.clone())
  }

  #[napi(js_name = "removeChild")]
  pub fn remove_child(&self, child: &NodeRepr) -> NodeRepr {
    Self::detach_node(&child.0);
    NodeRepr(child.0.clone())
  }

  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    Self::detach_node(&new_child.0);
    self.0.children.borrow_mut().insert(0, new_child.0.clone());
    new_child.0.parent.set(Some(Rc::downgrade(&self.0)));
  }

  #[napi(js_name = "after")]
  pub fn after(&self, new_sibling: &NodeRepr) {
    Self::detach_node(&new_sibling.0);
    if let Some(parent) = super::get_parent(&self.0) {
      let mut children = parent.children.borrow_mut();
      if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
        children.insert(pos + 1, new_sibling.0.clone());
        new_sibling.0.parent.set(Some(Rc::downgrade(&parent)));
      }
    }
  }

  #[napi(js_name = "before")]
  pub fn before(&self, new_sibling: &NodeRepr) {
    Self::detach_node(&new_sibling.0);
    if let Some(parent) = super::get_parent(&self.0) {
      let mut children = parent.children.borrow_mut();
      if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
        children.insert(pos, new_sibling.0.clone());
        new_sibling.0.parent.set(Some(Rc::downgrade(&parent)));
      }
    }
  }

  #[napi(js_name = "insertBefore")]
  pub fn insert_before_node(
    &self,
    new_node: &NodeRepr,
    ref_node: Option<&NodeRepr>,
  ) -> napi::Result<NodeRepr> {
    if let Some(ref_n) = ref_node {
      let parent = super::get_parent(&ref_n.0);
      let is_child = if let Some(p) = parent {
        Rc::ptr_eq(&p, &self.0)
      } else {
        false
      };

      if !is_child {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          "The node before which the new node is to be inserted is not a child of this node."
            .to_string(),
        ));
      }

      Self::detach_node(&new_node.0);
      let mut children = self.0.children.borrow_mut();
      if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &ref_n.0)) {
        children.insert(pos, new_node.0.clone());
        new_node.0.parent.set(Some(Rc::downgrade(&self.0)));
      }
    } else {
      self.append(new_node);
    }
    Ok(NodeRepr(new_node.0.clone()))
  }

  #[napi]
  pub fn remove(&self) {
    Self::detach_node(&self.0);
  }

  #[napi]
  pub fn set_attribute(&self, name: String, value: String) {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      let mut attributes = attrs.borrow_mut();
      if let Some(attr) = attributes
        .iter_mut()
        .find(|a| a.name.local.as_ref() == name)
      {
        attr.value = value.into();
      } else {
        attributes.push(html5ever::Attribute {
          name: QualName::new(None, ns!(), LocalName::from(name)),
          value: value.into(),
        });
      }
    }
  }

  #[napi]
  pub fn remove_attribute(&self, name: String) {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      let mut attributes = attrs.borrow_mut();
      if let Some(pos) = attributes
        .iter()
        .position(|a| a.name.local.as_ref() == name)
      {
        attributes.remove(pos);
      }
    }
  }

  #[napi(js_name = "toggleAttribute")]
  pub fn toggle_attribute(&self, name: String, force: Option<bool>) -> bool {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      let mut attributes = attrs.borrow_mut();
      let local_name = LocalName::from(name.clone());
      let idx = attributes.iter().position(|a| a.name.local == local_name);
      let has_attr = idx.is_some();

      let should_add = match force {
        Some(f) => f,
        None => !has_attr,
      };

      if should_add {
        if !has_attr {
          attributes.push(html5ever::Attribute {
            name: QualName::new(None, ns!(), local_name),
            value: StrTendril::from(""),
          });
        }
        true
      } else {
        if let Some(i) = idx {
          attributes.remove(i);
        }
        false
      }
    } else {
      false
    }
  }

  #[napi(js_name = "setAttributeNS")]
  pub fn set_attribute_ns(&self, namespace: Option<String>, name: String, value: String) {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      let (prefix, local) = if let Some(idx) = name.find(':') {
        (Some(name[..idx].to_string()), name[idx + 1..].to_string())
      } else {
        (None, name)
      };

      let ns = namespace.map(Into::into).unwrap_or(ns!());
      let local_name = LocalName::from(local);
      let prefix_atom = prefix.map(Into::into);

      let qual_name = QualName::new(prefix_atom, ns, local_name);

      let mut attributes = attrs.borrow_mut();
      if let Some(attr) = attributes.iter_mut().find(|a| a.name == qual_name) {
        attr.value = value.into();
      } else {
        attributes.push(html5ever::Attribute {
          name: qual_name,
          value: value.into(),
        });
      }
    }
  }

  #[napi(js_name = "removeAttributeNS")]
  pub fn remove_attribute_ns(&self, namespace: Option<String>, local_name: String) {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      let ns = namespace.map(Into::into).unwrap_or(ns!());
      let local = LocalName::from(local_name);
      let mut attributes = attrs.borrow_mut();
      if let Some(pos) = attributes
        .iter()
        .position(|a| a.name.ns == ns && a.name.local == local)
      {
        attributes.remove(pos);
      }
    }
  }

  #[napi(js_name = "createElement")]
  pub fn create_element(&self, tag_name: String) -> NodeRepr {
    let qual_name = QualName::new(None, ns!(html), LocalName::from(tag_name.to_lowercase()));
    let node = Node::new(NodeData::Element {
      name: qual_name,
      attrs: RefCell::new(vec![]),
      template_contents: RefCell::new(None),
      mathml_annotation_xml_integration_point: false,
    });
    NodeRepr(node)
  }

  #[napi(js_name = "createTextNode")]
  pub fn create_text_node(&self, data: String) -> NodeRepr {
    let node = Node::new(NodeData::Text {
      contents: RefCell::new(data.into()),
    });
    NodeRepr(node)
  }

  #[napi(js_name = "createComment")]
  pub fn create_comment(&self, data: String) -> NodeRepr {
    let node = Node::new(NodeData::Comment {
      contents: data.into(),
    });
    NodeRepr(node)
  }

  #[napi(js_name = "createDocumentFragment")]
  pub fn create_document_fragment(&self) -> NodeRepr {
    let qual_name = QualName::new(None, ns!(), LocalName::from("#document-fragment"));
    let node = Node::new(NodeData::Element {
      name: qual_name,
      attrs: RefCell::new(vec![]),
      template_contents: RefCell::new(None),
      mathml_annotation_xml_integration_point: false,
    });
    NodeRepr(node)
  }

  #[napi(js_name = "createProcessingInstruction")]
  pub fn create_processing_instruction(&self, target: String, data: String) -> NodeRepr {
    let node = Node::new(NodeData::ProcessingInstruction {
      target: target.into(),
      contents: data.into(),
    });
    NodeRepr(node)
  }

  #[napi(js_name = "importNode")]
  pub fn import_node(&self, external_node: &NodeRepr, deep: Option<bool>) -> NodeRepr {
    external_node.clone_node(deep)
  }

  #[napi(js_name = "adoptNode")]
  pub fn adopt_node(&self, external_node: &NodeRepr) -> NodeRepr {
    Self::detach_node(&external_node.0);
    NodeRepr(external_node.0.clone())
  }

  #[napi(js_name = "replaceChild")]
  pub fn replace_child(
    &self,
    new_child: &NodeRepr,
    old_child: &NodeRepr,
  ) -> napi::Result<NodeRepr> {
    let parent = super::get_parent(&old_child.0);
    if let Some(parent) = parent {
      if Rc::ptr_eq(&parent, &self.0) {
        Self::detach_node(&new_child.0);
        let mut children = self.0.children.borrow_mut();
        if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &old_child.0)) {
          children[pos] = new_child.0.clone();
          new_child.0.parent.set(Some(Rc::downgrade(&self.0)));
          old_child.0.parent.set(None);
          return Ok(NodeRepr(old_child.0.clone()));
        }
      }
    }
    Err(napi::Error::new(
      napi::Status::InvalidArg,
      "The node to be replaced is not a child of this node.".to_string(),
    ))
  }

  #[napi(js_name = "replaceWith")]
  pub fn replace_with(&self, new_node: &NodeRepr) {
    if let Some(parent) = super::get_parent(&self.0) {
      Self::detach_node(&new_node.0);
      let mut children = parent.children.borrow_mut();
      if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
        children[pos] = new_node.0.clone();
        new_node.0.parent.set(Some(Rc::downgrade(&parent)));
        self.0.parent.set(None);
      }
    }
  }
}
