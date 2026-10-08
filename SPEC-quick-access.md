# Spec do Quick Access — para portar do protótipo Swift para o fork

Fonte: documentação do 1Password 8 (lida a 10 set 2026) + o que já está implementado e testado
em `QuickAccess.swift`. Este ficheiro é o que sobrevive do protótipo; o resto (rbw, painel Swift,
LaunchAgent) fica para trás porque a app oficial já traz cofre, cadeado, `globalShortcut`,
janela modal e limpeza de área de transferência.

## Atalhos

| Acção | Atalho | Notas |
|---|---|---|
| Abrir/fechar | `⇧` `⌘` `Espaço` | igual ao 1Password; configurável |
| Copiar nome de utilizador | `⌘` `C` | |
| Copiar palavra-passe | `⇧` `⌘` `C` | |
| Copiar código de uso único | `⌥` `⌘` `C` | |
| Abrir no navegador | `⌥` `↩` | primeiro URI `http(s)` do item |
| Abrir o item na app | `⇧` `⌘` `O` | |
| Mais acções | `→` | `←` volta |
| Executar acção seleccionada | `↩` | no menu de acções |
| Navegar | `↑` `↓` | lista **ou** menu de acções, conforme o modo |
| Trocar de colecção | `⌘` `1`…`⌘` `9` | pastas/colecções; selecção persiste |
| Limpar a pesquisa | `Esc` | só fecha se a pesquisa já estiver vazia |
| Fechar | `Esc` com campo vazio, clicar fora, ou X no campo | |

`↩` sem modificador: abre no navegador se o item tiver URI, senão copia a palavra-passe.

## Pesquisa

Pontuação por termo, todos os termos têm de aparecer. Comparação sempre com o texto dobrado
(`diacriticInsensitive` + `caseInsensitive`) — sem isto "Apple" não encontra "ID Apple" e
"acao" não encontra "Ação Social", que foram bugs reais do protótipo.

| Condição | Pontos |
|---|---|
| Nome começa pelo termo | 4 |
| Alguma palavra do nome começa pelo termo | 3 |
| Nome contém o termo | 2 |
| Utilizador, URI ou pasta contém o termo | 1 |
| Nada | descarta o item |

Empate → ordem alfabética pelo nome dobrado.

Casos que o self-check cobre (`quick-access --selftest`, a portar para teste unitário):
`apple` → `ID Apple` · `id apple` → `ID Apple` · `APPLE` → `ID Apple` · `acao` → `Ação Social` ·
`icloud` → `ID Apple` (encontra pelo utilizador) · `zzz` → vazio ·
`a` → `Amazon` antes de `ID Apple` · filtro por colecção isola a colecção.

## Estado e navegação

**Índices separados para a lista e para o menu de acções.** Partilhar um só índice foi bug:
navegar no menu de acções trocava o item seleccionado por baixo.

As acções mostradas dependem do item: sem nome de utilizador não aparece "copiar nome de
utilizador"; sem URI não aparece "abrir no navegador"; notas não mostram "copiar palavra-passe".

## Segurança

- Área de transferência marcada `org.nspasteboard.ConcealedType` e limpa aos 45 s, **só se o
  conteúdo ainda for o nosso** — não apagar o que o utilizador copiou entretanto.
  No fork isto já existe: `ClipboardMain`.
- Janela fora de capturas de ecrã (`sharingType = .none`; no Electron, `setContentProtection`).
- Cofre bloqueado: o atalho tem de abrir o painel de desbloqueio, nunca falhar em silêncio.
- Itens com *re-prompt* de palavra-passe mestra têm de voltar a pedir também aqui.
- Bloquear no sleep, no bloqueio de ecrã e por timeout — no fork herda-se da app.

## Acessibilidade

- Cada linha é um elemento com rótulo `nome, utilizador - uri` e estado de seleccionada.
- Confirmação de cópia anunciada ao leitor de ecrã, não só visual.
- Ícones decorativos escondidos do leitor de ecrã.
- Anel de foco visível e permanente no campo de pesquisa (2,5 px) — é o indicador de que se
  está a escrever. No protótipo, aparecer de forma intermitente foi queixa real.
- Respeitar *Reduce Motion* no scroll da lista.
- Contraste ≥ 4,5:1 no texto sobre as cores dos ícones.

## Fora de âmbito nesta fase

Preenchimento automático, sugestões pela app em primeiro plano
(`NSWorkspace.frontmostApplication`), favicons reais, filtros avançados por etiqueta/categoria.
