# LSP expresssion calculator

A very simple expression calculator with variable autocompletion. This can be used as a starting point for an embeddable
configurator, where editing configuration feels just like normal coding.

## current step: POC
- [x] monaco editor in frontend (from ...)
- [x] tower-lsp in backend  (from ...)
- [ ] frontend and backend talk to each other via Websocket
- [ ] backend defines autocomplete variables
- [ ] syntax inspections according to some expression language

## Usage

Development setup only. start client and server as separate processes

from client directory:
- Run `npm install`. Contrary to the upstream template, I can not suggest pnpm yet because of problems with storybook. 
- `npm run dev` for development.

from project root:
- `cargo run`