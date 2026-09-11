# Browser regression checks

Build and serve the site with `trunk serve --release`. Open the preview in a
Playwright MCP browser, then execute the contents of `workshop.browser.js` with
`browser_run_code` (or `browser_run_code_unsafe` in newer MCP versions). The suite uses the open page's origin, so it also checks a
published deployment without changing the script.

The suite exercises the rendered app, navigation, terminal session, command
completion, keyboard escape, full article output, static assets, and responsive
layouts. It throws at the first failed assertion and reports browser errors.
