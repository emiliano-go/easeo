# @easeo/sveltekit

SvelteKit integration for [`easeo`](https://easeo.emiliano-go.com/). Build payloads in load functions and render them with `<EaseoHead />`.

## Install

```bash
npm install @easeo/core @easeo/sveltekit
```

## Usage

```svelte
<script>
  import { buildEaseoPayload, EaseoHead } from "@easeo/sveltekit";

  const seo = buildEaseoPayload(
    { entityType: "post", title: "Hello World", description: "An example post." },
    "/blog/hello",
    { canonicalHost: "example.com", publicBaseUrl: "https://example.com" }
  );
</script>

<EaseoHead {seo} />
```

## Documentation

https://easeo.emiliano-go.com/integrations/sveltekit/

## License

MIT
