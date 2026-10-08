import { test } from "node:test";
import assert from "node:assert/strict";
import { searchItems, filterFolder } from "./search.js";

const items = [
  { name: "ID Apple", subtitle: "user@icloud.com" },
  { name: "Amazon" },
  { name: "Gmail", subtitle: "eu@gmail.com", folder: "Trabalho" },
  { name: "Ação Social" },
];
const names = (q, list = items) => searchItems(list, q).map((i) => i.name);

test("termo no meio do nome", () => assert.deepEqual(names("apple"), ["ID Apple"]));
test("todos os termos", () => {
  assert.deepEqual(names("id apple"), ["ID Apple"]);
  assert.deepEqual(names("apple amazon"), []);
});
test("maiúsculas", () => assert.deepEqual(names("APPLE"), ["ID Apple"]));
test("acentos", () => assert.deepEqual(names("acao"), ["Ação Social"]));
test("subtítulo", () => assert.deepEqual(names("icloud"), ["ID Apple"]));
test("sem correspondência", () => assert.deepEqual(names("zzz"), []));
test("prefixo do nome antes de prefixo de palavra", () => {
  const r = names("a");
  assert.ok(r.indexOf("Amazon") < r.indexOf("ID Apple"));
});
test("pesquisa vazia devolve tudo", () => {
  assert.equal(names("").length, items.length);
  assert.equal(names("   ").length, items.length);
});
test("favoritos primeiro sem pesquisa", () => {
  const f = [{ name: "Amazon" }, { name: "Zoom", favorite: true }, { name: "Apple" }, { name: "Banco", favorite: true }];
  assert.deepEqual(names("", f), ["Banco", "Zoom", "Amazon", "Apple"]);
});
test("com pesquisa manda a pontuação", () => {
  assert.deepEqual(names("am", [{ name: "Zoom", favorite: true }, { name: "Amazon" }]), ["Amazon"]);
});
test("filtro por colecção", () => {
  assert.deepEqual(filterFolder(items, "Trabalho").map((i) => i.name), ["Gmail"]);
  assert.equal(filterFolder(items, null).length, items.length);
});
