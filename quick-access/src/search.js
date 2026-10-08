// Pontuação da pesquisa do Quick Access (porta de quick-access-search.ts).
// Dobrar acentos e maiúsculas, e exigir que *todos* os termos apareçam.

// Item: { name, subtitle?, folder?, favorite? }

// Sem pesquisa, favoritos primeiro (só se o rbw trouxer o campo), depois alfabético.
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

// Filtro de colecção (⌘1–9); sem pasta devolve tudo.
export const filterFolder = (items, folder) =>
  folder ? items.filter((i) => i.folder === folder) : items;
