---
default: minor
---

# Serve every MCP App on one connection

The new `apps.expose_all` option serves every app on connections that have no `app` query parameter. Those connections list each app's tools and UI resource alongside the operation tools and the enabled built-in tools, so one server URL can offer several apps and the `execute`, `introspect`, `search`, and `validate` tools together. The option is off by default, and a connection that has the `app` parameter still sees only that app.

```yaml
apps:
  expose_all: true
```

With the option on, app names and app tool names must be unique, or the server fails to start. An operation or built-in tool takes precedence over an app tool that has the same name.

An app built for a single target no longer needs the `appTarget` query parameter. When the URL names no target, the server serves the one resource the app has instead of returning `Invalid apps target`.
