use super::NodeRepr;

#[napi]
impl NodeRepr {
  /// Inserts a set of Node objects or DOMString objects after the last child of the Element.
  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    self.0.append(&new_child.0);
  }

  /// Adds a node to the end of the list of children of a specified parent node.
  #[napi(js_name = "appendChild")]
  pub fn append_child(&self, new_child: &NodeRepr) -> NodeRepr {
    NodeRepr(self.0.append_child(&new_child.0))
  }

  /// Removes a child node from the DOM and returns the removed node.
  #[napi(js_name = "removeChild")]
  pub fn remove_child(&self, child: &NodeRepr) -> napi::Result<NodeRepr> {
    self
      .0
      .remove_child(&child.0)
      .map(NodeRepr)
      .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
  }

  /// Inserts a set of Node objects or DOMString objects before the first child of the Element.
  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    self.0.prepend(&new_child.0);
  }

  /// Inserts a set of Node or DOMString objects in the children list of this Element's parent, just after this Element.
  #[napi(js_name = "after")]
  pub fn after(&self, new_sibling: &NodeRepr) {
    self.0.after(&new_sibling.0);
  }

  /// Inserts a set of Node or DOMString objects in the children list of this Element's parent, just before this Element.
  #[napi(js_name = "before")]
  pub fn before(&self, new_sibling: &NodeRepr) {
    self.0.before(&new_sibling.0);
  }

  /// Inserts a node before a reference node as a child of a specified parent node.
  #[napi(js_name = "insertBefore")]
  pub fn insert_before_node(
    &self,
    new_node: &NodeRepr,
    ref_node: Option<&NodeRepr>,
  ) -> napi::Result<NodeRepr> {
    self
      .0
      .insert_before_node(&new_node.0, ref_node.map(|n| &n.0))
      .map(NodeRepr)
      .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
  }

  /// Removes the object from the tree it belongs to.
  #[napi]
  pub fn remove(&self) {
    self.0.remove();
  }

  /// Sets the value of an attribute on the specified element.
  #[napi]
  pub fn set_attribute(&self, name: String, value: String) {
    self.0.set_attribute(name, value);
  }

  /// Removes an attribute from the specified element.
  #[napi]
  pub fn remove_attribute(&self, name: String) {
    self.0.remove_attribute(name);
  }

  /// Toggles a boolean attribute (removing it if it is present and adding it if it is not present) on the given element.
  #[napi(js_name = "toggleAttribute")]
  pub fn toggle_attribute(&self, name: String, force: Option<bool>) -> bool {
    self.0.toggle_attribute(name, force)
  }

  /// Sets the value of an attribute on the specified element.
  #[napi(js_name = "setAttributeNS")]
  pub fn set_attribute_ns(&self, namespace: Option<String>, name: String, value: String) {
    self.0.set_attribute_ns(namespace, name, value);
  }

  /// Removes an attribute from the specified element.
  #[napi(js_name = "removeAttributeNS")]
  pub fn remove_attribute_ns(&self, namespace: Option<String>, local_name: String) {
    self.0.remove_attribute_ns(namespace, local_name);
  }

  /// Creates the HTML element specified by tagName.
  #[napi(js_name = "createElement")]
  pub fn create_element(&self, tag_name: String) -> NodeRepr {
    NodeRepr(self.0.create_element(tag_name))
  }

  /// Creates a new Text node.
  #[napi(js_name = "createTextNode")]
  pub fn create_text_node(&self, data: String) -> NodeRepr {
    NodeRepr(self.0.create_text_node(data))
  }

  /// Creates a new Comment node.
  #[napi(js_name = "createComment")]
  pub fn create_comment(&self, data: String) -> NodeRepr {
    NodeRepr(self.0.create_comment(data))
  }

  /// Creates a new empty DocumentFragment.
  #[napi(js_name = "createDocumentFragment")]
  pub fn create_document_fragment(&self) -> NodeRepr {
    NodeRepr(self.0.create_document_fragment())
  }

  #[napi(js_name = "createProcessingInstruction")]
  pub fn create_processing_instruction(&self, target: String, data: String) -> NodeRepr {
    NodeRepr(self.0.create_processing_instruction(target, data))
  }

  #[napi(js_name = "importNode")]
  pub fn import_node(&self, external_node: &NodeRepr, deep: Option<bool>) -> NodeRepr {
    NodeRepr(self.0.import_node(&external_node.0, deep))
  }

  #[napi(js_name = "adoptNode")]
  pub fn adopt_node(&self, external_node: &NodeRepr) -> NodeRepr {
    NodeRepr(self.0.adopt_node(&external_node.0))
  }

  #[napi(js_name = "replaceChild")]
  pub fn replace_child(
    &self,
    new_child: &NodeRepr,
    old_child: &NodeRepr,
  ) -> napi::Result<NodeRepr> {
    self
      .0
      .replace_child(&new_child.0, &old_child.0)
      .map(NodeRepr)
      .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
  }

  #[napi(js_name = "replaceWith")]
  pub fn replace_with(&self, new_node: &NodeRepr) {
    self.0.replace_with(&new_node.0);
  }
}
