// Quick Access search scoring (port of quick-access-search.ts).
// Fold accents and case, and require that *all* terms match.

// Item: { name, subtitle?, folder?, favorite? }

// With no query, favorites first (only if rbw provides the field), then alphabetical.
function favoritesFirst(items) {
  return [...items].sort((a, b) => {
    if (!!a.favorite !== !!b.favorite) return a.favorite ? -1 : 1;
    return fold(a.name).localeCompare(fold(b.name));
  });
}

export function fold(value) {
  return value.normalize("NFD").replace(/\p{Diacritic}/gu, "").toLowerCase();
}

function scoreTerm(term, name, haystack) {
  if (name.startsWith(term)) return 4;
  if (name.split(" ").some((w) => w.startsWith(term))) return 3;
  if (name.includes(term)) return 2;
  if (haystack.includes(term)) return 1;
  return 0;
}

export function searchItems(items, query) {
  const terms = fold(query).split(" ").filter((t) => t.length > 0);
  if (terms.length === 0) return favoritesFirst(items);

  const scored = [];
  for (const item of items) {
    const name = fold(item.name ?? "");
    const haystack = fold(`${item.name ?? ""} ${item.subtitle ?? ""} ${item.folder ?? ""}`);
    let total = 0;
    let matchedAll = true;
    for (const term of terms) {
      const score = scoreTerm(term, name, haystack);
      if (score === 0) {
        matchedAll = false;
        break;
      }
      total += score;
    }
    if (matchedAll) scored.push({ item, score: total, name });
  }

  return scored
    .sort((a, b) => (a.score !== b.score ? b.score - a.score : a.name.localeCompare(b.name)))
    .map((s) => s.item);
}

// Folder filter (⌘1–9); with no folder returns everything.
export const filterFolder = (items, folder) =>
  folder ? items.filter((i) => i.folder === folder) : items;
