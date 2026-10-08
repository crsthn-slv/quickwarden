import { test } from "node:test";
import assert from "node:assert/strict";
import { searchItems, filterFolder } from "./search.js";

const items = [
  { name: "ID Apple", subtitle: "user@icloud.com" },
  { name: "Amazon" },
  { name: "Gmail", subtitle: "eu@gmail.com", folder: "Work" },
  { name: "Ação Social" },
];
const names = (q, list = items) => searchItems(list, q).map((i) => i.name);

test("term in the middle of the name", () => assert.deepEqual(names("apple"), ["ID Apple"]));
test("all terms", () => {
  assert.deepEqual(names("id apple"), ["ID Apple"]);
  assert.deepEqual(names("apple amazon"), []);
});
test("case", () => assert.deepEqual(names("APPLE"), ["ID Apple"]));
test("accents", () => assert.deepEqual(names("acao"), ["Ação Social"]));
test("subtitle", () => assert.deepEqual(names("icloud"), ["ID Apple"]));
test("no match", () => assert.deepEqual(names("zzz"), []));
test("name prefix before word prefix", () => {
  const r = names("a");
  assert.ok(r.indexOf("Amazon") < r.indexOf("ID Apple"));
});
test("empty query returns everything", () => {
  assert.equal(names("").length, items.length);
  assert.equal(names("   ").length, items.length);
});
test("favorites first with no query", () => {
  const f = [{ name: "Amazon" }, { name: "Zoom", favorite: true }, { name: "Apple" }, { name: "Banco", favorite: true }];
  assert.deepEqual(names("", f), ["Banco", "Zoom", "Amazon", "Apple"]);
});
test("with a query, score wins", () => {
  assert.deepEqual(names("am", [{ name: "Zoom", favorite: true }, { name: "Amazon" }]), ["Amazon"]);
});
test("folder filter", () => {
  assert.deepEqual(filterFolder(items, "Work").map((i) => i.name), ["Gmail"]);
  assert.equal(filterFolder(items, null).length, items.length);
});
