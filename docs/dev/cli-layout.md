# fletch-cli layout (ROUTE-aligned)

| Layer | Owns |
|-------|------|
| `main.rs` | parse + match dispatch |
| `cli.rs` | clap `Cli` / `Commands` / args |
| `commands.rs` | command entry helpers |
| `types.rs` / `constants.rs` | shared types + registry HTML |
| `support/{io,misc,print,validate}` | helpers |

New logic lands in `support/*` or `commands`, not fat `main` arms.
