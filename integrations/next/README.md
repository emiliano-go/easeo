# @easeo/next

Next.js integration for [`easeo`](https://easeo.emiliano-go.com/). Turns a content entity into a Next.js Metadata object.

## Install

```bash
npm install @easeo/core @easeo/next
```

## Usage

```typescript
import { easeoMetadata } from "@easeo/next";

export const metadata = easeoMetadata({
  entity: {
    entityType: "post",
    title: "Hello World",
    description: "An example post.",
  },
  route: "/blog/hello",
  config: { canonicalHost: "example.com", publicBaseUrl: "https://example.com" },
});
```

## Documentation

https://easeo.emiliano-go.com/integrations/next/

## License

MIT
