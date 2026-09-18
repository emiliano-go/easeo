---
title: "Payload Model"
---

# Payload Model { #payload-model }

An `SEOPayload` is the single output of `build_seo_payload`. It is structured,
hashable, renderable, and serializable.

## Structure { #structure }

| Field | Type | Description |
|---|---|---|
| `title` | `str` | Resolved title, after the template |
| `description` | `str` | Resolved description |
| `canonical` | `str` | Fully normalized canonical URL |
| `robots` | `str` | Robots meta content |
| `og` / `openGraph` | `OGPayload` | Open Graph fields |
| `twitter` | `TwitterPayload` | Twitter Card fields |
| `schema_jsonld` / `schemaJsonLd` | `dict \| list \| None` | JSON-LD |

## Methods { #methods }

| Purpose | Python | JavaScript |
|---|---|---|
| Full head | `render_html()` | `renderHtml()` |
| Open Graph only | `render_opengraph()` | `renderOpengraph()` |
| Twitter only | `render_twitter()` | `renderTwitter()` |
| JSON-LD only | `render_jsonld()` | `renderJsonld()` |
| Canonical dict | `to_dict()` | `toDict()` |
| CamelCase object | - | `toObject()` |
| JSON string | `to_json()` | `toJSONString()` / `toString()` |
| Hash | `hash()` | `hash()` |
| ETag | `etag()` | `etag()` |

## Dict access { #dict-access }

Python payloads are dict-compatible, which makes them easy to drop into
templates and tests:

```python
payload["title"]
payload.get("title", "fallback")
"title" in payload
list(payload)
len(payload)
payload == payload.to_dict()
```

JavaScript payloads expose the equivalents:

```js
payload.get("title");
payload.has("title");
payload.equals(other);
```

## The camelCase view { #camelcase }

In JavaScript, `toObject()` returns camelCase keys (`openGraph`,
`schemaJsonLd`) and is what `JSON.stringify` uses. `toDict()` and
`toJSONString()` return the canonical snake_case wire format shared with
Python, Rust, and the published JSON schemas.

```js
JSON.stringify(payload);
// {"title":...,"openGraph":{...},"schemaJsonLd":{...}}

payload.toDict();
// {"title":...,"og":{...},"schema_jsonld":{...}}
```

## Render order { #render-order }

`render_html()` emits tags in a fixed order:

1. `<title>`
2. `<meta name="description">` when a description exists
3. `<link rel="canonical">`
4. `<meta name="robots">`
5. Open Graph tags
6. Twitter Card tags
7. JSON-LD `<script>` when a schema exists

Fixed order keeps output diffable and snapshot-friendly.

## Escaping { #escaping }

Text fields are HTML-escaped. The JSON-LD script escapes `<` so a value
containing a closing script tag cannot break out of the block. Both are done
in the Rust core, so Python and JavaScript behave identically. Framework
integrations that inject the payload into a page rely on this.

## Extras and hooks { #extras }

A hook can add top-level fields to the payload. Those extras are preserved
through `hash()`, `render_html()`, and serialization, and appear as enumerable
properties alongside the standard fields.

## Recap { #recap }

* The payload has seven standard fields and a small method surface.
* Dict access and equality make it test-friendly.
* Rendering order and escaping are fixed and cross-language.
