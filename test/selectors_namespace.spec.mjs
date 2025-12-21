
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('select and selectAll', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="root">
      <div class="a">
        <span class="b">1</span>
        <span class="b">2</span>
      </div>
      <div class="a">
        <span class="b">3</span>
      </div>
      <div class="c">
        <span class="d">4</span>
      </div>
    </div>
  `, 'text/html');

  const root = doc.getElementById('root');
  
  // select (find first descendant)
  const firstSpan = root.select('.b');
  assert.strictEqual(firstSpan.textContent, '1');

  const firstDivA = root.select('.a');
  assert.strictEqual(firstDivA.select('.b').textContent, '1');

  // selectAll (find all descendants)
  const allSpans = root.selectAll('.b');
  assert.strictEqual(allSpans.length, 3);
  assert.strictEqual(allSpans[0].textContent, '1');
  assert.strictEqual(allSpans[1].textContent, '2');
  assert.strictEqual(allSpans[2].textContent, '3');

  const allDivs = root.selectAll('.a');
  assert.strictEqual(allDivs.length, 2);

  // Nested selection
  const nested = root.select('.c .d');
  assert.strictEqual(nested.textContent, '4');
  
  const nestedAll = root.selectAll('.a .b');
  assert.strictEqual(nestedAll.length, 3);
});

test('matches and closest', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="grandparent" class="ancestor">
      <div id="parent" class="ancestor">
        <div id="child" class="target"></div>
      </div>
    </div>
  `, 'text/html');

  const child = doc.getElementById('child');
  const parent = doc.getElementById('parent');
  const grandparent = doc.getElementById('grandparent');

  // matches
  assert.strictEqual(child.matches('#child'), true);
  assert.strictEqual(child.matches('.target'), true);
  assert.strictEqual(child.matches('div'), true);
  assert.strictEqual(child.matches('#parent'), false);
  assert.strictEqual(child.matches('.ancestor'), false);
  
  // matches with ancestor combinator
  assert.strictEqual(child.matches('#parent #child'), true);
  assert.strictEqual(child.matches('.ancestor .target'), true);
  assert.strictEqual(child.matches('#grandparent #child'), true);
  assert.strictEqual(child.matches('#grandparent #parent #child'), true);

  // closest
  const closestAncestor = child.closest('.ancestor');
  assert.strictEqual(closestAncestor.id, 'parent');

  const closestDiv = child.closest('div');
  assert.strictEqual(closestDiv.id, 'child');

  const closestGrandparent = child.closest('#grandparent');
  assert.strictEqual(closestGrandparent.id, 'grandparent');

  const noMatch = child.closest('.non-existent');
  assert.strictEqual(noMatch, null);
  
  // closest with complex selector
  const closestComplex = child.closest('#grandparent .ancestor');
  assert.strictEqual(closestComplex.id, 'parent');
});

test('isDefaultNamespace', () => {
  const doc = new DOMParser().parseFromString(`
    <root xmlns="http://example.com/ns">
      <child xmlns="">
        <grandchild />
      </child>
    </root>
  `, 'text/html');

  const root = doc.querySelector('root');
  const child = doc.querySelector('child');
  const grandchild = doc.querySelector('grandchild');

  assert.strictEqual(root.isDefaultNamespace('http://example.com/ns'), true);
  assert.strictEqual(root.isDefaultNamespace('http://other.com'), false);

  assert.strictEqual(child.isDefaultNamespace(''), true);
  assert.strictEqual(child.isDefaultNamespace('http://example.com/ns'), false);

  // grandchild inherits from child (empty namespace)
  assert.strictEqual(grandchild.isDefaultNamespace(''), true);
});
