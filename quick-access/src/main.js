import { searchItems, filterFolder } from "./search.js";

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { getCurrentWindow } = window.__TAURI__.window;
const { LogicalSize } = window.__TAURI__.dpi;

const ROW = 38, MAX_ROWS = 9;

const ACTIONS = [
  { id: "user", label: "Copiar Nome Do Usuário", keys: "⌘C", ok: (i) => !!i.user, no: "Este item não tem nome de usuário" },
  { id: "password", label: "Copiar Senha", keys: "⇧⌘C", ok: (i) => i.type !== "Note", no: "Este item não tem senha" },
  { id: "totp", label: "Copiar Código De Uso Único", keys: "⌥⌘C", ok: () => true },
  { id: "browser", label: "Abrir No Navegador", keys: "⌥↩", ok: (i) => !!i.uri, no: "Este item não tem endereço web" },
  { id: "bitwarden", label: "Abrir No Bitwarden", keys: "⇧⌘O", ok: () => true },
];
const DONE = { user: "Nome de usuário copiado", password: "Senha copiada", totp: "Código copiado" };

// Um só objecto de estado; índices separados para lista e menu de acções.
const state = { items: [], query: "", folder: null, mode: "list", index: 0, actionIndex: 0, toast: null, locked: false, loading: true };
let toastTimer;

const $ = (id) => document.getElementById(id);
const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c]));

const folders = () => [...new Set(state.items.map((i) => i.folder).filter(Boolean))].sort();
const results = () => searchItems(filterFolder(state.items, state.folder), state.query);
const selected = () => results()[state.index];
const visibleActions = () => { const i = selected(); return i ? ACTIONS.filter((a) => a.ok(i)) : []; };

function prepare(it) {
  it.uri = (it.uris ?? []).find((u) => /^https?:\/\//.test(u));
  it.subtitle = [it.user, it.uri].filter(Boolean).join(" - ");
  return it;
}

function icon(name) {
  const w = name.split(" ");
  const ini = (w.length > 1 ? w[0][0] + w[1][0] : name.slice(0, 2)).toUpperCase();
  let h = 0;
  for (const c of name.toLowerCase()) h = (h * 31 + c.charCodeAt(0)) % 360;
  return `<span class="ico" aria-hidden="true" style="background:hsl(${h} 50% 30%)">${esc(ini)}</span>`;
}

function render() {
  const list = results();
  const acts = visibleActions();
  const item = list[state.index];
  const inActions = state.mode === "actions" && item;

  const q = $("q");
  q.placeholder = state.loading ? "A carregar o cofre…" : "Pesquisar no Bitwarden";
  $("folder").hidden = !state.folder;
  $("folder").textContent = state.folder ?? "";
  $("toast").textContent = state.toast ?? ""; // sobreposto; nunca altera o tamanho da janela

  let html, rows;
  if (inActions) {
    rows = acts.length;
    html = `<div class="head" aria-hidden="true">${icon(item.name)}<div><b>${esc(item.name)}</b><small>${esc(item.subtitle)}</small></div></div>` +
      acts.map((a, i) => `<div class="row act${i === state.actionIndex ? " on" : ""}" role="option" id="act-${i}" aria-selected="${i === state.actionIndex}"><span>${a.label}</span><kbd>${a.keys}</kbd></div>`).join("");
  } else if (state.loading) {
    rows = 0; html = `<div class="empty">A carregar o cofre…</div>`;
  } else if (!list.length) {
    rows = 0; html = `<div class="empty">${state.query ? `Nenhum resultado para “${esc(state.query)}”` : "Cofre vazio"}</div>`;
  } else {
    rows = Math.min(list.length, MAX_ROWS);
    html = list.map((it, i) => {
      const on = i === state.index;
      return `<div class="row${on ? " on" : ""}" role="option" id="opt-${i}" aria-selected="${on}" aria-label="${esc(it.subtitle ? `${it.name}, ${it.subtitle}` : it.name)}">${icon(it.name)}` +
        `<span class="t"><b>${esc(it.name)}</b>${it.subtitle ? `<span class="s">- ${esc(it.subtitle)}</span>` : ""}</span>` +
        (on && (it.uri || ACTIONS[1].ok(it)) ? `<span class="hint">${it.uri ? "Abrir no navegador" : "Copiar senha"}</span>` : "") + `</div>`;
    }).join("");
  }
  $("body").innerHTML = html;
  $("body").setAttribute("aria-label", inActions ? `Ações para ${item.name}` : "Itens do cofre");
  const on = $("body").querySelector(".on");
  if (on) q.setAttribute("aria-activedescendant", on.id); else q.removeAttribute("aria-activedescendant");
  on?.scrollIntoView({ block: "nearest", behavior: "auto" });

  // barra: só as acções que o item seleccionado suporta (mesmo predicado do menu)
  const hint = (k, t) => `<span><kbd>${k}</kbd>${t}</span>`;
  $("bar").innerHTML = inActions
    ? hint("↩", "Executar") + hint("⎋", "Voltar")
    : (item ? ACTIONS.filter((a) => (a.id === "user" || a.id === "password") && a.ok(item)).map((a) => hint(a.keys, a.label)).join("") + hint("→", "Mais ações") : "") +
      (folders().length ? `<span class="sp"></span>${hint("⌘1-9", "Colecções")}` : "");

  // altura = pesquisa 62 + corpo + barra 40 (como o painel Swift)
  const body = inActions ? Math.max(1, rows) * ROW + 62 : rows ? rows * ROW + 10 : 90;
  getCurrentWindow().setSize(new LogicalSize(700, 62 + body + 40));
}

async function show() {
  Object.assign(state, { items: [], query: "", folder: null, mode: "list", index: 0, actionIndex: 0, toast: null, locked: false, loading: true });
  clearTimeout(toastTimer);
  $("q").value = "";
  $("q").focus();
  render();
  try {
    state.items = (await invoke("list_items")).map(prepare);
  } catch (e) {
    flash(String(e), 1500);
  }
  state.loading = false;
  state.index = 0;
  $("q").focus();
  render();
}

const hide = () => invoke("hide");

// hideMs: sucesso, esconde o painel; sem hideMs (erro) o painel fica e o aviso desaparece sozinho
function flash(msg, hideMs) {
  clearTimeout(toastTimer);
  Object.assign(state, { toast: msg, locked: !!hideMs });
  render();
  toastTimer = setTimeout(() => {
    if (hideMs) return hide();
    Object.assign(state, { toast: null, locked: false });
    render();
  }, hideMs || 2500);
}

async function run(id) {
  const item = selected();
  if (!item) return;
  const a = ACTIONS.find((x) => x.id === id);
  if (!a.ok(item)) return flash(a.no);
  try {
    if (DONE[id]) {
      await invoke("copy", { id: item.id, field: id });
      flash(DONE[id], 1000);
    } else {
      await invoke(id === "browser" ? "open_url" : "open_bitwarden", { id: item.id });
      hide();
    }
  } catch (e) {
    flash(String(e));
  }
}

function onKey(e) {
  if (state.locked) return e.preventDefault();
  const { metaKey: cmd, shiftKey: shift, altKey: opt, key, code } = e;
  const handled = (fn) => { e.preventDefault(); fn(); render(); };
  const inActions = state.mode === "actions";

  if (cmd && !shift && !opt && /^[1-9]$/.test(key)) {
    return handled(() => { state.folder = folders()[key - 1] ?? null; state.index = 0; });
  }
  if (cmd && code === "KeyC") return handled(() => run(opt ? "totp" : shift ? "password" : "user"));
  if (cmd && code === "KeyO") return handled(() => run("bitwarden"));
  if (key === "Enter") {
    return handled(() => {
      if (opt) run("browser");
      else if (inActions) { const a = visibleActions()[state.actionIndex]; if (a) run(a.id); }
      else run(selected()?.uri ? "browser" : "password");
    });
  }
  if (key === "Escape") {
    return handled(() => {
      if (inActions) { state.mode = "list"; state.actionIndex = 0; }
      else if (state.query || state.folder) { state.query = ""; state.folder = null; state.index = 0; $("q").value = ""; }
      else hide();
    });
  }
  if (key === "ArrowUp") return handled(() => { inActions ? (state.actionIndex = Math.max(0, state.actionIndex - 1)) : (state.index = Math.max(0, state.index - 1)); });
  if (key === "ArrowDown") {
    return handled(() => {
      if (inActions) state.actionIndex = Math.min(Math.max(0, visibleActions().length - 1), state.actionIndex + 1);
      else state.index = Math.min(Math.max(0, results().length - 1), state.index + 1);
    });
  }
  if (key === "ArrowRight" && !inActions && selected()) return handled(() => { state.mode = "actions"; state.actionIndex = 0; });
  if (key === "ArrowLeft" && inActions) return handled(() => { state.mode = "list"; state.actionIndex = 0; });
}

window.addEventListener("keydown", onKey);
$("q").addEventListener("input", (e) => { state.query = e.target.value; state.index = 0; render(); });
listen("show", show);
render();
