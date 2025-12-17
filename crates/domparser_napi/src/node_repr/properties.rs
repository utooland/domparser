use super::NodeRepr;
use indexmap::IndexMap;
use kuchikiki::{parse_html, traits::*, NodeData};

#[napi]
impl NodeRepr {
  #[napi(getter)]
  pub fn node_type(&self) -> i32 {
    match self.0.data() {
      NodeData::Element(_) => 1,
      NodeData::Text(_) => 3,
      NodeData::Comment(_) => 8,
      NodeData::Document(_) => 9,
      NodeData::Doctype(_) => 10,
      NodeData::DocumentFragment => 11,
      _ => 0,
    }
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    match self.0.data() {
      NodeData::Element(data) => data.name.local.to_string().to_uppercase(),
      NodeData::Text(_) => "#text".to_string(),
      NodeData::Comment(_) => "#comment".to_string(),
      NodeData::Document(_) => "#document".to_string(),
      NodeData::Doctype(data) => data.name.to_string(),
      NodeData::DocumentFragment => "#document-fragment".to_string(),
      NodeData::ProcessingInstruction(data) => data.borrow().0.to_string(),
    }
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> Option<String> {
    self
      .0
      .as_element()
      .map(|data| data.name.local.to_string().to_uppercase())
  }

  #[napi(getter, js_name = "namespaceURI")]
  pub fn namespace_uri(&self) -> Option<String> {
    self.0.as_element().map(|data| data.name.ns.to_string())
  }

  #[napi(getter)]
  pub fn prefix(&self) -> Option<String> {
    self
      .0
      .as_element()
      .and_then(|data| data.name.prefix.as_ref().map(|p| p.to_string()))
  }

  #[napi(getter, js_name = "localName")]
  pub fn local_name(&self) -> Option<String> {
    self.0.as_element().map(|data| data.name.local.to_string())
  }

  #[napi(getter)]
  pub fn id(&self) -> String {
    self.get_attribute("id".to_string()).unwrap_or_default()
  }

  #[napi(setter)]
  pub fn set_id(&self, id: String) {
    self.set_attribute("id".to_string(), id);
  }

  #[napi(getter)]
  pub fn class_name(&self) -> String {
    self.get_attribute("class".to_string()).unwrap_or_default()
  }

  #[napi(setter)]
  pub fn set_class_name(&self, class_name: String) {
    self.set_attribute("class".to_string(), class_name);
  }

  #[napi(getter)]
  pub fn parent_node(&self) -> Option<NodeRepr> {
    self.0.parent().map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn first_child(&self) -> Option<NodeRepr> {
    self.0.first_child().map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn last_child(&self) -> Option<NodeRepr> {
    self.0.last_child().map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn previous_sibling(&self) -> Option<NodeRepr> {
    self.0.previous_sibling().map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn next_sibling(&self) -> Option<NodeRepr> {
    self.0.next_sibling().map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn parent_element(&self) -> Option<NodeRepr> {
    self.0.parent().and_then(|n| {
      if n.as_element().is_some() {
        Some(NodeRepr::from(n))
      } else {
        None
      }
    })
  }

  #[napi(getter)]
  pub fn first_element_child(&self) -> Option<NodeRepr> {
    self
      .0
      .children()
      .find(|n| n.as_element().is_some())
      .map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn last_element_child(&self) -> Option<NodeRepr> {
    self
      .0
      .children()
      .rev()
      .find(|n| n.as_element().is_some())
      .map(NodeRepr::from)
  }

  #[napi(getter)]
  pub fn previous_element_sibling(&self) -> Option<NodeRepr> {
    let mut current = self.0.previous_sibling();
    while let Some(node) = current {
      if node.as_element().is_some() {
        return Some(NodeRepr::from(node));
      }
      current = node.previous_sibling();
    }
    None
  }

  #[napi(getter)]
  pub fn next_element_sibling(&self) -> Option<NodeRepr> {
    let mut current = self.0.next_sibling();
    while let Some(node) = current {
      if node.as_element().is_some() {
        return Some(NodeRepr::from(node));
      }
      current = node.next_sibling();
    }
    None
  }

  #[napi(getter)]
  pub fn children(&self) -> Vec<NodeRepr> {
    self
      .0
      .children()
      .filter(|n| n.as_element().is_some())
      .map(NodeRepr::from)
      .collect()
  }

  #[napi(getter)]
  pub fn child_element_count(&self) -> u32 {
    self
      .0
      .children()
      .filter(|n| n.as_element().is_some())
      .count() as u32
  }

  #[napi(js_name = "getRootNode")]
  pub fn get_root_node(&self) -> NodeRepr {
    let mut current = self.0.clone();
    while let Some(parent) = current.parent() {
      current = parent;
    }
    NodeRepr::from(current)
  }

  #[napi(getter)]
  pub fn node_value(&self) -> Option<String> {
    match self.0.data() {
      NodeData::Text(t) => Some(t.borrow().to_string()),
      NodeData::Comment(c) => Some(c.borrow().to_string()),
      _ => None,
    }
  }

  #[napi(setter)]
  pub fn set_node_value(&self, value: Option<String>) {
    if let Some(val) = value {
      match self.0.data() {
        NodeData::Text(t) => {
          *t.borrow_mut() = val.into();
        }
        NodeData::Comment(c) => {
          *c.borrow_mut() = val.into();
        }
        _ => {}
      }
    }
  }

  #[napi(getter)]
  pub fn data(&self) -> Option<String> {
    self.node_value()
  }

  #[napi(setter)]
  pub fn set_data(&self, value: String) {
    self.set_node_value(Some(value));
  }

  #[napi(getter, js_name = "textContent")]
  pub fn text_content_getter(&self) -> String {
    self.text()
  }

  #[napi(setter, js_name = "textContent")]
  pub fn set_text_content(&self, text: String) {
    self.0.children().for_each(|child| child.detach());
    self.0.append(kuchikiki::NodeRef::new_text(text.into()));
  }

  #[napi(js_name = "isSameNode")]
  pub fn is_same_node(&self, other_node: &NodeRepr) -> bool {
    self.0 == other_node.0
  }

  #[napi(getter, js_name = "innerHTML")]
  pub fn inner_html_getter(&self) -> String {
    self.inner_html()
  }

  #[napi(setter, js_name = "innerHTML")]
  pub fn set_inner_html(&self, html: String) {
    self.0.children().for_each(|child| child.detach());
    let parser = parse_html();
    let new_doc = parser.one(html);
    let nodes = if let Ok(body) = new_doc.select_first("body") {
      body.as_node().children()
    } else {
      new_doc.children()
    };
    for child in nodes {
      self.0.append(child);
    }
  }

  #[napi(getter, js_name = "outerHTML")]
  pub fn outer_html_getter(&self) -> String {
    self.outer_html()
  }

  #[napi(setter, js_name = "outerHTML")]
  pub fn set_outer_html(&self, html: String) {
    let parser = parse_html();
    let new_doc = parser.one(html);
    let nodes = if let Ok(body) = new_doc.select_first("body") {
      body.as_node().children()
    } else {
      new_doc.children()
    };
    for child in nodes {
      self.0.insert_before(child);
    }
    self.0.detach();
  }

  #[napi(getter)]
  pub fn owner_document(&self) -> Option<NodeRepr> {
    let mut current = self.0.clone();
    loop {
      if let Some(parent) = current.parent() {
        current = parent;
      } else {
        break;
      }
    }
    if let NodeData::Document(_) = current.data() {
      Some(NodeRepr::from(current))
    } else {
      None
    }
  }

  #[napi(getter, js_name = "childNodes")]
  pub fn child_nodes(&self) -> Vec<NodeRepr> {
    self.0.children().map(Into::into).collect()
  }

  #[napi(getter, js_name = "isConnected")]
  pub fn is_connected(&self) -> bool {
    self.owner_document().is_some()
  }

  #[napi(getter)]
  pub fn doctype(&self) -> Option<NodeRepr> {
    if let NodeData::Document(_) = self.0.data() {
      self
        .0
        .children()
        .find(|n| matches!(n.data(), NodeData::Doctype(_)))
        .map(NodeRepr::from)
    } else {
      None
    }
  }

  #[napi(getter, js_name = "publicId")]
  pub fn public_id(&self) -> String {
    if let NodeData::Doctype(data) = self.0.data() {
      data.public_id.to_string()
    } else {
      "".to_string()
    }
  }

  #[napi(getter, js_name = "systemId")]
  pub fn system_id(&self) -> String {
    if let NodeData::Doctype(data) = self.0.data() {
      data.system_id.to_string()
    } else {
      "".to_string()
    }
  }

  #[napi(getter)]
  pub fn name(&self) -> String {
    if let NodeData::Doctype(data) = self.0.data() {
      data.name.to_string()
    } else {
      "".to_string()
    }
  }

  #[napi(getter)]
  pub fn target(&self) -> String {
    if let NodeData::ProcessingInstruction(data) = self.0.data() {
      data.borrow().0.to_string()
    } else {
      "".to_string()
    }
  }

  // ClassList helpers
  #[napi]
  pub fn class_list_add(&self, class_name: String) {
    if let Some(ele) = self.0.as_element() {
      let mut attributes = ele.attributes.borrow_mut();
      if let Some(class_attr) = attributes.get_mut("class") {
        let mut classes: Vec<&str> = class_attr.split_whitespace().collect();
        if !classes.contains(&class_name.as_str()) {
          classes.push(&class_name);
          *class_attr = classes.join(" ").into();
        }
      } else {
        attributes.insert("class", class_name.into());
      }
    }
  }

  #[napi]
  pub fn class_list_remove(&self, class_name: String) {
    if let Some(ele) = self.0.as_element() {
      let mut attributes = ele.attributes.borrow_mut();
      if let Some(class_attr) = attributes.get_mut("class") {
        let mut classes: Vec<&str> = class_attr.split_whitespace().collect();
        if let Some(pos) = classes.iter().position(|&c| c == class_name) {
          classes.remove(pos);
          *class_attr = classes.join(" ").into();
        }
      }
    }
  }

  #[napi]
  pub fn class_list_toggle(&self, class_name: String, force: Option<bool>) -> bool {
    if let Some(ele) = self.0.as_element() {
      let mut attributes = ele.attributes.borrow_mut();
      let has_class = if let Some(class_attr) = attributes.get("class") {
        class_attr.split_whitespace().any(|c| c == class_name)
      } else {
        false
      };

      let should_add = match force {
        Some(f) => f,
        None => !has_class,
      };

      if should_add {
        if !has_class {
          if let Some(class_attr) = attributes.get_mut("class") {
            let mut classes: Vec<&str> = class_attr.split_whitespace().collect();
            classes.push(&class_name);
            *class_attr = classes.join(" ").into();
          } else {
            attributes.insert("class", class_name.into());
          }
        }
        true
      } else {
        if has_class {
          if let Some(class_attr) = attributes.get_mut("class") {
            let mut classes: Vec<&str> = class_attr.split_whitespace().collect();
            if let Some(pos) = classes.iter().position(|&c| c == class_name) {
              classes.remove(pos);
              *class_attr = classes.join(" ").into();
            }
          }
        }
        false
      }
    } else {
      false
    }
  }

  #[napi]
  pub fn class_list_contains(&self, class_name: String) -> bool {
    if let Some(ele) = self.0.as_element() {
      if let Some(class_attr) = ele.attributes.borrow().get("class") {
        return class_attr.split_whitespace().any(|c| c == class_name);
      }
    }
    false
  }

  // Dataset helpers
  #[napi]
  pub fn dataset_get(&self) -> IndexMap<String, String> {
    let mut map = IndexMap::new();
    if let Some(ele) = self.0.as_element() {
      for (name, attr) in ele.attributes.borrow().map.iter() {
        let local = name.local.to_string();
        if local.starts_with("data-") {
          let key = local[5..].to_string();
          // Convert kebab-case to camelCase
          let camel_key = key
            .split('-')
            .enumerate()
            .map(|(i, part)| {
              if i == 0 {
                part.to_string()
              } else {
                let mut chars = part.chars();
                match chars.next() {
                  Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                  None => "".to_string(),
                }
              }
            })
            .collect::<String>();
          map.insert(camel_key, attr.value.to_string());
        }
      }
    }
    map
  }

  #[napi]
  pub fn dataset_set(&self, key: String, value: String) {
    if let Some(ele) = self.0.as_element() {
      // Convert camelCase to kebab-case
      let kebab_key = key
        .chars()
        .enumerate()
        .map(|(i, c)| {
          if c.is_uppercase() {
            let prefix = if i > 0 { "-" } else { "" };
            format!("{}{}", prefix, c.to_lowercase())
          } else {
            c.to_string()
          }
        })
        .collect::<String>();

      let attr_name = format!("data-{}", kebab_key);
      ele.attributes.borrow_mut().insert(attr_name, value.into());
    }
  }

  #[napi]
  pub fn dataset_remove(&self, key: String) {
    if let Some(ele) = self.0.as_element() {
      // Convert camelCase to kebab-case
      let kebab_key = key
        .chars()
        .enumerate()
        .map(|(i, c)| {
          if c.is_uppercase() {
            let prefix = if i > 0 { "-" } else { "" };
            format!("{}{}", prefix, c.to_lowercase())
          } else {
            c.to_string()
          }
        })
        .collect::<String>();

      let attr_name = format!("data-{}", kebab_key);
      ele.attributes.borrow_mut().remove(attr_name);
    }
  }
}
