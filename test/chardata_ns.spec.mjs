
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('should support CharacterData methods on Text nodes', () => {
  const doc = new DOMParser().parseFromString('<div>Hello World</div>', 'text/html');
  const text = doc.querySelector('div').firstChild;

  // substringData
  assert.strictEqual(text.substringData(0, 5), 'Hello');
  assert.strictEqual(text.substringData(6, 5), 'World');

  // appendData
  text.appendData('!');
  assert.strictEqual(text.data, 'Hello World!');

  // insertData
  text.insertData(5, ',');
  assert.strictEqual(text.data, 'Hello, World!');

  // deleteData
  text.deleteData(5, 1);
  assert.strictEqual(text.data, 'Hello World!');

  // replaceData
  text.replaceData(6, 5, 'Universe');
  assert.strictEqual(text.data, 'Hello Universe!');
});

test('should support splitText', () => {
  const doc = new DOMParser().parseFromString('<div>Hello World</div>', 'text/html');
  const div = doc.querySelector('div');
  const text = div.firstChild;

  const newText = text.splitText(6);
  
  assert.strictEqual(text.data, 'Hello ');
  assert.strictEqual(newText.data, 'World');
  assert.strictEqual(div.childNodes.length, 2);
  assert.strictEqual(div.childNodes[0].isSameNode(text), true);
  assert.strictEqual(div.childNodes[1].isSameNode(newText), true);
});

test('should support AttributeNS methods', () => {
  const doc = new DOMParser().parseFromString('<svg xmlns="http://www.w3.org/2000/svg"></svg>', 'text/html');
  const svg = doc.querySelector('svg');
  const ns = 'http://www.w3.org/1999/xlink';

  svg.setAttributeNS(ns, 'xlink:href', 'http://example.com');
  
  assert.strictEqual(svg.hasAttributeNS(ns, 'href'), true);
  assert.strictEqual(svg.getAttributeNS(ns, 'href'), 'http://example.com');
  
  svg.removeAttributeNS(ns, 'href');
  assert.strictEqual(svg.hasAttributeNS(ns, 'href'), false);
});

test('should support importNode and adoptNode', () => {
  const doc1 = new DOMParser().parseFromString('<div>Doc1</div>', 'text/html');
  const doc2 = new DOMParser().parseFromString('<div>Doc2</div>', 'text/html');
  const node = doc2.querySelector('div');

  // importNode
  const imported = doc1.importNode(node, true);
  assert.strictEqual(imported.outerHTML, '<div>Doc2</div>');
  assert.notStrictEqual(imported.isSameNode(node), true);
  // In our implementation, ownerDocument is dynamic, but imported node is detached initially
  assert.strictEqual(imported.parentNode, null);

  // adoptNode
  const adopted = doc1.adoptNode(node);
  assert.strictEqual(adopted.isSameNode(node), true);
  assert.strictEqual(node.parentNode, null); // Should be detached from doc2
});
