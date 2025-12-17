
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('should support insertAdjacentHTML', () => {
  const doc = new DOMParser().parseFromString('<div id="target">Target</div>', 'text/html');
  const target = doc.getElementById('target');

  target.insertAdjacentHTML('beforebegin', '<p>Before Begin</p>');
  target.insertAdjacentHTML('afterbegin', '<span>After Begin</span>');
  target.insertAdjacentHTML('beforeend', '<span>Before End</span>');
  target.insertAdjacentHTML('afterend', '<p>After End</p>');
  assert.strictEqual(
    doc.body.innerHTML,
    '<p>Before Begin</p><div id="target"><span>After Begin</span>Target<span>Before End</span></div><p>After End</p>',
  );
});

test('should support insertAdjacentText', () => {
  const doc = new DOMParser().parseFromString('<div id="target">Target</div>', 'text/html');
  const target = doc.getElementById('target');

  target.insertAdjacentText('beforebegin', 'Before Begin');
  target.insertAdjacentText('afterbegin', 'After Begin');
  target.insertAdjacentText('beforeend', 'Before End');
  target.insertAdjacentText('afterend', 'After End');
  assert.strictEqual(
    doc.body.innerHTML,
    'Before Begin<div id="target">After BeginTargetBefore End</div>After End',
  );
});

test('should support insertAdjacentElement', () => {
  const doc = new DOMParser().parseFromString('<div id="target">Target</div>', 'text/html');
  const target = doc.getElementById('target');
  
  const el1 = doc.createElement('span');
  el1.textContent = 'Before Begin';
  target.insertAdjacentElement('beforebegin', el1);

  const el2 = doc.createElement('span');
  el2.textContent = 'After Begin';
  target.insertAdjacentElement('afterbegin', el2);

  const el3 = doc.createElement('span');
  el3.textContent = 'Before End';
  target.insertAdjacentElement('beforeend', el3);

  const el4 = doc.createElement('span');
  el4.textContent = 'After End';
  target.insertAdjacentElement('afterend', el4);
  assert.strictEqual(
    doc.body.innerHTML,
    '<span>Before Begin</span><div id="target"><span>After Begin</span>Target<span>Before End</span></div><span>After End</span>',
  );
});

test('should support normalize', () => {
  const doc = new DOMParser().parseFromString('<div></div>', 'text/html');
  const div = doc.querySelector('div');

  const text1 = doc.createTextNode('Hello ');
  const text2 = doc.createTextNode('World');
  const text3 = doc.createTextNode(''); // Empty text node

  div.appendChild(text1);
  div.appendChild(text2);
  div.appendChild(text3);
  assert.strictEqual(div.childNodes.length, 3);
  
  div.normalize();
  assert.strictEqual(div.childNodes.length, 1);
  assert.strictEqual(div.textContent, 'Hello World');
});
