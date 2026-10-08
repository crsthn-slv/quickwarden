# Quick Access do 1Password 8 — existe equivalente para Bitwarden no macOS?

Investigação fechada a **10 de setembro de 2026**. Todas as datas indicadas são as que constavam nas fontes nessa altura.

Convenções usadas em todo o documento:

- **[DOC]** — está escrito na documentação oficial do fabricante.
- **[CÓDIGO]** — verificado por leitura directa do código-fonte público.
- **[COMUNIDADE]** — vem de fórum, issue ou README de terceiros.
- **[NÃO VERIFICADO]** — não foi possível confirmar numa fonte primária.

Nada foi instalado, executado ou autorizado durante esta investigação.

---

## Parte 1 — Como funciona o Quick Access do 1Password 8

### Atalho e invocação

| Acção | Atalho macOS | Fonte |
|---|---|---|
| Abrir/fechar Quick Access | `⇧` `⌘` `Espaço` | [DOC] |
| Personalizar o atalho | Settings › General › Shortcuts | [DOC] |
| Alternativa sem teclado | Botão direito no ícone da barra de menus › `Open Quick Access` | [DOC] |

Há ainda uma opção para o Quick Access abrir directamente ao clicar no ícone da barra de menus, em vez de abrir a app principal. [DOC]

### Pesquisa

Ao abrir, o Quick Access já mostra sugestões antes de se escrever fosse o que for: cruza a app ou o site que está em primeiro plano com os itens usados com mais frequência. Este é o comportamento que define a funcionalidade — não é uma caixa de pesquisa, é uma caixa de pesquisa que já sabe onde estás. [DOC]

A partir daí:

- escrever filtra o cofre inteiro; [DOC]
- os ícones de conta/colecção no campo de pesquisa restringem o âmbito, e `All Accounts` alarga-o; [DOC]
- `⌘` `1` a `⌘` `9` alternam entre colecções, que persistem entre sessões; [DOC]
- a pesquisa avançada aceita filtros por tag, categoria, cofre, favoritos e itens sem tag. [DOC]

### Acções sobre o item seleccionado

| Acção | Atalho | Fonte |
|---|---|---|
| Copiar utilizador / campo principal | `⌘` `C` | [DOC] |
| Copiar palavra-passe | `⇧` `⌘` `C` | [DOC] |
| Copiar código de uso único (TOTP) | `⌥` `⌘` `C` | [DOC] |
| Preencher na app em foco | `⇧` `Return` | [DOC] |
| Abrir o site e preencher | `⌥` `Return` | [DOC] |
| Abrir item em janela separada | `⌘` `O` | [DOC] |
| Abrir item na app 1Password | `⇧` `⌘` `O` | [DOC] |

As acções disponíveis mudam consoante o tipo de item. Num item de Wi-Fi aparece copiar palavra-passe da estação base, palavra-passe da rede ou nome da rede; num documento de identificação, copiar número da carta, nome ou validade. [DOC]

### Preenchimento fora do browser

O Universal Autofill (`⌘` `\`) preenche apps de secretária e diálogos do próprio sistema. Quando há vários itens compatíveis, o Quick Access apresenta as opções. [DOC]

Desde **29 de maio de 2026**, na versão 8.12.22, o 1Password entrou em beta público como fornecedor nativo de AutoFill do macOS, através da Passwords API da Apple. Exige **macOS 14 Sonoma em Apple Silicon** e funciona ao lado do Universal Autofill, não em substituição. [DOC]

### Tipos de item confirmados

Logins, cartões de crédito, moradas de entrega, registos médicos, chaves de licença de software e códigos de uso único. [DOC]

### Desbloqueio

O Quick Access herda o estado da app principal. `⇧` `⌘` `L` bloqueia globalmente. O desbloqueio biométrico com Touch ID ou Apple Watch é configurado na app, não no Quick Access. [DOC]

### Segurança

A definição `Remove copied information and authentication codes after 90 seconds` está **activa por predefinição** e vive em Settings › Security. [DOC]

### O que não está documentado

- **Passkeys no Quick Access** — nenhuma das quatro páginas oficiais consultadas menciona passkeys na lista de acções do Quick Access. **[NÃO VERIFICADO]**
- **Chaves SSH no Quick Access** — o agente SSH do 1Password é uma funcionalidade à parte. A chave privada nunca sai da app e cada cliente SSH tem de ser autorizado explicitamente; o utilizador controla quando é pedida aprovação e durante quanto tempo o agente a memoriza. Não há documentação que ligue o agente ao Quick Access. [DOC]
- **Campos adicionais** — há um pedido aberto desde **9 de maio de 2022** no fórum oficial para expor campos personalizados no Quick Access (o caso concreto era passphrases de chaves SSH). A 12 de maio de 2022 um funcionário registou-o internamente como `IDEA-I-969`, sem compromisso nem calendário. Vários utilizadores descreveram o Quick Access como "muito orientado para logins básicos" face ao antigo 1Password mini. Não encontrei indicação de que tenha sido resolvido. [COMUNIDADE]

---

## Parte 2 — O que a Bitwarden oferece oficialmente para o mesmo problema

Resposta curta: **não há equivalente.** Nem parcial. Não existe atalho global que abra uma caixa de pesquisa do cofre no macOS.

### App de secretária

Todos os atalhos documentados são internos — só funcionam com a janela do Bitwarden já em foco. [DOC]

| Acção | Atalho |
|---|---|
| Pesquisar no cofre | `⌘` `F` |
| Copiar utilizador | `⌘` `U` |
| Copiar palavra-passe | `⌘` `P` |
| Copiar TOTP | `⌘` `T` |
| Bloquear cofre | `⌘` `L` |
| Esconder na barra de menus | `⌘` `⇧` `M` |
| Gerador | `⌘` `G` |

Definições relevantes: desbloqueio por Touch ID, desbloqueio por PIN, `Keep running in background` (dá acesso pela barra de menus), limpeza automática da área de transferência com temporizador, minimizar ao copiar, bloquear captura de ecrã, arranque automático no login. [DOC]

A barra de menus dá acesso à app. Não permite pesquisar. Há um pedido aberto no fórum exactamente sobre isso. [COMUNIDADE]

### Extensão de browser

| Acção | Atalho |
|---|---|
| Abrir a extensão | `⌘` `⇧` `Y` |
| Preencher último login (repetir cicla nos itens) | `⌘` `⇧` `L` |
| Gerar palavra-passe | `⌘` `⇧` `9` |
| Bloquear cofre | `⌘` `⇧` `N` |

Estes atalhos só actuam dentro do browser. [DOC]

### Agente SSH

É oficial e está disponível no Windows, macOS (App Store e .dmg), Linux, Snap e Flatpak. Activa-se nas definições e tem uma opção `Ask for authorization when using SSH agent`. Com o cofre desbloqueado, os pedidos passam sem prompt adicional; bloqueado, pedidos de assinatura exigem desbloqueio. É preciso apontar `SSH_AUTH_SOCK` para a socket do Bitwarden, cujo caminho varia com a plataforma e o método de instalação. [DOC]

### Spotlight

Não encontrei qualquer documentação de integração com Spotlight. **[NÃO VERIFICADO]** — o mais provável é simplesmente não existir.

### Fornecedor nativo de AutoFill do macOS

Isto merece nota, porque está em movimento.

O repositório `bitwarden/clients` já contém uma crate `autofill_provider` com um README que descreve, com detalhe, um **MacOS Native Passkey Provider** introduzido no PR `#13963`: extensão nativa em Swift empacotada em `PlugIns` (à semelhança da extensão Safari), IPC sobre socket unix implementado em Rust com bindings UniFFI + NAPI, e um "modal mode" na app Electron para as operações de passkey e SSH. O próprio README diz que, nesse PR, **só se fornecem passkeys** — não palavras-passe. [CÓDIGO]

Do lado público:

- pedido `Register as macOS Password Provider`, aberto a **10 de fevereiro de 2026**, ainda descreve a opção como inexistente no macOS; [COMUNIDADE]
- pedido `Support macOS Native AutoFill Provider Framework`, de **22 de junho de 2026**, foi fechado como duplicado pelo moderador, apontando para os pedidos anteriores; [COMUNIDADE]
- as notas de lançamento consultadas não mencionam o fornecedor de passkeys do macOS. As referências mais próximas são `SSH agent forwarding` (2025.3.3) e `FIDO2 two-step login for macOS desktop` (2025.2.1), que são outra coisa. [DOC]

Conclusão: **[NÃO VERIFICADO]** se já está activo numa versão estável. O código existe, a funcionalidade pública não está confirmada, e mesmo quando chegar cobre passkeys — não é um Quick Access.

### CLI

É aqui que assenta praticamente toda a comunidade.

- Instalação: executáveis nativos (Windows/macOS/Linux x64), `npm install -g @bitwarden/cli`, Chocolatey, Snap, Flatpak. **Em ARM64, a Bitwarden recomenda o npm** — relevante para Apple Silicon. [DOC]
- Autenticação: `bw login` com e-mail e palavra-passe mestra, `bw login --apikey` com `client_id`/`client_secret`, ou `bw login --sso`. [DOC]
- Desbloqueio: `bw unlock` devolve uma chave de sessão, exportada como `BW_SESSION` ou passada com `--session`. Aceita `--passwordenv <var>` e `--passwordfile <caminho>`. [DOC]
- A documentação diz explicitamente que encadear os factores num único comando **"isn't recommended for security reasons"**, e que o ficheiro de palavra-passe deve ficar acessível apenas ao utilizador que corre o `bw unlock`. [DOC]
- `bw lock` e `bw logout` invalidam a sessão activa. [DOC]
- `bw serve` levanta um servidor Express local (porta 8087, ligado a localhost) que expõe as acções do CLI por REST. Por predefinição bloqueia qualquer pedido com cabeçalho `Origin`; desactivar essa protecção "is not recommended", e `--hostname all` abre-o a toda a rede. [DOC]

**Risco conhecido e sem resposta oficial:** uma discussão no fórum de **28 de janeiro de 2025** descreve que, após o unlock, fica em disco um `data.json` desbloqueado, e que o `export BW_SESSION=...` acaba tipicamente no `~/.bash_history`. O autor nota ainda que backups versionados e Time Machine passam a conter esses valores. Nenhuma resposta oficial no fio; foi encaminhado para o programa HackerOne. [COMUNIDADE]

---

## Parte 3 — Alternativas encontradas

### Tabela comparativa

| # | Projecto | URL | Última actividade | Licença | Tecnologia | Instalação | Apple Silicon | Vaultwarden | Precisa do CLI | Estado |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | Bitwarden Vault (Raycast) | `github.com/raycast/extensions/tree/main/extensions/bitwarden` | 13 jul 2026 | MIT | TypeScript / React | Raycast Store | Sim | Sim | Traz o seu | **Activo** |
| 2 | Bitwarden Accelerator | `github.com/ajrosen/Bitwarden-Accelerator` | Activo | GPL-3.0 | Shell + AppleScript (Alfred) | `.alfredworkflow` ou `brew tap ajrosen/tap` | Sim | Não referido | Sim (+ `jq`) | Activo, com reserva |
| 3 | bitwarden-alfred-workflow | `github.com/blacs30/bitwarden-alfred-workflow` | **Arquivado 10 jun 2024** | MIT | Go | Release GitHub | n/d | Sim (`SERVER_URL`) | Sim (≥1.19) | **Abandonado** |
| 4 | alfred-bitwarden | `github.com/twio142/alfred-bitwarden` | Recente | GPL-3.0 | Swift | Manual | Sim | Não referido | Sim | **Imaturo** (21 commits, 0 estrelas, sem releases) |
| 5 | Wenigwarden | `github.com/cyprieng/wenigwarden` | v0.0.4, 9 mar 2025 | MIT | Swift | DMG não assinado | Sim | Sim | Não | **Imaturo** (4 estrelas) |
| 6 | bitwarden-menubar | `github.com/jnsdrtlf/bitwarden-menubar` | Parado | GPL-3.0 | Swift + extensão v1.48.1 | DMG não assinado | n/d | n/d | Não | **Morto** |
| 7 | Swiftwarden | `github.com/jesse231/Swiftwarden` | WIP | n/d no README | Swift / SwiftUI | Build manual | Sim | Sim | Não | **WIP** |
| 8 | Goldwarden | `github.com/quexten/goldwarden` | **Pausado indefinidamente** | n/d no README | Go | Binário / Flatpak | Build existe | Sim | Não | **Descontinuado** |
| 9 | rbw + rbw-agent | `github.com/doy/rbw` | v1.15.0, 31 dez 2025 | MIT | Rust | `brew install rbw` | Sim (bottle) | Sim (`base_url`) | Não — substitui-o | **Estável, só CLI** |

### Cobertura funcional face ao Quick Access

| Capacidade do Quick Access | Raycast | Accelerator | Wenigwarden | rbw |
|---|---|---|---|---|
| Atalho global de qualquer app | Sim (hotkey por comando) | Sim (Alfred) | Sim (hotkey configurável) | Não |
| Sugestões pelo contexto da app activa | **Não** | Parcial — lê o domínio do separador do browser | Não | Não |
| Pesquisa fuzzy no cofre | Sim | Sim | Sim | Sim |
| Copiar utilizador / palavra-passe / TOTP | Sim | Sim | Copiar credenciais; TOTP não referido | Sim |
| Colar na app em foco | Sim (`Paste to Active App`) | Sim | Não referido | Não |
| Preenchimento verdadeiro fora do browser | **Não** | **Não** | **Não** | **Não** |
| Passkeys | Não | Não | Não | Não |
| Chaves SSH | Não | Não | Não | Sim (agente próprio) |
| Re-prompt da palavra-passe mestra por item | Sim | Não referido | Não referido | n/a |
| Limpeza automática da área de transferência | Via Raycast | Sim, configurável | Não referido | n/a |

### Auditoria individual

#### 1. Bitwarden Vault para Raycast — a opção séria

Autor `jomifepe`, 15 contribuidores listados, MIT, cerca de 61 156 instalações no Raycast Store. Vive dentro do monorepo `raycast/extensions`, o que significa revisão por pares da Raycast antes de cada publicação — uma diferença real face a um `.alfredworkflow` que se descarrega solto.

Actividade recente no CHANGELOG: `2026-07-13` correcção de TOTP com espaços no segredo, `2026-05-28` correcção do tratamento de sessão na sincronização, `2026-05-27` actualização do CLI para v2026.4.2, `2026-04-03` actualização do CLI empacotado de v2025.11.0 para v2026.2.0 por causa de erros `Invalid session token` provocados por upgrades de KDF no servidor. [CÓDIGO]

Comandos publicados: `Search Vault`, `Authenticator`, `Generate Password`, `Generate Password (Quick)`, `Create Folder`, `Create Login`, `Lock Vault`, `Logout`, `Create Send`, `Search Sends`, `Receive Send`. [CÓDIGO]

O que verifiquei no código, e que é o mais importante aqui:

- **A palavra-passe mestra não passa por argumentos.** O unlock é `bw unlock --passwordenv BW_PASSWORD --raw`, com `BW_PASSWORD` injectada no ambiente do processo filho. Nada aparece em `ps`. [CÓDIGO]
- **A chave de sessão vai no ambiente**, nunca em `--session` na linha de comandos. [CÓDIGO]
- **Persistência da sessão:** o token fica na `LocalStorage` da Raycast, chave `sessionToken`. A Raycast documenta essa storage como "Raycast's local encrypted database" e garante que "Extensions can *not* access the storage of other extensions". [CÓDIGO] + [DOC]
- **Re-prompt:** existe `sessionRepromptHash` e uma preferência `repromptIgnoreDuration`, ou seja, honra o campo de re-prompt do Bitwarden por item. [CÓDIGO]
- **Cache:** guarda apenas a parte visível e não sensível do cofre, cifrada com chave derivada da palavra-passe mestra. Palavras-passe, campos de identidade, cartões e notas seguras nunca são guardadas. Desactivável nas preferências. [CÓDIGO] + [COMUNIDADE]
- **Isolamento de dados:** define `BITWARDENCLI_APPDATA_DIR` para o `supportPath` da própria extensão, portanto o `data.json` do CLI empacotado não se mistura com o `~/.config` de um `bw` instalado pelo utilizador. [CÓDIGO]
- **TOTP é gerado localmente** com `@otplib`, sem chamada ao CLI. [CÓDIGO]
- **Vaultwarden:** há preferências `serverUrl` e `serverCertsPath` (caminho para certificado TLS auto-assinado). [CÓDIGO]

Pontos negativos, sem rodeios:

- **`clientId` e `clientSecret` da API pessoal ficam guardados nas preferências da extensão.** São do tipo `password`, o que na Raycast significa Keychain, mas continua a ser uma credencial de longa duração em repouso.
- **Depende da Raycast**, que é software proprietário e de código fechado. Isto é uma decisão de confiança, não um detalhe.
- **Não há detecção de contexto.** É pesquisa, não sugestão. Falta exactamente a parte que torna o Quick Access rápido.
- **Colar não é preencher.** `Paste to Active App` faz `⌘V` no campo em foco; não identifica campos, não distingue utilizador de palavra-passe, não responde a diálogos do sistema.
- **O `data.json` do CLI está em disco** — herda integralmente a preocupação levantada no fórum da Bitwarden em janeiro de 2025.
- Issue `#26173`, aberta a **9 de março de 2026**, relatava a extensão inutilizável com `Cannot read properties of null (reading 'toWrappedAccountCryptographicState')`. Está fechada, e a entrada do CHANGELOG de 3 de abril de 2026 sobre a actualização do CLI empacotado corresponde à causa descrita. [COMUNIDADE] + [CÓDIGO]

**Facilidade de auditoria: alta.** Código TypeScript legível, num repositório público com histórico e revisão.

#### 2. Bitwarden Accelerator (Alfred) — funcional, com uma decisão de design que me trava

99 estrelas, GPL-3.0, instalável por `brew tap ajrosen/tap && brew install bitwarden-accelerator`, o que resolve as dependências `bitwarden-cli` e `jq`. Funcionalmente é rico: login por palavra-passe ou API key, 2FA por app autenticadora, YubiKey OTP ou e-mail, pesquisa automática pelo domínio do separador activo do browser, copiar utilizador/palavra-passe/TOTP/notas, edição de itens sem sair do Alfred, download de anexos, sincronização em fundo por Launch Agent, limpeza automática da área de transferência, timeout por inactividade e bloqueio ao bloquear o ecrã. [COMUNIDADE]

O problema: o desbloqueio por Touch ID assenta em guardar a palavra-passe mestra e, nas palavras do próprio README, "It does this by using *sudo* to store and retrieve your password in a secure location". Guardar a palavra-passe mestra é uma decisão defensável; fazê-lo através de `sudo` em vez do Keychain, não obviamente. Não auditei o mecanismo concreto. **[NÃO VERIFICADO]** — e é precisamente por isso que não o recomendo sem auditoria prévia.

Nada no README menciona Vaultwarden. Não é o mesmo que ser incompatível, mas não está testado nem suportado. **[NÃO VERIFICADO]**

Nota lateral do README: `SSO` não é suportado, e `FIDO2`/`Duo` não são suportados pelo CLI da Bitwarden.

#### 3. blacs30/bitwarden-alfred-workflow — descartado

O projecto com mais estrelas de todos os que encontrei (439) e o mais fácil de recomendar por reflexo. Está **arquivado desde 10 de junho de 2024**, com a nota "If someone wants to take this project over please contact me". Pior: o próprio README admite que "The workflow's internal decryption mechanism is currently not working" desde a v2.2.0, obrigando a recorrer ao CLI para desencriptar. Software de gestão de segredos arquivado e com cripto interna assumidamente avariada não se instala.

#### 4. twio142/alfred-bitwarden — cedo demais

Reescrita em Swift do Accelerator, com ranking por domínio do browser, itens recentes e favoritos. 21 commits, 0 estrelas, sem releases, sem discussão de segurança. Guarda a palavra-passe mestra no Keychain do macOS para re-unlock silencioso, sem qualquer análise do trade-off. Um projecto novo, de autor único e sem revisão externa, não é onde se põe a chave do cofre.

#### 5. Wenigwarden — a arquitectura certa, a maturidade errada

É o único que faz o que eu faria: cliente nativo em Swift, sem CLI, na barra de menus, com pesquisa global, navegação por teclado, atalhos configuráveis e auto-lock. Suporta Vaultwarden. MIT.

E tem 4 estrelas, última release **v0.0.4 a 9 de março de 2025** e não é assinado com certificado da Apple — o README manda correr `sudo xattr -rd com.apple.quarantine` antes de abrir. Não tem gerador de palavras-passe, não gere o cofre, não suporta organizações, e o TOTP não é mencionado.

Vale como referência de design. Não vale como coisa onde se põe o cofre.

#### 6, 7, 8. bitwarden-menubar, Swiftwarden, Goldwarden — descartados

- **bitwarden-menubar** empacota a extensão de browser da Bitwarden **na versão 1.48.1** e o autor diz que não recebe actualizações, com um "Use at your own risk!" à cabeça.
- **Swiftwarden** anuncia-se como WIP e a própria lista de tarefas inclui "Properly implement other Bitwarden features" e "Other encryption methods".
- **Goldwarden** tem no topo do README: "Development paused indefinitely". A justificação é boa notícia para quem usa Bitwarden — SSH items, SSH agent, memory security e biometria em Linux foram todos integrados no cliente oficial. Mas os builds para Mac são descritos pelo autor como "somewhat feature-stripped" e **untested**.

#### 9. rbw — não é a solução, é a fundação

Cliente CLI não-oficial em Rust, 1,4k estrelas, MIT, `brew install rbw` com bottles para Apple Silicon, v1.15.0 de **31 de dezembro de 2025**. Contagens do Homebrew: 78 instalações em 30 dias, 227 em 90, 1101 em 365.

O que o distingue do `bw` oficial está no primeiro parágrafo do README: o CLI da Bitwarden é *stateless* e obriga a passar chaves temporárias em variáveis de ambiente, "which makes it very difficult to use". O `rbw` resolve isso com um processo de fundo, o `rbw-agent`, que **mantém as chaves em memória** — o mesmo modelo do `ssh-agent` ou do `gpg-agent`. Não há `BW_SESSION` a circular por scripts nem a cair no histórico da shell.

Outras propriedades relevantes: `base_url` configurável, portanto Vaultwarden; `lock_timeout` (predefinição 3600s) e `sync_interval` configuráveis; perfis via `RBW_PROFILE`, cada um com cofre e agente próprios; agente SSH incorporado; prompts através de `pinentry`.

Limitações honestas do próprio autor: considera o projecto "essentially feature-complete" e diz que é improvável implementar funcionalidades novas por iniciativa própria, embora aceite PRs. 2FA suportado: e-mail, app autenticadora e YubiKey OTP — **WebAuthn/passkey e Duo não são suportados**. Contra o servidor oficial é preciso correr `rbw register` com a API key pessoal, porque a Bitwarden tende a classificar tráfego de linha de comandos como bot.

Não tem interface gráfica. Os frontends listados são todos de Linux (rofi, fuzzel, ulauncher). **Para macOS não existe frontend.**

---

## Parte 4 — Recomendação

### O veredicto

**Não existe hoje equivalente ao Quick Access do 1Password 8 para Bitwarden no macOS.** O que existe cobre metade do problema: encontrar e copiar. A outra metade — o sistema saber que app está à frente e escrever nos campos certos — não está implementada por ninguém, nem oficialmente nem na comunidade.

### O que instalar agora

**Extensão Bitwarden Vault para Raycast.** É a única com manutenção activa, revisão por pares, licença permissiva, tratamento correcto de segredos e suporte declarado a Vaultwarden. Fecha talvez 60% da distância até ao Quick Access.

Não a recomendo por ser a mais popular. Recomendo-a porque foi a única onde li o código e não encontrei nada que me travasse: a palavra-passe mestra entra por variável de ambiente e não por `argv`, o token de sessão fica na storage cifrada e isolada da Raycast, a cache exclui explicitamente campos sensíveis, o re-prompt por item é respeitado e o `data.json` do CLI fica confinado ao directório da extensão.

O que ela não faz, e convém aceitar antes de instalar: não sugere itens pelo contexto, não preenche — cola — e não toca em passkeys nem em chaves SSH.

### Como instalar e validar com segurança

Ordem proposta. Nada disto foi executado.

**Antes de instalar**

1. Criar a API key pessoal na conta Bitwarden (Settings › Security › Keys › API Key). Não usar a palavra-passe mestra como método de login da extensão.
2. Se o alvo for Vaultwarden, ter à mão o URL do servidor e, se o certificado for auto-assinado, o caminho do certificado.

**Instalação**

3. Instalar a Raycast e, na Store, a extensão `Bitwarden Vault` de `jomifepe`. Deixar que use o CLI empacotado em vez de apontar `cliPath` para um `bw` instalado à parte — assim o `data.json` fica isolado no `supportPath` da extensão.
4. Preencher `clientId`, `clientSecret` e, se aplicável, `serverUrl` e `serverCertsPath`.

**Endurecimento**

5. Definir `repromptIgnoreDuration` para o valor mais baixo tolerável.
6. Desactivar `shouldCacheVaultItems` se preferires não ter sequer metadados em cache, aceitando o custo em velocidade.
7. Atribuir um atalho global ao comando `Search Vault` — a Raycast documenta que um hotkey "launches a Raycast command from anywhere on your system", incluindo com a Raycast em fundo. `⌥` `⌘` `Espaço` evita colisão com o Spotlight. Isto é o que mais se aproxima de `⇧` `⌘` `Espaço`.
8. Na app de secretária da Bitwarden, confirmar que a limpeza automática da área de transferência está activa. É o análogo dos 90 segundos que o 1Password tem por predefinição.

**Validação**

9. Confirmar que o binário do CLI que a extensão descarrega corresponde à versão anunciada no CHANGELOG e que a origem é a Bitwarden.
10. Verificar as permissões e o conteúdo do `supportPath` da extensão: confirmar que o `data.json` está cifrado em repouso e que nenhum ficheiro fica legível por outros utilizadores.
11. Correr uma vez com o `Console.app` aberto, filtrado pela extensão, e confirmar que nenhum segredo aparece em log.
12. Confirmar que ao bloquear o ecrã a extensão bloqueia o cofre (`VAULT_LOCK_MESSAGES` inclui `SYSTEM_LOCK` e `SYSTEM_SLEEP`, mas convém verificar na prática).
13. Testar com um item marcado com re-prompt e confirmar que a palavra-passe mestra é pedida.

Se qualquer um dos passos 9 a 13 falhar, para e reavalia.

### E depois

Instalar a extensão da Raycast não resolve o problema que motivou esta pesquisa; resolve a parte fácil. A Parte 5 existe porque a parte difícil — sugestão por contexto e preenchimento real — continua por fazer e é implementável.

---

## Parte 5 — Especificação inicial de uma app nativa de Quick Access para Bitwarden

Isto é um ponto de partida para discussão, não um plano fechado.

### Objectivo

Uma app de barra de menus para macOS que, com um atalho global, mostre uma caixa de pesquisa já preenchida com sugestões baseadas na app em primeiro plano, e que permita copiar ou preencher credenciais em apps nativas — cobrindo Bitwarden Cloud e Vaultwarden.

### Princípios não negociáveis

1. Nenhum segredo em `argv`, em variáveis de ambiente herdáveis, em logs ou em ficheiros não cifrados.
2. A palavra-passe mestra nunca é persistida. Nem no Keychain.
3. As chaves derivadas vivem só em memória, num processo separado da interface.
4. A área de transferência é sempre temporária, com limpeza garantida.
5. Código auditável: sem dependências opacas no caminho dos segredos.

### Arquitectura sugerida

Três processos, com o mínimo de superfície entre eles:

```
┌──────────────────────────┐
│  QuickAccess.app         │  Swift + SwiftUI, LSUIElement
│  barra de menus + painel │  nunca vê a chave mestra
└───────────┬──────────────┘
            │ XPC (NSXPCConnection, code-signing requirement)
┌───────────▼──────────────┐
│  QuickAccessAgent        │  daemon; mantém chaves em memória
│  índice + cripto         │  auto-lock por timeout/sleep/lock
└───────────┬──────────────┘
            │ HTTPS
┌───────────▼──────────────┐
│  Bitwarden Cloud         │
│  ou Vaultwarden          │
└──────────────────────────┘
```

**Decisão em aberto e a mais importante do projecto: como falar com o servidor.**

| Opção | A favor | Contra |
|---|---|---|
| A — embrulhar o `rbw-agent` | Cripto já implementada e testada por 1,4k utilizadores; modelo de agente em memória já resolvido; Vaultwarden suportado; MIT | Autor declarou o projecto feature-complete; sem WebAuthn/Duo; dependência externa em Rust |
| B — embrulhar o `bw` oficial | Suportado pelo fabricante; segue upgrades de KDF | `data.json` em disco; chave de sessão a atravessar fronteiras de processo; latência; foi exactamente isto que partiu a extensão da Raycast em abril de 2026 |
| C — implementar o protocolo em Swift/Rust | Controlo total; sem processos externos; Vaultwarden trivial | Cripto de raiz num gestor de palavras-passe; sem auditoria externa; risco desproporcionado para um projecto pessoal |

A minha inclinação é **A**, com o `rbw-agent` isolado e o `pinentry` substituído por um prompt nativo com Touch ID. Fica por decidir.

### Autenticação e sessão

- Login por API key pessoal (`client_id`/`client_secret`), guardada no Keychain com `kSecAttrAccessibleWhenUnlockedThisDeviceOnly` e ACL exigindo `LAContext`.
- Palavra-passe mestra pedida num painel próprio, mantida em `Data` bloqueada em memória (`mlock`) e apagada com escrita explícita a zeros após derivar a chave.
- Desbloqueio subsequente por Touch ID a libertar a chave derivada guardada na Secure Enclave, com `kSecAccessControlBiometryCurrentSet` — a invalidação automática ao mudar biometria é o comportamento desejado.
- Auto-lock em três gatilhos: timeout de inactividade configurável (predefinição 15 min), `NSWorkspace.willSleepNotification` e bloqueio de ecrã.

### Pesquisa local

- Índice em memória apenas com metadados: `id`, nome, utilizador, `uris`, pasta, favorito, flag de re-prompt. Nunca palavras-passe.
- O segredo é obtido do agente **só no momento da acção**, item a item.
- Correspondência fuzzy com pontuação por: correspondência de domínio com a app em primeiro plano, recência, favoritos, correspondência de prefixo no nome.
- Detecção de contexto: `NSWorkspace.frontmostApplication` para o bundle ID; para browsers, o domínio do separador activo por Apple Events, com pedido de permissão explícito e desactivável.
- Mapa `bundleID → domínio` mantido localmente e editável pelo utilizador (ex.: `com.tinyspeck.slackmacgap → slack.com`).

### Atalho global

`HotKey` via Carbon `RegisterEventHotKey` ou `MASShortcut`, configurável, predefinição `⌥` `⌘` `Espaço`. O painel é um `NSPanel` `.nonactivatingPanel` para não roubar o foco à app de destino — condição necessária para o preenchimento funcionar.

### Área de transferência temporária

- Escrever com `NSPasteboard` marcando `org.nspasteboard.ConcealedType`, que gestores de histórico decentes respeitam.
- Limpeza por temporizador (predefinição 45s) **e** verificação de que o conteúdo ainda é o nosso antes de limpar, para não apagar o que o utilizador copiou entretanto.
- Não escrever no pasteboard `.general` quando o preenchimento directo estiver disponível.

### Preenchimento

Três níveis, do mais correcto para o mais frágil:

1. **Fornecedor nativo de AutoFill (`ASCredentialProviderExtension`)** — o caminho certo, requer macOS 14+ em Apple Silicon, é o mesmo que o 1Password passou a usar em maio de 2026. Cobre apps que adoptem a API. Alvo de médio prazo.
2. **Accessibility API (`AXUIElement`)** — localizar `AXTextField`/`AXSecureTextField` na janela em foco e escrever directamente. Exige `AXIsProcessTrustedWithOptions`. É assim que se cobre a maioria das apps que não adoptam a API da Apple.
3. **Envio de eventos de teclado (`CGEvent`)** — último recurso. Frágil, sensível a layout de teclado e a apps que interceptam eventos. Só com confirmação explícita do utilizador.

### Permissões exigidas

| Permissão | Para quê | Degradação sem ela |
|---|---|---|
| Accessibility | Preenchimento nível 2 e 3 | Cai para copiar/colar |
| Automation (por browser) | Domínio do separador activo | Perde sugestão contextual em browsers |
| Keychain | API key e chave derivada | Pede credenciais a cada arranque |
| Rede (saída) | Sincronização | Só cofre em cache |

Nenhuma é pedida no arranque. Cada uma é pedida no momento em que a funcionalidade correspondente é usada pela primeira vez, com explicação do porquê.

### Modelo de ameaças

| Ameaça | Mitigação |
|---|---|
| Malware a ler memória do processo | Chaves só no agente; `mlock`; hardened runtime; sem `com.apple.security.get-task-allow` em release |
| Outro processo do mesmo utilizador a ler o `data.json` | Cofre cifrado em repouso; ficheiros a `0600`; nunca gravar material desbloqueado |
| Segredos em logs ou crash reports | Tipo `Secret` que não implementa `CustomStringConvertible`; `os_log` com `%{private}`; crash reporter desactivado ou sem payload |
| Segredos em `ps` ou histórico de shell | Zero invocações de CLI com segredos em `argv`; comunicação só por XPC |
| App maliciosa a chamar o agente | XPC com `NSXPCConnection` a validar o code-signing requirement do cliente |
| Screenshot ou gravação de ecrã do painel | `NSWindow.sharingType = .none` |
| Persistência da área de transferência | `ConcealedType` + limpeza verificada |
| Shoulder-surfing | Campos ocultos por predefinição, revelação por acção explícita |
| Compromisso do servidor Vaultwarden | Cripto ponta-a-ponta preservada; pinning opcional do certificado |
| Utilizador a preencher na app errada | Mostrar sempre o nome da app de destino antes de preencher |

Fora de âmbito, e assumido: um atacante com root ou com acesso físico ao equipamento desbloqueado.

### Testes

- **Unitários** — derivação de chave contra vectores conhecidos, parsing do cofre, motor de pontuação da pesquisa, transições da máquina de estados do lock.
- **Integração** — contra uma instância Vaultwarden local em Docker: login, sync, unlock, timeout, re-prompt, rotação de KDF.
- **Segurança** — teste automático que corre a app e faz `grep` a logs, ficheiros temporários e pasteboard à procura de segredos conhecidos; verificação de que `ps -E` não expõe nada; teste de que a chave é apagada da memória após lock.
- **Interface** — testes de UI XCTest para navegação por teclado sem rato.
- **Manuais** — matriz de preenchimento por app: Safari, Chrome, Firefox, Slack, Terminal, iTerm2, diálogos do sistema, VPN, apps Electron.

### Distribuição

- Developer ID + notarização. **Não** App Store, porque a sandbox impede Accessibility e complica o XPC.
- Sparkle 2 para actualizações, com feed assinado por EdDSA.
- Homebrew cask como método principal de instalação.
- Repositório público, builds reproduzíveis, checksums publicados.
- Uma nota clara no README: é um cliente não-oficial, sem afiliação à Bitwarden Inc.

### Faseamento sugerido

| Fase | Entrega | Esforço estimado |
|---|---|---|
| 0 | Decidir A/B/C; protótipo do agente a autenticar e sincronizar contra Vaultwarden | 1–2 semanas |
| 1 | Barra de menus, atalho global, pesquisa, copiar com limpeza temporária | 2–3 semanas |
| 2 | Detecção de contexto e sugestões | 1–2 semanas |
| 3 | Preenchimento via Accessibility | 2–3 semanas |
| 4 | Endurecimento, testes de segurança, notarização, cask | 2 semanas |
| 5 | `ASCredentialProviderExtension` e passkeys | por definir |

Estimativas para uma pessoa a trabalhar em part-time. **[NÃO VERIFICADO]** — são o meu palpite, não uma medição.

---

## Fontes

**1Password (oficial)**

- [Get to know Quick Access](https://support.1password.com/quick-access/?mac) — consultado a 10 set 2026
- [How to Use Quick Access to View Your Passwords](https://1password.com/features/how-to-use-quick-access-in-1password-8) — consultado a 10 set 2026
- [How to navigate 1Password like a pro with Quick Access](https://1password.com/blog/navigate-1password-quick-access) — consultado a 10 set 2026
- [1Password keyboard shortcuts](https://support.1password.com/keyboard-shortcuts/) — consultado a 10 set 2026
- [1Password SSH agent](https://www.1password.dev/ssh/agent/) — consultado a 10 set 2026
- [May 2026 at 1Password: Native macOS AutoFill](https://www.1password.community/announcements-52/may-2026-at-1password-native-macos-autofill-a-new-developer-site-and-more-24666) — beta público a 29 mai 2026, v8.12.22

**1Password (comunidade)**

- [1Password 8 Quick Access Mac — more fields](https://www.1password.community/1password-at-home-31/1password-8-quick-access-mac-more-fields-12802) — 9 a 30 mai 2022, `IDEA-I-969`

**Bitwarden (oficial)**

- [App Settings](https://bitwarden.com/help/app-settings/) — consultado a 10 set 2026
- [Keyboard Shortcuts](https://bitwarden.com/help/keyboard-shortcuts/) — consultado a 10 set 2026
- [Bitwarden SSH Agent](https://bitwarden.com/help/ssh-agent/) — consultado a 10 set 2026
- [Password Manager CLI](https://bitwarden.com/help/cli/) — consultado a 10 set 2026
- [Release Notes](https://bitwarden.com/help/releasenotes/) — consultado a 10 set 2026

**Bitwarden (código e comunidade)**

- [`autofill_provider` README](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autofill_provider/README.md) — PR `#13963`, MacOS Native Passkey Provider
- [In the OSX app, global hotkey to search vault](https://community.bitwarden.com/t/in-the-osx-app-global-hotkey-to-search-vault/9141) — aberto a 27 nov 2019, sem resposta oficial
- [Register as macOS Password Provider](https://community.bitwarden.com/t/register-as-macos-password-and-passkey-provider/93723) — 10 fev 2026
- [Support macOS Native AutoFill Provider Framework](https://community.bitwarden.com/t/support-macos-native-autofill-provider-framework-for-passkeys-passwords/97993) — 22 jun 2026, fechado como duplicado
- [Bitwarden CLI usage can easily result in secrets stored on disk](https://community.bitwarden.com/t/bitwarden-cli-usage-can-easily-result-in-secrets-stored-on-disk/80009) — 28 jan 2025, sem resposta oficial

**Alternativas**

- [Bitwarden Vault (Raycast Store)](https://www.raycast.com/jomifepe/bitwarden) — 61 156 instalações
- [Código da extensão](https://github.com/raycast/extensions/tree/main/extensions/bitwarden) — CHANGELOG até 13 jul 2026
- [Issue #26173](https://github.com/raycast/extensions/issues/26173) — 9 mar 2026, fechada
- [ajrosen/Bitwarden-Accelerator](https://github.com/ajrosen/Bitwarden-Accelerator) — GPL-3.0, 99 estrelas
- [blacs30/bitwarden-alfred-workflow](https://github.com/blacs30/bitwarden-alfred-workflow) — arquivado a 10 jun 2024
- [twio142/alfred-bitwarden](https://github.com/twio142/alfred-bitwarden) — GPL-3.0, 21 commits
- [cyprieng/wenigwarden](https://github.com/cyprieng/wenigwarden) — MIT, v0.0.4 a 9 mar 2025
- [jnsdrtlf/bitwarden-menubar](https://github.com/3j14/bitwarden-menubar) — parado na extensão v1.48.1
- [jesse231/Swiftwarden](https://github.com/jesse231/Swiftwarden) — WIP
- [quexten/goldwarden](https://github.com/quexten/goldwarden) — desenvolvimento pausado
- [doy/rbw](https://github.com/doy/rbw) — MIT, 1,4k estrelas, v1.15.0 a 31 dez 2025
- [Homebrew: rbw](https://formulae.brew.sh/formula/rbw) — 78/227/1101 instalações em 30/90/365 dias

**Raycast (oficial)**

- [Storage API](https://developers.raycast.com/api-reference/storage) — "local encrypted database", isolamento entre extensões
- [Command Aliases & Hotkeys](https://manual.raycast.com/command-aliases-and-hotkeys) — hotkeys globais por comando
