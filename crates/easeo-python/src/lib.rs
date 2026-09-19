#![allow(unexpected_cfgs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple, PyType};
use serde_json::Value as JsonValue;
use std::ffi::CString;
use std::sync::OnceLock;

/// Type alias for Python objects (replaces removed PyObject in PyO3 0.29)
type PyObject = Py<PyAny>;

use easeo_core as core;

// ── Python exception hierarchy ───────────────────────────────────────
//
// The exception types are built at module import time (not with
// `create_exception!`) so that `EaseoError` inherits from both `Exception`
// and `ValueError`. That keeps `except ValueError` working for code coming
// from seoslug, while `except EaseoError` still catches everything.

static EXC_TYPES: OnceLock<ExceptionTypes> = OnceLock::new();

struct ExceptionTypes {
    easeo: Py<PyType>,
    invalid_url: Py<PyType>,
    configuration: Py<PyType>,
    entity: Py<PyType>,
    schema: Py<PyType>,
    contract: Py<PyType>,
}

/// Build an exception type with multiple bases by calling `type(name, bases, {})`.
/// This is the supported way to express multiple inheritance for exceptions.
fn make_exception_tuple(
    py: Python<'_>,
    qualified: &CString,
    bases: &Bound<'_, PyTuple>,
) -> PyResult<Py<PyType>> {
    let builtins = py.import("builtins")?;
    let type_callable = builtins.getattr("type")?;
    let name = qualified.to_str().unwrap().rsplit('.').next().unwrap();
    let ns = PyDict::new(py);
    let ty = type_callable.call1((name, bases, ns))?;
    Ok(ty.cast_into::<PyType>()?.unbind())
}

fn build_exception_types(py: Python<'_>) -> PyResult<ExceptionTypes> {
    let builtins = py.import("builtins")?;
    let value_error = builtins.getattr("ValueError")?;

    // `ValueError` already subclasses `Exception`, so it alone gives both
    // `except ValueError` (seoslug compat) and `except Exception` coverage.
    let base_bases = PyTuple::new(py, [&value_error])?;
    let easeo = make_exception_tuple(
        py,
        &CString::new("_easeo_native.EaseoError").unwrap(),
        &base_bases,
    )?;

    let sub_bases = PyTuple::new(py, [easeo.bind(py).as_any()])?;
    let invalid_url = make_exception_tuple(
        py,
        &CString::new("_easeo_native.InvalidUrlError").unwrap(),
        &sub_bases,
    )?;
    let configuration = make_exception_tuple(
        py,
        &CString::new("_easeo_native.ConfigurationError").unwrap(),
        &sub_bases,
    )?;
    let entity = make_exception_tuple(
        py,
        &CString::new("_easeo_native.EntityError").unwrap(),
        &sub_bases,
    )?;
    let schema = make_exception_tuple(
        py,
        &CString::new("_easeo_native.SchemaError").unwrap(),
        &sub_bases,
    )?;
    let contract = make_exception_tuple(
        py,
        &CString::new("_easeo_native.ContractError").unwrap(),
        &sub_bases,
    )?;

    Ok(ExceptionTypes {
        easeo,
        invalid_url,
        configuration,
        entity,
        schema,
        contract,
    })
}

fn exception_types(py: Python<'_>) -> PyResult<&'static ExceptionTypes> {
    if let Some(types) = EXC_TYPES.get() {
        return Ok(types);
    }
    let types = build_exception_types(py)?;
    // A concurrent initializer may have won the race; either value is valid.
    let _ = EXC_TYPES.set(types);
    Ok(EXC_TYPES.get().expect("exception types initialized"))
}

fn raise(py: Python<'_>, exc: &Py<PyType>, msg: String) -> PyErr {
    PyErr::from_type(exc.bind(py).clone(), msg)
}

fn convert_core_error(e: core::EaseoError) -> PyErr {
    Python::attach(|py| {
        let Ok(types) = exception_types(py) else {
            return PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string());
        };
        match e {
            core::EaseoError::InvalidUrl(msg) => raise(py, &types.invalid_url, msg),
            core::EaseoError::InvalidConfiguration(msg) => raise(py, &types.configuration, msg),
            core::EaseoError::InvalidEntity(msg) => raise(py, &types.entity, msg),
            core::EaseoError::InvalidSchema(msg) => raise(py, &types.schema, msg),
            core::EaseoError::SerializationError(msg) => raise(py, &types.easeo, msg),
            core::EaseoError::ContractError(msg) => raise(py, &types.contract, msg),
            core::EaseoError::URLPolicyError(msg) => raise(py, &types.configuration, msg),
        }
    })
}

// ── Python wrapper for SEOAuthor ─────────────────────────────────────

#[pyclass(from_py_object)]
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

#[pyclass(from_py_object)]
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
        let dict = PyDict::new(py);
        dict.set_item("type", &self.r#type)?;
        if let Some(ref t) = self.title {
            dict.set_item("title", t)?;
        }
        if let Some(ref d) = self.description {
            dict.set_item("description", d)?;
        }
        if let Some(ref u) = self.url {
            dict.set_item("url", u)?;
        }
        if let Some(ref i) = self.image {
            dict.set_item("image", i)?;
        }
        if let Some(w) = self.image_width {
            dict.set_item("image_width", w)?;
        }
        if let Some(h) = self.image_height {
            dict.set_item("image_height", h)?;
        }
        if let Some(ref a) = self.image_alt {
            dict.set_item("image_alt", a)?;
        }
        if let Some(ref s) = self.site_name {
            dict.set_item("site_name", s)?;
        }
        if let Some(ref l) = self.locale {
            dict.set_item("locale", l)?;
        }
        if let Some(ref locs) = self.locale_alternate {
            dict.set_item("locale_alternate", locs)?;
        }
        if let Some(ref a) = self.audio {
            dict.set_item("audio", a)?;
        }
        if let Some(ref v) = self.video {
            dict.set_item("video", v)?;
        }
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

#[pyclass(from_py_object)]
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
        let dict = PyDict::new(py);
        dict.set_item("card", &self.card)?;
        if let Some(ref t) = self.title {
            dict.set_item("title", t)?;
        }
        if let Some(ref d) = self.description {
            dict.set_item("description", d)?;
        }
        if let Some(ref i) = self.image {
            dict.set_item("image", i)?;
        }
        if let Some(ref a) = self.image_alt {
            dict.set_item("image_alt", a)?;
        }
        if let Some(ref s) = self.site {
            dict.set_item("site", s)?;
        }
        if let Some(ref c) = self.creator {
            dict.set_item("creator", c)?;
        }
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

#[pyclass(from_py_object)]
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

#[pyclass(from_py_object)]
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
        Self {
            r#match,
            expect,
            severity,
        }
    }
}

impl From<&core::SEOContractRule> for SEOContractRule {
    fn from(rule: &core::SEOContractRule) -> Self {
        Self {
            r#match: rule.r#match.clone(),
            expect: SEOExpectation {
                inner: rule.expect.clone(),
            },
            severity: rule.severity.as_ref().map(|s| match s {
                core::ContractSeverity::Error => "error".to_string(),
                core::ContractSeverity::Warning => "warning".to_string(),
                core::ContractSeverity::Info => "info".to_string(),
            }),
        }
    }
}

// ── Python wrapper for SchemaRegistry ────────────────────────────────

#[pyclass(skip_from_py_object)]
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

    // Custom schema generators are registered from Python through
    // `easeo.registry.SchemaRegistry`, which stores callables and applies
    // them around the build. This native type is introspection only.

    fn has(&self, schema_type: &str) -> bool {
        let registry = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        registry.has(schema_type)
    }

    fn list_types(&self) -> Vec<String> {
        let registry = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        registry.list_types()
    }
}

// ── Python wrapper for SEOImage ───────────────────────────────────────

#[pyclass(from_py_object)]
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
        Self {
            url,
            width,
            height,
            alt,
        }
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("url", &self.url)?;
        if let Some(w) = self.width {
            dict.set_item("width", w)?;
        }
        if let Some(h) = self.height {
            dict.set_item("height", h)?;
        }
        if let Some(ref a) = self.alt {
            dict.set_item("alt", a)?;
        }
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

#[pyclass(from_py_object)]
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
        Self {
            name: bc.name.clone(),
            url: bc.url.clone(),
        }
    }
}

impl From<&Breadcrumb> for core::Breadcrumb {
    fn from(bc: &Breadcrumb) -> Self {
        Self {
            name: bc.name.clone(),
            url: bc.url.clone(),
        }
    }
}

// ── Python wrapper for FAQItem ────────────────────────────────────────

#[pyclass(from_py_object)]
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
        Self {
            question: item.question.clone(),
            answer: item.answer.clone(),
        }
    }
}

impl From<&FAQItem> for core::FAQItem {
    fn from(item: &FAQItem) -> Self {
        Self {
            question: item.question.clone(),
            answer: item.answer.clone(),
        }
    }
}

// ── Python wrapper for Robots ─────────────────────────────────────────

#[pyclass(from_py_object)]
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
        Self {
            index,
            follow,
            max_snippet,
            max_image_preview,
            max_video_preview,
        }
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

#[pyclass(from_py_object)]
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
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "invalid trailing_slash: {}",
                    p.trailing_slash
                )))
            }
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

#[pyclass(from_py_object)]
struct SEOConfig {
    inner: core::SEOConfig,
    /// Config-scoped ``HookRegistry`` (or any object with a ``run`` method).
    /// Opaque to Rust; consulted by the Python-level ``run_hooks`` wrapper.
    hooks: Option<PyObject>,
    /// Config-scoped ``SchemaRegistry`` with Python-callable generators.
    schema_registry: Option<PyObject>,
}

impl Clone for SEOConfig {
    fn clone(&self) -> Self {
        // Python handles are GIL-bound and cannot be cloned here; clone only
        // the Rust config and drop the Python-side handles.
        Self {
            inner: self.inner.clone(),
            hooks: None,
            schema_registry: None,
        }
    }
}

#[pymethods]
impl SEOConfig {
    #[new]
    #[pyo3(signature = (canonical_host, public_base_url, *, url_policy=None, default_robots=None, default_og_image=None, site_name=None, title_template=None, search_robots=None, auto_generate_schema=true, publisher_name=None, publisher_logo=None, locale=None, locale_alternate=None, twitter_site=None, emit_warnings=false, schema_type_map=None, search_url_template=None, hooks=None, schema_registry=None))]
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
        search_url_template: Option<String>,
        hooks: Option<PyObject>,
        schema_registry: Option<PyObject>,
    ) -> PyResult<Self> {
        let up = match url_policy {
            Some(p) => (&p).try_into()?,
            None => core::URLPolicy::default(),
        };
        let dr = default_robots.map(|r| (&r).into()).unwrap_or_default();
        let sr = search_robots
            .map(|r| (&r).into())
            .unwrap_or_else(|| core::Robots {
                index: false,
                follow: true,
                ..Default::default()
            });
        let doi = default_og_image.map(|i| (&i).into());
        let sm = schema_type_map
            .map(|v| v.into_iter().map(|(k, v)| (k, Some(v))).collect())
            .unwrap_or_else(|| core::SEOConfig::default().schema_type_map);

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
            search_url_template,
        };
        config.validate().map_err(convert_core_error)?;
        Ok(Self {
            inner: config,
            hooks,
            schema_registry,
        })
    }

    #[getter]
    fn hooks(&self, py: Python<'_>) -> Option<PyObject> {
        self.hooks.as_ref().map(|h| h.clone_ref(py))
    }

    #[setter]
    fn set_hooks(&mut self, hooks: Option<PyObject>) {
        self.hooks = hooks;
    }

    #[getter]
    fn schema_registry(&self, py: Python<'_>) -> Option<PyObject> {
        self.schema_registry.as_ref().map(|r| r.clone_ref(py))
    }

    #[setter]
    fn set_schema_registry(&mut self, registry: Option<PyObject>) {
        self.schema_registry = registry;
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_str = serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let json_val: JsonValue = serde_json::from_str(&json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        json_to_pydict(py, &json_val)
    }
}

// ── Python wrapper for SEOEntity ──────────────────────────────────────

#[pyclass(from_py_object)]
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
        let et = core::EntityType::from_str(entity_type).map_err(convert_core_error)?;
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
        let json_val: JsonValue = serde_json::from_str(&json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        json_to_pydict(py, &json_val)
    }
}

// ── Python wrapper for SEOOverrides ───────────────────────────────────

#[pyclass(from_py_object)]
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
                let json_module = py.import("json")?;
                let json_str = json_module
                    .call_method1("dumps", (obj,))?
                    .extract::<String>()?;
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

#[pyclass(from_py_object)]
#[derive(Clone)]
struct SEOPayload {
    inner: core::SEOPayload,
}

#[pymethods]
impl SEOPayload {
    /// Dict-style access: payload["title"].
    fn __getitem__<'py>(&self, py: Python<'py>, key: &str) -> PyResult<PyObject> {
        let dict = self.to_dict(py)?;
        match dict.get_item(key)? {
            Some(value) => Ok(value.into()),
            None => Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                key.to_string(),
            )),
        }
    }

    #[pyo3(signature = (key, default=None))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        key: &str,
        default: Option<PyObject>,
    ) -> PyResult<PyObject> {
        let dict = self.to_dict(py)?;
        match dict.get_item(key)? {
            Some(value) => Ok(value.into()),
            None => Ok(default.unwrap_or_else(|| py.None())),
        }
    }

    fn keys<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let dict = self.to_dict(py)?;
        let keys: Vec<PyObject> = dict.keys().into_iter().map(|k| k.into()).collect();
        PyList::new(py, keys)
    }

    fn __contains__<'py>(&self, py: Python<'py>, key: &str) -> PyResult<bool> {
        let dict = self.to_dict(py)?;
        Ok(dict.get_item(key)?.is_some())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let keys = self.keys(py)?;
        keys.call_method0("__iter__")
    }

    fn __len__<'py>(&self, py: Python<'py>) -> PyResult<usize> {
        Ok(self.to_dict(py)?.len())
    }

    /// Equality against another payload or a plain dict. This is what makes
    /// `assert payload == expected_dict` work for snapshot testing.
    fn __eq__(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let py = other.py();
        if let Ok(other_payload) = other.extract::<PyRef<SEOPayload>>() {
            return Ok(self.inner == other_payload.inner);
        }
        if let Ok(dict) = other.cast::<PyDict>() {
            let mine = self.to_dict(py)?;
            return mine.eq(dict);
        }
        Ok(false)
    }

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
        self.inner
            .schema_jsonld
            .as_ref()
            .and_then(|v| json_to_pyobject(py, v).ok().map(|o| o.into()))
    }

    fn render_html(&self) -> PyResult<String> {
        self.inner
            .render_html()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn render_opengraph(&self) -> String {
        self.inner.render_opengraph()
    }

    fn render_twitter(&self) -> String {
        self.inner.render_twitter()
    }

    fn render_jsonld(&self) -> PyResult<String> {
        self.inner
            .render_jsonld()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_val = self
            .inner
            .to_dict()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        json_to_pydict(py, &json_val)
    }

    fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json_pretty()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn hash(&self) -> PyResult<String> {
        core::hash_payload(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn etag(&self) -> PyResult<String> {
        core::etag_payload(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

// ── Python wrapper for SEOContract ────────────────────────────────────

#[pyclass(from_py_object)]
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
        let json_val = serde_json::to_value(&self.inner.site)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        json_to_pydict(py, &json_val)
    }

    fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let json_str = serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let json_val: JsonValue = serde_json::from_str(&json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        json_to_pydict(py, &json_val)
    }

    fn hash(&self) -> PyResult<String> {
        self.inner
            .hash()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn write(&self, path: &str) -> PyResult<()> {
        self.inner
            .write(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
    }
}

// ── Python wrapper for SEOContractConfig ──────────────────────────────

#[pyclass(from_py_object)]
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
        let r = rules
            .map(|v| {
                v.into_iter()
                    .map(|r| core::SEOContractRule {
                        r#match: r.r#match,
                        expect: r.expect.inner,
                        severity: r.severity.as_ref().map(|s| match s.as_str() {
                            "error" => core::ContractSeverity::Error,
                            "warning" => core::ContractSeverity::Warning,
                            "info" => core::ContractSeverity::Info,
                            _ => core::ContractSeverity::Error,
                        }),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let ex = exceptions
            .map(|v| v.into_iter().map(|(k, e)| (k, e.inner)).collect())
            .unwrap_or_default();
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

#[pyclass(skip_from_py_object)]
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

const JSON_MAX_DEPTH: usize = 128;

fn json_to_pydict<'py>(py: Python<'py>, val: &JsonValue) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    if let JsonValue::Object(map) = val {
        for (k, v) in map {
            dict.set_item(k, json_to_pyobject_depth(py, v, 0)?)?;
        }
    }
    Ok(dict)
}

fn json_to_pyobject<'py>(py: Python<'py>, val: &JsonValue) -> PyResult<Bound<'py, PyAny>> {
    json_to_pyobject_depth(py, val, 0)
}

/// Convert an arbitrary Python object to `serde_json::Value` via
/// ``json.dumps``. Used to read hook output back into a payload.
fn py_to_json_value(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<JsonValue> {
    let json_module = py.import("json")?;
    let json_str = json_module
        .call_method1("dumps", (obj,))?
        .extract::<String>()?;
    serde_json::from_str(&json_str)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

fn json_to_pyobject_depth<'py>(
    py: Python<'py>,
    val: &JsonValue,
    depth: usize,
) -> PyResult<Bound<'py, PyAny>> {
    if depth > JSON_MAX_DEPTH {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "JSON structure too deeply nested (max 128 levels)",
        ));
    }
    match val {
        JsonValue::Null => Ok(py.None().into_bound(py)),
        JsonValue::Bool(b) => Ok(b.into_pyobject(py)?.to_owned().into_any()),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.to_owned().into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.to_owned().into_any())
            } else {
                Ok(n.to_string().into_pyobject(py)?.to_owned().into_any())
            }
        }
        JsonValue::String(s) => Ok(s.into_pyobject(py)?.to_owned().into_any()),
        JsonValue::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_to_pyobject_depth(py, item, depth + 1)?)?;
            }
            Ok(list.into_any())
        }
        JsonValue::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_to_pyobject_depth(py, v, depth + 1)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

// ── Module functions ──────────────────────────────────────────────────

/// Run the config-scoped hooks (if any) over the freshly built payload.
///
/// Hooks are a Python-level concept, so we round-trip through the wire
/// format: payload -> dict -> hooks -> payload. The result stays
/// deterministic because the hook registry is part of the config.
fn apply_hooks(
    py: Python<'_>,
    payload: core::SEOPayload,
    entity: &SEOEntity,
    config: &SEOConfig,
) -> PyResult<core::SEOPayload> {
    let Some(hooks) = config.hooks.as_ref() else {
        return Ok(payload);
    };

    let as_dict = payload.to_dict().map_err(convert_core_error)?;
    let dict = json_to_pydict(py, &as_dict)?;
    let entity_obj = Py::new(py, entity.clone())?;
    let config_obj = Py::new(py, config.clone())?;

    let result = hooks.bind(py).getattr("run")?.call1((
        "post_process",
        dict.clone(),
        entity_obj,
        config_obj,
    ))?;

    // Hooks must return a dict (mirrors seoslug). Tolerate `None` by
    // treating it as "no change" so a forgotten return does not crash.
    let mutated = if result.is_none() {
        dict
    } else {
        result.cast_into::<PyDict>().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyTypeError, _>("post_process hooks must return a dict")
        })?
    };

    let json_val = py_to_json_value(py, mutated.as_any())?;
    core::SEOPayload::from_dict(&json_val).map_err(convert_core_error)
}

/// Replace the auto-generated schema with a registered generator's output
/// when one matches the resolved `@type`. Runs before hooks so hooks see
/// the final schema.
fn apply_schema_registry(
    py: Python<'_>,
    payload: core::SEOPayload,
    entity: &SEOEntity,
    config: &SEOConfig,
) -> PyResult<core::SEOPayload> {
    let Some(registry) = config.schema_registry.as_ref() else {
        return Ok(payload);
    };

    let Some(schema_type) = payload
        .schema_jsonld
        .as_ref()
        .and_then(|s| s.get("@type"))
        .and_then(|t| t.as_str())
        .map(str::to_string)
    else {
        return Ok(payload);
    };

    let registry = registry.bind(py);
    if !registry
        .call_method1("has", (&schema_type,))?
        .extract::<bool>()?
    {
        return Ok(payload);
    }

    let og_image = payload.og.image.clone();
    let generated = registry.call_method1(
        "generate",
        (
            &schema_type,
            Py::new(py, entity.clone())?,
            Py::new(py, config.clone())?,
            &payload.canonical,
            &payload.title,
            &payload.description,
            og_image,
        ),
    )?;

    if generated.is_none() {
        return Ok(payload);
    }

    let json_val = py_to_json_value(py, generated.as_any())?;
    let mut updated = payload;
    updated.schema_jsonld = Some(json_val);
    Ok(updated)
}

/// Emit Python warnings for validation issues when `emit_warnings` is set.
fn maybe_emit_warnings(
    py: Python<'_>,
    payload: &core::SEOPayload,
    config: &SEOConfig,
) -> PyResult<()> {
    if !config.inner.emit_warnings {
        return Ok(());
    }
    let issues = core::validate_payload(payload);
    if issues.is_empty() {
        return Ok(());
    }
    let warnings = py.import("warnings")?;
    for issue in issues {
        let location = issue
            .url
            .as_deref()
            .map(|u| format!(" ({u})"))
            .unwrap_or_default();
        let message = format!("[{}] {}{}", issue.rule_id, issue.message, location);
        warnings.call_method1("warn", (message,))?;
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (entity, route, config, overrides=None))]
fn build_seo_payload(
    py: Python<'_>,
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: Option<&SEOOverrides>,
) -> PyResult<SEOPayload> {
    let default_ov = core::SEOOverrides::default();
    let ov = overrides.map(|o| &o.inner).unwrap_or(&default_ov);
    let payload = core::build_seo_payload_with_overrides(&entity.inner, route, &config.inner, ov)
        .map_err(convert_core_error)?;
    let payload = apply_schema_registry(py, payload, entity, config)?;
    let payload = apply_hooks(py, payload, entity, config)?;
    maybe_emit_warnings(py, &payload, config)?;
    Ok(SEOPayload { inner: payload })
}

#[pyfunction]
fn build_seo_payload_with_overrides(
    py: Python<'_>,
    entity: &SEOEntity,
    route: &str,
    config: &SEOConfig,
    overrides: &SEOOverrides,
) -> PyResult<SEOPayload> {
    let payload = core::build_seo_payload_with_overrides(
        &entity.inner,
        route,
        &config.inner,
        &overrides.inner,
    )
    .map_err(convert_core_error)?;
    let payload = apply_schema_registry(py, payload, entity, config)?;
    let payload = apply_hooks(py, payload, entity, config)?;
    maybe_emit_warnings(py, &payload, config)?;
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
    let payload = build_seo_payload(py, entity, route, config, overrides)?;
    payload.to_dict(py)
}

#[pyfunction]
fn build_seo_contract(config: &SEOContractConfig) -> PyResult<SEOContract> {
    let contract = core::build_seo_contract(&config.inner).map_err(convert_core_error)?;
    Ok(SEOContract { inner: contract })
}

#[pyfunction]
fn validate_payload(payload: &SEOPayload) -> Vec<SEOIssue> {
    core::validate_payload(&payload.inner)
        .iter()
        .map(|i| i.into())
        .collect()
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
    let dict = PyDict::new(py);
    dict.set_item("url", &result.url)?;
    let removed = PyDict::new(py);
    for (k, v) in &result.removed_params {
        removed.set_item(k.as_str(), v.as_str())?;
    }
    dict.set_item("removed_params", removed)?;
    let cleaned = PyDict::new(py);
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
    // Exceptions (built dynamically so they inherit from ValueError too)
    let types = exception_types(m.py())?;
    m.add("EaseoError", types.easeo.clone_ref(m.py()))?;
    m.add("InvalidUrlError", types.invalid_url.clone_ref(m.py()))?;
    m.add("ConfigurationError", types.configuration.clone_ref(m.py()))?;
    m.add("EntityError", types.entity.clone_ref(m.py()))?;
    m.add("SchemaError", types.schema.clone_ref(m.py()))?;
    m.add("ContractError", types.contract.clone_ref(m.py()))?;

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
