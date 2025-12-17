import test from "node:test";
import assert from "node:assert/strict";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import beautify from "js-beautify";

import { parse } from "../domparser.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function formatHtml(html) {
  return beautify
    .html(html, { preserve_newlines: false })
    .replace(/\r\n/g, "\n");
}

test("should sanitise html", () => {
  const $ = parse(
    '<div class="one">first</div><body>Bad body</body><div id="two">second</div>',
  );
  assert.strictEqual(
    $.outerHtml(),
    '<html><head></head><body><div class="one">first</div>Bad body<div id="two">second</div></body></html>',
  );
});

test("should not change the original html structure", () => {
  const jqueryHtml = fs.readFileSync(path.resolve(__dirname, "jquery.html"), {
    encoding: "utf8",
  });
  const $ = parse(jqueryHtml);
  assert.strictEqual(formatHtml($.outerHtml()), formatHtml(jqueryHtml));
});

test("should select node and get attributes with ns correctly", () => {
  const $ = parse(`
<!DOCTYPE html>
<html>
<head>
  <title>HTML and SVG Namespace Example</title>
</head>
<body>
  <h1>This is an HTML Heading</h1>
  <p>This is an HTML paragraph.</p>
  <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
  </svg>
</body>
</html>
`);
  assert.deepStrictEqual($.select("svg").getAttributes(), {
    xmlns: "http://www.w3.org/2000/svg",
    width: "100",
    height: "100",
  });

  assert.strictEqual($.select("svg").getAttribute("width"), "100");
});

test("should select first correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first<div id="two">second</div></div></body></html>',
  );

  assert.strictEqual(
    $.select(".one").select("#two").outerHtml(),
    '<div id="two">second</div>',
  );
});

test("should select all correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div><div id="two">second</div></body></html>',
  );

  assert.deepStrictEqual(
    $.selectAll("div").map((e) => e.outerHtml()),
    ['<div class="one">first</div>', '<div id="two">second</div>'],
  );

  assert.deepStrictEqual(
    $.selectAll("body>*").map((e) => e.outerHtml()),
    ['<div class="one">first</div>', '<div id="two">second</div>'],
  );
});

test("should get all childs correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div><div id="two">second</div></body></html>',
  );

  assert.deepStrictEqual(
    $.select("body")
      .getChildren()
      .map((e) => e.outerHtml()),
    ['<div class="one">first</div>', '<div id="two">second</div>'],
  );
});

test("should get text correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div><div id="two">second</div></body></html>',
  );

  assert.deepStrictEqual(
    $.select("body").text()["firstsecond"],
    undefined,
  );
});

test("should append child correctly", () => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").append($2.select("#two"));

  assert.strictEqual(
    $1.select(".one").outerHtml(),
    '<div class="one">first<div id="two">second</div></div>',
  );
});

test("should prepend child correctly", () => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").prepend($2.select("#two"));

  assert.strictEqual(
    $1.select(".one").outerHtml(),
    '<div class="one"><div id="two">second</div>first</div>',
  );
});

test("should insert child after correctly", () => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div><div>three</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").after($2.select("#two"));

  assert.strictEqual(
    $1.select("body").outerHtml(),
    '<body><div class="one">first</div><div id="two">second</div><div>three</div></body>',
  );
});

test("should insert child before correctly", () => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div><div>three</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").before($2.select("#two"));

  assert.strictEqual(
    $1.select("body").outerHtml(),
    '<body><div id="two">second</div><div class="one">first</div><div>three</div></body>',
  );
});

test("should remove node correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first<div>second</div></div><div>three</div></body></html>',
  );

  $.select(".one").remove();

  assert.strictEqual(
    $.select("body").outerHtml(),
    "<body><div>three</div></body>",
  );
});

test("should set attribute correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );

  $.select(".one").setAttribute("id", "Hello");

  assert.deepStrictEqual($.select(".one").getAttribute("id"), "Hello");
});

test("should remove attribute correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one" id="Hello">first</div></body></html>',
  );

  $.select(".one").removeAttribute("class");

  assert.deepStrictEqual($.select("#Hello").getAttributes(), {
    id: "Hello",
  });
});

test("should clone correctly", () => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div><div id="two">second</div></body></html>',
  );

  assert.strictEqual(
    $.select(".one").clone().outerHtml(),
    '<div class="one"></div>',
  );
  assert.strictEqual(
    $.select(".one").cloneRecursive().outerHtml(),
    '<div class="one">first</div>',
  );

  const $cloned = $.select(".one").cloneRecursive();
  $cloned.select(".one").getChildren()[0].remove();
  assert.strictEqual(
    $cloned.select(".one").outerHtml(),
    '<div class="one"></div>',
  );
  assert.strictEqual(
    $.select(".one").outerHtml(),
    '<div class="one">first</div>',
  );
});
