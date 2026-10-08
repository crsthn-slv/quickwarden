# Quick Access

Painel de pesquisa para o Bitwarden no macOS. Corre ao lado da Bitwarden.app oficial (sem fork). Cofre via `rbw` / `rbw-agent` contra o mesmo servidor Vaultwarden.

## O que é

| Aspecto | Valor |
|---|---|
| Função | Pesquisar, copiar, abrir URL |
| Edição | Não. Editar continua na Bitwarden.app |
| Cofre | `rbw` / `rbw-agent`, mesmo servidor Vaultwarden |
| Segredos | Nunca chegam ao webview/JS |
| Atalho de abertura | `⇧⌘Espaço` |
| Stack | Tauri 2 (Rust) + HTML/JS estático, sem bundler |

## Instalar

```sh
./install.sh
"/Applications/Quick Access.app/Contents/MacOS/quick-access" --enroll
```

| Passo | O que faz |
|---|---|
| `./install.sh` | Compila, assina com a identidade local `Quick Access Local Signing`, copia para `/Applications`, instala `~/.local/bin/qa-pinentry`, configura `rbw config set pinentry`, cria LaunchAgent `local.quick-access` |
| `--enroll` | Guarda a palavra-passe mestra no Keychain. O pinentry-mac pede-a duas vezes. Fazer uma vez |

Mudou a palavra-passe mestra → correr `--enroll` outra vez.

## Atalhos

| Acção | Atalho |
|---|---|
| Abrir / fechar | `⇧⌘Espaço` |
| Copiar nome de utilizador | `⌘C` |
| Copiar palavra-passe | `⇧⌘C` |
| Copiar código de uso único (TOTP) | `⌥⌘C` |
| Abrir no navegador (primeiro URL http/https) | `⌥↩` |
| Abrir no Bitwarden (abre a app, não o item) | `⇧⌘O` |
| Mais acções | `→` |
| Voltar ao lista | `←` |
| Navegar lista ou menu de acções | `↑` `↓` |
| Executar | `↩` (com menu: acção seleccionada; sem menu: abre URL se existir, senão copia palavra-passe) |
| Colecção 1–9 | `⌘1`…`⌘9` |
| Limpar pesquisa / voltar / fechar | `Esc` (limpa pesquisa ou colecção; no menu volta à lista; com campo vazio fecha) |

Acções só aparecem se o item as suporta (sem utilizador → sem "Copiar nome de utilizador"; sem URL → sem "Abrir no navegador").

## Touch ID

O mesmo binário funciona como pinentry do `rbw`:

| Passo | Quem |
|---|---|
| `rbw` pede palavra-passe mestra | `rbw-agent` chama `~/.local/bin/qa-pinentry` → `quick-access --pinentry` |
| `GETPIN` → Touch ID (`LAContext`, só biometria) | `pinentry.rs` |
| Touch ID ok → palavra-passe do Keychain (service `local.quick-access`, account `rbw-master-password`) | `pinentry.rs` |
| Cancelado / falhou / sem inscrição / qualquer pedido que não seja a palavra-passe mestra | Reencaminha para `pinentry-mac` (repete os comandos recebidos) |

Cobre também o re-prompt de palavra-passe mestra por item do `rbw`.

## Modelo de segurança e limites

| Tema | Como funciona | Limite |
|---|---|---|
| Keychain | Sem Developer ID, o Keychain protegido devolve `-34018` (`errSecMissingEntitlement`). Logo, não há item com `biometryCurrentSet` | Touch ID imposto pelo nosso código (`LAContext`), não pelo Keychain |
| ACL do item | Presa à assinatura do binário | Por isso a identidade local estável importa: rebuild com assinatura diferente perde acesso |
| Área de transferência | `org.nspasteboard.ConcealedType`; limpa aos 45 s só se `changeCount` não mudou | Se copiar outra coisa antes dos 45 s, não é apagada |
| Capturas de ecrã | Janela `contentProtected` (fora de capturas) | Depende do sistema |
| Bloqueio | `rbw lock` ao adormecer, ao apagar o ecrã e ao bloquear o ecrã | Além disso, `lock_timeout` do rbw = 14400 s (240 min), igual ao timeout do cofre da Bitwarden.app |
| Webview | CSP estrita, sem scripts inline | — |
| Comandos Tauri | Devolvem só itens sem segredos e `Ok`/erro | Segredos saem apenas pelo Rust para o clipboard |

## Limitações conhecidas

| Limitação | Estado |
|---|---|
| Não aparece sobre apps em ecrã inteiro | `FullScreenAuxiliary` não chegou. Próximo passo: NSPanel via `tauri-nspanel` |
| Sem sincronização de bloqueio/desbloqueio com a Bitwarden.app | A app não expõe evento de bloqueio. "Shared unlock" da Bitwarden está em flags não lançadas |
| `⇧⌘O` abre a Bitwarden.app, não o item | Sem deep link |
| Só macOS | `rbw` não corre em Windows |
| Sem favicons | Fora de âmbito |

## Desenvolver

| Tarefa | Comando |
|---|---|
| Testes de pesquisa | `node --test quick-access/src/` |
| Correr em dev | `cd quick-access/src-tauri && cargo tauri dev` |
| Build de debug | `cd quick-access/src-tauri && cargo tauri build --debug --bundles app` |

## Histórico

Substitui um fork Electron de `bitwarden/clients` e um protótipo em Swift. O fork tinha 10 commits, preservados em `../quick-access-fork.bundle` (base `bitwarden/clients@037a68b`). A especificação de atalhos e pesquisa sobrevive em `../SPEC-quick-access.md`.
