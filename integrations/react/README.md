# @easeo/react

React component for [`easeo`](https://easeo.emiliano-go.com/). Builds a payload and injects the meta tags into the document head. SSR-safe: renders nothing when `document` is unavailable.

## Install

```bash
npm install @easeo/core @easeo/react
```

## Usage

```tsx
import { EaseoHead } from "@easeo/react";

export function BlogPost() {
  return (
    <>
      <EaseoHead
        entity={{ entityType: "post", title: "Hello World", description: "An example post." }}
        route="/blog/hello"
        config={{ canonicalHost: "example.com", publicBaseUrl: "https://example.com" }}
      />
      <h1>Hello World</h1>
    </>
  );
}
```

## Documentation

https://easeo.emiliano-go.com/integrations/react/

## License

MIT
