# @easeo/core

Deterministic SEO payload generation for content platforms. Node.js bindings for the `easeo` Rust core.

Given the same entity, route, and config, `@easeo/core` always produces the same payload: canonical URL, title, description, robots directives, Open Graph, Twitter Cards, and JSON-LD.

## Install

```bash
npm install @easeo/core
```

Prebuilt native bindings ship for Linux x64 (glibc and musl), Linux arm64, macOS x64 and arm64, and Windows x64.

## Usage

```typescript
import { buildSeoPayload } from "@easeo/core";

const payload = buildSeoPayload(
  { entityType: "post", title: "Hello World", description: "An example post." },
  "/blog/hello",
  { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
);

console.log(payload.renderHtml());
```

## Documentation

Full reference at [easeo.emiliano-go.com](https://easeo.emiliano-go.com/), including the [JavaScript API](https://easeo.emiliano-go.com/reference/javascript-api/).

## License

MIT
