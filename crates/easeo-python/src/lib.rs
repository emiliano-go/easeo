#![allow(unexpected_cfgs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value as JsonValue;

use easeo_core as core;

// ── Python exception hierarchy ───────────────────────────────────────

pyo3::create_exception!(_easeo_native, EaseoError, pyo3::exceptions::PyException);
pyo3::create_exception!(_easeo_native, InvalidUrlError, EaseoError);
pyo3::create_exception!(_easeo_native, ConfigurationError, EaseoError);
pyo3::create_exception!(_easeo_native, EntityError, EaseoError);
pyo3::create_exception!(_easeo_native, SchemaError, EaseoError);
pyo3::create_exception!(_easeo_native, ContractError, EaseoError);

fn convert_core_error(e: core::EaseoError) -> PyErr {
    match e {
        core::EaseoError::InvalidUrl(msg) => InvalidUrlError::new_err(msg),
        core::EaseoError::InvalidConfiguration(msg) => ConfigurationError::new_err(msg),
        core::EaseoError::InvalidEntity(msg) => EntityError::new_err(msg),
        core::EaseoError::InvalidSchema(msg) => SchemaError::new_err(msg),
        core::EaseoError::SerializationError(msg) => EaseoError::new_err(msg),
        core::EaseoError::ContractError(msg) => ContractError::new_err(msg),
        core::EaseoError::URLPolicyError(msg) => ConfigurationError::new_err(msg),
    }
}

// ── Python wrapper for SEOAuthor ─────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOAuthor {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    url: Option<String>,
}

#[pymethods]
impl SEOAuthor {
    #[new]
    #[pyo3(signature = (name, *, url=None))]
    fn new(name: String, url: Option<String>) -> Self {
        Self { name, url }
    }
}

// ── Python wrapper for OGPayload ─────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct OGPayload {
    #[pyo3(get)]
    r#type: String,
    #[pyo3(get)]
    title: Option<String>,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    url: Option<String>,
    #[pyo3(get)]
    image: Option<String>,
    #[pyo3(get)]
    image_width: Option<u32>,
    #[pyo3(get)]
    image_height: Option<u32>,
    #[pyo3(get)]
    image_alt: Option<String>,
    #[pyo3(get)]
    site_name: Option<String>,
    #[pyo3(get)]
    locale: Option<String>,
    #[pyo3(get)]
    locale_alternate: Option<Vec<String>>,
    #[pyo3(get)]
    audio: Option<String>,
    #[pyo3(get)]
    video: Option<String>,
}

#[pymethods]
impl OGPayload {
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("type", &self.r#type)?;
        if let Some(ref t) = self.title { dict.set_item("title", t)?; }
        if let Some(ref d) = self.description { dict.set_item("description", d)?; }
        if let Some(ref u) = self.url { dict.set_item("url", u)?; }
        if let Some(ref i) = self.image { dict.set_item("image", i)?; }
        if let Some(w) = self.image_width { dict.set_item("image_width", w)?; }
        if let Some(h) = self.image_height { dict.set_item("image_height", h)?; }
        if let Some(ref a) = self.image_alt { dict.set_item("image_alt", a)?; }
        if let Some(ref s) = self.site_name { dict.set_item("site_name", s)?; }
        if let Some(ref l) = self.locale { dict.set_item("locale", l)?; }
        if let Some(ref locs) = self.locale_alternate { dict.set_item("locale_alternate", locs)?; }
        if let Some(ref a) = self.audio { dict.set_item("audio", a)?; }
        if let Some(ref v) = self.video { dict.set_item("video", v)?; }
        Ok(dict)
    }
}

impl From<&core::OGPayload> for OGPayload {
    fn from(og: &core::OGPayload) -> Self {
        Self {
            r#type: og.og_type.clone(),
            title: og.title.clone(),
            description: og.description.clone(),
            url: og.url.clone(),
            image: og.image.clone(),
            image_width: og.image_width,
            image_height: og.image_height,
            image_alt: og.image_alt.clone(),
            site_name: og.site_name.clone(),
            locale: og.locale.clone(),
            locale_alternate: og.locale_alternate.clone(),
            audio: og.audio.clone(),
            video: og.video.clone(),
        }
    }
}

// ── Python wrapper for TwitterPayload ────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct TwitterPayload {
    #[pyo3(get)]
    card: String,
    #[pyo3(get)]
    title: Option<String>,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    image: Option<String>,
    #[pyo3(get)]
    image_alt: Option<String>,
    #[pyo3(get)]
    site: Option<String>,
    #[pyo3(get)]
    creator: Option<String>,
}

#[pymethods]
impl TwitterPayload {
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("card", &self.card)?;
        if let Some(ref t) = self.title { dict.set_item("title", t)?; }
        if let Some(ref d) = self.description { dict.set_item("description", d)?; }
        if let Some(ref i) = self.image { dict.set_item("image", i)?; }
        if let Some(ref a) = self.image_alt { dict.set_item("image_alt", a)?; }
        if let Some(ref s) = self.site { dict.set_item("site", s)?; }
        if let Some(ref c) = self.creator { dict.set_item("creator", c)?; }
        Ok(dict)
    }
}

impl From<&core::TwitterPayload> for TwitterPayload {
    fn from(tw: &core::TwitterPayload) -> Self {
        Self {
            card: tw.card.clone(),
            title: tw.title.clone(),
            description: tw.description.clone(),
            image: tw.image.clone(),
            image_alt: tw.image_alt.clone(),
            site: tw.site.clone(),
            creator: tw.creator.clone(),
        }
    }
}

// ── Python wrapper for SEOExpectation ────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOExpectation {
    inner: core::SEOExpectation,
}

#[pymethods]
impl SEOExpectation {
    #[new]
    #[pyo3(signature = (*, required=None, forbidden=None, equals=None, not_equals=None, contains=None, matches=None, one_of=None, min_length=None, max_length=None, min_items=None, max_items=None, indexable=None, canonical=None, schema_required=None, schema_types=None, og_required=None, twitter_required=None, sitemap_required=None, hreflang_required=None, title=None, description=None))]
    fn new(
        required: Option<bool>,
        forbidden: Option<bool>,
        equals: Option<String>,
        not_equals: Option<String>,
        contains: Option<String>,
        matches: Option<String>,
        one_of: Option<Vec<String>>,
        min_length: Option<usize>,
        max_length: Option<usize>,
        min_items: Option<usize>,
        max_items: Option<usize>,
        indexable: Option<bool>,
        canonical: Option<String>,
        schema_required: Option<bool>,
        schema_types: Option<Vec<String>>,
        og_required: Option<bool>,
        twitter_required: Option<bool>,
        sitemap_required: Option<bool>,
        hreflang_required: Option<bool>,
        title: Option<SEOExpectation>,
        description: Option<SEOExpectation>,
    ) -> Self {
        let schema = schema_required.map(|required| core::SchemaExpectation {
            required: Some(required),
            types: schema_types,
        });
        let og = og_required.map(|required| core::FieldExpectation {
            required: Some(required),
        });
        let twitter = twitter_required.map(|required| core::FieldExpectation {
            required: Some(required),
        });
        let sitemap = sitemap_required.map(|required| core::FieldExpectation {
            required: Some(required),
        });
        let hreflang = hreflang_required.map(|required| core::FieldExpectation {
            required: Some(required),
        });
        Self {
            inner: core::SEOExpectation {
                required,
                forbidden,
                equals,
                not_equals,
                contains,
                matches,
                one_of,
                min_length,
                max_length,
                min_items,
                max_items,
                indexable,
                canonical,
                schema,
                open_graph: og,
                twitter,
                sitemap,
                hreflang,
                title: title.map(|t| Box::new(t.inner)),
                description: description.map(|d| Box::new(d.inner)),
            },
        }
    }

    #[getter]
    fn required(&self) -> Option<bool> {
        self.inner.required
    }

    #[getter]
    fn indexable(&self) -> Option<bool> {
        self.inner.indexable
    }

    #[getter]
    fn canonical(&self) -> Option<&str> {
        self.inner.canonical.as_deref()
    }
}

// ── Python wrapper for SEOContractRule ────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOContractRule {
    #[pyo3(get)]
    r#match: String,
    #[pyo3(get)]
    expect: SEOExpectation,
    #[pyo3(get)]
    severity: Option<String>,
}

#[pymethods]
impl SEOContractRule {
    #[new]
    #[pyo3(signature = (r#match, expect, *, severity=None))]
    fn new(r#match: String, expect: SEOExpectation, severity: Option<String>) -> Self {
        Self { r#match, expect, severity }
    }
}

impl From<&core::SEOContractRule> for SEOContractRule {
    fn from(rule: &core::SEOContractRule) -> Self {
        Self {
            r#match: rule.r#match.clone(),
            expect: SEOExpectation { inner: rule.expect.clone() },
            severity: rule.severity.as_ref().map(|s| match s {
                core::ContractSeverity::Error => "error".to_string(),
                core::ContractSeverity::Warning => "warning".to_string(),
                core::ContractSeverity::Info => "info".to_string(),
            }),
        }
    }
}

// ── Python wrapper for SchemaRegistry ────────────────────────────────

#[pyclass]
struct SchemaRegistry {
    inner: std::sync::Mutex<core::registry::SchemaRegistry>,
}

#[pymethods]
impl SchemaRegistry {
    #[new]
    fn new() -> Self {
        Self {
            inner: std::sync::Mutex::new(core::registry::SchemaRegistry::new()),
        }
    }

    /// Register a custom schema type.
    /// Since Python callables cannot be stored in Rust's SchemaRegistry,
    /// this records the type name for later use in schema_type_map.
    fn register(&self, schema_type: &str, _builder: PyObject) {
        let mut registry = self.inner.lock().unwrap();
        // We can't store the Python callable, but we can track the type name.
        // The registry.get() won't find it, but has()/list_types() will report it.
        // For actual schema generation, the type must be in config.schema_type_map.
        if !registry.has(schema_type) {
            // Register a fallback builder that returns an empty object
            registry.register(schema_type, |_ctx| serde_json::json!({}));
        }
    }

    fn has(&self, schema_type: &str) -> bool {
        let registry = self.inner.lock().unwrap();
        registry.has(schema_type)
    }

    fn list_types(&self) -> Vec<String> {
        let registry = self.inner.lock().unwrap();
        registry.list_types()
    }
}

// ── Python wrapper for SEOImage ───────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOImage {
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    width: Option<u32>,
    #[pyo3(get)]
    height: Option<u32>,
    #[pyo3(get)]
    alt: Option<String>,
}

#[pymethods]
impl SEOImage {
    #[new]
    #[pyo3(signature = (url, *, width=None, height=None, alt=None))]
    fn new(url: String, width: Option<u32>, height: Option<u32>, alt: Option<String>) -> Self {
        Self { url, width, height, alt }
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("url", &self.url)?;
        if let Some(w) = self.width { dict.set_item("width", w)?; }
        if let Some(h) = self.height { dict.set_item("height", h)?; }
        if let Some(ref a) = self.alt { dict.set_item("alt", a)?; }
        Ok(dict)
    }
}

impl From<&core::SEOImage> for SEOImage {
    fn from(img: &core::SEOImage) -> Self {
        Self {
            url: img.url.clone(),
            width: img.width,
            height: img.height,
            alt: img.alt.clone(),
        }
    }
}

impl From<&SEOImage> for core::SEOImage {
    fn from(img: &SEOImage) -> Self {
        Self {
            url: img.url.clone(),
            width: img.width,
            height: img.height,
            alt: img.alt.clone(),
        }
    }
}

// ── Python wrapper for Breadcrumb ─────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct Breadcrumb {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    url: String,
}

#[pymethods]
impl Breadcrumb {
    #[new]
    fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

impl From<&core::Breadcrumb> for Breadcrumb {
    fn from(bc: &core::Breadcrumb) -> Self {
        Self { name: bc.name.clone(), url: bc.url.clone() }
    }
}

impl From<&Breadcrumb> for core::Breadcrumb {
    fn from(bc: &Breadcrumb) -> Self {
        Self { name: bc.name.clone(), url: bc.url.clone() }
    }
}

// ── Python wrapper for FAQItem ────────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct FAQItem {
    #[pyo3(get)]
    question: String,
    #[pyo3(get)]
    answer: String,
}

#[pymethods]
impl FAQItem {
    #[new]
    fn new(question: String, answer: String) -> Self {
        Self { question, answer }
    }
}

impl From<&core::FAQItem> for FAQItem {
    fn from(item: &core::FAQItem) -> Self {
        Self { question: item.question.clone(), answer: item.answer.clone() }
    }
}

impl From<&FAQItem> for core::FAQItem {
    fn from(item: &FAQItem) -> Self {
        Self { question: item.question.clone(), answer: item.answer.clone() }
    }
}

// ── Python wrapper for Robots ─────────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct Robots {
    #[pyo3(get)]
    index: bool,
    #[pyo3(get)]
    follow: bool,
    #[pyo3(get)]
    max_snippet: Option<i32>,
    #[pyo3(get)]
    max_image_preview: Option<String>,
    #[pyo3(get)]
    max_video_preview: Option<i32>,
}

#[pymethods]
impl Robots {
    #[new]
    #[pyo3(signature = (*, index=true, follow=true, max_snippet=None, max_image_preview=None, max_video_preview=None))]
    fn new(
        index: bool,
        follow: bool,
        max_snippet: Option<i32>,
        max_image_preview: Option<String>,
        max_video_preview: Option<i32>,
    ) -> Self {
        Self { index, follow, max_snippet, max_image_preview, max_video_preview }
    }

    fn serialize(&self) -> String {
        let mut parts = vec![
            if self.index { "index" } else { "noindex" }.to_string(),
            if self.follow { "follow" } else { "nofollow" }.to_string(),
        ];
        if let Some(v) = self.max_snippet {
            parts.push(format!("max-snippet:{}", v));
        }
        if let Some(ref v) = self.max_image_preview {
            parts.push(format!("max-image-preview:{}", v));
        }
        if let Some(v) = self.max_video_preview {
            parts.push(format!("max-video-preview:{}", v));
        }
        parts.join(",")
    }
}

impl From<&core::Robots> for Robots {
    fn from(r: &core::Robots) -> Self {
        Self {
            index: r.index,
            follow: r.follow,
            max_snippet: r.max_snippet,
            max_image_preview: r.max_image_preview.clone(),
            max_video_preview: r.max_video_preview,
        }
    }
}

impl From<&Robots> for core::Robots {
    fn from(r: &Robots) -> Self {
        Self {
            index: r.index,
            follow: r.follow,
            max_snippet: r.max_snippet,
            max_image_preview: r.max_image_preview.clone(),
            max_video_preview: r.max_video_preview,
        }
    }
}

// ── Python wrapper for URLPolicy ──────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct URLPolicy {
    #[pyo3(get)]
    enforce_https: bool,
    #[pyo3(get)]
    lowercase_paths: bool,
    #[pyo3(get)]
    trailing_slash: String,
    #[pyo3(get)]
    collapse_duplicate_slashes: bool,
    #[pyo3(get)]
    strip_tracking_params: bool,
    #[pyo3(get)]
    allowed_query_params: Vec<String>,
}

#[pymethods]
impl URLPolicy {
    #[new]
    #[pyo3(signature = (*, enforce_https=true, lowercase_paths=true, trailing_slash="never", collapse_duplicate_slashes=true, strip_tracking_params=true, allowed_query_params=None))]
    fn new(
        enforce_https: bool,
        lowercase_paths: bool,
        trailing_slash: &str,
        collapse_duplicate_slashes: bool,
        strip_tracking_params: bool,
        allowed_query_params: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            enforce_https,
            lowercase_paths,
            trailing_slash: trailing_slash.to_string(),
            collapse_duplicate_slashes,
            strip_tracking_params,
            allowed_query_params: allowed_query_params.unwrap_or_default(),
        })
    }
}

impl From<&core::URLPolicy> for URLPolicy {
    fn from(p: &core::URLPolicy) -> Self {
        Self {
            enforce_https: p.enforce_https,
            lowercase_paths: p.lowercase_paths,
            trailing_slash: match p.trailing_slash {
                core::TrailingSlash::Always => "always".to_string(),
                core::TrailingSlash::Never => "never".to_string(),
                core::TrailingSlash::Preserve => "preserve".to_string(),
            },
            collapse_duplicate_slashes: p.collapse_duplicate_slashes,
            strip_tracking_params: p.strip_tracking_params,
            allowed_query_params: p.allowed_query_params.clone(),
        }
    }
}

impl TryFrom<&URLPolicy> for core::URLPolicy {
    type Error = PyErr;

    fn try_from(p: &URLPolicy) -> Result<Self, Self::Error> {
        let trailing_slash = match p.trailing_slash.as_str() {
            "always" => core::TrailingSlash::Always,
            "never" => core::TrailingSlash::Never,
            "preserve" => core::TrailingSlash::Preserve,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("invalid trailing_slash: {}", p.trailing_slash)
            )),
        };
        Ok(Self {
            enforce_https: p.enforce_https,
            lowercase_paths: p.lowercase_paths,
            trailing_slash,
            collapse_duplicate_slashes: p.collapse_duplicate_slashes,
            strip_tracking_params: p.strip_tracking_params,
            allowed_query_params: p.allowed_query_params.clone(),
        })
    }
}

// ── Python wrapper for SEOConfig ──────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOConfig {
    inner: core::SEOConfig,
}

#[pymethods]
impl SEOConfig {
    #[new]
    #[pyo3(signature = (canonical_host, public_base_url, *, url_policy=None, default_robots=None, default_og_image=None, site_name=None, title_template=None, search_robots=None, auto_generate_schema=true, publisher_name=None, publisher_logo=None, locale=None, locale_alternate=None, twitter_site=None, emit_warnings=false, schema_type_map=None))]
    fn new(
        canonical_host: String,
        public_base_url: String,
        url_policy: Option<URLPolicy>,
        default_robots: Option<Robots>,
        default_og_image: Option<SEOImage>,
        site_name: Option<String>,
        title_template: Option<String>,
        search_robots: Option<Robots>,
        auto_generate_schema: bool,
        publisher_name: Option<String>,
        publisher_logo: Option<String>,
        locale: Option<String>,
        locale_alternate: Option<Vec<String>>,
        twitter_site: Option<String>,
        emit_warnings: bool,
        schema_type_map: Option<Vec<(String, String)>>,
    ) -> PyResult<Self> {
        let up = match url_policy {
            Some(p) => (&p).try_into()?,
            None => core::URLPolicy::default(),
        };
        let dr = default_robots.map(|r| (&r).into()).unwrap_or_default();
        let sr = search_robots.map(|r| (&r).into()).unwrap_or_else(|| core::Robots { index: false, follow: true, ..Default::default() });
        let doi = default_og_image.map(|i| (&i).into());
        let sm = schema_type_map.map(|v| {
            v.into_iter().map(|(k, v)| (k, Some(v))).collect()
        }).unwrap_or_else(|| core::SEOConfig::default().schema_type_map);

        let config = core::SEOConfig {
            canonical_host,
            public_base_url,
            url_policy: up,
            default_robots: dr,
            default_og_image: doi,
            site_name,
            title_template,
            search_robots: sr,
            schema_type_map: sm,
            auto_generate_schema,
            publisher_name,
            publisher_logo,
            locale,
            locale_alternate,
            twitter_site,
            emit_warnings,
        };
        config.validate().map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(Self { inner: config })
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_str = serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let json_val: JsonValue = serde_json::from_str(&json_str).unwrap();
        json_to_pydict(py, &json_val)
    }
}

// ── Python wrapper for SEOEntity ──────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOEntity {
    inner: core::SEOEntity,
}

#[pymethods]
impl SEOEntity {
    #[new]
    #[pyo3(signature = (entity_type, *, slug=None, title=None, excerpt=None, body_html=None, status=None, featured_image=None, published_at=None, updated_at=None, author_name=None, breadcrumbs=None, sku=None, price=None, price_currency=None, availability=None, same_as=None, address=None, faq_items=None))]
    fn new(
        entity_type: &str,
        slug: Option<String>,
        title: Option<String>,
        excerpt: Option<String>,
        body_html: Option<String>,
        status: Option<String>,
        featured_image: Option<SEOImage>,
        published_at: Option<String>,
        updated_at: Option<String>,
        author_name: Option<String>,
        breadcrumbs: Option<Vec<Breadcrumb>>,
        sku: Option<String>,
        price: Option<String>,
        price_currency: Option<String>,
        availability: Option<String>,
        same_as: Option<Vec<String>>,
        address: Option<String>,
        faq_items: Option<Vec<FAQItem>>,
    ) -> PyResult<Self> {
        let et = core::EntityType::from_str(entity_type)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let fi = featured_image.map(|i| (&i).into());
        let bcs = breadcrumbs.map(|v| v.iter().map(|b| b.into()).collect());
        let faq = faq_items.map(|v| v.iter().map(|f| f.into()).collect());

        Ok(Self {
            inner: core::SEOEntity {
                entity_type: et,
                slug,
                title,
                excerpt,
                body_html,
                status,
                featured_image: fi,
                published_at,
                updated_at,
                author_name,
                breadcrumbs: bcs,
                sku,
                price,
                price_currency,
                availability,
                same_as,
                address,
                faq_items: faq,
            },
        })
    }

    #[getter]
    fn entity_type(&self) -> &str {
        self.inner.entity_type.as_str()
    }

    #[getter]
    fn title(&self) -> Option<&str> {
        self.inner.title.as_deref()
    }

    #[getter]
    fn excerpt(&self) -> Option<&str> {
        self.inner.excerpt.as_deref()
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_str = serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let json_val: JsonValue = serde_json::from_str(&json_str).unwrap();
        json_to_pydict(py, &json_val)
    }
}

// ── Python wrapper for SEOOverrides ───────────────────────────────────

#[pyclass]
#[derive(Clone, Default)]
struct SEOOverrides {
    inner: core::SEOOverrides,
}

#[pymethods]
impl SEOOverrides {
    #[new]
    #[pyo3(signature = (*, meta_title=None, meta_description=None, canonical_url=None, robots=None, og_title=None, og_description=None, og_image=None, twitter_card=None, twitter_title=None, twitter_description=None, twitter_image=None, schema_jsonld=None, omit_schema=false, skip_title_template=false, twitter_creator=None, og_audio=None, og_video=None))]
    fn new<'py>(
        py: Python<'py>,
        meta_title: Option<String>,
        meta_description: Option<String>,
        canonical_url: Option<String>,
        robots: Option<Robots>,
        og_title: Option<String>,
        og_description: Option<String>,
        og_image: Option<SEOImage>,
        twitter_card: Option<String>,
        twitter_title: Option<String>,
        twitter_description: Option<String>,
        twitter_image: Option<SEOImage>,
        schema_jsonld: Option<PyObject>,
        omit_schema: bool,
        skip_title_template: bool,
        twitter_creator: Option<String>,
        og_audio: Option<String>,
        og_video: Option<String>,
    ) -> PyResult<Self> {
        let r = robots.map(|r| (&r).into());
        let oi = og_image.map(|i| (&i).into());
        let ti = twitter_image.map(|i| (&i).into());
        // Convert Python dict/list to serde_json::Value via json.dumps
        let sj = match schema_jsonld {
            Some(obj) => {
                let json_module = py.import_bound("json")?;
                let json_str = json_module.call_method1("dumps", (obj,))?.extract::<String>()?;
                Some(serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Null))
            }
            None => None,
        };

        Ok(Self {
            inner: core::SEOOverrides {
                meta_title,
                meta_description,
                canonical_url,
                robots: r,
                og_title,
                og_description,
                og_image: oi,
                twitter_card,
                twitter_title,
                twitter_description,
                twitter_image: ti,
                schema_jsonld: sj,
                omit_schema,
                skip_title_template,
                twitter_creator,
                og_audio,
                og_video,
            },
        })
    }
}

// ── Python wrapper for SEOPayload ─────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOPayload {
    inner: core::SEOPayload,
}

#[pymethods]
impl SEOPayload {
    #[getter]
    fn title(&self) -> &str {
        &self.inner.title
    }

    #[getter]
    fn description(&self) -> &str {
        &self.inner.description
    }

    #[getter]
    fn canonical(&self) -> &str {
        &self.inner.canonical
    }

    #[getter]
    fn robots(&self) -> &str {
        &self.inner.robots
    }

    #[getter]
    fn og(&self) -> OGPayload {
        (&self.inner.og).into()
    }

    #[getter]
    fn twitter(&self) -> TwitterPayload {
        (&self.inner.twitter).into()
    }

    #[getter]
    fn schema_jsonld<'py>(&self, py: Python<'py>) -> Option<PyObject> {
        self.inner.schema_jsonld.as_ref().and_then(|v| {
            json_to_pyobject(py, v).ok().map(|o| o.into())
        })
    }

    fn render_html(&self) -> String {
        self.inner.render_html()
    }

    fn render_opengraph(&self) -> String {
        self.inner.render_opengraph()
    }

    fn render_twitter(&self) -> String {
        self.inner.render_twitter()
    }

    fn render_jsonld(&self) -> String {
        self.inner.render_jsonld()
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_val = self.inner.to_dict();
        json_to_pydict(py, &json_val)
    }

    fn to_json(&self) -> String {
        self.inner.to_json_pretty()
    }

    fn hash(&self) -> String {
        core::hash_payload(&self.inner)
    }

    fn etag(&self) -> String {
        core::etag_payload(&self.inner)
    }
}

// ── Python wrapper for SEOContract ────────────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOContract {
    inner: core::SEOContract,
}

#[pymethods]
impl SEOContract {
    #[getter]
    fn contract_version(&self) -> &str {
        &self.inner.contract_version
    }

    #[getter]
    fn site<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_val = serde_json::to_value(&self.inner.site).unwrap();
        json_to_pydict(py, &json_val)
    }

    fn to_json(&self) -> String {
        self.inner.to_json()
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_str = serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let json_val: JsonValue = serde_json::from_str(&json_str).unwrap();
        json_to_pydict(py, &json_val)
    }

    fn hash(&self) -> String {
        self.inner.hash()
    }

    fn write(&self, path: &str) -> PyResult<()> {
        self.inner.write(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
    }
}

// ── Python wrapper for SEOContractConfig ──────────────────────────────

#[pyclass]
#[derive(Clone)]
struct SEOContractConfig {
    inner: core::SEOContractConfig,
}

#[pymethods]
impl SEOContractConfig {
    #[new]
    #[pyo3(signature = (canonical_host, *, scheme="https", defaults=None, rules=None, exceptions=None))]
    fn new(
        canonical_host: String,
        scheme: &str,
        defaults: Option<SEOExpectation>,
        rules: Option<Vec<SEOContractRule>>,
        exceptions: Option<Vec<(String, SEOExpectation)>>,
    ) -> Self {
        let d = defaults.map(|e| e.inner).unwrap_or_default();
        let r = rules.map(|v| v.into_iter().map(|r| {
            core::SEOContractRule {
                r#match: r.r#match,
                expect: r.expect.inner,
                severity: r.severity.as_ref().map(|s| match s.as_str() {
                    "error" => core::ContractSeverity::Error,
                    "warning" => core::ContractSeverity::Warning,
                    "info" => core::ContractSeverity::Info,
                    _ => core::ContractSeverity::Error,
                }),
            }
        }).collect()).unwrap_or_default();
        let ex = exceptions.map(|v| v.into_iter().map(|(k, e)| (k, e.inner)).collect()).unwrap_or_default();
        Self {
            inner: core::SEOContractConfig {
                canonical_host,
                scheme: scheme.to_string(),
                defaults: d,
                rules: r,
                exceptions: ex,
            },
        }
    }
}

// ── Python wrapper for SEOIssue ───────────────────────────────────────

#[pyclass]
struct SEOIssue {
    #[pyo3(get)]
    rule_id: String,
    #[pyo3(get)]
    severity: String,
    #[pyo3(get)]
    message: String,
    #[pyo3(get)]
    url: Option<String>,
    details_json: String,
}

#[pymethods]
impl SEOIssue {
    #[getter]
    fn details(&self, py: Python) -> PyResult<PyObject> {
        let val: JsonValue = serde_json::from_str(&self.details_json)
            .unwrap_or(JsonValue::Object(serde_json::Map::new()));
        json_to_pyobject(py, &val).map(|o| o.into())
    }
}

impl From<&core::validation::SEOIssue> for SEOIssue {
    fn from(issue: &core::validation::SEOIssue) -> Self {
        Self {
            rule_id: issue.rule_id.clone(),
            severity: match issue.severity {
                core::validation::Severity::Error => "error".to_string(),
                core::validation::Severity::Warning => "warning".to_string(),
                core::validation::Severity::Info => "info".to_string(),
            },
            message: issue.message.clone(),
            url: issue.url.clone(),
            details_json: serde_json::to_string(&issue.details).unwrap_or_default(),
        }
    }
}

// ── Helper: JSON → Python ────────────────────────────────────────────

fn json_to_pydict<'py>(py: Python<'py>, val: &JsonValue) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new_bound(py);
    if let JsonValue::Object(map) = val {
        for (k, v) in map {
            dict.set_item(k, json_to_pyobject(py, v)?)?;
        }
    }
    Ok(dict)
}

fn json_to_pyobject<'py>(py: Python<'py>, val: &JsonValue) -> PyResult<Bound<'py, PyAny>> {
    match val {
        JsonValue::Null => Ok(py.None().into_bound(py)),
        JsonValue::Bool(b) => Ok(b.into_py(py).into_bound(py)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py).into_bound(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py).into_bound(py))
            } else {
                Ok(n.to_string().into_py(py).into_bound(py))
            }
        }
        JsonValue::String(s) => Ok(s.into_py(py).into_bound(py)),
        JsonValue::Array(arr) => {
            let list = PyList::empty_bound(py);
            for item in arr {
                list.append(json_to_pyobject(py, item)?)?;
            }
            Ok(list.into_any())
        }
        JsonValue::Object(map) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in map {
                dict.set_item(k, json_to_pyobject(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

// ── Module functions ──────────────────────────────────────────────────

#[pyfunction]
#[pyo3(signature = (entity, route, config, overrides=None))]
fn build_seo_payload(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: Option<&SEOOverrides>,
) -> PyResult<SEOPayload> {
    let default_ov = core::SEOOverrides::default();
    let ov = overrides.map(|o| &o.inner).unwrap_or(&default_ov);
    let payload = core::build_seo_payload_with_overrides(&entity.inner, route, &config.inner, ov)
        .map_err(convert_core_error)?;
    Ok(SEOPayload { inner: payload })
}

#[pyfunction]
fn build_seo_payload_with_overrides(
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: &SEOOverrides,
) -> PyResult<SEOPayload> {
    let payload = core::build_seo_payload_with_overrides(&entity.inner, route, &config.inner, &overrides.inner)
        .map_err(convert_core_error)?;
    Ok(SEOPayload { inner: payload })
}

#[pyfunction]
#[pyo3(signature = (entity, route, config, overrides=None))]
fn build_seo_payload_dict<'py>(
    py: Python<'py>,
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: Option<&SEOOverrides>,
) -> PyResult<Bound<'py, PyDict>> {
    let payload = build_seo_payload(entity, route, config, overrides)?;
    payload.to_dict(py)
}

#[pyfunction]
fn build_seo_contract(config: &SEOContractConfig) -> PyResult<SEOContract> {
    let contract = core::build_seo_contract(&config.inner)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(SEOContract { inner: contract })
}

#[pyfunction]
fn validate_payload(payload: &SEOPayload) -> Vec<SEOIssue> {
    core::validate_payload(&payload.inner).iter().map(|i| i.into()).collect()
}

#[pyfunction]
fn normalize_path_fn(path: &str, policy: &URLPolicy) -> PyResult<String> {
    let up: core::URLPolicy = policy.try_into()?;
    core::url::normalize_path(path, &up)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn normalize_public_url_fn(url: &str, config: &SEOConfig) -> PyResult<String> {
    core::url::normalize_public_url(url, &config.inner)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn clean_url_fn<'py>(py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyDict>> {
    let result = core::detrack::clean_url(url);
    let dict = PyDict::new_bound(py);
    dict.set_item("url", &result.url)?;
    let removed = PyDict::new_bound(py);
    for (k, v) in &result.removed_params {
        removed.set_item(k.as_str(), v.as_str())?;
    }
    dict.set_item("removed_params", removed)?;
    let cleaned = PyDict::new_bound(py);
    for (k, v) in &result.cleaned_params {
        cleaned.set_item(k.as_str(), v.as_str())?;
    }
    dict.set_item("cleaned_params", cleaned)?;
    Ok(dict)
}

#[pyfunction]
fn clean_query_fn(query: &str) -> String {
    core::detrack::clean_query(query)
}

// ── Module definition ─────────────────────────────────────────────────

#[pymodule]
fn _easeo_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Exceptions
    m.add("EaseoError", m.py().get_type_bound::<EaseoError>())?;
    m.add("InvalidUrlError", m.py().get_type_bound::<InvalidUrlError>())?;
    m.add("ConfigurationError", m.py().get_type_bound::<ConfigurationError>())?;
    m.add("EntityError", m.py().get_type_bound::<EntityError>())?;
    m.add("SchemaError", m.py().get_type_bound::<SchemaError>())?;
    m.add("ContractError", m.py().get_type_bound::<ContractError>())?;

    // Types
    m.add_class::<SEOImage>()?;
    m.add_class::<SEOAuthor>()?;
    m.add_class::<OGPayload>()?;
    m.add_class::<TwitterPayload>()?;
    m.add_class::<Breadcrumb>()?;
    m.add_class::<FAQItem>()?;
    m.add_class::<Robots>()?;
    m.add_class::<URLPolicy>()?;
    m.add_class::<SEOConfig>()?;
    m.add_class::<SEOEntity>()?;
    m.add_class::<SEOOverrides>()?;
    m.add_class::<SEOPayload>()?;
    m.add_class::<SEOContract>()?;
    m.add_class::<SEOContractConfig>()?;
    m.add_class::<SEOContractRule>()?;
    m.add_class::<SEOExpectation>()?;
    m.add_class::<SEOIssue>()?;
    m.add_class::<SchemaRegistry>()?;

    // Functions
    m.add_function(wrap_pyfunction!(build_seo_payload, m)?)?;
    m.add_function(wrap_pyfunction!(build_seo_payload_with_overrides, m)?)?;
    m.add_function(wrap_pyfunction!(build_seo_payload_dict, m)?)?;
    m.add_function(wrap_pyfunction!(build_seo_contract, m)?)?;
    m.add_function(wrap_pyfunction!(validate_payload, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_path_fn, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_public_url_fn, m)?)?;
    m.add_function(wrap_pyfunction!(clean_url_fn, m)?)?;
    m.add_function(wrap_pyfunction!(clean_query_fn, m)?)?;
    Ok(())
}
