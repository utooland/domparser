
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('should support classList', () => {
  const doc = new DOMParser().parseFromString('<div class="foo bar"></div>', 'text/html');
  const div = doc.querySelector('div');
  
  assert.strictEqual(div.classList.contains('foo'), true);
  assert.strictEqual(div.classList.contains('baz'), false);
  
  div.classList.add('baz');
  assert.strictEqual(div.className, 'foo bar baz');
  
  div.classList.remove('bar');
  assert.strictEqual(div.className, 'foo baz');
  
  div.classList.toggle('foo');
  assert.strictEqual(div.className, 'baz');
  
  div.classList.toggle('foo');
  assert.strictEqual(div.className, 'baz foo');
  
  div.classList.toggle('foo', false);
  assert.strictEqual(div.className, 'baz');
  
  div.classList.toggle('foo', true);
  assert.strictEqual(div.className, 'baz foo');
});

test('should support dataset', () => {
  const doc = new DOMParser().parseFromString('<div data-foo="bar" data-hello-world="123"></div>', 'text/html');
  const div = doc.querySelector('div');
  
  assert.strictEqual(div.dataset.foo, 'bar');
  assert.strictEqual(div.dataset.helloWorld, '123');
  
  div.dataset.foo = 'baz';
  assert.strictEqual(div.getAttribute('data-foo'), 'baz');
  
  div.dataset.newProp = 'test';
  assert.strictEqual(div.getAttribute('data-new-prop'), 'test');
  
  delete div.dataset.helloWorld;
  assert.strictEqual(div.hasAttribute('data-hello-world'), false);
});

test('should support ProcessingInstruction', () => {
  const doc = new DOMParser().parseFromString('<div></div>', 'text/html');
  const pi = doc.createProcessingInstruction('xml-stylesheet', 'href="style.css"');
  
  assert.strictEqual(pi.nodeName, 'xml-stylesheet');
  assert.strictEqual(pi.target, 'xml-stylesheet');
});

test('should support Doctype properties', () => {
    const doc = new DOMParser().parseFromString('<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"><html></html>', 'text/html');
    const doctype = doc.doctype;
    
    assert.notStrictEqual(doctype, null);
    assert.strictEqual(doctype.name, 'html');
    assert.strictEqual(doctype.publicId, '-//W3C//DTD XHTML 1.0 Transitional//EN');
    assert.strictEqual(doctype.systemId, 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd');
});
