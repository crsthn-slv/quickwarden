# Decisão: fork do cliente oficial vs. app nova

Complemento à investigação de 10 de setembro de 2026. Convenções de fonte iguais às do documento anterior: **[DOC]**, **[CÓDIGO]**, **[COMUNIDADE]**, **[NÃO VERIFICADO]**.

## Requisitos actualizados

| # | Requisito | Impacto |
|---|---|---|
| 1 | Bitwarden Cloud **e** Vaultwarden | Elimina implementações que assumam só o servidor oficial |
| 2 | Copiar chega — não é preciso preencher | **Reduz o projecto em cerca de dois terços** |
| 3 | Sem Raycast, sem Alfred | Elimina a recomendação anterior |
| 4 | macOS, Windows e Linux | Elimina Swift nativo |

O requisito 2 é o que mais muda. Sem preenchimento não é preciso Accessibility API, nem `AXUIElement`, nem `CGEvent`, nem `ASCredentialProviderExtension`, nem as permissões de Automation, nem a matriz de testes manuais por aplicação. Some quase toda a complexidade e quase todo o risco.

O requisito 4 mata a app nativa em Swift. Escrever três apps — Swift, WinUI, GTK — para uma funcionalidade é desproporcionado.

---

## Antes: o que é o `rbw-agent`

Ficou por responder. O `rbw` é um cliente de linha de comandos não-oficial para Bitwarden, em Rust. O `rbw-agent` é o processo de fundo que o acompanha.

O problema que ele resolve: o `bw` oficial não guarda estado. Cada comando exige que passes a chave de sessão numa variável de ambiente, o que na prática significa `export BW_SESSION=...` a circular por scripts, por ficheiros e pelo histórico da shell.

O `rbw-agent` faz o que o `ssh-agent` e o `gpg-agent` fazem: fica a correr, guarda as chaves derivadas **em memória**, e responde a pedidos por uma socket local. Escreves `rbw get github` e ele devolve a palavra-passe sem nunca haver uma chave de sessão a atravessar variáveis de ambiente. Tem `lock_timeout` configurável (predefinição 3600s) e agente SSH próprio. [COMUNIDADE]

Era a fundação que eu propunha para uma app de raiz. Com a decisão abaixo, deixa de ser necessário.

---

## A descoberta que decide isto

Fui ler o código do `bitwarden/clients` para perceber quanto trabalho seria enxertar Quick Access na app oficial. O que encontrei muda a conversa.

### Já existe um atalho global registado

`apps/desktop/src/autofill/main/main-desktop-autotype-mvp.service.ts` — 158 linhas, no processo principal do Electron:

```ts
import { ipcMain, globalShortcut } from "electron";
import { autotype_mvp } from "@bitwarden/desktop-napi";
```

O serviço regista um atalho global, valida-o, permite reconfigurá-lo por IPC, e no callback obtém o título da janela em primeiro plano e envia-o para o renderer. Atalho por omissão: `Control` `Alt` `B`. [CÓDIGO]

### Já existe uma janela modal always-on-top

`apps/desktop/src/main/window.main.ts` tem `loadUrl(targetPath, modal)` e `createWindow("modal-app")`. `apps/desktop/src/platform/popup-modal-styles.ts` aplica-lhe: 600×600, `setResizable(false)`, `setAlwaysOnTop(true)`, barra de menus escondida, centrada ou posicionada em coordenadas. Existe um `modalMode$` no `DesktopSettingsService`, e ao sair do modo modal a janela principal esconde-se, porque — nas palavras do comentário no código — "modal is used in front of another app". [CÓDIGO]

Foi construído para as operações de passkey e SSH, que carregam a rota `/passkeys`. É exactamente a mesma primitiva de que um Quick Access precisa.

### O que falta no macOS e no Linux é literalmente isto

`apps/desktop/desktop_native/autotype/src/mvp/macos.rs`, na íntegra:

```rust
// MVP, delete with PM-41067

pub fn get_foreground_window_title() -> anyhow::Result<String> {
    todo!("Bitwarden does not yet support macOS autotype");
}

pub fn type_input(_input: &[u16], _keyboard_shortcut: &[String]) -> anyhow::Result<()> {
    todo!("Bitwarden does not yet support macOS autotype");
}
```

`linux.rs` é idêntico, com "Linux" no lugar de "macOS". O `Cargo.toml` da crate só declara dependências `cfg(windows)`. [CÓDIGO]

Toda a estrutura — atalho global, IPC, validação, configuração, ligação ao renderer — é TypeScript e já é multiplataforma. O buraco são duas funções Rust, e só do lado do *autotype*.

E como decidiste que copiar chega, **não precisas de nenhuma das duas.** `type_input` é preenchimento. `get_foreground_window_title` só serve para sugerir por contexto, que é um extra.

### Estado da funcionalidade oficial

Duas feature flags em `libs/common/src/enums/feature-flag.enum.ts`:

```ts
WindowsDesktopAutotype = "windows-desktop-autotype",
WindowsDesktopAutotypeGA = "windows-desktop-autotype-ga",
```

Ambas com valor `FALSE` por omissão. O nome começa por `Windows`. O comentário `// MVP, delete with PM-41067` aparece em sete ficheiros. [CÓDIGO]

Leitura: a Bitwarden está a construir isto, primeiro para Windows, ainda atrás de flag. Não há sinal público de calendário para macOS e Linux. **[NÃO VERIFICADO]**

---

## Comparação

| Critério | Fork do `bitwarden/clients` | App nova |
|---|---|---|
| Criptografia do cofre | Já existe, auditada, mantida | A construir ou a delegar |
| Sync, KDF, rotação de chaves | Já existe | A construir; é o que partiu a extensão da Raycast em abr 2026 |
| Bitwarden Cloud + Vaultwarden | Já suportado | A construir |
| Organizações, colecções, itens partilhados | Já existe | Muito trabalho |
| TOTP | Já existe | A construir |
| Desbloqueio biométrico nos 3 SO | Já existe | A construir três vezes |
| Bloqueio automático, timeout, sleep, lock de ecrã | Já existe | A construir |
| Agente SSH | Já existe | A construir |
| Três sistemas operativos | Electron, já resolvido | Três implementações ou outro Electron |
| Atalho global | **Já lá está** (`globalShortcut`) | A construir |
| Janela modal always-on-top | **Já lá está** (`popup-modal-styles`) | A construir |
| Superfície de código nova | Uma rota Angular e um serviço no main | Uma aplicação inteira |
| Superfície de ataque nova | Pequena e localizada | Toda a app |
| Manter actualizado | Rebase sobre `main` | Perseguir mudanças de protocolo sozinho |
| Licença | GPL-3.0 — fork permitido | Livre |
| Marca | Tens de mudar nome e ícones | Sem problema |
| Actualizações automáticas | Herdas o Sparkle/electron-updater, mas apontado a ti | A construir |
| Assinatura e notarização | Necessária, e paga | Igual |

---

## Recomendação

**Fork do `bitwarden/clients`.** Não é uma decisão difícil, dados os requisitos.

O argumento não é "poupar trabalho". É que a alternativa obriga a reimplementar criptografia de um gestor de palavras-passe, e a mantê-la a par de mudanças de protocolo que a Bitwarden faz sem avisar ninguém — o incidente de KDF de abril de 2026, que deixou a extensão da Raycast inutilizável com `toWrappedAccountCryptographicState`, é exactamente esse risco a materializar-se num projecto de terceiros.

O fork inverte a relação: em vez de reimplementares o cliente e correres atrás dele, acrescentas uma janela a um cliente que já está correcto.

### O que muda no código

Três peças. Nenhuma toca em criptografia.

**1. `MainQuickAccessService`** — no processo principal, modelado a partir do `MainDesktopAutotypeMvpService`. Regista um `globalShortcut` configurável; no callback, chama `windowMain.loadUrl("/quick-access", true)`, que já aplica os estilos de modal e mostra a janela. Ao fechar, sai do modo modal, o que já esconde a janela principal.

**2. Rota `/quick-access`** — um componente Angular novo em `apps/desktop/src/vault/app/quick-access/`. Campo de pesquisa, lista de resultados, navegação por setas, `Enter` copia. Consome os serviços de cofre que já existem — `CipherService`, `SearchService`, `TotpService`. Não fala com a rede nem com o disco directamente.

**3. Definições** — painel para activar, escolher o atalho, definir o que `Enter` copia por omissão e o tempo de limpeza da área de transferência. Reaproveita `DesktopSettingsService`. A limpeza da área de transferência já existe na app (`ClipboardMain`).

Atalhos dentro do painel, espelhando o 1Password onde faz sentido:

| Acção | Atalho |
|---|---|
| Abrir/fechar | configurável, sugestão `⌥` `⌘` `Espaço` / `Ctrl` `Alt` `Espaço` |
| Copiar utilizador | `⌘` `C` / `Ctrl` `C` |
| Copiar palavra-passe | `⇧` `⌘` `C` / `Ctrl` `Shift` `C` |
| Copiar TOTP | `⌥` `⌘` `C` / `Ctrl` `Alt` `C` |
| Abrir item na app | `⇧` `⌘` `O` / `Ctrl` `Shift` `O` |
| Fechar | `Esc` |

### Pontos de atenção

- **Cofre bloqueado.** O atalho tem de abrir alguma coisa útil quando o cofre está bloqueado — painel de desbloqueio com biometria, e depois a pesquisa. Não pode falhar em silêncio.
- **Re-prompt por item.** Itens marcados com re-prompt têm de pedir a palavra-passe mestra também aqui. É fácil esquecer numa superfície nova.
- **`setAlwaysOnTop(true)` já está no `applyPopupModalStyles`.** Falta confirmar se a janela rouba o foco à app anterior no macOS; se roubar, é preciso um `NSPanel` não-activador, e isso pode não ser trivial em Electron. **[NÃO VERIFICADO]** — é o primeiro risco a testar.
- **Captura de ecrã.** A app tem definição para bloquear captura; confirmar que se aplica à janela modal.
- **Conflito de atalhos.** `globalShortcut.register()` devolve `false` se o atalho já estiver tomado. O serviço de autotype já trata disto; copiar o padrão e mostrar erro ao utilizador.

### Restrições legais e práticas

- **Licença: GPL-3.0** em `apps/desktop`. Fork permitido, com a obrigação de manter GPL-3.0 e publicar o código. A pasta `/bitwarden_license` tem outra licença e deve ficar de fora. [DOC]
- **Marca.** "Bitwarden" é marca registada da Bitwarden Inc. Um fork distribuído tem de mudar nome, ícone e identificadores de aplicação. Não encontrei ficheiro de política de marca no repositório — na dúvida, rebrand completo. **[NÃO VERIFICADO]**
- **Assinatura.** Developer ID e notarização no macOS; certificado de assinatura no Windows. Sem isto, os utilizadores levam avisos. É um custo anual real.
- **Actualizações.** Apontar o `electron-updater` para o teu próprio feed. Nunca deixar apontado ao da Bitwarden.

### Contribuir a montante — vale a pena tentar, mas não bloqueia

A Bitwarden aceita contribuições, com duas condições: assinar o Contributor Agreement em `cla-assistant.io/bitwarden/clients`, e **discutir funcionalidades significativas antes de escrever código**, criando um post na categoria Password Manager das GitHub Discussions. [DOC]

Recomendo abrir essa discussão logo no início, por dois motivos práticos. Primeiro, saber se já há trabalho interno em curso — dado o `PM-41067` e as flags de autotype, é bem possível que haja. Segundo, se aceitarem, deixas de manter um fork, que é o custo escondido de toda esta abordagem.

Mas não esperes pela resposta para começar. O fork funciona de qualquer maneira, e uma implementação a funcionar é melhor argumento do que uma proposta.

---

## Plano

Estimativas para uma pessoa em part-time. São palpites meus, não medições. **[NÃO VERIFICADO]**

| Fase | O quê | Como se sabe que acabou | Esforço |
|---|---|---|---|
| 0 | Clonar, compilar nos três SO, correr a app a partir do código | `npx nx serve desktop` arranca e desbloqueia contra Vaultwarden e Cloud | 2–4 dias |
| 1 | **Spike do risco maior**: janela modal que aparece por atalho global sem roubar foco, nos três SO | Vídeo curto do comportamento em cada SO | 3–5 dias |
| 2 | `MainQuickAccessService` + rota `/quick-access` com pesquisa e cópia | Atalho abre, pesquisa, `Enter` copia, `Esc` fecha | 1–2 semanas |
| 3 | Cofre bloqueado, re-prompt, TOTP, limpeza da área de transferência | Matriz de estados testada | 1 semana |
| 4 | Painel de definições e configuração do atalho | Atalho configurável e persistente | 3–5 dias |
| 5 | Rebrand, assinatura, notarização, feed de actualizações, builds nos três SO | Instaladores assinados que actualizam | 1–2 semanas |
| 6 | *Opcional*: implementar `get_foreground_window_title` em macOS e Linux para sugestões por contexto | Sugestões aparecem antes de escrever | 1 semana por SO |

A Fase 1 existe porque é onde o projecto pode morrer. Se o Electron não conseguir mostrar uma janela sobre outra app sem lhe roubar o foco no macOS, a experiência degrada-se e é melhor saber isso na primeira semana do que na sexta.

A Fase 6 é o que separa "pesquisa rápida" de "Quick Access". Fica para depois de haver algo a funcionar, e a implementação em macOS passa por `NSWorkspace.frontmostApplication` via FFI — não é difícil, mas não é o caminho crítico.

---

## Fontes desta segunda parte

Todo o código citado foi lido em `bitwarden/clients@main` a **10 de setembro de 2026**, versão `apps/desktop` **2026.9.0**.

- [`LICENSE.txt`](https://github.com/bitwarden/clients/blob/main/LICENSE.txt) — GPL-3.0 por omissão; Bitwarden License só em `/bitwarden_license`
- [`apps/desktop/package.json`](https://github.com/bitwarden/clients/blob/main/apps/desktop/package.json) — `"license": "GPL-3.0"`, versão 2026.9.0
- [`main-desktop-autotype-mvp.service.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/autofill/main/main-desktop-autotype-mvp.service.ts) — `globalShortcut`, IPC, callback
- [`main-autotype-keyboard-shortcut.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/autofill/models/main-autotype-keyboard-shortcut.ts) — `DEFAULT_KEYBOARD_SHORTCUT = ["Control", "Alt", "B"]`, nota de suporte só Windows
- [`window.main.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/main/window.main.ts) — `loadUrl(path, modal)`, `createWindow("modal-app")`, `modalMode$`
- [`popup-modal-styles.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/platform/popup-modal-styles.ts) — 600×600, não redimensionável, always-on-top
- [`autotype/src/mvp/macos.rs`](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autotype/src/mvp/macos.rs) e [`linux.rs`](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autotype/src/mvp/linux.rs) — `todo!()`
- [`feature-flag.enum.ts`](https://github.com/bitwarden/clients/blob/main/libs/common/src/enums/feature-flag.enum.ts) — `WindowsDesktopAutotype`, `WindowsDesktopAutotypeGA`, ambas `FALSE`
- [`autofill_provider/README.md`](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autofill_provider/README.md) — modal mode e PR `#13963`
- [Contributing Guidelines](https://contributing.bitwarden.com/contributing/) — CLA obrigatório, discussão prévia para funcionalidades grandes
- [Clients — Desktop](https://contributing.bitwarden.com/getting-started/clients/desktop/) — `npx nx serve desktop`, módulo nativo Rust compilado à parte
- [doy/rbw](https://github.com/doy/rbw) — modelo do `rbw-agent`
