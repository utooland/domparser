
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('should support namespace properties', () => {
  const doc = new DOMParser().parseFromString('<html><body><svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0h10v10H0z"/></svg></body></html>', 'text/html');
  const svg = doc.querySelector('svg');
  const path = doc.querySelector('path');

  assert.strictEqual(svg.namespaceURI, 'http://www.w3.org/2000/svg');
  assert.strictEqual(svg.localName, 'svg');
  assert.strictEqual(path.namespaceURI, 'http://www.w3.org/2000/svg');
  
  const div = doc.createElement('div');
  assert.strictEqual(div.namespaceURI, 'http://www.w3.org/1999/xhtml');
});

test('should support hasChildNodes', () => {
  const doc = new DOMParser().parseFromString('<div><span></span></div>', 'text/html');
  const div = doc.querySelector('div');
  const span = doc.querySelector('span');

  assert.strictEqual(div.hasChildNodes(), true);
  assert.strictEqual(span.hasChildNodes(), false);
});

test('should support compareDocumentPosition', () => {
  const doc = new DOMParser().parseFromString('<div id="parent"><div id="child1"></div><div id="child2"></div></div>', 'text/html');
  const parent = doc.getElementById('parent');
  const child1 = doc.getElementById('child1');
  const child2 = doc.getElementById('child2');

  // parent.compareDocumentPosition(child1)
  // child1 is contained by parent (16) + child1 follows parent (4) = 20
  assert.strictEqual(parent.compareDocumentPosition(child1), 20);
  
  // child1.compareDocumentPosition(parent)
  // parent contains child1 (8) + parent precedes child1 (2) = 10
  assert.strictEqual(child1.compareDocumentPosition(parent), 10);

  // child1.compareDocumentPosition(child2)
  // child2 follows child1 (4)
  assert.strictEqual(child1.compareDocumentPosition(child2), 4);
  
  // child2.compareDocumentPosition(child1)
  // child1 precedes child2 (2)
  assert.strictEqual(child2.compareDocumentPosition(child1), 2);
  
  // Same node
  assert.strictEqual(parent.compareDocumentPosition(parent), 0);
  
  // Disconnected
  const otherDoc = new DOMParser().parseFromString('<div></div>', 'text/html');
  const otherDiv = otherDoc.querySelector('div');
  // 1 (disconnected) + 32 (implementation specific) = 33
  assert.strictEqual(parent.compareDocumentPosition(otherDiv) & 1, 1);
});
