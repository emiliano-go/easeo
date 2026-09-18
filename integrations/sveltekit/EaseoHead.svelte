<script>
  /** @type {import("@easeo/core").SEOPayload} */
  export let seo;

  // Escape "<" so a closing script tag inside schema data cannot break out.
  $: safeJsonLd = seo.schemaJsonLd
    ? JSON.stringify(seo.schemaJsonLd).replace(/</g, "\\u003c")
    : null;
</script>

<svelte:head>
  <title>{seo.title}</title>
  <meta name="description" content={seo.description} />
  <link rel="canonical" href={seo.canonical} />
  <meta name="robots" content={seo.robots} />

  <meta property="og:title" content={seo.openGraph.title} />
  <meta property="og:description" content={seo.openGraph.description} />
  <meta property="og:url" content={seo.openGraph.url} />
  <meta property="og:image" content={seo.openGraph.image} />
  <meta property="og:site_name" content={seo.openGraph.siteName} />
  <meta property="og:type" content={seo.openGraph.type} />

  <meta name="twitter:card" content={seo.twitter.card} />
  <meta name="twitter:title" content={seo.twitter.title} />
  <meta name="twitter:description" content={seo.twitter.description} />
  {#if seo.twitter.image}
    <meta name="twitter:image" content={seo.twitter.image} />
  {/if}

  {#if safeJsonLd}
    <script type="application/ld+json">{@html safeJsonLd}</script>
  {/if}
</svelte:head>
