use html5ever::{namespace_url, ns, tendril::StrTendril, LocalName, Namespace, QualName};
use kuchikiki::{Attribute, ExpandedName, NodeData, NodeRef};

use super::NodeRepr;

#[napi]
impl NodeRepr {
  /// Append a child node to this node, after existing children.
  ///
  /// The child node will be remove from its previous position.
  ///
  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    self.0.append(new_child.0.clone())
  }

  #[napi(js_name = "appendChild")]
  pub fn append_child(&self, new_child: &NodeRepr) -> NodeRepr {
    self.append(new_child);
    NodeRepr::from(new_child.0.clone())
  }

  #[napi(js_name = "removeChild")]
  pub fn remove_child(&self, child: &NodeRepr) -> NodeRepr {
    child.0.detach();
    NodeRepr::from(child.0.clone())
  }

  /// Prepend a child node to this node, before existing children.
  ///
  /// The child node will be remove from its previous position.
  ///
  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    self.0.prepend(new_child.0.clone())
  }

  /// Insert a new sibling after this node.
  ///
  /// The sibling node will be remove from its previous position.
  ///
  #[napi(js_name = "after")]
  pub fn after(&self, new_sibling: &NodeRepr) {
    self.0.insert_after(new_sibling.0.clone())
  }

  /// Insert a new sibling before this node.
  ///
  /// The sibling node will be remove from its previous position.
  ///
  #[napi(js_name = "before")]
  pub fn before(&self, new_sibling: &NodeRepr) {
    self.0.insert_before(new_sibling.0.clone())
  }

  #[napi(js_name = "insertBefore")]
  pub fn insert_before_node(
    &self,
    new_node: &NodeRepr,
    ref_node: Option<&NodeRepr>,
  ) -> napi::Result<NodeRepr> {
    if let Some(ref_n) = ref_node {
      if ref_n.0.parent() != Some(self.0.clone()) {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          "The node before which the new node is to be inserted is not a child of this node."
            .to_string(),
        ));
      }
      ref_n.0.insert_before(new_node.0.clone());
    } else {
      self.0.append(new_node.0.clone());
    }
    Ok(NodeRepr::from(new_node.0.clone()))
  }

  /// Remove a node from its parent and siblings. Children are not affected.
  ///
  #[napi]
  pub fn remove(&self) {
    self.0.detach()
  }

  /// Assign an attribute K-V to this node
  ///
  #[napi]
  pub fn set_attribute(&self, name: String, value: String) {
    if let Some(ele) = self.0.as_element() {
      ele
        .attributes
        .borrow_mut()
        .insert(LocalName::from(name), StrTendril::from(value));
    }
  }

  /// Remove an attribute of this node by name.
  ///
  #[napi]
  pub fn remove_attribute(&self, name: String) {
    if let Some(ele) = self.0.as_element() {
      ele.attributes.borrow_mut().remove(LocalName::from(name));
    }
  }

  #[napi(js_name = "toggleAttribute")]
  pub fn toggle_attribute(&self, name: String, force: Option<bool>) -> bool {
    if let Some(ele) = self.0.as_element() {
      let mut attributes = ele.attributes.borrow_mut();
      let local_name = LocalName::from(name);
      let has_attr = attributes.contains(local_name.clone());

      let should_add = match force {
        Some(f) => f,
        None => !has_attr,
      };

      if should_add {
        if !has_attr {
          attributes.insert(local_name, StrTendril::from(""));
        }
        true
      } else {
        if has_attr {
          attributes.remove(local_name);
        }
        false
      }
    } else {
      false
    }
  }

  #[napi(js_name = "setAttributeNS")]
  pub fn set_attribute_ns(&self, namespace: Option<String>, name: String, value: String) {
    if let Some(ele) = self.0.as_element() {
      let (prefix, local) = if let Some(idx) = name.find(':') {
        (Some(name[..idx].to_string()), name[idx + 1..].to_string())
      } else {
        (None, name)
      };

      let ns = namespace.map(Into::into).unwrap_or(Namespace::from(""));
      let local_name = LocalName::from(local);
      let prefix_atom = prefix.map(Into::into);

      let expanded = ExpandedName {
        ns: ns,
        local: local_name,
      };

      ele.attributes.borrow_mut().map.insert(
        expanded,
        Attribute {
          prefix: prefix_atom,
          value: StrTendril::from(value),
        },
      );
    }
  }

  #[napi(js_name = "removeAttributeNS")]
  pub fn remove_attribute_ns(&self, namespace: Option<String>, local_name: String) {
    if let Some(ele) = self.0.as_element() {
      let ns = namespace.map(Into::into).unwrap_or(Namespace::from(""));
      let local = LocalName::from(local_name);
      let expanded = ExpandedName {
        ns: ns,
        local: local,
      };
      ele.attributes.borrow_mut().map.shift_remove(&expanded);
    }
  }

  #[napi(js_name = "createElement")]
  pub fn create_element(&self, tag_name: String) -> NodeRepr {
    let qual_name = QualName::new(None, ns!(html), LocalName::from(tag_name.to_lowercase()));
    let node = NodeRef::new_element(qual_name, vec![]);
    NodeRepr::from(node)
  }

  #[napi(js_name = "createTextNode")]
  pub fn create_text_node(&self, data: String) -> NodeRepr {
    let node = NodeRef::new_text(StrTendril::from(data));
    NodeRepr::from(node)
  }

  #[napi(js_name = "createComment")]
  pub fn create_comment(&self, data: String) -> NodeRepr {
    let node = NodeRef::new_comment(StrTendril::from(data));
    NodeRepr::from(node)
  }

  #[napi(js_name = "createDocumentFragment")]
  pub fn create_document_fragment(&self) -> NodeRepr {
    NodeRepr::from(NodeRef::new(NodeData::DocumentFragment))
  }

  #[napi(js_name = "createProcessingInstruction")]
  pub fn create_processing_instruction(&self, target: String, data: String) -> NodeRepr {
    let node =
      NodeRef::new_processing_instruction(StrTendril::from(target), StrTendril::from(data));
    NodeRepr::from(node)
  }

  #[napi(js_name = "importNode")]
  pub fn import_node(&self, external_node: &NodeRepr, deep: Option<bool>) -> NodeRepr {
    external_node.clone_node(deep)
  }

  #[napi(js_name = "adoptNode")]
  pub fn adopt_node(&self, external_node: &NodeRepr) -> NodeRepr {
    external_node.0.detach();
    NodeRepr::from(external_node.0.clone())
  }

  #[napi(js_name = "replaceChild")]
  pub fn replace_child(
    &self,
    new_child: &NodeRepr,
    old_child: &NodeRepr,
  ) -> napi::Result<NodeRepr> {
    if let Some(parent) = old_child.0.parent() {
      if parent == self.0 {
        old_child.0.insert_after(new_child.0.clone());
        old_child.0.detach();
        return Ok(NodeRepr::from(old_child.0.clone()));
      }
    }
    Err(napi::Error::new(
      napi::Status::InvalidArg,
      "The node to be replaced is not a child of this node.".to_string(),
    ))
  }

  #[napi(js_name = "replaceWith")]
  pub fn replace_with(&self, new_node: &NodeRepr) {
    self.0.insert_after(new_node.0.clone());
    self.0.detach();
  }

  #[napi(js_name = "insertAdjacentElement")]
  pub fn insert_adjacent_element(
    &self,
    position: String,
    element: &NodeRepr,
  ) -> napi::Result<Option<NodeRepr>> {
    match position.to_lowercase().as_str() {
      "beforebegin" => {
        self.0.insert_before(element.0.clone());
        Ok(Some(NodeRepr::from(element.0.clone())))
      }
      "afterbegin" => {
        self.0.prepend(element.0.clone());
        Ok(Some(NodeRepr::from(element.0.clone())))
      }
      "beforeend" => {
        self.0.append(element.0.clone());
        Ok(Some(NodeRepr::from(element.0.clone())))
      }
      "afterend" => {
        self.0.insert_after(element.0.clone());
        Ok(Some(NodeRepr::from(element.0.clone())))
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "The position must be one of 'beforebegin', 'afterbegin', 'beforeend', or 'afterend'."
          .to_string(),
      )),
    }
  }

  #[napi(js_name = "insertAdjacentText")]
  pub fn insert_adjacent_text(&self, position: String, data: String) -> napi::Result<()> {
    let text_node = NodeRef::new_text(StrTendril::from(data));
    match position.to_lowercase().as_str() {
      "beforebegin" => {
        self.0.insert_before(text_node);
        Ok(())
      }
      "afterbegin" => {
        self.0.prepend(text_node);
        Ok(())
      }
      "beforeend" => {
        self.0.append(text_node);
        Ok(())
      }
      "afterend" => {
        self.0.insert_after(text_node);
        Ok(())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "The position must be one of 'beforebegin', 'afterbegin', 'beforeend', or 'afterend'."
          .to_string(),
      )),
    }
  }

  #[napi(js_name = "insertAdjacentHTML")]
  pub fn insert_adjacent_html(&self, position: String, html: String) -> napi::Result<()> {
    use kuchikiki::parse_html;
    use kuchikiki::traits::TendrilSink;

    let parser = parse_html();
    let new_doc = parser.one(html);
    let nodes = if let Ok(body) = new_doc.select_first("body") {
      body.as_node().children()
    } else {
      new_doc.children()
    };

    // We need to collect nodes first because we can't iterate and modify at the same time if we were moving them,
    // but here we are moving them from new_doc to self.
    let nodes_vec: Vec<_> = nodes.collect();

    match position.to_lowercase().as_str() {
      "beforebegin" => {
        for child in nodes_vec {
          self.0.insert_before(child);
        }
        Ok(())
      }
      "afterbegin" => {
        // Prepend in reverse order to maintain order
        for child in nodes_vec.into_iter().rev() {
          self.0.prepend(child);
        }
        Ok(())
      }
      "beforeend" => {
        for child in nodes_vec {
          self.0.append(child);
        }
        Ok(())
      }
      "afterend" => {
        // Insert after in reverse order to maintain order
        for child in nodes_vec.into_iter().rev() {
          self.0.insert_after(child);
        }
        Ok(())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "The position must be one of 'beforebegin', 'afterbegin', 'beforeend', or 'afterend'."
          .to_string(),
      )),
    }
  }

  #[napi]
  pub fn normalize(&self) {
    let mut current_child = self.0.first_child();
    while let Some(child) = current_child {
      let next_sibling = child.next_sibling();
      if let NodeData::Text(ref t) = *child.data() {
        if t.borrow().is_empty() {
          child.detach();
        } else if let Some(ref next) = next_sibling {
          if let NodeData::Text(ref next_t) = *next.data() {
            t.borrow_mut().push_tendril(&next_t.borrow());
            next.detach();
            // Don't advance current_child, check the new next_sibling (which was next->next)
            current_child = Some(child);
            continue;
          }
        }
      }
      current_child = next_sibling;
    }
    // Recurse for children
    for child in self.0.children() {
      NodeRepr::from(child).normalize();
    }
  }

  #[napi(js_name = "splitText")]
  pub fn split_text(&self, offset: u32) -> napi::Result<NodeRepr> {
    let offset = offset as usize;
    match self.0.data() {
      NodeData::Text(t) => {
        let text = t.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let (first, second) = text.split_at(offset);

        *t.borrow_mut() = StrTendril::from(first);

        let new_node = NodeRef::new_text(StrTendril::from(second));
        self.0.insert_after(new_node.clone());

        Ok(NodeRepr::from(new_node))
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Node is not a Text node".to_string(),
      )),
    }
  }

  #[napi(js_name = "substringData")]
  pub fn substring_data(&self, offset: u32, count: u32) -> napi::Result<String> {
    let offset = offset as usize;
    let count = count as usize;
    match self.0.data() {
      NodeData::Text(t) => {
        let text = t.borrow();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let end = std::cmp::min(offset + count, text.len());
        Ok(text[offset..end].to_string())
      }
      NodeData::Comment(c) => {
        let text = c.borrow();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let end = std::cmp::min(offset + count, text.len());
        Ok(text[offset..end].to_string())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Node is not a CharacterData node".to_string(),
      )),
    }
  }

  #[napi(js_name = "appendData")]
  pub fn append_data(&self, data: String) -> napi::Result<()> {
    match self.0.data() {
      NodeData::Text(t) => {
        let mut text = t.borrow().to_string();
        text.push_str(&data);
        *t.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      NodeData::Comment(c) => {
        let mut text = c.borrow().to_string();
        text.push_str(&data);
        *c.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Node is not a CharacterData node".to_string(),
      )),
    }
  }

  #[napi(js_name = "insertData")]
  pub fn insert_data(&self, offset: u32, data: String) -> napi::Result<()> {
    let offset = offset as usize;
    match self.0.data() {
      NodeData::Text(t) => {
        let mut text = t.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        text.insert_str(offset, &data);
        *t.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      NodeData::Comment(c) => {
        let mut text = c.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        text.insert_str(offset, &data);
        *c.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Node is not a CharacterData node".to_string(),
      )),
    }
  }

  #[napi(js_name = "deleteData")]
  pub fn delete_data(&self, offset: u32, count: u32) -> napi::Result<()> {
    let offset = offset as usize;
    let count = count as usize;
    match self.0.data() {
      NodeData::Text(t) => {
        let mut text = t.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let end = std::cmp::min(offset + count, text.len());
        text.replace_range(offset..end, "");
        *t.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      NodeData::Comment(c) => {
        let mut text = c.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let end = std::cmp::min(offset + count, text.len());
        text.replace_range(offset..end, "");
        *c.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Node is not a CharacterData node".to_string(),
      )),
    }
  }

  #[napi(js_name = "replaceData")]
  pub fn replace_data(&self, offset: u32, count: u32, data: String) -> napi::Result<()> {
    let offset = offset as usize;
    let count = count as usize;
    match self.0.data() {
      NodeData::Text(t) => {
        let mut text = t.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let end = std::cmp::min(offset + count, text.len());
        text.replace_range(offset..end, &data);
        *t.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      NodeData::Comment(c) => {
        let mut text = c.borrow().to_string();
        if offset > text.len() {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "IndexSizeError".to_string(),
          ));
        }
        let end = std::cmp::min(offset + count, text.len());
        text.replace_range(offset..end, &data);
        *c.borrow_mut() = StrTendril::from(text);
        Ok(())
      }
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Node is not a CharacterData node".to_string(),
      )),
    }
  }
}
