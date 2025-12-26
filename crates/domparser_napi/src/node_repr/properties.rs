use super::NodeRepr;
use std::collections::HashMap;

#[napi]
impl NodeRepr {
  /// Returns an integer representing the type of the node.
  #[napi(getter)]
  pub fn node_type(&self) -> i32 {
    self.0.node_type()
  }

  /// Returns a string containing the name of the Node.
  #[napi(getter)]
  pub fn node_name(&self) -> String {
    self.0.node_name()
  }

  /// Returns the name of the element.
  #[napi(getter)]
  pub fn tag_name(&self) -> Option<String> {
    self.0.tag_name()
  }

  /// Returns the namespace URI of the element, or null if the element is not in a namespace.
  #[napi(getter, js_name = "namespaceURI")]
  pub fn namespace_uri(&self) -> Option<String> {
    self.0.namespace_uri()
  }

  /// Returns the namespace prefix of the specified element, or null if no prefix is specified.
  #[napi(getter)]
  pub fn prefix(&self) -> Option<String> {
    self.0.prefix()
  }

  /// Returns the local part of the qualified name of an element.
  #[napi(getter)]
  pub fn local_name(&self) -> Option<String> {
    self.0.local_name()
  }

  /// Returns the value of the id attribute of the element.
  #[napi(getter)]
  pub fn id(&self) -> String {
    self.0.id()
  }

  /// Sets the value of the id attribute of the element.
  #[napi(setter)]
  pub fn set_id(&self, id: String) {
    self.0.set_id(id);
  }

  /// Returns the value of the class attribute of the element.
  #[napi(getter)]
  pub fn class_name(&self) -> String {
    self.0.class_name()
  }

  /// Sets the value of the class attribute of the element.
  #[napi(setter)]
  pub fn set_class_name(&self, class_name: String) {
    self.0.set_class_name(class_name);
  }

  /// Returns the parent of the specified node in the DOM tree.
  #[napi(getter)]
  pub fn parent_node(&self) -> Option<NodeRepr> {
    self.0.parent_node().map(NodeRepr)
  }

  /// Returns the first child of the node.
  #[napi(getter)]
  pub fn first_child(&self) -> Option<NodeRepr> {
    self.0.first_child().map(NodeRepr)
  }

  /// Returns the last child of the node.
  #[napi(getter)]
  pub fn last_child(&self) -> Option<NodeRepr> {
    self.0.last_child().map(NodeRepr)
  }

  /// Returns the node immediately preceding the specified one in its parent's childNodes list.
  #[napi(getter)]
  pub fn previous_sibling(&self) -> Option<NodeRepr> {
    self.0.previous_sibling().map(NodeRepr)
  }

  /// Returns the node immediately following the specified one in its parent's childNodes list.
  #[napi(getter)]
  pub fn next_sibling(&self) -> Option<NodeRepr> {
    self.0.next_sibling().map(NodeRepr)
  }

  /// Returns the DOM node's parent Element, or null if the node either has no parent, or its parent isn't a DOM Element.
  #[napi(getter)]
  pub fn parent_element(&self) -> Option<NodeRepr> {
    self.0.parent_element().map(NodeRepr)
  }

  /// Returns the first child that is an element, or null if there is none.
  #[napi(getter)]
  pub fn first_element_child(&self) -> Option<NodeRepr> {
    self.0.first_element_child().map(NodeRepr)
  }

  /// Returns the last child that is an element, or null if there is none.
  #[napi(getter)]
  pub fn last_element_child(&self) -> Option<NodeRepr> {
    self.0.last_element_child().map(NodeRepr)
  }

  /// Returns the Element immediately prior to the specified one in its parent's children list, or null if the specified element is the first one in the list.
  #[napi(getter)]
  pub fn previous_element_sibling(&self) -> Option<NodeRepr> {
    self.0.previous_element_sibling().map(NodeRepr)
  }

  /// Returns the Element immediately following the specified one in its parent's children list, or null if the specified element is the last one in the list.
  #[napi(getter)]
  pub fn next_element_sibling(&self) -> Option<NodeRepr> {
    self.0.next_element_sibling().map(NodeRepr)
  }

  /// Returns a live HTMLCollection which contains all of the child elements of the node upon which it was called.
  #[napi(getter)]
  pub fn children(&self) -> Vec<NodeRepr> {
    self.0.children().into_iter().map(NodeRepr).collect()
  }

  /// Returns the number of child elements of the given element.
  #[napi(getter)]
  pub fn child_element_count(&self) -> u32 {
    self.0.child_element_count()
  }

  /// Returns the context object's root.
  #[napi(js_name = "getRootNode")]
  pub fn get_root_node(&self) -> NodeRepr {
    NodeRepr(self.0.get_root_node())
  }

  /// Returns or sets the value of the current node.
  #[napi(getter)]
  pub fn node_value(&self) -> Option<String> {
    self.0.node_value()
  }

  /// Returns or sets the value of the current node.
  #[napi(setter)]
  pub fn set_node_value(&self, value: Option<String>) {
    self.0.set_node_value(value);
  }

  /// Returns the target of the processing instruction.
  #[napi(getter)]
  pub fn target(&self) -> Option<String> {
    self.0.target()
  }

  /// Returns the name of the attribute.
  #[napi(getter)]
  pub fn name(&self) -> Option<String> {
    self.0.name()
  }

  /// Returns the public identifier of the document type.
  #[napi(getter)]
  pub fn public_id(&self) -> Option<String> {
    self.0.public_id()
  }

  /// Returns the system identifier of the document type.
  #[napi(getter)]
  pub fn system_id(&self) -> Option<String> {
    self.0.system_id()
  }

  /// Returns the Document Type Declaration (DTD) associated with current document.
  #[napi(getter)]
  pub fn doctype(&self) -> Option<NodeRepr> {
    self.0.doctype().map(NodeRepr)
  }

  /// Returns the character data of the node.
  #[napi(getter)]
  pub fn data(&self) -> Option<String> {
    self.0.data()
  }

  /// Sets the character data of the node.
  #[napi(setter)]
  pub fn set_data(&self, value: String) {
    self.0.set_data(value);
  }

  /// Returns the text content of the node and its descendants.
  #[napi(getter, js_name = "textContent")]
  pub fn text_content_getter(&self) -> String {
    self.0.text_content_getter()
  }

  /// Sets the text content of the node and its descendants.
  #[napi(setter, js_name = "textContent")]
  pub fn set_text_content(&self, text: String) {
    self.0.set_text_content(text);
  }

  /// Returns a boolean value indicating whether the two nodes are the same (that is, they reference the same object).
  #[napi(js_name = "isSameNode")]
  pub fn is_same_node(&self, other_node: &NodeRepr) -> bool {
    self.0.is_same_node(&other_node.0)
  }

  /// Returns the HTML serialization of the element's descendants.
  #[napi(getter, js_name = "innerHTML")]
  pub fn inner_html_getter(&self) -> String {
    self.0.inner_html_getter()
  }

  /// Returns the number of characters in the data.
  #[napi(getter)]
  pub fn length(&self) -> u32 {
    self.0.length()
  }

  /// Sets the HTML serialization of the element's descendants.
  #[napi(setter, js_name = "innerHTML")]
  pub fn set_inner_html(&self, html: String) {
    self.0.set_inner_html(html);
  }

  /// Returns the HTML serialization of the element and its descendants.
  #[napi(getter, js_name = "outerHTML")]
  pub fn outer_html_getter(&self) -> String {
    self.0.outer_html_getter()
  }

  /// Sets the HTML serialization of the element and its descendants.
  #[napi(setter, js_name = "outerHTML")]
  pub fn set_outer_html(&self, html: String) {
    self.0.set_outer_html(html);
  }

  /// Returns the top-level document object for this node.
  #[napi(getter)]
  pub fn owner_document(&self) -> Option<NodeRepr> {
    self.0.owner_document().map(NodeRepr)
  }

  /// Returns a string representation of the object.
  #[napi(js_name = "toString")]
  pub fn to_string_js(&self) -> String {
    self.0.to_string_js()
  }

  /// Returns a live NodeList containing all the children of this node.
  #[napi(getter)]
  pub fn child_nodes(&self) -> Vec<NodeRepr> {
    self.0.child_nodes().into_iter().map(NodeRepr).collect()
  }

  #[napi(js_name = "_classListAdd")]
  pub fn _class_list_add(&self, token: String) {
    self.0._class_list_add(token);
  }

  #[napi(js_name = "_classListRemove")]
  pub fn _class_list_remove(&self, token: String) {
    self.0._class_list_remove(token);
  }

  #[napi(js_name = "_classListToggle")]
  pub fn _class_list_toggle(&self, token: String, force: Option<bool>) -> bool {
    self.0._class_list_toggle(token, force)
  }

  #[napi(js_name = "_classListContains")]
  pub fn _class_list_contains(&self, token: String) -> bool {
    self.0._class_list_contains(token)
  }

  #[napi(js_name = "_datasetGet")]
  pub fn _dataset_get(&self) -> HashMap<String, String> {
    self.0._dataset_get()
  }

  #[napi(js_name = "_datasetSet")]
  pub fn _dataset_set(&self, key: String, value: String) {
    self.0._dataset_set(key, value);
  }

  #[napi(js_name = "_datasetRemove")]
  pub fn _dataset_remove(&self, key: String) {
    self.0._dataset_remove(key);
  }

  /// Returns a string containing the part of CharacterData.data of the specified length and starting at the specified offset.
  #[napi(js_name = "substringData")]
  pub fn substring_data(&self, offset: u32, count: u32) -> String {
    self.0.substring_data(offset, count)
  }

  /// Appends the given string to the CharacterData.data string; when this method returns, data contains the concatenated string.
  #[napi(js_name = "appendData")]
  pub fn append_data(&self, data: String) {
    self.0.append_data(data);
  }

  /// Inserts the specified string at the specified offset.
  #[napi(js_name = "insertData")]
  pub fn insert_data(&self, offset: u32, data: String) {
    self.0.insert_data(offset, data);
  }

  /// Removes the specified amount of characters, starting at the specified offset.
  #[napi(js_name = "deleteData")]
  pub fn delete_data(&self, offset: u32, count: u32) {
    self.0.delete_data(offset, count);
  }

  /// Replaces the specified amount of characters, starting at the specified offset, with the specified string.
  #[napi(js_name = "replaceData")]
  pub fn replace_data(&self, offset: u32, count: u32, data: String) {
    self.0.replace_data(offset, count, data);
  }

  /// Breaks the Text node into two nodes at the specified offset, keeping both in the tree as siblings.
  #[napi(js_name = "splitText")]
  pub fn split_text(&self, offset: u32) -> Option<NodeRepr> {
    self.0.split_text(offset).map(NodeRepr)
  }

  /// Parses the specified text as HTML or XML and inserts the resulting nodes into the DOM tree at a specified position.
  #[napi(js_name = "insertAdjacentHTML")]
  pub fn insert_adjacent_html(&self, position: String, html: String) {
    self.0.insert_adjacent_html(position, html);
  }

  /// Inserts a given text node at a given position relative to the element it is invoked upon.
  #[napi(js_name = "insertAdjacentText")]
  pub fn insert_adjacent_text(&self, position: String, text: String) {
    self.0.insert_adjacent_text(position, text);
  }

  /// Inserts a given element node at a given position relative to the element it is invoked upon.
  #[napi(js_name = "insertAdjacentElement")]
  pub fn insert_adjacent_element(&self, position: String, element: &NodeRepr) {
    self.0.insert_adjacent_element(position, &element.0);
  }

  /// Puts the specified node and all of its subtree into a "normalized" form.
  #[napi]
  pub fn normalize(&self) {
    self.0.normalize();
  }

  /// Returns the namespace URI associated with the given prefix.
  #[napi(js_name = "lookupNamespaceURI")]
  pub fn lookup_namespace_uri(&self, prefix: Option<String>) -> Option<String> {
    self.0.lookup_namespace_uri(prefix)
  }

  /// Returns the prefix for a given namespace URI, if present, and null if not.
  #[napi(js_name = "lookupPrefix")]
  pub fn lookup_prefix(&self, namespace: String) -> Option<String> {
    self.0.lookup_prefix(namespace)
  }

  /// Compares the position of the current node against another node in any other document.
  #[napi(js_name = "compareDocumentPosition")]
  pub fn compare_document_position(&self, other: &NodeRepr) -> u32 {
    self.0.compare_document_position(&other.0)
  }
}
