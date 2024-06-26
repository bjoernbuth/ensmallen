#[allow(unused_variables)]
use super::*;
use pyo3::class::basic::CompareOp;
#[allow(unused_imports)]
use pyo3::{wrap_pyfunction, wrap_pymodule};
use rayon::iter::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use strsim::*;

/// Returns the given method name separated in the component parts.
///
/// # Implementative details
/// The methods contains terms such as:
/// * `node_name`
/// * `node_type_id`
/// * `node_id`
///
/// Since these terms are functionally a single word, we do not split
/// the terms composed by the words:
/// * `id` or `ids`
/// * `type` or `types`
/// * `name` or `names`
///
/// # Arguments
/// * `method_name`: &str - Name of the method to split.
fn split_words(method_name: &str) -> Vec<String> {
    method_name
        .split("_")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_lowercase())
        .collect()
}

pub fn register_ensmallen(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_class::<Chain>()?;
    _m.add_class::<Circle>()?;
    _m.add_class::<Clique>()?;
    _m.add_class::<DendriticTree>()?;
    _m.add_class::<Graph>()?;
    _m.add_class::<GraphBuilder>()?;
    _m.add_class::<GraphCSVBuilder>()?;
    _m.add_class::<NodeTuple>()?;
    _m.add_class::<ShortestPathsDjkstra>()?;
    _m.add_class::<ShortestPathsResultBFS>()?;
    _m.add_class::<Star>()?;
    _m.add_class::<Tendril>()?;
    env_logger::init();
    let submod = PyModule::new(_py, "edge_list_utils")?;
    register_edge_list_utils(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "utils")?;
    register_utils(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "preprocessing")?;
    register_preprocessing(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "models")?;
    register_models(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "express_measures")?;
    register_express_measures(_py, submod)?;
    _m.add_submodule(submod)?;
    Ok(())
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct Chain {
    pub inner: graph::Chain,
}

impl From<graph::Chain> for Chain {
    fn from(val: graph::Chain) -> Chain {
        Chain { inner: val }
    }
}

impl From<Chain> for graph::Chain {
    fn from(val: Chain) -> graph::Chain {
        val.inner
    }
}

impl<'a> From<&'a Chain> for &'a graph::Chain {
    fn from(val: &'a Chain) -> &'a graph::Chain {
        &val.inner
    }
}

#[pymethods]
impl Chain {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the chain
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the chain
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the chain
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the chain
    pub fn get_chain_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_chain_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the chain.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_chain_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_chain_node_ids(k.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the chain.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_chain_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_chain_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node names of the nodes composing the chain
    pub fn get_chain_node_names(&self) -> Vec<String> {
        self.inner
            .get_chain_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const CHAIN_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_chain_node_ids",
    "get_first_k_chain_node_ids",
    "get_first_k_chain_node_names",
    "get_chain_node_names",
];

pub const CHAIN_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "chain", "ids", "first", "k", "names",
];

pub const CHAIN_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("chain", 0.21014561),
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
    ],
    &[
        ("chain", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("chain", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("chain", 0.21014561),
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
    ],
];

#[pymethods]
impl Chain {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Chain {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = CHAIN_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = CHAIN_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, CHAIN_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", CHAIN_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct Circle {
    pub inner: graph::Circle,
}

impl From<graph::Circle> for Circle {
    fn from(val: graph::Circle) -> Circle {
        Circle { inner: val }
    }
}

impl From<Circle> for graph::Circle {
    fn from(val: Circle) -> graph::Circle {
        val.inner
    }
}

impl<'a> From<&'a Circle> for &'a graph::Circle {
    fn from(val: &'a Circle) -> &'a graph::Circle {
        &val.inner
    }
}

#[pymethods]
impl Circle {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the Circle
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the circle
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Circle
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the Circle
    pub fn get_circle_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_circle_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the Circle.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_circle_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_circle_node_ids(k.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the Circle.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_circle_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_circle_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node names of the nodes composing the Circle
    pub fn get_circle_node_names(&self) -> Vec<String> {
        self.inner
            .get_circle_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const CIRCLE_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_circle_node_ids",
    "get_first_k_circle_node_ids",
    "get_first_k_circle_node_names",
    "get_circle_node_names",
];

pub const CIRCLE_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "circle", "ids", "first", "k", "names",
];

pub const CIRCLE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("circle", 0.21014561),
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
    ],
    &[
        ("circle", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("circle", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("circle", 0.21014561),
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
    ],
];

#[pymethods]
impl Circle {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Circle {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = CIRCLE_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = CIRCLE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, CIRCLE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", CIRCLE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct Clique {
    pub inner: graph::Clique,
}

impl From<graph::Clique> for Clique {
    fn from(val: graph::Clique) -> Clique {
        Clique { inner: val }
    }
}

impl From<Clique> for graph::Clique {
    fn from(val: Clique) -> graph::Clique {
        val.inner
    }
}

impl<'a> From<&'a Clique> for &'a graph::Clique {
    fn from(val: &'a Clique) -> &'a graph::Clique {
        &val.inner
    }
}

#[pymethods]
impl Clique {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Clique
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the clique
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node names of the nodes composing the Clique
    pub fn get_node_names(&self) -> Vec<String> {
        self.inner
            .get_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const CLIQUE_METHODS_NAMES: &[&str] = &["len", "get_node_ids", "get_node_names"];

pub const CLIQUE_TERMS: &[&str] = &["len", "get", "node", "ids", "names"];

pub const CLIQUE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[("len", 1.3203471)],
    &[
        ("get", 0.18176937),
        ("ids", 0.37932625),
        ("node", 0.18176937),
    ],
    &[
        ("get", 0.18176937),
        ("names", 0.37932625),
        ("node", 0.18176937),
    ],
];

#[pymethods]
impl Clique {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Clique {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = CLIQUE_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = CLIQUE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, CLIQUE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", CLIQUE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct DendriticTree {
    pub inner: graph::DendriticTree,
}

impl From<graph::DendriticTree> for DendriticTree {
    fn from(val: graph::DendriticTree) -> DendriticTree {
        DendriticTree { inner: val }
    }
}

impl From<DendriticTree> for graph::DendriticTree {
    fn from(val: DendriticTree) -> graph::DendriticTree {
        val.inner
    }
}

impl<'a> From<&'a DendriticTree> for &'a graph::DendriticTree {
    fn from(val: &'a DendriticTree) -> &'a graph::DendriticTree {
        &val.inner
    }
}

#[pymethods]
impl DendriticTree {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the type of the dendritic tree
    pub fn get_dendritic_tree_type(&self) -> String {
        self.inner.get_dendritic_tree_type().to_string()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the root node ID of the dendritic tree
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a tree
    pub fn is_tree(&self) -> bool {
        self.inner.is_tree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a tendril
    pub fn is_tendril(&self) -> bool {
        self.inner.is_tendril().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is a proper dentritic tree
    pub fn is_dendritic_tree(&self) -> bool {
        self.inner.is_dendritic_tree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a free-floating chain
    pub fn is_free_floating_chain(&self) -> bool {
        self.inner.is_free_floating_chain().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a star
    pub fn is_star(&self) -> bool {
        self.inner.is_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a star of tendrils
    pub fn is_tendril_star(&self) -> bool {
        self.inner.is_tendril_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a dendritic star
    pub fn is_dendritic_star(&self) -> bool {
        self.inner.is_dendritic_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a dendritic tendril star
    pub fn is_dendritic_tendril_star(&self) -> bool {
        self.inner.is_dendritic_tendril_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the depth of the dentritic tree
    pub fn get_depth(&self) -> NodeT {
        self.inner.get_depth().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the root node name of the DendriticTree
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of nodes involved in the dendritic tree
    pub fn get_number_of_involved_nodes(&self) -> NodeT {
        self.inner.get_number_of_involved_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of edges involved in the dendritic tree
    pub fn get_number_of_involved_edges(&self) -> EdgeT {
        self.inner.get_number_of_involved_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the DendriticTree
    pub fn get_dentritic_trees_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_dentritic_trees_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the DendriticTree.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_dentritic_trees_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_dentritic_trees_node_ids(k.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the DendriticTree.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_dentritic_trees_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_dentritic_trees_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node names of the nodes composing the DendriticTree
    pub fn get_dentritic_trees_node_names(&self) -> Vec<String> {
        self.inner
            .get_dentritic_trees_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const DENDRITICTREE_METHODS_NAMES: &[&str] = &[
    "get_dendritic_tree_type",
    "get_root_node_id",
    "is_tree",
    "is_tendril",
    "is_dendritic_tree",
    "is_free_floating_chain",
    "is_star",
    "is_tendril_star",
    "is_dendritic_star",
    "is_dendritic_tendril_star",
    "get_depth",
    "get_root_node_name",
    "get_number_of_involved_nodes",
    "get_number_of_involved_edges",
    "get_dentritic_trees_node_ids",
    "get_first_k_dentritic_trees_node_ids",
    "get_first_k_dentritic_trees_node_names",
    "get_dentritic_trees_node_names",
];

pub const DENDRITICTREE_TERMS: &[&str] = &[
    "get",
    "dendritic",
    "tree",
    "type",
    "root",
    "node",
    "id",
    "is",
    "tendril",
    "free",
    "floating",
    "chain",
    "star",
    "depth",
    "name",
    "number",
    "of",
    "involved",
    "nodes",
    "edges",
    "dentritic",
    "trees",
    "ids",
    "first",
    "k",
    "names",
];

pub const DENDRITICTREE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("dendritic", 0.509799),
        ("get", 0.2099079),
        ("tree", 0.59874874),
        ("type", 0.8986398),
    ],
    &[
        ("get", 0.2099079),
        ("id", 0.8986398),
        ("node", 0.37964714),
        ("root", 0.7178391),
    ],
    &[("is", 0.6956208), ("tree", 1.4629598)],
    &[("is", 0.6956208), ("tendril", 1.4629598)],
    &[
        ("dendritic", 0.76747227),
        ("is", 0.4285964),
        ("tree", 0.9013809),
    ],
    &[
        ("chain", 0.8986398),
        ("floating", 0.8986398),
        ("free", 0.8986398),
        ("is", 0.28469825),
    ],
    &[("is", 0.6956208), ("star", 1.2456234)],
    &[
        ("is", 0.4285964),
        ("star", 0.76747227),
        ("tendril", 0.9013809),
    ],
    &[
        ("dendritic", 0.76747227),
        ("is", 0.4285964),
        ("star", 0.76747227),
    ],
    &[
        ("dendritic", 0.509799),
        ("is", 0.28469825),
        ("star", 0.509799),
        ("tendril", 0.59874874),
    ],
    &[("depth", 2.195702), ("get", 0.5128809)],
    &[
        ("get", 0.2099079),
        ("name", 0.8986398),
        ("node", 0.37964714),
        ("root", 0.7178391),
    ],
    &[
        ("get", 0.14818765),
        ("involved", 0.50676936),
        ("nodes", 0.63440835),
        ("number", 0.50676936),
        ("of", 0.50676936),
    ],
    &[
        ("edges", 0.63440835),
        ("get", 0.14818765),
        ("involved", 0.50676936),
        ("number", 0.50676936),
        ("of", 0.50676936),
    ],
    &[
        ("dentritic", 0.3599003),
        ("get", 0.14818765),
        ("ids", 0.50676936),
        ("node", 0.26801765),
        ("trees", 0.3599003),
    ],
    &[
        ("dentritic", 0.20459273),
        ("first", 0.2880835),
        ("get", 0.084240325),
        ("ids", 0.2880835),
        ("k", 0.2880835),
        ("node", 0.15236016),
        ("trees", 0.20459273),
    ],
    &[
        ("dentritic", 0.20459273),
        ("first", 0.2880835),
        ("get", 0.084240325),
        ("k", 0.2880835),
        ("names", 0.2880835),
        ("node", 0.15236016),
        ("trees", 0.20459273),
    ],
    &[
        ("dentritic", 0.3599003),
        ("get", 0.14818765),
        ("names", 0.50676936),
        ("node", 0.26801765),
        ("trees", 0.3599003),
    ],
];

#[pymethods]
impl DendriticTree {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl DendriticTree {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = DENDRITICTREE_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = DENDRITICTREE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, DENDRITICTREE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        DENDRITICTREE_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
/// This is the main struct in Ensmallen, it allows to load and manipulate Graphs efficently.
///  You are not supposed to directly instantiate this struct but instead you should use the
///  static method `from_csv`, which allows to load the graph from an edge-list.
///
///  To get information about a loaded graph, you can call the `textual_report` method which
///  generates an human-readable HTML report.
///
///  By default we use EliasFano to store the Adjacency Matrix, this allows to save memory but
///  is slower than a CSR. For this reason you can use the `enable` method to enable optimizzations
///  which speeds up the operations at the cost of more memory usage. You can check the memory usage
///  in bytes using `get_total_memory_used` and you can get a detailed memory report of each data-structure
///  inside Graph using `memory_stats`.
///
///  You can pre-compute the memory needed (in bits) to store the adjacency matrix of a Graph with $|E|$ edges and $|V|$ nodes:
///   $$2 |E| + |E| \\left\\lceil \\log_2 \\frac{|V|^2}{|E|} \\right\\rceil$$
///
///  Most Graph properties are automatically cached to speed up.
#[derive(Debug, Clone)]
pub struct Graph {
    pub inner: graph::Graph,
}

impl From<graph::Graph> for Graph {
    fn from(val: graph::Graph) -> Graph {
        Graph { inner: val }
    }
}

impl From<Graph> for graph::Graph {
    fn from(val: Graph) -> graph::Graph {
        val.inner
    }
}

impl<'a> From<&'a Graph> for &'a graph::Graph {
    fn from(val: &'a Graph) -> &'a graph::Graph {
        &val.inner
    }
}

#[pymethods]
impl Graph {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is not a singleton nor a singleton with selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_connected_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner
            .is_unchecked_connected_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is not a singleton nor a singleton with selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn is_connected_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.is_connected_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a singleton or a singleton with selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_disconnected_node_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner
            .is_unchecked_disconnected_node_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_singleton_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner
            .is_unchecked_singleton_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    pub fn is_singleton_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.is_singleton_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    pub unsafe fn is_unchecked_singleton_with_selfloops_from_node_id(
        &self,
        node_id: NodeT,
    ) -> bool {
        self.inner
            .is_unchecked_singleton_with_selfloops_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    pub fn is_singleton_with_selfloops_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .is_singleton_with_selfloops_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Nota that this method will raise a panic if caled with unproper
    /// parametrization.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node name does not exist in the graph this method will panic.
    pub unsafe fn is_unchecked_singleton_from_node_name(&self, node_name: &str) -> bool {
        self.inner
            .is_unchecked_singleton_from_node_name(node_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to be checked for.
    ///
    pub fn is_singleton_from_node_name(&self, node_name: &str) -> PyResult<bool> {
        Ok(pe!(self.inner.is_singleton_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns whether the graph has the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Name of the node.
    ///
    pub fn has_node_name(&self, node_name: &str) -> bool {
        self.inner.has_node_name(node_name).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Returns whether the graph has the given node type id.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     id of the node.
    ///
    pub fn has_node_type_id(&self, node_type_id: NodeTypeT) -> bool {
        self.inner.has_node_type_id(node_type_id.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Returns whether the graph has the given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     Name of the node.
    ///
    pub fn has_node_type_name(&self, node_type_name: &str) -> bool {
        self.inner.has_node_type_name(node_type_name).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns whether the graph has the given edge type id.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     id of the edge.
    ///
    pub fn has_edge_type_id(&self, edge_type_id: EdgeTypeT) -> bool {
        self.inner.has_edge_type_id(edge_type_id.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns whether the graph has the given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     Name of the edge.
    ///
    pub fn has_edge_type_name(&self, edge_type_name: &str) -> bool {
        self.inner.has_edge_type_name(edge_type_name).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns whether edge passing between given node ids exists.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node id.
    /// dst: int
    ///     Destination node id.
    ///
    pub fn has_edge_from_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.inner
            .has_edge_from_node_ids(src.clone(), dst.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns whether the given node ID has a selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Source node id.
    ///
    pub fn has_selfloop_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner.has_selfloop_from_node_id(node_id.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
    /// Returns whether edge with the given type passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node of the edge.
    /// dst: int
    ///     The destination node of the edge.
    /// edge_type: Optional[int]
    ///     The (optional) edge type.
    ///
    pub fn has_edge_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> bool {
        self.inner
            .has_edge_from_node_ids_and_edge_type_id(src.clone(), dst.clone(), edge_type)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a trap.
    ///
    /// If the provided node_id is higher than the number of nodes in the graph,
    /// the method will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_trap_node_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner
            .is_unchecked_trap_node_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a trap.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_trap_node_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.is_trap_node_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a trap.
    ///
    /// If the provided node_id is higher than the number of nodes in the graph,
    /// the method will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_trap_node_with_selfloops_from_node_id(
        &self,
        node_id: NodeT,
    ) -> bool {
        self.inner
            .is_unchecked_trap_node_with_selfloops_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a trap with selfloops.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_trap_node_with_selfloops_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .is_trap_node_with_selfloops_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_id, second_node_id)")]
    /// Returns whether two provided nodes IDs are isomorphic to one another.
    ///
    /// Parameters
    /// ----------
    /// first_node_id: int
    ///     The first node to check for.
    /// second_node_id: int
    ///     The first node to check for.
    ///
    ///
    /// Safety
    /// ------
    /// This method assumes that the two provided node IDs are effectively within
    ///  the set of nodes in the graph. Out of bound errors might be raised with
    ///  improper parametrization of the method.
    pub unsafe fn are_unchecked_isomorphic_from_node_ids(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> bool {
        self.inner
            .are_unchecked_isomorphic_from_node_ids(first_node_id.clone(), second_node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_id, second_node_id)")]
    /// Returns whether two provided nodes IDs are isomorphic to one another.
    ///
    /// Parameters
    /// ----------
    /// first_node_id: int
    ///     The first node to check for.
    /// second_node_id: int
    ///     The first node to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     ValueError: This method assumes that the two provided node IDs are effectively within
    ///
    pub fn are_isomorphic_from_node_ids(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .are_isomorphic_from_node_ids(first_node_id.clone(), second_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
    /// Returns whether two provided nodes names are isomorphic to one another.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     The first node name to check for.
    /// second_node_name: str
    ///     The first node name to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     ValueError: This method assumes that the two provided node names are effectively within
    ///
    pub fn are_isomorphic_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .are_isomorphic_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name, node_type_name)")]
    /// Returns whether the given node name and node type name exist in current graph.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name.
    /// node_type_name: Optional[List[str]]
    ///     The node types name.
    ///
    pub fn has_node_name_and_node_type_name(
        &self,
        node_name: &str,
        node_type_name: Option<Vec<String>>,
    ) -> bool {
        self.inner
            .has_node_name_and_node_type_name(node_name, node_type_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name)")]
    /// Returns whether if edge passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src_name: str
    ///     The source node name of the edge.
    /// dst_name: str
    ///     The destination node name of the edge.
    ///
    pub fn has_edge_from_node_names(&self, src_name: &str, dst_name: &str) -> bool {
        self.inner
            .has_edge_from_node_names(src_name, dst_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name, edge_type_name)")]
    /// Returns whether if edge with type passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src_name: str
    ///     The source node name of the edge.
    /// dst_name: str
    ///     The destination node name of the edge.
    /// edge_type_name: Optional[&str]
    ///     The (optional) edge type name.
    ///
    pub fn has_edge_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> bool {
        self.inner
            .has_edge_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, edge_type_id)")]
    /// Returns whether a given node ID has at least an edge of the given edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node of which to check connected edges' type.
    /// edge_type_id: Optional[int]
    ///     The edge type to look for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given source node does not exist in the current graph.
    /// ValueError
    ///     If the given edge type does not exist in the current graph.
    ///
    pub fn has_edge_from_node_id_and_edge_type_id(
        &self,
        src: NodeT,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .has_edge_from_node_id_and_edge_type_id(src.clone(), edge_type_id))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, edge_type_id)")]
    /// Returns whether a given node ID has at least an edge of the given edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node of which to check connected edges' type.
    /// edge_type_id: Optional[int]
    ///     The edge type to look for.
    ///
    ///
    /// Safety
    /// ------
    /// When
    pub unsafe fn has_unchecked_edge_from_node_id_and_edge_type_id(
        &self,
        src: NodeT,
        edge_type_id: Option<EdgeTypeT>,
    ) -> bool {
        self.inner
            .has_unchecked_edge_from_node_id_and_edge_type_id(src.clone(), edge_type_id)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns list of nodes of the various strongly connected components.
    ///
    /// This is an implementation of Tarjan algorithm.
    pub fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        self.inner
            .strongly_connected_components()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Jaccard coo matrix
    pub fn get_jaccard_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_jaccard_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns jaccard weighted graph
    pub fn get_jaccard_graph(&self) -> Graph {
        self.inner.get_jaccard_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns neighbours intersection size coo matrix
    pub fn get_neighbours_intersection_size_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_neighbours_intersection_size_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns neighbours intersection size weighted graph
    pub fn get_neighbours_intersection_size_graph(&self) -> Graph {
        self.inner.get_neighbours_intersection_size_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns shared ancestors size coo matrix.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_shared_ancestors_size_coo_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) =
            self.inner.get_shared_ancestors_size_coo_matrix(bfs.into());
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns shared ancestors size weighted graph.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_shared_ancestors_size_graph(&self, bfs: &ShortestPathsResultBFS) -> Graph {
        self.inner
            .get_shared_ancestors_size_graph(bfs.into())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns Ancestors Jaccard coo matrix.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_ancestors_jaccard_coo_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_ancestors_jaccard_coo_matrix(bfs.into());
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns Ancestors Jaccard weighted graph.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_ancestors_jaccard_graph(&self, bfs: &ShortestPathsResultBFS) -> Graph {
        self.inner.get_ancestors_jaccard_graph(bfs.into()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Adamic-adar coo matrix
    pub fn get_adamic_adar_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_adamic_adar_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Adamic-Adar weighted graph
    pub fn get_adamic_adar_graph(&self) -> Graph {
        self.inner.get_adamic_adar_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Laplacian coo matrix
    pub fn get_laplacian_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Laplacian weighted graph
    pub fn get_laplacian_graph(&self) -> Graph {
        self.inner.get_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns left normalized Laplacian coo matrix
    pub fn get_left_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_left_normalized_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns left normalized Laplacian weighted graph
    pub fn get_left_normalized_laplacian_graph(&self) -> Graph {
        self.inner.get_left_normalized_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns right normalized Laplacian coo matrix
    pub fn get_right_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_right_normalized_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns right normalized Laplacian weighted graph
    pub fn get_right_normalized_laplacian_graph(&self) -> Graph {
        self.inner.get_right_normalized_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns symmetric normalized Laplacian coo matrix
    pub fn get_symmetric_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_symmetric_normalized_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns symmetric normalized Laplacian weighted graph
    pub fn get_symmetric_normalized_laplacian_graph(&self) -> Graph {
        self.inner.get_symmetric_normalized_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns graph with node IDs sorted by increasing outbound node degree
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        self.inner.sort_by_increasing_outbound_node_degree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns graph with node IDs sorted by decreasing outbound node degree
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        self.inner.sort_by_decreasing_outbound_node_degree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns graph with node IDs sorted by lexicographic order
    pub fn sort_by_node_lexicographic_order(&self) -> Graph {
        self.inner.sort_by_node_lexicographic_order().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, root_node_id)")]
    /// Returns topological sorting map using breadth-first search from the given node.
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int
    ///     Node ID of node to be used as root of BFS
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given root node ID does not exist in the graph
    ///
    pub fn get_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_bfs_topological_sorting_from_node_id(root_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, root_node_id)")]
    /// Returns topological sorting reversed map using breadth-first search from the given node.
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int
    ///     Node ID of node to be used as root of BFS
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given root node ID does not exist in the graph
    ///
    pub fn get_reversed_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_reversed_bfs_topological_sorting_from_node_id(root_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, root_node_id)")]
    /// Returns graph with node IDs sorted using a BFS
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int
    ///     Node ID of node to be used as root of BFS
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given root node ID does not exist in the graph
    ///
    pub fn sort_by_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .sort_by_bfs_topological_sorting_from_node_id(root_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, path, source_column_name, destination_column_name, prediction_column_name, allowed_source_nodes, allowed_destination_nodes, allowed_source_node_prefixes, allowed_destination_node_prefixes, min_prediction, max_prediction, bins, separator, remove_chevrons, remove_spaces, support_balanced_quotes, comment_symbol, max_rows_number)"
    )]
    /// Return the number of existing and non-existing edges in the provided file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     Path to the file to read.
    /// source_column_name: Optional[str]
    ///     Name of the source column.
    /// destination_column_name: Optional[str]
    ///     Name of the destination column.
    /// prediction_column_name: Optional[str]
    ///     Name of the prediction column.
    /// allowed_source_nodes: Optional[List[str]]
    ///     List of allowed source nodes.
    /// allowed_destination_nodes: Optional[List[str]]
    ///     List of allowed destination nodes.
    /// allowed_source_node_prefixes: Optional[List[str]]
    ///     List of allowed source node prefixes.
    /// allowed_destination_node_prefixes: Optional[List[str]]
    ///     List of allowed destination node prefixes.
    /// min_prediction: Optional[float]
    ///     Minimum prediction to consider.
    /// max_prediction: Optional[float]
    ///     Maximum prediction to consider.
    /// bins: Optional[int]
    ///     Number of bins to use for the histogram.
    /// separator: Optional[str]
    ///     Separator used in the file.
    /// remove_chevrons: Optional[bool]
    ///     Whether to remove chevrons from the file.
    /// remove_spaces: Optional[bool]
    ///     Whether to remove spaces from the file.
    /// support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes in the file.
    /// comment_symbol: Optional[str]
    ///     Symbol used to indicate that a line is a comment.
    /// max_rows_number: Optional[int]
    ///     Maximum number of rows to read.
    ///
    pub fn get_predictions_histograms(
        &self,
        path: String,
        source_column_name: Option<String>,
        destination_column_name: Option<String>,
        prediction_column_name: Option<String>,
        allowed_source_nodes: Option<Vec<String>>,
        allowed_destination_nodes: Option<Vec<String>>,
        allowed_source_node_prefixes: Option<Vec<String>>,
        allowed_destination_node_prefixes: Option<Vec<String>>,
        min_prediction: Option<f32>,
        max_prediction: Option<f32>,
        bins: Option<usize>,
        separator: Option<char>,
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        support_balanced_quotes: Option<bool>,
        comment_symbol: Option<String>,
        max_rows_number: Option<usize>,
    ) -> PyResult<(Py<PyArray1<usize>>, Py<PyArray1<usize>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_predictions_histograms(
                path.into(),
                source_column_name,
                destination_column_name,
                prediction_column_name,
                allowed_source_nodes,
                allowed_destination_nodes,
                allowed_source_node_prefixes,
                allowed_destination_node_prefixes,
                min_prediction,
                max_prediction,
                bins,
                separator,
                remove_chevrons,
                remove_spaces,
                support_balanced_quotes,
                comment_symbol,
                max_rows_number
            ))?
            .into();
            (
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_0, usize)
                },
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_1, usize)
                },
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, path, source_column_name, destination_column_name, prediction_column_name, allowed_source_nodes, allowed_destination_nodes, allowed_source_node_prefixes, allowed_destination_node_prefixes, min_prediction, max_prediction, exclude_existing_edges, exclude_non_existing_edges, separator, remove_chevrons, remove_spaces, support_balanced_quotes, comment_symbol, max_rows_number)"
    )]
    /// Returns triple with source, destination and predictions according to provided filters.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     Path to the file to read.
    /// source_column_name: Optional[str]
    ///     Name of the source column.
    /// destination_column_name: Optional[str]
    ///     Name of the destination column.
    /// prediction_column_name: Optional[str]
    ///     Name of the prediction column.
    /// allowed_source_nodes: Optional[List[str]]
    ///     List of allowed source nodes.
    /// allowed_destination_nodes: Optional[List[str]]
    ///     List of allowed destination nodes.
    /// allowed_source_node_prefixes: Optional[List[str]]
    ///     List of allowed source node prefixes.
    /// allowed_destination_node_prefixes: Optional[List[str]]
    ///     List of allowed destination node prefixes.
    /// min_prediction: Optional[float]
    ///     Minimum prediction to consider.
    /// max_prediction: Optional[float]
    ///     Maximum prediction to consider.
    /// exclude_existing_edges: Optional[bool]
    ///     Whether to exclude existing edges.
    /// exclude_non_existing_edges: Optional[bool]
    ///     Whether to exclude non existing edges.
    /// separator: Optional[str]
    ///     Separator used in the file.
    /// remove_chevrons: Optional[bool]
    ///     Whether to remove chevrons from the file.
    /// remove_spaces: Optional[bool]
    ///     Whether to remove spaces from the file.
    /// support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes in the file.
    /// comment_symbol: Optional[str]
    ///     Symbol used to indicate that a line is a comment.
    /// max_rows_number: Optional[int]
    ///     Maximum number of rows to read.
    ///
    pub fn get_filtered_predictions(
        &self,
        path: String,
        source_column_name: Option<String>,
        destination_column_name: Option<String>,
        prediction_column_name: Option<String>,
        allowed_source_nodes: Option<Vec<String>>,
        allowed_destination_nodes: Option<Vec<String>>,
        allowed_source_node_prefixes: Option<Vec<String>>,
        allowed_destination_node_prefixes: Option<Vec<String>>,
        min_prediction: Option<f32>,
        max_prediction: Option<f32>,
        exclude_existing_edges: Option<bool>,
        exclude_non_existing_edges: Option<bool>,
        separator: Option<char>,
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        support_balanced_quotes: Option<bool>,
        comment_symbol: Option<String>,
        max_rows_number: Option<usize>,
    ) -> PyResult<Vec<(String, String, f32)>> {
        Ok(pe!(self.inner.get_filtered_predictions(
            path.into(),
            source_column_name,
            destination_column_name,
            prediction_column_name,
            allowed_source_nodes,
            allowed_destination_nodes,
            allowed_source_node_prefixes,
            allowed_destination_node_prefixes,
            min_prediction,
            max_prediction,
            exclude_existing_edges,
            exclude_non_existing_edges,
            separator,
            remove_chevrons,
            remove_spaces,
            support_balanced_quotes,
            comment_symbol,
            max_rows_number
        ))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1, subresult_2) = x;
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_names, node_types, edge_types, minimum_component_size, top_k_components)"
    )]
    /// Return a new graph with solely the requested connected components.
    ///
    /// Parameters
    /// ----------
    /// node_names: Optional[List[str]]
    ///     The name of the nodes of which components to keep.
    /// node_types: Optional[&List[Optional[&str]]]
    ///     The types of the nodes of which components to keep.
    /// edge_types: Optional[&List[Optional[&str]]]
    ///     The types of the edges of which components to keep.
    /// minimum_component_size: Optional[int]
    ///     Optional, Minimum size of the components to keep.
    /// top_k_components: Optional[int]
    ///     Optional, number of components to keep sorted by number of nodes.
    ///
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<&str>>>,
        edge_types: Option<Vec<Option<&str>>>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_components(
            node_names,
            node_types.as_ref().map(|x| x.as_slice()),
            edge_types.as_ref().map(|x| x.as_slice()),
            minimum_component_size,
            top_k_components
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Return whether given graph has any edge overlapping with current graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The graph to check against.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If a graph is directed and the other is undirected.
    /// ValueError
    ///     If one of the two graphs has edge weights and the other does not.
    /// ValueError
    ///     If one of the two graphs has node types and the other does not.
    /// ValueError
    ///     If one of the two graphs has edge types and the other does not.
    ///
    pub fn overlaps(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.overlaps(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Return true if given graph edges are all contained within current graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The graph to check against.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If a graph is directed and the other is undirected.
    /// ValueError
    ///     If one of the two graphs has edge weights and the other does not.
    /// ValueError
    ///     If one of the two graphs has node types and the other does not.
    /// ValueError
    ///     If one of the two graphs has edge types and the other does not.
    ///
    pub fn contains(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.contains(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"
    )]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameters
    /// ----------
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// first_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the second set of nodes of the graph.
    ///
    pub fn get_bipartite_edges(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_bipartite_edges(
                    removed_existing_edges,
                    first_nodes_set,
                    second_nodes_set,
                    first_node_types_set,
                    second_node_types_set
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"
    )]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameters
    /// ----------
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// first_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the second set of nodes of the graph.
    ///
    pub fn get_bipartite_edge_names(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_bipartite_edge_names(
            removed_existing_edges,
            first_nodes_set,
            second_nodes_set,
            first_node_types_set,
            second_node_types_set
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"
    )]
    /// Return vector of tuple of Node IDs that form the edges of the required star.
    ///
    /// Parameters
    /// ----------
    /// central_node: str
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the set of star points.
    ///
    pub fn get_star_edges(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_star_edges(
                    central_node.into(),
                    removed_existing_edges,
                    star_points_nodes_set,
                    star_points_node_types_set
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"
    )]
    /// Return vector of tuple of Node names that form the edges of the required star.
    ///
    /// Parameters
    /// ----------
    /// central_node: str
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the set of star points.
    ///
    pub fn get_star_edge_names(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_star_edge_names(
            central_node.into(),
            removed_existing_edges,
            star_points_nodes_set,
            star_points_node_types_set
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"
    )]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ----------
    /// directed: Optional[bool]
    ///     Whether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_selfloops: Optional[bool]
    ///     Whether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Optional[Set[str]]
    ///     Node types to include in the clique.
    /// allow_node_set: Optional[Set[str]]
    ///     Nodes to include i the clique.
    ///
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        allow_selfloops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(
            gil,
            self.inner.get_clique_edges(
                directed,
                allow_selfloops,
                removed_existing_edges,
                allow_node_type_set,
                allow_node_set
            ),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"
    )]
    /// Return vector of tuple of Node names that form the edges of the required clique.
    ///
    /// Parameters
    /// ----------
    /// directed: Optional[bool]
    ///     Whether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_selfloops: Optional[bool]
    ///     Whether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Optional[Set[str]]
    ///     Node types to include in the clique.
    /// allow_node_set: Optional[Set[str]]
    ///     Nodes to include i the clique.
    ///
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        allow_selfloops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.inner
            .get_clique_edge_names(
                directed,
                allow_selfloops,
                removed_existing_edges,
                allow_node_type_set,
                allow_node_set,
            )
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Validates provided node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     node ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exists in the graph.
    ///
    pub fn validate_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.validate_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Validates all provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     node IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node ID does not exists in the graph.
    ///
    pub fn validate_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.validate_node_ids(node_ids))?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Validates provided edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     Edge ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge ID does not exists in the graph.
    ///
    pub fn validate_edge_id(&self, edge_id: EdgeT) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.validate_edge_id(edge_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_ids)")]
    /// Validates provided edge IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_ids: List[int]
    ///     Edge IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given edge ID does not exists in the graph.
    ///
    pub fn validate_edge_ids(&self, edge_ids: Vec<EdgeT>) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.validate_edge_ids(edge_ids))?, EdgeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph contains unknown node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain node types.
    /// ValueError
    ///     If the graph contains unknown node types.
    ///
    pub fn must_not_contain_unknown_node_types(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_not_contain_unknown_node_types())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph contains unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge types.
    /// ValueError
    ///     If the graph contains unknown edge types.
    ///
    pub fn must_not_contain_unknown_edge_types(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_not_contain_unknown_edge_types())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Validates provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: Optional[int]
    ///     Node type ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node type ID does not exists in the graph.
    ///
    pub fn validate_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> PyResult<Option<NodeTypeT>> {
        Ok(pe!(self.inner.validate_node_type_id(node_type_id))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids)")]
    /// Validates provided node type IDs.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids: List[Optional[int]]
    ///     Vector of node type IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn validate_node_type_ids(
        &self,
        node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        Ok(pe!(self.inner.validate_node_type_ids(&node_type_ids))?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Validates provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     edge type ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge type ID does not exists in the graph.
    ///
    pub fn validate_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self.inner.validate_edge_type_id(edge_type_id))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_ids)")]
    /// Validates provided edge type IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_type_ids: List[Optional[int]]
    ///     Vector of edge type IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn validate_edge_type_ids(
        &self,
        edge_type_ids: Vec<Option<EdgeTypeT>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.validate_edge_type_ids(edge_type_ids))?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph's nodes do not have detectable ontologies.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes with detectable ontologies.
    ///
    pub fn must_have_node_ontologies(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_have_node_ontologies())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph is not undirected.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is directed.
    ///
    pub fn must_be_undirected(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_be_undirected())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph is not a directed acyclic.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is directed.
    ///
    pub fn must_be_directed_acyclic(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_be_directed_acyclic())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph contains trap nodes.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph contains trap nodes.
    ///
    pub fn must_not_have_trap_nodes(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_not_have_trap_nodes())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph does not have edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not a multigraph.
    ///
    pub fn must_be_multigraph(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_be_multigraph())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph does not have edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn must_not_be_multigraph(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_not_be_multigraph())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph does not include the identity matrix.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn must_contain_identity_matrix(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_contain_identity_matrix())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph contains zero weighted degree.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edges.
    ///
    pub fn must_not_contain_weighted_singleton_nodes(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_not_contain_weighted_singleton_nodes())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph has a maximal weighted
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edges.
    ///
    pub fn must_have_edges(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_have_edges())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph does not have any node.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have nodes.
    ///
    pub fn must_have_nodes(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_have_nodes())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph is not connected.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not connected.
    ///
    pub fn must_be_connected(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_be_connected())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Raises an error if the provided graph does not a node vocabulary compatible with the current graph instance.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided graph does not share a compatible node vocabulary with the current instance.
    ///
    pub fn must_share_node_vocabulary(&self, other: &Graph) -> PyResult<()> {
        Ok(pe!(self.inner.must_share_node_vocabulary(&other.inner))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return total edge weights, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_total_edge_weights(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_total_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the minimum weight, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_mininum_edge_weight(&self) -> PyResult<WeightT> {
        Ok(pe!(self.inner.get_mininum_edge_weight())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the maximum weight, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_maximum_edge_weight(&self) -> PyResult<WeightT> {
        Ok(pe!(self.inner.get_maximum_edge_weight())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the maximum node degree.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    pub unsafe fn get_unchecked_maximum_node_degree(&self) -> NodeT {
        self.inner.get_unchecked_maximum_node_degree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the minimum node degree.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    pub unsafe fn get_unchecked_minimum_node_degree(&self) -> NodeT {
        self.inner.get_unchecked_minimum_node_degree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the maximum weighted node degree
    pub fn get_weighted_maximum_node_degree(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_maximum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the minimum weighted node degree
    pub fn get_weighted_minimum_node_degree(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_minimum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the number of weighted singleton nodes, i.e. nodes with weighted node degree equal to zero
    pub fn get_number_of_weighted_singleton_nodes(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_weighted_singleton_nodes())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of self-loops, including also those in eventual multi-edges.
    pub fn get_number_of_selfloops(&self) -> EdgeT {
        self.inner.get_number_of_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    pub fn get_number_of_unique_selfloops(&self) -> NodeT {
        self.inner.get_number_of_unique_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, features, neighbours_number, max_degree, distance_name, verbose)"
    )]
    /// Returns graph with edges added extracted from given node_features.
    ///
    /// This operation might distrupt the graph topology.
    /// Proceed with caution!
    ///
    /// Parameters
    /// ----------
    /// features: List[List[float]]
    ///     node_features to use to identify the new neighbours.
    /// neighbours_number: Optional[int]
    ///     Number of neighbours to add.
    /// max_degree: Optional[int]
    ///     The maximum degree a node can have its neighbours augmented. By default 0, that is, only singletons are augmented.
    /// distance_name: Optional[&str]
    ///     Name of distance to use. Can either be L2 or COSINE. By default COSINE.
    /// verbose: Optional[bool]
    ///     Whether to show loading bars.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have nodes.
    /// ValueError
    ///     If the given node_features are not provided exactly for each node.
    /// ValueError
    ///     If the node_features do not have a consistent shape.
    /// ValueError
    ///     If the provided number of neighbours is zero.
    ///
    pub fn generate_new_edges_from_node_features(
        &self,
        features: Vec<Vec<f64>>,
        neighbours_number: Option<NodeT>,
        max_degree: Option<NodeT>,
        distance_name: Option<&str>,
        verbose: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.generate_new_edges_from_node_features(
            features,
            neighbours_number,
            max_degree,
            distance_name,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, name)")]
    /// Set the name of the graph.
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     Name of the graph.
    ///
    pub fn set_name(&mut self, name: String) {
        self.inner.set_name(name.into());
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type)")]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This happens INPLACE, that is edits the current graph instance.
    ///
    /// Parameters
    /// ----------
    /// edge_type: str
    ///     The edge type to assing to all the edges.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edges.
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn set_inplace_all_edge_types(&mut self, edge_type: String) -> PyResult<()> {
        Ok({
            pe!(self.inner.set_inplace_all_edge_types(edge_type))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type)")]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// edge_type: str
    ///     The edge type to assing to all the edges.
    ///
    pub fn set_all_edge_types(&self, edge_type: String) -> PyResult<Graph> {
        Ok(pe!(self.inner.set_all_edge_types(edge_type))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type)")]
    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// Parameters
    /// ----------
    /// node_type: str
    ///     The node type to assing to all the nodes.
    ///
    pub fn set_inplace_all_node_types(&mut self, node_type: String) -> PyResult<()> {
        Ok({
            pe!(self.inner.set_inplace_all_node_types(node_type))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type)")]
    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// node_type: str
    ///     The node type to assing to all the nodes.
    ///
    pub fn set_all_node_types(&self, node_type: String) -> PyResult<Graph> {
        Ok(pe!(self.inner.set_all_node_types(node_type))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids_to_remove)")]
    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids_to_remove: List[int]
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type ID does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_ids(
        &mut self,
        node_type_ids_to_remove: Vec<NodeTypeT>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .remove_inplace_node_type_ids(node_type_ids_to_remove))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_singleton_node_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_singleton_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id, node_name_prefixes)")]
    /// Assigns inplace given node type id to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_id_from_node_name_prefixes_inplace(
        &mut self,
        node_type_id: NodeTypeT,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<()> {
        Ok({
            pe!(self.inner.add_node_type_id_from_node_name_prefixes_inplace(
                node_type_id.clone(),
                node_name_prefixes
            ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_id, source_node_type_ids, destination_node_type_ids)"
    )]
    /// Replaces inplace given edge type id to the nodes with given source and destination node type IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type ID to replace with.
    /// source_node_type_ids: List[Optional[int]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_ids: List[Optional[int]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_id_from_edge_node_type_ids_inplace(
        &mut self,
        edge_type_id: EdgeTypeT,
        source_node_type_ids: Vec<Option<NodeTypeT>>,
        destination_node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .replace_edge_type_id_from_edge_node_type_ids_inplace(
                    edge_type_id.clone(),
                    &source_node_type_ids,
                    &destination_node_type_ids
                ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_id, source_node_type_ids, destination_node_type_ids)"
    )]
    /// Replaces given edge type id to the nodes with given source and destination node type IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type ID to replace with.
    /// source_node_type_ids: List[Optional[int]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_ids: List[Optional[int]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_id_from_edge_node_type_ids(
        &mut self,
        edge_type_id: EdgeTypeT,
        source_node_type_ids: Vec<Option<NodeTypeT>>,
        destination_node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self.inner.replace_edge_type_id_from_edge_node_type_ids(
                edge_type_id.clone(),
                &source_node_type_ids,
                &destination_node_type_ids
            ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id, node_name_prefixes)")]
    /// Assigns given node type id to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_id_from_node_name_prefixes(
        &self,
        node_type_id: NodeTypeT,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .add_node_type_id_from_node_name_prefixes(node_type_id.clone(), node_name_prefixes))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Add node type name to the graph in place.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type name to add.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node type name already exists in the graph.
    ///
    pub fn add_node_type_name_inplace(&mut self, node_type_name: String) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.add_node_type_name_inplace(node_type_name.into()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name, node_name_prefixes)")]
    /// Assigns inplace given node type name to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_name_from_node_name_prefixes_inplace(
        &mut self,
        node_type_name: String,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .add_node_type_name_from_node_name_prefixes_inplace(
                    node_type_name.into(),
                    node_name_prefixes
                ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Add edge type name to the graph in place.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name to add.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge type name already exists in the graph.
    ///
    pub fn add_edge_type_name_inplace(&mut self, edge_type_name: String) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.add_edge_type_name_inplace(edge_type_name.into()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_name, source_node_type_names, destination_node_type_names)"
    )]
    /// Replaces inplace given edge type name to the nodes with given source and destination node type names.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name to replace with.
    /// source_node_type_names: List[Optional[&str]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_names: List[Optional[&str]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_name_from_edge_node_type_names_inplace(
        &mut self,
        edge_type_name: String,
        source_node_type_names: Vec<Option<&str>>,
        destination_node_type_names: Vec<Option<&str>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .replace_edge_type_name_from_edge_node_type_names_inplace(
                    edge_type_name.into(),
                    &source_node_type_names,
                    &destination_node_type_names
                ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_name, source_node_type_names, destination_node_type_names)"
    )]
    /// Replaces given edge type name to the nodes with given source and destination node type names.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name to replace with.
    /// source_node_type_names: List[Optional[&str]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_names: List[Optional[&str]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_name_from_edge_node_type_names(
        &mut self,
        edge_type_name: String,
        source_node_type_names: Vec<Option<&str>>,
        destination_node_type_names: Vec<Option<&str>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self.inner.replace_edge_type_name_from_edge_node_type_names(
                edge_type_name.into(),
                &source_node_type_names,
                &destination_node_type_names
            ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name, node_name_prefixes)")]
    /// Assigns given node type name to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_name_from_node_name_prefixes(
        &self,
        node_type_name: String,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.add_node_type_name_from_node_name_prefixes(
            node_type_name.into(),
            node_name_prefixes
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove homogeneous node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_homogeneous_node_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_homogeneous_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_ids_to_remove)")]
    /// Remove given edge type ID from all edges.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_ids(
        &mut self,
        edge_type_ids_to_remove: Vec<EdgeTypeT>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .remove_inplace_edge_type_ids(edge_type_ids_to_remove))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_inplace_singleton_edge_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_singleton_edge_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Remove given node type names from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[&str]
    ///     The node type names to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_names(&mut self, node_type_names: Vec<&str>) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_node_type_names(node_type_names))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type names to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_name(&mut self, node_type_name: &str) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_node_type_name(node_type_name))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type ID does not exists in the graph.
    ///
    pub fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_type_id(node_type_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_singleton_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_singleton_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove homogeneous node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_homogeneous_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_homogeneous_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove inplace isomorphic node types.
    ///
    /// This will leave for each isomorphic node tyoe group only an element.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_isomorphic_node_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_isomorphic_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove isomorphic node types.
    ///
    /// This will leave for each isomorphic node tyoe group only an element.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_isomorphic_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_isomorphic_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Remove inplace isomorphic edge types.
    ///
    /// This will leave for each isomorphic edge tyoe group only an element.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_inplace_isomorphic_edge_types(
        &mut self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .remove_inplace_isomorphic_edge_types(minimum_number_of_edges))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Remove isomorphic edge types.
    ///
    /// This will leave for each isomorphic edge tyoe group only an element.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_isomorphic_edge_types(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .remove_isomorphic_edge_types(minimum_number_of_edges))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Remove given node type names from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[&str]
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_node_type_names(&self, node_type_names: Vec<&str>) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_type_names(node_type_names))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_node_type_name(&self, node_type_name: &str) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_type_name(node_type_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type name does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_name(&mut self, edge_type_name: &str) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_edge_type_name(edge_type_name))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Remove given edge type ID from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_type_id(edge_type_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_singleton_edge_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_singleton_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type name does not exists in the graph.
    ///
    pub fn remove_edge_type_name(&self, edge_type_name: &str) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_type_name(edge_type_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove node types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_node_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove node types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove edge types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn remove_inplace_edge_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_edge_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove edge types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_edge_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove edge weights from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn remove_inplace_edge_weights(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_edge_weights())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove edge weights from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn remove_edge_weights(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
    /// Divide edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn divide_edge_weights_inplace(&mut self, denominator: WeightT) -> PyResult<()> {
        Ok(pe!(self
            .inner
            .divide_edge_weights_inplace(denominator.clone()))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
    /// Divide edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn divide_edge_weights(&self, denominator: WeightT) -> PyResult<Graph> {
        Ok(pe!(self.inner.divide_edge_weights(denominator.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Normalize edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn normalize_edge_weights_inplace(&mut self) -> PyResult<()> {
        Ok(pe!(self.inner.normalize_edge_weights_inplace())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Normalize edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn normalize_edge_weights(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.normalize_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
    /// Multiply edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn multiply_edge_weights_inplace(&mut self, denominator: WeightT) -> PyResult<()> {
        Ok(pe!(self
            .inner
            .multiply_edge_weights_inplace(denominator.clone()))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
    /// Multiply edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn multiply_edge_weights(&self, denominator: WeightT) -> PyResult<Graph> {
        Ok(pe!(self.inner.multiply_edge_weights(denominator.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_ids, destination_node_ids, directed)")]
    /// Returns bipartite graph between the provided source and destination node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_ids: List[int]
    ///     The source node IDs.
    /// destination_node_ids: List[int]
    ///     The destination node IDs.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_ids(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.build_bipartite_graph_from_edge_node_ids(
            source_node_ids,
            destination_node_ids,
            directed.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids, directed)")]
    /// Returns clique graph between the provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node IDs.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_ids(node_ids, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_names, destination_node_names, directed)")]
    /// Returns bipartite graph between the provided source and destination node names.
    ///
    /// Parameters
    /// ----------
    /// source_node_names: List[&str]
    ///     The source node names.
    /// destination_node_names: List[&str]
    ///     The destination node names.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_names(
        &self,
        source_node_names: Vec<&str>,
        destination_node_names: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.build_bipartite_graph_from_edge_node_names(
            source_node_names,
            destination_node_names,
            directed.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names, directed)")]
    /// Returns clique graph between the provided node names.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[&str]
    ///     The node names.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_names(
        &self,
        node_names: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_names(node_names, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_prefixes, destination_node_prefixes, directed)")]
    /// Returns bipartite graph between the provided source and destination node prefixes.
    ///
    /// Parameters
    /// ----------
    /// source_node_prefixes: List[str]
    ///     The source node prefixes.
    /// destination_node_prefixes: List[str]
    ///     The destination node prefixes.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_prefixes(
        &self,
        source_node_prefixes: Vec<&str>,
        destination_node_prefixes: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(
            pe!(self.inner.build_bipartite_graph_from_edge_node_prefixes(
                &source_node_prefixes,
                &destination_node_prefixes,
                directed.clone()
            ))?
            .into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_prefixes, directed)")]
    /// Returns clique graph between the nodes with the provided prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_prefixes: List[str]
    ///     The node name prefixes.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_prefixes(
        &self,
        node_prefixes: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_prefixes(&node_prefixes, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_types, destination_node_types, directed)")]
    /// Returns bipartite graph between the provided source and destination node types.
    ///
    /// Parameters
    /// ----------
    /// source_node_types: List[Optional[&str]]
    ///     The source node types.
    /// destination_node_types: List[Optional[&str]]
    ///     The destination node types.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_types(
        &self,
        source_node_types: Vec<Option<&str>>,
        destination_node_types: Vec<Option<&str>>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.build_bipartite_graph_from_edge_node_types(
            &source_node_types,
            &destination_node_types,
            directed.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names, directed)")]
    /// Returns clique graph between the nodes with the provided node types.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[&str]]
    ///     The node name types.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_type_names(
        &self,
        node_type_names: Vec<Option<&str>>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_type_names(&node_type_names, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_circle, compute_circle_nodes)")]
    /// Return vector of Circles in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_circles(
        &self,
        minimum_number_of_nodes_per_circle: Option<NodeT>,
        compute_circle_nodes: Option<bool>,
    ) -> PyResult<Vec<Circle>> {
        Ok(pe!(self
            .inner
            .get_circles(minimum_number_of_nodes_per_circle, compute_circle_nodes))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a string describing the memory usage of all the fields of all the
    /// structures used to store the current graph
    pub fn get_memory_stats(&self) -> String {
        self.inner.get_memory_stats().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the given graph
    pub fn get_total_memory_used(&self) -> usize {
        self.inner.get_total_memory_used().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the nodes
    pub fn get_nodes_total_memory_requirement(&self) -> usize {
        self.inner.get_nodes_total_memory_requirement().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the nodes
    pub fn get_nodes_total_memory_requirement_human_readable(&self) -> String {
        self.inner
            .get_nodes_total_memory_requirement_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the edges
    pub fn get_edges_total_memory_requirement(&self) -> usize {
        self.inner.get_edges_total_memory_requirement().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the edges
    pub fn get_edges_total_memory_requirement_human_readable(&self) -> String {
        self.inner
            .get_edges_total_memory_requirement_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the edge weights
    pub fn get_edge_weights_total_memory_requirements(&self) -> usize {
        self.inner
            .get_edge_weights_total_memory_requirements()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the edge weights
    pub fn get_edge_weights_total_memory_requirements_human_readable(&self) -> String {
        self.inner
            .get_edge_weights_total_memory_requirements_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the node types
    pub fn get_node_types_total_memory_requirements(&self) -> PyResult<usize> {
        Ok(pe!(self.inner.get_node_types_total_memory_requirements())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the node types
    pub fn get_node_types_total_memory_requirements_human_readable(&self) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_node_types_total_memory_requirements_human_readable())?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the edge types
    pub fn get_edge_types_total_memory_requirements(&self) -> PyResult<usize> {
        Ok(pe!(self.inner.get_edge_types_total_memory_requirements())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the edge types
    pub fn get_edge_types_total_memory_requirements_human_readable(&self) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_edge_types_total_memory_requirements_human_readable())?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns number of triangles in the graph.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar. By default, True.
    ///
    pub fn get_number_of_triangles(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_number_of_triangles(approach, insert_only_source, verbose))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns number of squares in the graph.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar. By default, True.
    ///
    pub fn get_number_of_squares(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_number_of_squares(approach, insert_only_source, verbose))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns number of squares in the graph.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar. By default, True.
    ///
    pub fn get_number_of_squares_per_node(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_number_of_squares_per_node(
                    approach,
                    insert_only_source,
                    verbose
                ))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns total number of triads in the graph without taking into account weights
    pub fn get_number_of_triads(&self) -> EdgeT {
        self.inner.get_number_of_triads().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns total number of triads in the weighted graph
    pub fn get_number_of_weighted_triads(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_number_of_weighted_triads())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns transitivity of the graph without taking into account weights.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_transitivity(&self, verbose: Option<bool>) -> f64 {
        self.inner.get_transitivity(verbose).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns number of triangles for all nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar. By default, True.
    ///
    pub fn get_number_of_triangles_per_node(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_number_of_triangles_per_node(
                    approach,
                    insert_only_source,
                    verbose
                ))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns clustering coefficients for all nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_clustering_coefficient_per_node(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_clustering_coefficient_per_node(approach, insert_only_source, verbose),
            f64
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns the graph clustering coefficient.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_clustering_coefficient(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> f64 {
        self.inner
            .get_clustering_coefficient(approach, insert_only_source, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, insert_only_source, verbose)")]
    /// Returns the graph average clustering coefficient.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the increasing node degree order is used.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination. By default only the source is inserted.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_average_clustering_coefficient(
        &self,
        approach: Option<&str>,
        insert_only_source: Option<bool>,
        verbose: Option<bool>,
    ) -> f64 {
        self.inner
            .get_average_clustering_coefficient(approach, insert_only_source, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Return whether nodes are remappable to those of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     graph towards remap the nodes to.
    ///
    pub fn are_nodes_remappable(&self, other: &Graph) -> bool {
        self.inner.are_nodes_remappable(&other.inner).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns graph remapped using given node IDs ordering.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node Ids to remap the graph to.
    ///
    ///
    /// Safety
    /// ------
    /// This method will cause a panic if the node IDs are either:
    ///  * Not unique
    ///  * Not available for each of the node IDs of the graph.
    pub unsafe fn remap_unchecked_from_node_ids(&self, node_ids: Vec<NodeT>) -> Graph {
        self.inner.remap_unchecked_from_node_ids(node_ids).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns graph remapped using given node IDs ordering.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node Ids to remap the graph to.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs are not unique.
    /// ValueError
    ///     If the given node IDs are not available for all the values in the graph.
    ///
    pub fn remap_from_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_node_ids(node_ids))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names)")]
    /// Returns graph remapped using given node names ordering.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[&str]
    ///     The node names to remap the graph to.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node names are not unique.
    /// ValueError
    ///     If the given node names are not available for all the values in the graph.
    ///
    pub fn remap_from_node_names(&self, node_names: Vec<&str>) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_node_names(node_names))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names_map)")]
    /// Returns graph remapped using given node names mapping hashmap.
    ///
    /// Parameters
    /// ----------
    /// node_names_map: Dict[str, str]
    ///     The node names to remap the graph to.
    ///
    pub fn remap_from_node_names_map(
        &self,
        node_names_map: HashMap<String, String>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_node_names_map(node_names_map.into()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Return graph remapped towards nodes of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The graph to remap towards.
    ///
    pub fn remap_from_graph(&self, other: &Graph) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_graph(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, number_of_negative_samples, random_state, only_from_same_component, minimum_node_degree, maximum_node_degree, source_node_types_names, destination_node_types_names, source_edge_types_names, destination_edge_types_names, source_nodes_prefixes, destination_nodes_prefixes, graph_to_avoid, support, use_scale_free_distribution, sample_edge_types, sample_edge_weights, enforce_node_type_connection_consistency, number_of_sampling_attempts)"
    )]
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    /// Parameters
    /// ----------
    /// number_of_negative_samples: int
    ///     Number of negatives edges to include.
    /// random_state: Optional[int]
    ///     random_state to use to reproduce negative edge set.
    /// only_from_same_component: Optional[bool]
    ///     Whether to sample negative edges only from nodes that are from the same component.
    /// minimum_node_degree: Optional[int]
    ///     The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// maximum_node_degree: Optional[int]
    ///     The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// destination_node_types_names: Optional[List[str]]
    ///     Node type names of the nodes to be samples as destinations. If a node has any of the provided node types, it can be sampled as a destination node.
    /// source_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// destination_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as destinations. If a node has any of the provided edge types, it can be sampled as a destination node.
    /// source_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// destination_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as destinations. If a node starts with any of the provided prefixes, it can be sampled as a destinations node.
    /// graph_to_avoid: Optional[&Graph]
    ///     Compatible graph whose edges are not to be sampled.
    /// support: Optional[&Graph]
    ///     Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees and connected components are sampled from this support graph when provided. Useful when sampling negative edges for a test graph. In this latter case, the support graph should be the training graph.
    /// use_scale_free_distribution: Optional[bool]
    ///     Whether to sample the nodes using scale_free distribution. By default True. Not using this may cause significant biases.
    /// sample_edge_types: Optional[bool]
    ///     Whether to sample edge types, following the edge type counts distribution. By default it is true only when the current graph instance has edge types.
    /// sample_edge_weights: Optional[bool]
    ///     Whether to sample edge weights, following the edge weight distribution. By default it is true only when the current graph instance has edge weights.
    /// enforce_node_type_connection_consistency: Optional[bool]
    ///     Whether to enforce that the sampled negative edges have the same node types as the positive edges. By default it is true only when the current graph instance has node types.
    /// number_of_sampling_attempts: Optional[int]
    ///     Number of times to attempt to sample edges before giving up.
    ///
    pub fn sample_negative_graph(
        &self,
        number_of_negative_samples: EdgeT,
        random_state: Option<EdgeT>,
        only_from_same_component: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        source_node_types_names: Option<Vec<String>>,
        destination_node_types_names: Option<Vec<String>>,
        source_edge_types_names: Option<Vec<String>>,
        destination_edge_types_names: Option<Vec<String>>,
        source_nodes_prefixes: Option<Vec<String>>,
        destination_nodes_prefixes: Option<Vec<String>>,
        graph_to_avoid: Option<&Graph>,
        support: Option<&Graph>,
        use_scale_free_distribution: Option<bool>,
        sample_edge_types: Option<bool>,
        sample_edge_weights: Option<bool>,
        enforce_node_type_connection_consistency: Option<bool>,
        number_of_sampling_attempts: Option<usize>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.sample_negative_graph(
            number_of_negative_samples.clone(),
            random_state,
            only_from_same_component,
            minimum_node_degree,
            maximum_node_degree,
            source_node_types_names,
            destination_node_types_names,
            source_edge_types_names,
            destination_edge_types_names,
            source_nodes_prefixes,
            destination_nodes_prefixes,
            graph_to_avoid.map(|sg| &sg.inner),
            support.map(|sg| &sg.inner),
            use_scale_free_distribution,
            sample_edge_types,
            sample_edge_weights,
            enforce_node_type_connection_consistency,
            number_of_sampling_attempts
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, number_of_samples, random_state, minimum_node_degree, maximum_node_degree, source_node_types_names, destination_node_types_names, source_edge_types_names, destination_edge_types_names, source_nodes_prefixes, destination_nodes_prefixes, edge_type_names, support, number_of_sampling_attempts)"
    )]
    /// Returns Graph with given amount of subsampled edges.
    ///
    /// Parameters
    /// ----------
    /// number_of_samples: int
    ///     Number of edges to include.
    /// random_state: Optional[int]
    ///     random_state to use to reproduce negative edge set.
    /// minimum_node_degree: Optional[int]
    ///     The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// maximum_node_degree: Optional[int]
    ///     The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// destination_node_types_names: Optional[List[str]]
    ///     Node type names of the nodes to be samples as destinations. If a node has any of the provided node types, it can be sampled as a destination node.
    /// source_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// destination_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as destinations. If a node has any of the provided edge types, it can be sampled as a destination node.
    /// source_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// destination_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as destinations. If a node starts with any of the provided prefixes, it can be sampled as a destinations node.
    /// edge_type_names: Optional[&List[Optional[&str]]]
    ///     Edge type names of the edges to sample. Only edges with ANY of these edge types will be kept.
    /// support: Optional[&Graph]
    ///     Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees are sampled from this support graph when provided. Useful when sampling positive edges for a test graph. In this latter case, the support graph should be the training graph.
    /// number_of_sampling_attempts: Optional[int]
    ///     Number of times to attempt to sample edges before giving up.
    ///
    pub fn sample_positive_graph(
        &self,
        number_of_samples: usize,
        random_state: Option<EdgeT>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        source_node_types_names: Option<Vec<String>>,
        destination_node_types_names: Option<Vec<String>>,
        source_edge_types_names: Option<Vec<String>>,
        destination_edge_types_names: Option<Vec<String>>,
        source_nodes_prefixes: Option<Vec<String>>,
        destination_nodes_prefixes: Option<Vec<String>>,
        edge_type_names: Option<Vec<Option<&str>>>,
        support: Option<&Graph>,
        number_of_sampling_attempts: Option<usize>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.sample_positive_graph(
            number_of_samples.clone(),
            random_state,
            minimum_node_degree,
            maximum_node_degree,
            source_node_types_names,
            destination_node_types_names,
            source_edge_types_names,
            destination_edge_types_names,
            source_nodes_prefixes,
            destination_nodes_prefixes,
            edge_type_names.as_ref().map(|x| x.as_slice()),
            support.map(|sg| &sg.inner),
            number_of_sampling_attempts
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, train_size, random_state, edge_types, include_all_edge_types, minimum_node_degree, maximum_node_degree, verbose)"
    )]
    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_size) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// In the option where a list of edge types has been provided, these
    /// edge types will be those put into the validation set.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     Rate target to reserve for training.
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    /// edge_types: Optional[&List[Optional[&str]]]
    ///     Edge types to be selected for in the validation set.
    /// include_all_edge_types: Optional[bool]
    ///     Whether to include all the edges between two nodes.
    /// minimum_node_degree: Optional[int]
    ///     The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// maximum_node_degree: Optional[int]
    ///     The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge types have been specified but the graph does not have edge types.
    /// ValueError
    ///     If the required training size is not a real value between 0 and 1.
    /// ValueError
    ///     If the current graph does not allow for the creation of a spanning tree for the requested training size.
    ///
    pub fn connected_holdout(
        &self,
        train_size: f64,
        random_state: Option<EdgeT>,
        edge_types: Option<Vec<Option<&str>>>,
        include_all_edge_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.connected_holdout(
                train_size.clone(),
                random_state,
                edge_types.as_ref().map(|x| x.as_slice()),
                include_all_edge_types,
                minimum_node_degree,
                maximum_node_degree,
                verbose
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, train_size, random_state, include_all_edge_types, edge_types, min_number_overlaps, verbose)"
    )]
    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    /// include_all_edge_types: Optional[bool]
    ///     Whether to include all the edges between two nodes.
    /// edge_types: Optional[&List[Optional[&str]]]
    ///     The edges to include in validation set.
    /// min_number_overlaps: Optional[int]
    ///     The minimum number of overlaps to include the edge into the validation set.
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge types have been specified but the graph does not have edge types.
    /// ValueError
    ///     If the minimum number of overlaps have been specified but the graph is not a multigraph.
    /// ValueError
    ///     If one or more of the given edge type names is not present in the graph.
    ///
    pub fn random_holdout(
        &self,
        train_size: f64,
        random_state: Option<EdgeT>,
        include_all_edge_types: Option<bool>,
        edge_types: Option<Vec<Option<&str>>>,
        min_number_overlaps: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.random_holdout(
                train_size.clone(),
                random_state,
                include_all_edge_types,
                edge_types.as_ref().map(|x| x.as_slice()),
                min_number_overlaps,
                verbose
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If stratification is requested but the graph has a single node type.
    /// ValueError
    ///     If stratification is requested but the graph has a multilabel node types.
    ///
    pub fn get_node_label_holdout_indices(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_indices(
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_0, NodeT)
                },
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_1, NodeT)
                },
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If stratification is requested but the graph has a single node type.
    /// ValueError
    ///     If stratification is requested but the graph has a multilabel node types.
    ///
    pub fn get_node_label_holdout_labels(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(
        Vec<Option<Py<PyArray1<NodeTypeT>>>>,
        Vec<Option<Py<PyArray1<NodeTypeT>>>>,
    )> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_labels(
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (
                subresult_0
                    .into_iter()
                    .map(|x| {
                        x.map(|x| {
                            let gil = pyo3::Python::acquire_gil();
                            to_ndarray_1d!(gil, x, NodeTypeT)
                        })
                    })
                    .collect::<Vec<_>>(),
                subresult_1
                    .into_iter()
                    .map(|x| {
                        x.map(|x| {
                            let gil = pyo3::Python::acquire_gil();
                            to_ndarray_1d!(gil, x, NodeTypeT)
                        })
                    })
                    .collect::<Vec<_>>(),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If stratification is requested but the graph has a single node type.
    /// ValueError
    ///     If stratification is requested but the graph has a multilabel node types.
    ///
    pub fn get_node_label_holdout_graphs(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_graphs(
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
    /// Returns edge-label holdout for training ML algorithms on the graph edge labels.
    /// This is commonly used for edge type prediction tasks.
    ///
    /// This method returns two graphs, the train and the test one.
    /// The edges of the graph will be splitted in the train and test graphs according
    /// to the `train_size` argument.
    ///
    /// If stratification is enabled, the train and test will have the same ratios of
    /// edge types.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use edge-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If stratification is required but the graph has singleton edge types.
    ///
    pub fn get_edge_label_holdout_graphs(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_label_holdout_graphs(
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes, random_state, verbose)")]
    /// Returns subgraph with given number of nodes.
    ///
    /// **This method creates a subset of the graph starting from a random node
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached**. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// This is meant to execute distributed node embeddings.
    /// It may also sample singleton nodes.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes: int
    ///     Number of nodes to extract.
    /// random_state: Optional[int]
    ///     Random random_state to use.
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the requested number of nodes is one or less.
    /// ValueError
    ///     If the graph has less than the requested number of nodes.
    ///
    pub fn get_random_subgraph(
        &self,
        number_of_nodes: NodeT,
        random_state: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .get_random_subgraph(number_of_nodes.clone(), random_state, verbose))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If stratification is requested but the graph has a single node type.
    /// ValueError
    ///     If stratification is requested but the graph has a multilabel node types.
    ///
    pub fn get_node_label_random_holdout(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_random_holdout(
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k, k_index, use_stratification, random_state)")]
    /// Returns node-label fold for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     The number of folds.
    /// k_index: int
    ///     Which fold to use for the validation.
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If stratification is requested but the graph has a single node type.
    /// ValueError
    ///     If stratification is requested but the graph has a multilabel node types.
    ///
    pub fn get_node_label_kfold(
        &self,
        k: usize,
        k_index: usize,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_kfold(
                k.clone(),
                k_index.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k, k_index, use_stratification, random_state)")]
    /// Returns edge-label kfold for training ML algorithms on the graph edge labels.
    /// This is commonly used for edge type prediction tasks.
    ///
    /// This method returns two graphs, the train and the test one.
    /// The edges of the graph will be splitted in the train and test graphs according
    /// to the `train_size` argument.
    ///
    /// If stratification is enabled, the train and test will have the same ratios of
    /// edge types.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     The number of folds.
    /// k_index: int
    ///     Which fold to use for the validation.
    /// use_stratification: Optional[bool]
    ///     Whether to use edge-label stratification,
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If stratification is required but the graph has singleton edge types.
    ///
    pub fn get_edge_label_kfold(
        &self,
        k: usize,
        k_index: usize,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_label_kfold(
                k.clone(),
                k_index.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k, k_index, edge_types, random_state, verbose)")]
    /// Returns train and test graph following kfold validation scheme.
    ///
    /// The edges are splitted into k chunks. The k_index-th chunk is used to build
    /// the validation graph, all the other edges create the training graph.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     The number of folds.
    /// k_index: int
    ///     Which fold to use for the validation.
    /// edge_types: Optional[&List[Optional[&str]]]
    ///     Edge types to be selected when computing the folds (All the edge types not listed here will be always be used in the training set).
    /// random_state: Optional[int]
    ///     The random_state (seed) to use for the holdout,
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of requested k folds is one or zero.
    /// ValueError
    ///     If the given k fold index is greater than the number of k folds.
    /// ValueError
    ///     If edge types have been specified but it's an empty list.
    /// ValueError
    ///     If the number of k folds is higher than the number of edges in the graph.
    ///
    pub fn get_edge_prediction_kfold(
        &self,
        k: usize,
        k_index: usize,
        edge_types: Option<Vec<Option<&str>>>,
        random_state: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_prediction_kfold(
                k.clone(),
                k_index.clone(),
                edge_types.as_ref().map(|x| x.as_slice()),
                random_state,
                verbose
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector of node tuples in the current graph instance
    pub fn get_node_tuples(&self) -> PyResult<Vec<NodeTuple>> {
        Ok(pe!(self.inner.get_node_tuples())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_chain, compute_chain_nodes)")]
    /// Return vector of chains in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_chains(
        &self,
        minimum_number_of_nodes_per_chain: Option<NodeT>,
        compute_chain_nodes: Option<bool>,
    ) -> PyResult<Vec<Chain>> {
        Ok(pe!(self
            .inner
            .get_chains(minimum_number_of_nodes_per_chain, compute_chain_nodes))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id)")]
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
        &self,
        src_node_id: NodeT,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                src_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_ids, maximal_depth)")]
    /// Returns shortest path result for the BFS from given source node IDs, treating the set of source nodes as an hyper-node.
    ///
    /// Parameters
    /// ----------
    /// src_node_ids: List[int]
    ///     Roots of the tree of minimum paths.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to run the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///  The provided list of node ids must be non-empty, or the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
                src_node_ids,
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, maximal_depth)")]
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to run the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_distances_parallel_from_node_id(
        &self,
        src_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                src_node_id.clone(),
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id)")]
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node ID does not exist in the graph the method will panic.
    ///
    ///  TODO! Explore chains accelerations!
    pub unsafe fn get_unchecked_breadth_first_search_distances_sequential_from_node_id(
        &self,
        src_node_id: NodeT,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_distances_sequential_from_node_id(
                src_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_ids, dst_node_id, compute_predecessors, maximal_depth)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested, treating the set of source nodes as an hyper-node.
    ///
    /// Parameters
    /// ----------
    /// src_node_ids: List[int]
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, the breadth first search will stop upon reaching this node.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_from_node_ids(
                src_node_ids,
                dst_node_id,
                compute_predecessors,
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, breadth first search will stop upon reaching this node.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_from_node_id(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_from_node_id(
                src_node_id.clone(),
                dst_node_id,
                compute_predecessors,
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, maximal_depth)")]
    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node is a selfloop.
    /// ValueError
    ///     If there is no path between the two given nodes.
    ///
    pub unsafe fn get_unchecked_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_unchecked_shortest_path_node_ids_from_node_ids(
                        src_node_id.clone(),
                        dst_node_id.clone(),
                        maximal_depth
                    ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, maximal_depth)")]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_shortest_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_unchecked_shortest_path_node_names_from_node_ids(
                src_node_id.clone(),
                dst_node_id.clone(),
                maximal_depth
            ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, maximal_depth)")]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node IDs do not exist in the current graph.
    ///
    pub fn get_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_shortest_path_node_ids_from_node_ids(
                    src_node_id.clone(),
                    dst_node_id.clone(),
                    maximal_depth
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, maximal_depth)")]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_shortest_path_node_ids_from_node_names(
                    src_node_name,
                    dst_node_name,
                    maximal_depth
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, maximal_depth)")]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_shortest_path_node_names_from_node_names(
            src_node_name,
            dst_node_name,
            maximal_depth
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, k)")]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// k: int
    ///     Number of paths to find.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_k_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        k: usize,
    ) -> Vec<Vec<NodeT>> {
        self.inner
            .get_unchecked_k_shortest_path_node_ids_from_node_ids(
                src_node_id.clone(),
                dst_node_id.clone(),
                k.clone(),
            )
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, k)")]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    /// k: int
    ///     Number of paths to find.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node IDs does not exist in the graph.
    ///
    pub fn get_k_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        k: usize,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        Ok(pe!(self.inner.get_k_shortest_path_node_ids_from_node_ids(
            src_node_id.clone(),
            dst_node_id.clone(),
            k.clone()
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, k)")]
    /// Return vector of the k minimum paths node IDs between given source node and destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// k: int
    ///     Number of paths to find.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names does not exist in the graph.
    ///
    pub fn get_k_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        k: usize,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        Ok(pe!(self.inner.get_k_shortest_path_node_ids_from_node_names(
            src_node_name,
            dst_node_name,
            k.clone()
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, k)")]
    /// Return vector of the k minimum paths node names between given source node and destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// k: int
    ///     Number of paths to find.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names does not exist in the graph.
    ///
    pub fn get_k_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        k: usize,
    ) -> PyResult<Vec<Vec<String>>> {
        Ok(
            pe!(self.inner.get_k_shortest_path_node_names_from_node_names(
                src_node_name,
                dst_node_name,
                k.clone()
            ))?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns unweighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
    /// Returns weighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_eccentricity_from_node_id(
                node_id.clone(),
                use_edge_weights_as_probabilities,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns unweighted eccentricity of the given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_eccentricity_and_most_distant_node_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<(NodeT, NodeT)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_eccentricity_and_most_distant_node_id_from_node_id(node_id.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
    /// Returns weighted eccentricity of the given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    /// ValueError
    ///     If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// ValueError
    ///     If the graph contains negative weights.
    ///
    pub fn get_weighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_weighted_eccentricity_from_node_id(
            node_id.clone(),
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns unweighted eccentricity of the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Node for which to compute the eccentricity.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node name does not exist in the current graph instance.
    ///
    pub fn get_eccentricity_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_eccentricity_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name, use_edge_weights_as_probabilities)")]
    /// Returns weighted eccentricity of the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node name does not exist in the graph.
    /// ValueError
    ///     If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// ValueError
    ///     If the graph contains negative weights.
    ///
    pub fn get_weighted_eccentricity_from_node_name(
        &self,
        node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_weighted_eccentricity_from_node_name(
            node_name,
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_ids, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested, from the given root nodes (treated as an hyper-node).
    ///
    /// Parameters
    /// ----------
    /// src_node_id: List[int]
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]]
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: bool
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_dijkstra_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        self.inner
            .get_unchecked_dijkstra_from_node_ids(
                src_node_ids,
                maybe_dst_node_id,
                maybe_dst_node_ids,
                compute_predecessors,
                maximal_depth,
                use_edge_weights_as_probabilities,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]]
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: bool
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_dijkstra_from_node_id(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        self.inner
            .get_unchecked_dijkstra_from_node_id(
                src_node_id.clone(),
                maybe_dst_node_id,
                maybe_dst_node_ids,
                compute_predecessors,
                maximal_depth,
                use_edge_weights_as_probabilities,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"
    )]
    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f32, Py<PyArray1<NodeT>>) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
                src_node_id.clone(),
                dst_node_id.clone(),
                use_edge_weights_as_probabilities,
                maximal_depth,
            );
        (subresult_0.into(), {
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, subresult_1, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"
    )]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_shortest_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f32, Vec<String>) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_weighted_shortest_path_node_names_from_node_ids(
                src_node_id.clone(),
                dst_node_id.clone(),
                use_edge_weights_as_probabilities,
                maximal_depth,
            );
        (
            subresult_0.into(),
            subresult_1
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"
    )]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node IDs do not exist in the current graph.
    ///
    pub fn get_weighted_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f32, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_ids_from_node_ids(
                    src_node_id.clone(),
                    dst_node_id.clone(),
                    use_edge_weights_as_probabilities,
                    maximal_depth
                ))?
            .into();
            (subresult_0.into(), {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            })
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"
    )]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f32, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_ids_from_node_names(
                    src_node_name,
                    dst_node_name,
                    use_edge_weights_as_probabilities,
                    maximal_depth
                ))?
            .into();
            (subresult_0.into(), {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            })
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"
    )]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f32, Vec<String>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_names_from_node_names(
                    src_node_name,
                    dst_node_name,
                    use_edge_weights_as_probabilities,
                    maximal_depth
                ))?
            .into();
            (
                subresult_0.into(),
                subresult_1
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<_>>(),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Node ID root of the tree of minimum paths.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute the DFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given source node ID does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node ID does not exist in the current graph.
    ///
    pub fn get_breadth_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<ShortestPathsResultBFS> {
        Ok(pe!(self.inner.get_breadth_first_search_from_node_ids(
            src_node_id.clone(),
            dst_node_id,
            compute_predecessors,
            maximal_depth
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Node ID root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]]
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the weights are to be used and the graph does not have weights.
    /// ValueError
    ///     If the given source node ID does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node ID does not exist in the current graph.
    /// ValueError
    ///     If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// ValueError
    ///     If the graph contains negative weights.
    ///
    pub fn get_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<ShortestPathsDjkstra> {
        Ok(pe!(self.inner.get_dijkstra_from_node_ids(
            src_node_id.clone(),
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns approximated diameter and tentative low eccentricity node for an UNDIRECTED graph.
    /// This method returns a lowerbound of the diameter by doing the following steps:
    /// * Find the most central node
    /// * Find the most distant node from the most central one (and get a first
    /// approximation of the diameter lowerbound)
    /// * Get the median node in this path
    /// * Find the most distant node from the median node
    /// * Find the most distant node form the last one, and get the second approx
    /// of the diameter lowerbound.
    ///
    /// This basically creates a "cross" that spans the graph
    pub fn get_four_sweep(&self) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self.inner.get_four_sweep();
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, ignore_infinity, verbose)")]
    /// Returns diameter of the graph using naive method.
    ///
    /// Note that there exists the non-naive method for undirected graphs
    /// and it is possible to implement a faster method for directed graphs
    /// but we still need to get to it, as it will require an updated
    /// succinct data structure.
    ///
    /// Parameters
    /// ----------
    /// ignore_infinity: Optional[bool]
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes.
    ///
    pub fn get_diameter_naive(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_diameter_naive(ignore_infinity, verbose))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, ignore_infinity, verbose)")]
    /// Returns diameter of the graph.
    ///
    /// Parameters
    /// ----------
    /// ignore_infinity: Optional[bool]
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components. By default True.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes.
    ///
    pub fn get_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_diameter(ignore_infinity, verbose))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, dst_node_name, compute_predecessors, maximal_depth)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Node name root of the tree of minimum paths.
    /// dst_node_name: Optional[&str]
    ///     Destination node name.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the weights are to be used and the graph does not have weights.
    /// ValueError
    ///     If the given source node name does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node name does not exist in the current graph.
    ///
    pub fn get_breadth_first_search_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: Option<&str>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<ShortestPathsResultBFS> {
        Ok(pe!(self.inner.get_breadth_first_search_from_node_names(
            src_node_name,
            dst_node_name,
            compute_predecessors,
            maximal_depth
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, maybe_dst_node_name, maybe_dst_node_names, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Node name root of the tree of minimum paths.
    /// maybe_dst_node_name: Optional[&str]
    ///     Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_names: Optional[List[&str]]
    ///     Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the weights are to be used and the graph does not have weights.
    /// ValueError
    ///     If the given source node name does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node name does not exist in the current graph.
    ///
    pub fn get_dijkstra_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<ShortestPathsDjkstra> {
        Ok(pe!(self.inner.get_dijkstra_from_node_names(
            src_node_name,
            maybe_dst_node_name,
            maybe_dst_node_names,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_type_ids, dst_node_type_ids, verbose)")]
    /// Returns histogram of distances between the given source node type ids and the given destination node type ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_type_ids: Optional[List[int]]
    ///     Source node types ids. If None, the nodes with unknown node type will be used.
    /// dst_node_type_ids: Optional[List[int]]
    ///     Destination node types ids. If None, the nodes with unknown node type will be used.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_distances_histogram_between_node_type_ids(
        &self,
        src_node_type_ids: Option<Vec<NodeTypeT>>,
        dst_node_type_ids: Option<Vec<NodeTypeT>>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<usize>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_distances_histogram_between_node_type_ids(
                    src_node_type_ids,
                    dst_node_type_ids,
                    verbose
                ))?,
                usize
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_type_names, dst_node_type_names, verbose)")]
    /// Returns histogram of distances between the given source node type names and the given destination node type names.
    ///
    /// Parameters
    /// ----------
    /// src_node_type_names: Optional[List[&str]]
    ///     Source node types names. If None, the nodes with unknown node type will be used.
    /// dst_node_type_names: Optional[List[&str]]
    ///     Destination node types names. If None, the nodes with unknown node type will be used.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_distances_histogram_between_node_type_names(
        &self,
        src_node_type_names: Option<Vec<&str>>,
        dst_node_type_names: Option<Vec<&str>>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<usize>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_distances_histogram_between_node_type_names(
                    src_node_type_names,
                    dst_node_type_names,
                    verbose
                ))?,
                usize
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_prefixes, dst_prefixes, verbose)")]
    /// Returns histogram of distances between the given source prefixes and the given destination prefixes.
    ///
    /// Parameters
    /// ----------
    /// src_prefixes: List[str]
    ///     Allowed source node prefixes.
    /// dst_prefixes: List[str]
    ///     Allowed destination node prefixes.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar.
    ///
    pub fn get_distances_histogram_between_prefixes(
        &self,
        src_prefixes: Vec<String>,
        dst_prefixes: Vec<String>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<usize>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_distances_histogram_between_prefixes(
                    src_prefixes,
                    dst_prefixes,
                    verbose
                ))?,
                usize
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn get_number_of_connected_components(
        &self,
        verbose: Option<bool>,
    ) -> (NodeT, NodeT, NodeT) {
        let (subresult_0, subresult_1, subresult_2) =
            self.inner.get_number_of_connected_components(verbose);
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of connected nodes in the graph.
    pub fn get_number_of_connected_nodes(&self) -> NodeT {
        self.inner.get_number_of_connected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton nodes with selfloops within the graph.
    pub fn get_number_of_singleton_nodes_with_selfloops(&self) -> NodeT {
        self.inner
            .get_number_of_singleton_nodes_with_selfloops()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton nodes within the graph.
    pub fn get_number_of_singleton_nodes(&self) -> NodeT {
        self.inner.get_number_of_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of disconnected nodes within the graph.
    /// A Disconnected node is a node which is nor a singleton nor a singleton
    /// with selfloops.
    pub fn get_number_of_disconnected_nodes(&self) -> NodeT {
        self.inner.get_number_of_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton node IDs of the graph.
    pub fn get_singleton_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_singleton_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton node names of the graph.
    pub fn get_singleton_node_names(&self) -> Vec<String> {
        self.inner
            .get_singleton_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton_with_selfloops node IDs of the graph.
    pub fn get_singleton_with_selfloops_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_singleton_with_selfloops_node_ids(),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton_with_selfloops node names of the graph.
    pub fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
        self.inner
            .get_singleton_with_selfloops_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns density of the graph.
    pub fn get_density(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_density())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    pub fn get_trap_nodes_rate(&self) -> f64 {
        self.inner.get_trap_nodes_rate().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of trap nodes present in the current graph
    pub fn get_trap_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_trap_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns unweighted mean node degree of the graph.
    pub fn get_node_degrees_mean(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_node_degrees_mean())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns weighted mean node degree of the graph.
    pub fn get_weighted_node_degrees_mean(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_node_degrees_mean())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of undirected edges of the graph.
    pub fn get_number_of_undirected_edges(&self) -> EdgeT {
        self.inner.get_number_of_undirected_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of undirected edges of the graph.
    pub fn get_number_of_unique_undirected_edges(&self) -> EdgeT {
        self.inner.get_number_of_unique_undirected_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of edges of the graph.
    pub fn get_number_of_edges(&self) -> EdgeT {
        self.inner.get_number_of_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of unique edges of the graph.
    pub fn get_number_of_unique_edges(&self) -> EdgeT {
        self.inner.get_number_of_unique_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns unweighted median node degree of the graph
    pub fn get_node_degrees_median(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degrees_median())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns weighted median node degree of the graph
    pub fn get_weighted_node_degrees_median(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_node_degrees_median())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum node degree of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain any node (is an empty graph).
    ///
    pub fn get_maximum_node_degree(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_maximum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum node degree of the graph.
    ///
    /// Safety
    /// ------
    /// This method fails with a panic if the graph does not have any node.
    pub unsafe fn get_unchecked_most_central_node_id(&self) -> NodeT {
        self.inner.get_unchecked_most_central_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum node degree of the graph.
    pub fn get_most_central_node_id(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_most_central_node_id())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns minimum node degree of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain any node (is an empty graph).
    ///
    pub fn get_minimum_node_degree(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_minimum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns mode node degree of the graph.
    pub fn get_node_degrees_mode(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degrees_mode())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns rate of self-loops.
    pub fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_selfloop_nodes_rate())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return name of the graph.
    pub fn get_name(&self) -> String {
        self.inner.get_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the number of traps nodes (nodes without any outgoing edges that are not singletons)
    pub fn get_number_of_trap_nodes(&self) -> NodeT {
        self.inner.get_number_of_trap_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the number of traps nodes with selfloops (nodes without any outgoing edges that are not singletons)
    pub fn get_number_of_trap_nodes_with_selfloops(&self) -> NodeT {
        self.inner.get_number_of_trap_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector of the non-unique source nodes.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_node_ids(&self, directed: bool) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_source_node_ids(directed.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector on the (non unique) directed source nodes of the graph
    pub fn get_directed_source_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_directed_source_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector of the non-unique source nodes names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.inner
            .get_source_names(directed.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_destination_node_ids(&self, directed: bool) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_destination_node_ids(directed.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector on the (non unique) directed destination nodes of the graph
    pub fn get_directed_destination_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_directed_destination_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector of the non-unique destination nodes names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.inner
            .get_destination_names(directed.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted nodes names
    pub fn get_node_names(&self) -> Vec<String> {
        self.inner
            .get_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the node URLs.
    pub fn get_node_urls(&self) -> Vec<Option<String>> {
        self.inner
            .get_node_urls()
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the node predicted ontology.
    pub fn get_node_ontologies(&self) -> Vec<Option<String>> {
        self.inner
            .get_node_ontologies()
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return node ontology for the provided node name, if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to query for.
    ///
    pub unsafe fn get_unchecked_ontology_from_node_name(&self, node_name: &str) -> Option<String> {
        self.inner
            .get_unchecked_ontology_from_node_name(node_name)
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return node ontology for the provided node id, if available.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id to query for.
    ///
    pub unsafe fn get_unchecked_ontology_from_node_id(&self, node_id: NodeT) -> Option<String> {
        self.inner
            .get_unchecked_ontology_from_node_id(node_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return node ontology for the provided node name, if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to query for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node name does not exist in the current graph.
    ///
    pub fn get_ontology_from_node_name(&self, node_name: &str) -> PyResult<Option<String>> {
        Ok(pe!(self.inner.get_ontology_from_node_name(node_name))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return node ontology for the provided node id, if available.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id to query for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node ID does not exist in the current graph.
    ///
    pub fn get_ontology_from_node_id(&self, node_id: NodeT) -> PyResult<Option<String>> {
        Ok(pe!(self.inner.get_ontology_from_node_id(node_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted nodes Ids
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed edge types of the edges
    pub fn get_directed_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_directed_edge_type_ids())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the upper triangular edge types of the edges
    pub fn get_upper_triangular_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_upper_triangular_edge_type_ids())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the lower triangular edge types of the edges
    pub fn get_lower_triangular_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_lower_triangular_edge_type_ids())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, imputation_edge_type_id)")]
    /// Return the inputed directed edge types of the edges.
    ///
    /// Parameters
    /// ----------
    /// imputation_edge_type_id: int
    ///     The edge type id value to impute with.
    ///
    pub fn get_imputed_directed_edge_type_ids(
        &self,
        imputation_edge_type_id: EdgeTypeT,
    ) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_imputed_directed_edge_type_ids(imputation_edge_type_id.clone()))?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, imputation_edge_type_id)")]
    /// Return the imputed upper triangular edge types of the edges.
    ///
    /// Parameters
    /// ----------
    /// imputation_edge_type_id: int
    ///     The edge type id value to impute with.
    ///
    pub fn get_imputed_upper_triangular_edge_type_ids(
        &self,
        imputation_edge_type_id: EdgeTypeT,
    ) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_imputed_upper_triangular_edge_type_ids(imputation_edge_type_id.clone()))?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, imputation_edge_type_id)")]
    /// Return the imputed lower triangular edge types of the edges.
    ///
    /// Parameters
    /// ----------
    /// imputation_edge_type_id: int
    ///     The edge type id value to impute with.
    ///
    pub fn get_imputed_lower_triangular_edge_type_ids(
        &self,
        imputation_edge_type_id: EdgeTypeT,
    ) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_imputed_lower_triangular_edge_type_ids(imputation_edge_type_id.clone()))?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed known edge types of the edges, dropping unknown ones
    pub fn get_directed_known_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_directed_known_edge_type_ids())?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the upper triangular known edge types of the edges, dropping unknown ones
    pub fn get_upper_triangular_known_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_upper_triangular_known_edge_type_ids())?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the lower triangular known edge types of the edges, dropping unknown ones
    pub fn get_lower_triangular_known_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_lower_triangular_known_edge_type_ids())?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed source node IDs with known edge types
    pub fn get_directed_source_nodes_with_known_edge_types(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_directed_source_nodes_with_known_edge_types())?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed destination node IDs with known edge types
    pub fn get_directed_destination_nodes_with_known_edge_types(
        &self,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_directed_destination_nodes_with_known_edge_types())?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the unique edge type IDs of the graph edges.
    pub fn get_unique_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unique_edge_type_ids())?, EdgeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed edge types names
    pub fn get_directed_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        Ok(pe!(self.inner.get_directed_edge_type_names())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the upper triangular edge type names of the edges
    pub fn get_upper_triangular_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        Ok(pe!(self.inner.get_upper_triangular_edge_type_names())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the lower triangular edge type names of the edges
    pub fn get_lower_triangular_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        Ok(pe!(self.inner.get_lower_triangular_edge_type_names())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the edge types names
    pub fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_edge_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed weights of the graph edges.
    pub fn get_directed_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_directed_edge_weights())?, WeightT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the undirected weights of the graph edges, filtering out edges where src > dst.
    pub fn get_undirected_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_undirected_edge_weights())?, WeightT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the weighted indegree (total weighted inbound edge weights) for each node.
    pub fn get_weighted_node_indegrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_node_indegrees())?, f64)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node types of the graph nodes.
    pub fn get_node_type_ids(&self) -> PyResult<Vec<Option<Py<PyArray1<NodeTypeT>>>>> {
        Ok(pe!(self.inner.get_node_type_ids())?
            .into_iter()
            .cloned()
            .map(|x| {
                x.map(|x| {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, x, NodeTypeT)
                })
            })
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean mask of known node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_known_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_known_node_types_mask())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean mask of unknown node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_unknown_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unknown_node_types_mask())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean mask of known edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_known_edge_types_mask())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean mask of unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_unknown_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unknown_edge_types_mask())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns one-hot encoded node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_one_hot_encoded_node_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(gil, pe!(self.inner.get_one_hot_encoded_node_types())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns one-hot encoded known node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_one_hot_encoded_known_node_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_one_hot_encoded_known_node_types())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns one-hot encoded edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_one_hot_encoded_edge_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(gil, pe!(self.inner.get_one_hot_encoded_edge_types())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns one-hot encoded known edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_one_hot_encoded_known_edge_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_one_hot_encoded_known_edge_types())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node types names.
    pub fn get_node_type_names(&self) -> PyResult<Vec<Option<Vec<String>>>> {
        Ok(pe!(self.inner.get_node_type_names())?
            .into_iter()
            .map(|x| x.map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the unique node type IDs of the graph nodes.
    pub fn get_unique_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unique_node_type_ids())?, NodeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the unique node types names.
    pub fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of the unique edges in the graph
    pub fn get_number_of_unique_directed_edges(&self) -> EdgeT {
        self.inner.get_number_of_unique_directed_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the nodes mapping
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.inner.get_nodes_mapping().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector with the sorted edge Ids.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_ids(&self, directed: bool) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.inner.get_edge_node_ids(directed.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed edge node IDs
    pub fn get_directed_edge_node_ids(&self) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.inner.get_directed_edge_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed triples with (source, edge_type, destination) IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge types.
    ///
    pub fn get_directed_edge_triples_ids(&self) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(gil, pe!(self.inner.get_directed_edge_triples_ids())?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector with the sorted edge names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_names(&self, directed: bool) -> Vec<(String, String)> {
        self.inner
            .get_edge_node_names(directed.clone())
            .into_iter()
            .map(|x| {
                let (subresult_0, subresult_1) = x;
                (subresult_0.into(), subresult_1.into())
            })
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed edge names
    pub fn get_directed_edge_node_names(&self) -> Vec<(String, String)> {
        self.inner
            .get_directed_edge_node_names()
            .into_iter()
            .map(|x| {
                let (subresult_0, subresult_1) = x;
                (subresult_0.into(), subresult_1.into())
            })
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed triples with (source, edge_type, destination) names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge types.
    ///
    pub fn get_directed_edge_triples_names(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_directed_edge_triples_names())?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of nodes with unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_number_of_unknown_node_types(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_unknown_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of node with known node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_number_of_known_node_types(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_known_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns rate of unknown node types over total nodes number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_unknown_node_types_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_unknown_node_types_rate())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns rate of known node types over total nodes number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_known_node_types_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_known_node_types_rate())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns minimum number of node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_minimum_number_of_node_types(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_minimum_number_of_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum number of node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_maximum_number_of_node_types(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_maximum_number_of_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of maximum multilabel count.
    ///
    /// This value is the maximum number of multilabel counts
    /// that appear in any given node in the graph
    pub fn get_maximum_multilabel_count(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_maximum_multilabel_count())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_number_of_singleton_node_types(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_singleton_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of homogeneous node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_number_of_homogeneous_node_types(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_homogeneous_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns list of homogeneous node type IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_homogeneous_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_homogeneous_node_type_ids())?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns list of homogeneous node type names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_homogeneous_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_homogeneous_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton node types IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_singleton_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_singleton_node_type_ids())?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton node types names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_singleton_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_singleton_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_number_of_unknown_edge_types(&self) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_number_of_unknown_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns directed edge IDs of the edges with unknown edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_directed_edge_ids_with_unknown_edge_types(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_directed_edge_ids_with_unknown_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns upper triangular edge IDs of the edges with unknown edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_upper_triangular_edge_ids_with_unknown_edge_types(
        &self,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_upper_triangular_edge_ids_with_unknown_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns lower triangular edge IDs of the edges with unknown edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_lower_triangular_edge_ids_with_unknown_edge_types(
        &self,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_lower_triangular_edge_ids_with_unknown_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns edge IDs of the edges with known edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_directed_edge_ids_with_known_edge_types(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_directed_edge_ids_with_known_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns upper triangular edge IDs of the edges with known edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_upper_triangular_edge_ids_with_known_edge_types(
        &self,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_upper_triangular_edge_ids_with_known_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns lower triangular edge IDs of the edges with known edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_lower_triangular_edge_ids_with_known_edge_types(
        &self,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_lower_triangular_edge_ids_with_known_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Returns edge node IDs of the edges with known edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to iterated the edges as a directed or undirected edge list.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_node_ids_with_known_edge_types(
        &self,
        directed: bool,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_with_known_edge_types(directed.clone()))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Returns edge node names of the edges with known edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to iterated the edges as a directed or undirected edge list.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_node_names_with_known_edge_types(
        &self,
        directed: bool,
    ) -> PyResult<Vec<(String, String)>> {
        Ok(pe!(self
            .inner
            .get_edge_node_names_with_known_edge_types(directed.clone()))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edges_with_unknown_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edges_with_unknown_edge_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each directed edge contains whether it has an
    /// unknown edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_directed_edges_with_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_directed_edges_with_known_edge_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector with known edge types from the upper triangular matrix.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the graph is not undirected.
    ///
    pub fn get_upper_triangular_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_upper_triangular_known_edge_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector with known edge types from the lower triangular matrix.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the graph is not undirected.
    ///
    pub fn get_lower_triangular_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_lower_triangular_known_edge_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns node IDs of the nodes with unknown node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_with_unknown_node_types(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_with_unknown_node_types())?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns node IDs of the nodes with known node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_with_known_node_types(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_with_known_node_types())?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns node names of the nodes with unknown node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_with_unknown_node_types(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_node_names_with_unknown_node_types())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Returns node IDs of the nodes with given node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_node_ids_from_node_type_id(node_type_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids)")]
    /// Returns node IDs of the nodes with given node type IDs.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids: List[Optional[int]]
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_ids(
        &self,
        node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_from_node_type_ids(&node_type_ids))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Returns node IDs of the nodes with given node type names.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[&str]]
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<&str>>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_node_ids_from_node_type_names(&node_type_names))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Returns node names of the nodes with given node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_node_names_from_node_type_id(node_type_id.clone()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Returns node IDs of the nodes with given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_from_node_type_name(node_type_name))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Returns node names of the nodes with given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_node_names_from_node_type_name(node_type_name))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns node names of the nodes with known node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_with_known_node_types(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_node_names_with_known_node_types())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_nodes_with_unknown_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_nodes_with_unknown_node_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// known node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_nodes_with_known_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_nodes_with_known_node_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of edge with known edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_number_of_known_edge_types(&self) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_number_of_known_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns rate of unknown edge types over total edges number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_unknown_edge_types_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_unknown_edge_types_rate())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns rate of known edge types over total edges number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_known_edge_types_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_known_edge_types_rate())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns minimum number of edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_minimum_number_of_edge_types(&self) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_minimum_number_of_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum number of edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_maximum_number_of_edge_types(&self) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_maximum_number_of_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_number_of_singleton_edge_types(&self) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_number_of_singleton_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton edge types IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_singleton_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_singleton_edge_type_ids())?,
                EdgeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton edge types names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_singleton_edge_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_singleton_edge_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of nodes in the graph
    pub fn get_number_of_nodes(&self) -> NodeT {
        self.inner.get_number_of_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Return a vector with the components each node belongs to.
    ///
    /// E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
    /// `[0, 1, 0, 0, 1, 1]`
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar.
    ///
    pub fn get_node_connected_component_ids(&self, verbose: Option<bool>) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_node_connected_component_ids(verbose),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of directed edges in the graph
    pub fn get_number_of_directed_edges(&self) -> EdgeT {
        self.inner.get_number_of_directed_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of edge types in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph.
    ///
    pub fn get_number_of_edge_types(&self) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_number_of_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of node types in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph.
    ///
    pub fn get_number_of_node_types(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the unweighted degree of every node in the graph
    pub fn get_node_degrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_degrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the indegree for each node.
    pub fn get_node_indegrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_indegrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the weighted degree of every node in the graph
    pub fn get_weighted_node_degrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_node_degrees())?, f64)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return set of nodes that are not singletons
    pub fn get_not_singletons_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_not_singletons_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return mapping from instance not trap nodes to dense nodes
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.inner.get_dense_nodes_mapping().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of edges that have multigraph syblings
    pub fn get_number_of_parallel_edges(&self) -> EdgeT {
        self.inner.get_number_of_parallel_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with node cumulative_node_degrees, that is the comulative node degree
    pub fn get_cumulative_node_degrees(&self) -> Py<PyArray1<EdgeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_cumulative_node_degrees().to_vec(),
            EdgeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector wit
    pub fn get_reciprocal_sqrt_degrees(&self) -> Py<PyArray1<WeightT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_reciprocal_sqrt_degrees(), WeightT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of the source nodes.
    pub fn get_number_of_unique_source_nodes(&self) -> NodeT {
        self.inner.get_number_of_unique_source_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns edge type IDs counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph instance.
    ///
    pub fn get_edge_type_id_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
        Ok(pe!(self.inner.get_edge_type_id_counts_hashmap())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns edge type names counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph instance.
    ///
    pub fn get_edge_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, EdgeT>> {
        Ok(pe!(self.inner.get_edge_type_names_counts_hashmap())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns node type IDs counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph instance.
    ///
    pub fn get_node_type_id_counts_hashmap(&self) -> PyResult<HashMap<NodeTypeT, NodeT>> {
        Ok(pe!(self.inner.get_node_type_id_counts_hashmap())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns node type names counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph instance.
    ///
    pub fn get_node_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, NodeT>> {
        Ok(pe!(self.inner.get_node_type_names_counts_hashmap())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, unknown_node_types_value)")]
    /// Returns 1D single labeled node types ids vector.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph has multilabel node types.
    ///
    pub fn get_single_label_node_type_ids(
        &self,
        unknown_node_types_value: Option<NodeTypeT>,
    ) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_single_label_node_type_ids(unknown_node_types_value))?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns 1D known single labeled node types ids vector.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph has multilabel node types.
    ///
    pub fn get_known_single_label_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_known_single_label_node_type_ids())?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, target_value, unknown_node_types_value)")]
    /// Returns 1D binarized node types ids vector
    pub fn get_boolean_node_type_ids(
        &self,
        target_value: Option<NodeTypeT>,
        unknown_node_types_value: Option<NodeTypeT>,
    ) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_boolean_node_type_ids(target_value, unknown_node_types_value))?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, target_value)")]
    /// Returns 1D binarized known node types ids vector
    pub fn get_known_boolean_node_type_ids(
        &self,
        target_value: NodeTypeT,
    ) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_known_boolean_node_type_ids(target_value.clone()))?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of root node ids, nodes with zero inbound degree and non-zero outbound degree
    pub fn get_root_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_root_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of root node names, nodes with zero inbound degree and non-zero outbound degree
    pub fn get_root_node_names(&self) -> Vec<String> {
        self.inner
            .get_root_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Convert inplace the graph to directed.
    pub fn to_directed_inplace(&mut self) {
        self.inner.to_directed_inplace();
        ()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return a new instance of the current graph as directed
    pub fn to_directed(&self) -> Graph {
        self.inner.to_directed().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed graph from the upper triangular adjacency matrix.
    pub fn to_upper_triangular(&self) -> Graph {
        self.inner.to_upper_triangular().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed graph from the lower triangular adjacency matrix.
    pub fn to_lower_triangular(&self) -> Graph {
        self.inner.to_lower_triangular().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the main diagonal adjacency matrix.
    pub fn to_main_diagonal(&self) -> Graph {
        self.inner.to_main_diagonal().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the anti-diagonal adjacency matrix.
    pub fn to_anti_diagonal(&self) -> Graph {
        self.inner.to_anti_diagonal().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the bidiagonal adjacency matrix.
    pub fn to_bidiagonal(&self) -> Graph {
        self.inner.to_bidiagonal().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the arrowhead adjacency matrix.
    pub fn to_arrowhead(&self) -> Graph {
        self.inner.to_arrowhead().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the transposed adjacency matrix.
    pub fn to_transposed(&self) -> Graph {
        self.inner.to_transposed().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph with all symmetric edges
    pub fn to_undirected(&self) -> Graph {
        self.inner.to_undirected().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the complementary graph.
    pub fn to_complementary(&self) -> Graph {
        self.inner.to_complementary().into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, maximal_hop_distance, change_layer_probability, random_walk_length, iterations)"
    )]
    /// Returns structural similarity multi-graph.
    ///
    /// Parameters
    /// ----------
    /// maximal_hop_distance: Optional[int]
    ///     The maximal hop distance. By default, equal to the graph diameter. By default, equal to the diameter.
    /// change_layer_probability: Optional[float]
    ///     Probability to change the layer during the random walk. By default 0.5.
    /// random_walk_length: Optional[int]
    ///     Length of the random walk to be used to compute the approximated stationary distribution. By default, 1024.
    /// iterations: Optional[int]
    ///     Number of iterations per node to compute the approximated stationary distribution. By default 1.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided graph does not have any edges.
    /// ValueError
    ///     If the provided change layer probability is not a probability.
    /// ValueError
    ///     If the provided random walk parameters are not valid.
    ///
    pub fn to_structural_similarity_multi_graph(
        &self,
        maximal_hop_distance: Option<usize>,
        change_layer_probability: Option<f32>,
        random_walk_length: Option<u64>,
        iterations: Option<NodeT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.to_structural_similarity_multi_graph(
            maximal_hop_distance,
            change_layer_probability,
            random_walk_length,
            iterations
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_degree, minimum_clique_size, clique_per_node, verbose)"
    )]
    /// Returns graph cliques with at least `minimum_degree` nodes.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph is directed.
    ///
    pub fn get_approximated_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<Vec<Clique>> {
        Ok(pe!(self.inner.get_approximated_cliques(
            minimum_degree,
            minimum_clique_size,
            clique_per_node,
            verbose
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the maximum clique in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph is directed.
    ///
    pub fn get_max_clique(&self) -> PyResult<Clique> {
        Ok(pe!(self.inner.get_max_clique())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_degree, minimum_clique_size, clique_per_node, verbose)"
    )]
    /// Returns number of graph cliques with at least `minimum_degree` nodes.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph is directed.
    ///
    pub fn get_approximated_number_of_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<usize> {
        Ok(pe!(self.inner.get_approximated_number_of_cliques(
            minimum_degree,
            minimum_clique_size,
            clique_per_node,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns report relative to the graph metrics
    ///
    /// The report includes the following metrics by default:
    /// * Name of the graph
    /// * Whether the graph is directed or undirected
    /// * Number of singleton nodes
    /// * Number of nodes
    /// - If the graph has nodes, we also compute:
    /// * Minimum unweighted node degree
    /// * Maximum unweighted node degree
    /// * Unweighted node degree mean
    /// * Number of edges
    /// * Number of self-loops
    /// * Number of singleton with self-loops
    /// * Whether the graph is a multigraph
    /// * Number of parallel edges
    /// * Number of directed edges
    /// - If the graph has edges, we also compute:
    /// * Rate of self-loops
    /// * Whether the graph has weighted edges
    /// - If the graph has weights, we also compute:
    /// * Minimum weighted node degree
    /// * Maximum weighted node degree
    /// * Weighted node degree mean
    /// * The total edge weights
    /// * Whether the graph has node types
    /// - If the graph has node types, we also compute:
    /// * Whether the graph has singleton node types
    /// * The number of node types
    /// * The number of nodes with unknown node types
    /// * The number of nodes with known node types
    /// * Whether the graph has edge types
    /// - If the graph has edge types, we also compute:
    /// * Whether the graph has singleton edge types
    /// * The number of edge types
    /// * The number of edges with unknown edge types
    /// * The number of edges with known edge types
    ///
    /// On request, since it takes more time to compute it, the method also provides:
    pub fn report(&self) -> HashMap<&'static str, String> {
        self.inner.report().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other, verbose)")]
    /// Return rendered textual report about the graph overlaps.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     graph to create overlap report with.
    /// verbose: Optional[bool]
    ///     Whether to shor the loading bars.
    ///
    pub fn overlap_textual_report(&self, other: &Graph, verbose: Option<bool>) -> PyResult<String> {
        Ok(pe!(self.inner.overlap_textual_report(&other.inner, verbose))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return human-readable html report of the given node.
    ///
    /// The report, by default, is rendered using html.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_id(&self, node_id: NodeT) -> PyResult<String> {
        Ok(pe!(self.inner.get_node_report_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return human-readable html report of the given node.
    ///
    /// The report, by default, is rendered using html.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_name(&self, node_name: &str) -> PyResult<String> {
        Ok(pe!(self.inner.get_node_report_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return html short textual report of the graph.
    ///
    /// TODO! Add reports on various node metrics
    /// TODO! Add reports on various edge metrics
    /// NOTE! Most of the above TODOs will require first to implement the
    /// support for the fast computation of the inbound edges in a directed
    /// graphs
    pub fn textual_report(&self) -> String {
        self.inner.textual_report().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash)")]
    /// Get a mask indicating the nodes that are isomorphic.
    ///
    /// This method identifies the nodes that are isomorphic based on their minimum
    /// degree and the number of neighbors used for hashing. It populates a deny mask
    /// where the nodes that are isomorphic are marked as `true`.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_isomorphic_nodes_mask(
        &self,
        minimum_node_degree: NodeT,
        number_of_neighbours_for_hash: usize,
    ) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_isomorphic_nodes_mask(
                    minimum_node_degree.clone(),
                    number_of_neighbours_for_hash.clone()
                ))?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, dtype)")]
    /// Returns vector with isomorphic node groups IDs.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 10.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    /// dtype: Optional[&str]
    ///     The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        Ok(pe!(self.inner.get_isomorphic_node_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            dtype
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash)")]
    /// Retrieves a vector of flattened and repeated isomorphic node IDs, that is by removing one per group.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     An optional parameter specifying the minimum degree a node must have to be included
    ///
    pub fn get_flat_repeated_isomorphic_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_flat_repeated_isomorphic_node_ids(
                    minimum_node_degree,
                    number_of_neighbours_for_hash
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, dtype)")]
    /// Retrieves the isomorphic node hashes
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_isomorphic_node_hashes(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> PyResult<Py<PyArray1<u64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_isomorphic_node_hashes(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    dtype
                ))?,
                u64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, dtype)")]
    /// Returns vector with isomorphic node groups names.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 10.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    /// dtype: Optional[&str]
    ///     The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_isomorphic_node_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            dtype
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, ignore_edges_including_isomorphic_nodes, dtype)"
    )]
    /// Returns vector with isomorphic edge groups IDs.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 10.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    /// ignore_edges_including_isomorphic_nodes: Optional[bool]
    ///     Whether to ignore edges including isomorphic nodes. By default, true.
    /// dtype: Optional[&str]
    ///     The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_edge_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        ignore_edges_including_isomorphic_nodes: Option<bool>,
        dtype: Option<&str>,
    ) -> PyResult<Vec<Vec<Py<PyArray1<NodeT>>>>> {
        Ok(pe!(self.inner.get_isomorphic_edge_node_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            ignore_edges_including_isomorphic_nodes,
            dtype
        ))?
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|x| {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, x.to_vec(), NodeT)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, ignore_edges_including_isomorphic_nodes, dtype)"
    )]
    /// Retrieves the isomorphic edge hashes
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_isomorphic_edge_hashes(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        ignore_edges_including_isomorphic_nodes: Option<bool>,
        dtype: Option<&str>,
    ) -> PyResult<Py<PyArray1<u64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_isomorphic_edge_hashes(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    ignore_edges_including_isomorphic_nodes,
                    dtype
                ))?,
                u64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, ignore_edges_including_isomorphic_nodes, dtype)"
    )]
    /// Returns vector with isomorphic edge groups names.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 10.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    /// ignore_edges_including_isomorphic_nodes: Optional[bool]
    ///     Whether to ignore edges including isomorphic nodes. By default, true.
    /// dtype: Optional[&str]
    ///     The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_edge_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        ignore_edges_including_isomorphic_nodes: Option<bool>,
        dtype: Option<&str>,
    ) -> PyResult<Vec<Vec<Vec<String>>>> {
        Ok(pe!(self.inner.get_isomorphic_edge_node_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            ignore_edges_including_isomorphic_nodes,
            dtype
        ))?
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|x| x.into_iter().cloned().map(|x| x.into()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, dtype)")]
    /// Returns vector with isomorphic tuple groups IDs.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 10.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    /// dtype: Optional[&str]
    ///     The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_tuple_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> PyResult<Vec<Vec<Py<PyArray1<NodeT>>>>> {
        Ok(pe!(self.inner.get_isomorphic_tuple_node_ids(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            dtype
        ))?
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|x| {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, x.to_vec(), NodeT)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, dtype)")]
    /// Retrieves the isomorphic tuple hashes
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_isomorphic_tuple_hashes(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> PyResult<Py<PyArray1<u64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_isomorphic_tuple_hashes(
                    minimum_node_degree,
                    number_of_neighbours_for_hash,
                    dtype
                ))?,
                u64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash, dtype)")]
    /// Returns vector with isomorphic tuple groups names.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 10.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    /// dtype: Optional[&str]
    ///     The data type of the hash. By default, `&str`.
    ///
    pub fn get_isomorphic_tuple_node_names(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
        dtype: Option<&str>,
    ) -> PyResult<Vec<Vec<Vec<String>>>> {
        Ok(pe!(self.inner.get_isomorphic_tuple_node_names(
            minimum_node_degree,
            number_of_neighbours_for_hash,
            dtype
        ))?
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|x| x.into_iter().cloned().map(|x| x.into()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(random_state, minimum_node_id, minimum_node_sampling, maximum_node_sampling, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new random connected graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    /// minimum_node_id: int
    ///     The minimum node ID for the connected graph.
    /// minimum_node_sampling: int
    ///     The minimum amount of nodes to sample per node.
    /// maximum_node_sampling: int
    ///     The maximum amount of nodes to sample per node.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Chain'.
    ///
    pub fn generate_random_connected_graph(
        random_state: Option<u64>,
        minimum_node_id: Option<NodeT>,
        minimum_node_sampling: Option<NodeT>,
        maximum_node_sampling: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_random_connected_graph(
            random_state,
            minimum_node_id,
            minimum_node_sampling,
            maximum_node_sampling,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(random_state, minimum_node_id, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new random connected graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    /// minimum_node_id: int
    ///     The minimum node ID for the connected graph.
    /// minimum_node_sampling: int
    ///     The minimum amount of nodes to sample per node.
    /// maximum_node_sampling: int
    ///     The maximum amount of nodes to sample per node.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Chain'.
    ///
    pub fn generate_random_spanning_tree(
        random_state: Option<u64>,
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_random_spanning_tree(
            random_state,
            minimum_node_id,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new star graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the star. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use for the star. By default 'star'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the star. By default 'star'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the star. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Star'.
    ///
    pub fn generate_star_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_star_graph(
            minimum_node_id,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new wheel graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the wheel. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use for the wheel. By default 'wheel'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the wheel. By default 'wheel'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the wheel. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Wheel'.
    ///
    pub fn generate_wheel_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_wheel_graph(
            minimum_node_id,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new circle graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the circle. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use for the circle. By default 'circle'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the circle. By default 'circle'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the circle. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Circle'.
    ///
    pub fn generate_circle_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_circle_graph(
            minimum_node_id,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new chain graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Chain'.
    ///
    pub fn generate_chain_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_chain_graph(
            minimum_node_id,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, number_of_nodes, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
    /// Creates new complete graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when combining graphs. By default 0.
    /// number_of_nodes: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[&str]
    ///     The node type to use. By default 'complete'.
    /// edge_type: Optional[&str]
    ///     The node type to use. By default 'complete'.
    /// weight: Optional[float]
    ///     The weight to use for the edges. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Complete'.
    ///
    pub fn generate_complete_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_complete_graph(
            minimum_node_id,
            number_of_nodes,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, left_clique_number_of_nodes, right_clique_number_of_nodes, chain_number_of_nodes, include_selfloops, left_clique_node_type, right_clique_node_type, chain_node_type, left_clique_edge_type, right_clique_edge_type, chain_edge_type, left_clique_weight, right_clique_weight, chain_weight, directed, name)"
    )]
    /// Creates new barbell graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// left_clique_number_of_nodes: Optional[int]
    ///     Number of nodes in the left clique. By default 10.
    /// right_clique_number_of_nodes: Optional[int]
    ///      Number of nodes in the right clique. By default equal to the left clique.
    /// chain_number_of_nodes: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// left_clique_node_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_node_type: Optional[&str]
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_edge_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_edge_type: Optional[&str]
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// right_clique_weight: Optional[float]
    ///     The weight to use for the edges in the right clique. By default None.
    /// chain_weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Barbell'.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge weights are provided only for a subset.
    ///
    pub fn generate_barbell_graph(
        minimum_node_id: Option<NodeT>,
        left_clique_number_of_nodes: Option<NodeT>,
        right_clique_number_of_nodes: Option<NodeT>,
        chain_number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        left_clique_node_type: Option<&str>,
        right_clique_node_type: Option<&str>,
        chain_node_type: Option<&str>,
        left_clique_edge_type: Option<&str>,
        right_clique_edge_type: Option<&str>,
        chain_edge_type: Option<&str>,
        left_clique_weight: Option<WeightT>,
        right_clique_weight: Option<WeightT>,
        chain_weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_barbell_graph(
            minimum_node_id,
            left_clique_number_of_nodes,
            right_clique_number_of_nodes,
            chain_number_of_nodes,
            include_selfloops,
            left_clique_node_type,
            right_clique_node_type,
            chain_node_type,
            left_clique_edge_type,
            right_clique_edge_type,
            chain_edge_type,
            left_clique_weight,
            right_clique_weight,
            chain_weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, clique_number_of_nodes, chain_number_of_nodes, include_selfloops, clique_node_type, chain_node_type, clique_edge_type, chain_edge_type, clique_weight, chain_weight, directed, name)"
    )]
    /// Creates new lollipop graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// clique_number_of_nodes: Optional[int]
    ///     Number of nodes in the left clique. By default 10.
    /// chain_number_of_nodes: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// clique_node_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'clique'.
    /// chain_node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// clique_edge_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'clique'.
    /// chain_edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// clique_weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// chain_weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Lollipop'.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge weights are provided only for a subset.
    ///
    pub fn generate_lollipop_graph(
        minimum_node_id: Option<NodeT>,
        clique_number_of_nodes: Option<NodeT>,
        chain_number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        clique_node_type: Option<&str>,
        chain_node_type: Option<&str>,
        clique_edge_type: Option<&str>,
        chain_edge_type: Option<&str>,
        clique_weight: Option<WeightT>,
        chain_weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_lollipop_graph(
            minimum_node_id,
            clique_number_of_nodes,
            chain_number_of_nodes,
            include_selfloops,
            clique_node_type,
            chain_node_type,
            clique_edge_type,
            chain_edge_type,
            clique_weight,
            chain_weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(text_signature = "(sides, minimum_node_id, node_type, weight, directed, name)")]
    /// Creates new squared lattice graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// sides: List[int]
    ///     Sides of the hyper-dimensional lattice with square cell.
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// node_type: Optional[&str]
    ///     The node type to use for the squared lattice. By default 'squared_lattice'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
    ///     Name of the graph. By default 'Lollipop'.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge weights are provided only for a subset.
    ///
    pub fn generate_squared_lattice_graph(
        sides: Vec<NodeT>,
        minimum_node_id: Option<NodeT>,
        node_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_squared_lattice_graph(
            &sides,
            minimum_node_id,
            node_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_ids_to_keep, node_ids_to_remove, node_names_to_keep_from_graph, node_names_to_remove_from_graph, node_prefixes_to_keep, node_prefixes_to_remove, node_type_ids_to_keep, node_type_ids_to_remove, node_type_id_to_keep, node_type_id_to_remove, source_node_ids_to_keep, source_node_ids_to_remove, source_node_names_to_keep_from_graph, source_node_names_to_remove_from_graph, source_node_prefixes_to_keep, source_node_prefixes_to_remove, source_node_type_ids_to_keep, source_node_type_ids_to_remove, source_node_type_id_to_keep, source_node_type_id_to_remove, destination_node_ids_to_keep, destination_node_ids_to_remove, destination_node_names_to_keep_from_graph, destination_node_names_to_remove_from_graph, destination_node_prefixes_to_keep, destination_node_prefixes_to_remove, destination_node_type_ids_to_keep, destination_node_type_ids_to_remove, destination_node_type_id_to_keep, destination_node_type_id_to_remove, edge_ids_to_keep, edge_ids_to_remove, edge_node_ids_to_keep, edge_node_ids_to_remove, edge_type_ids_to_keep, edge_type_ids_to_remove, min_edge_weight, max_edge_weight, min_node_degree, max_node_degree, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"
    )]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_ids_to_keep: Optional[List[int]]
    ///     List of node IDs to keep during filtering.
    /// node_ids_to_remove: Optional[List[int]]
    ///     List of node IDs to remove during filtering.
    /// node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept.
    /// node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed.
    /// node_prefixes_to_keep: Optional[List[str]]
    ///     List of node prefixes to keep during filtering.
    /// node_prefixes_to_remove: Optional[List[str]]
    ///     List of node prefixes to remove during filtering.
    /// node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_ids_to_remove: Optional[List[Optional[List[int]]]]
    ///     List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_id_to_remove: Optional[List[Optional[int]]]
    ///     List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// source_node_ids_to_keep: Optional[List[int]]
    ///     List of source node IDs to keep during filtering.
    /// source_node_ids_to_remove: Optional[List[int]]
    ///     List of source node IDs to remove during filtering.
    /// source_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are source nodes in this graph instance.
    /// source_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are source nodes in this graph instance.
    /// source_node_prefixes_to_keep: Optional[List[str]]
    ///     List of source node prefixes to keep during filtering.
    /// source_node_prefixes_to_remove: Optional[List[str]]
    ///     List of source node prefixes to remove during filtering.
    /// source_node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of source node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_ids_to_remove: Optional[List[Optional[List[int]]]]
    ///     List of source node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of source node type IDs to keep during filtering. Any of source node types must match with one of the node types given.
    /// source_node_type_id_to_remove: Optional[List[Optional[int]]]
    ///     List of source node type IDs to remove during filtering. Any of source node types must match with one of the node types given.
    /// destination_node_ids_to_keep: Optional[List[int]]
    ///     List of destination node IDs to keep during filtering.
    /// destination_node_ids_to_remove: Optional[List[int]]
    ///     List of destination node IDs to remove during filtering.
    /// destination_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are destination nodes in this graph instance.
    /// destination_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are destination nodes in this graph instance.
    /// destination_node_prefixes_to_keep: Optional[List[str]]
    ///     List of destination node prefixes to keep during filtering.
    /// destination_node_prefixes_to_remove: Optional[List[str]]
    ///     List of destination node prefixes to remove during filtering.
    /// destination_node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of destination node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_ids_to_remove: Optional[List[Optional[List[int]]]]
    ///     List of destination node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of destination node type IDs to keep during filtering. Any of destination node types must match with one of the node types given.
    /// destination_node_type_id_to_remove: Optional[List[Optional[int]]]
    ///     List of destination node type IDs to remove during filtering. Any of destination node types must match with one of the node types given.
    /// edge_ids_to_keep: Optional[List[int]]
    ///     List of edge IDs to keep during filtering.
    /// edge_ids_to_remove: Optional[List[int]]
    ///     List of edge IDs to remove during filtering.
    /// edge_node_ids_to_keep: Optional[List[Tuple[int, int]]]
    ///     List of tuple of node IDs to keep during filtering.
    /// edge_node_ids_to_remove: Optional[List[Tuple[int, int]]]
    ///     List of tuple of node IDs to remove during filtering.
    /// edge_type_ids_to_keep: Optional[List[Optional[int]]]
    ///     List of edge type IDs to keep during filtering.
    /// edge_type_ids_to_remove: Optional[List[Optional[int]]]
    ///     List of edge type IDs to remove during filtering.
    /// min_edge_weight: Optional[float]
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float]
    ///     Maximum edge weight. Values higher than this are removed.
    /// min_node_degree: Optional[int]
    ///     Minimum node degree. Values lower than this are removed.
    /// max_node_degree: Optional[int]
    ///     Maximum node degree. Values higher than this are removed.
    /// filter_singleton_nodes: Optional[bool]
    ///     Whether to filter out singleton nodes.
    /// filter_singleton_nodes_with_selfloop: Optional[bool]
    ///     Whether to filter out singleton nodes with selfloops.
    /// filter_selfloops: Optional[bool]
    ///     Whether to filter out selfloops.
    /// filter_parallel_edges: Optional[bool]
    ///     Whether to filter out parallel edges.
    /// verbose: Optional[bool]
    ///     Whether to show loading bar while building the graphs.
    ///
    pub fn filter_from_ids(
        &self,
        node_ids_to_keep: Option<Vec<NodeT>>,
        node_ids_to_remove: Option<Vec<NodeT>>,
        node_names_to_keep_from_graph: Option<&Graph>,
        node_names_to_remove_from_graph: Option<&Graph>,
        node_prefixes_to_keep: Option<Vec<String>>,
        node_prefixes_to_remove: Option<Vec<String>>,
        node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_ids_to_remove: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        node_type_id_to_remove: Option<Vec<Option<NodeTypeT>>>,
        source_node_ids_to_keep: Option<Vec<NodeT>>,
        source_node_ids_to_remove: Option<Vec<NodeT>>,
        source_node_names_to_keep_from_graph: Option<&Graph>,
        source_node_names_to_remove_from_graph: Option<&Graph>,
        source_node_prefixes_to_keep: Option<Vec<String>>,
        source_node_prefixes_to_remove: Option<Vec<String>>,
        source_node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        source_node_type_ids_to_remove: Option<Vec<Option<Vec<NodeTypeT>>>>,
        source_node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        source_node_type_id_to_remove: Option<Vec<Option<NodeTypeT>>>,
        destination_node_ids_to_keep: Option<Vec<NodeT>>,
        destination_node_ids_to_remove: Option<Vec<NodeT>>,
        destination_node_names_to_keep_from_graph: Option<&Graph>,
        destination_node_names_to_remove_from_graph: Option<&Graph>,
        destination_node_prefixes_to_keep: Option<Vec<String>>,
        destination_node_prefixes_to_remove: Option<Vec<String>>,
        destination_node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        destination_node_type_ids_to_remove: Option<Vec<Option<Vec<NodeTypeT>>>>,
        destination_node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        destination_node_type_id_to_remove: Option<Vec<Option<NodeTypeT>>>,
        edge_ids_to_keep: Option<Vec<EdgeT>>,
        edge_ids_to_remove: Option<Vec<EdgeT>>,
        edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>>,
        edge_node_ids_to_remove: Option<Vec<(NodeT, NodeT)>>,
        edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>>,
        edge_type_ids_to_remove: Option<Vec<Option<EdgeTypeT>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        min_node_degree: Option<NodeT>,
        max_node_degree: Option<NodeT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.filter_from_ids(
            node_ids_to_keep,
            node_ids_to_remove,
            node_names_to_keep_from_graph.map(|sg| &sg.inner),
            node_names_to_remove_from_graph.map(|sg| &sg.inner),
            node_prefixes_to_keep,
            node_prefixes_to_remove,
            node_type_ids_to_keep,
            node_type_ids_to_remove,
            node_type_id_to_keep,
            node_type_id_to_remove,
            source_node_ids_to_keep,
            source_node_ids_to_remove,
            source_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            source_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            source_node_prefixes_to_keep,
            source_node_prefixes_to_remove,
            source_node_type_ids_to_keep,
            source_node_type_ids_to_remove,
            source_node_type_id_to_keep,
            source_node_type_id_to_remove,
            destination_node_ids_to_keep,
            destination_node_ids_to_remove,
            destination_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            destination_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            destination_node_prefixes_to_keep,
            destination_node_prefixes_to_remove,
            destination_node_type_ids_to_keep,
            destination_node_type_ids_to_remove,
            destination_node_type_id_to_keep,
            destination_node_type_id_to_remove,
            edge_ids_to_keep,
            edge_ids_to_remove,
            edge_node_ids_to_keep,
            edge_node_ids_to_remove,
            edge_type_ids_to_keep,
            edge_type_ids_to_remove,
            min_edge_weight,
            max_edge_weight,
            min_node_degree,
            max_node_degree,
            filter_singleton_nodes,
            filter_singleton_nodes_with_selfloop,
            filter_selfloops,
            filter_parallel_edges
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_names_to_keep, node_names_to_remove, node_names_to_keep_from_graph, node_names_to_remove_from_graph, node_prefixes_to_keep, node_prefixes_to_remove, node_type_names_to_keep, node_type_names_to_remove, node_type_name_to_keep, node_type_name_to_remove, source_node_names_to_keep, source_node_names_to_remove, source_node_names_to_keep_from_graph, source_node_names_to_remove_from_graph, source_node_prefixes_to_keep, source_node_prefixes_to_remove, source_node_type_names_to_keep, source_node_type_names_to_remove, source_node_type_name_to_keep, source_node_type_name_to_remove, destination_node_names_to_keep, destination_node_names_to_remove, destination_node_names_to_keep_from_graph, destination_node_names_to_remove_from_graph, destination_node_prefixes_to_keep, destination_node_prefixes_to_remove, destination_node_type_names_to_keep, destination_node_type_names_to_remove, destination_node_type_name_to_keep, destination_node_type_name_to_remove, edge_node_names_to_keep, edge_node_names_to_remove, edge_type_names_to_keep, edge_type_names_to_remove, min_edge_weight, max_edge_weight, min_node_degree, max_node_degree, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"
    )]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_names_to_keep: Optional[List[&str]]
    ///     List of node names to keep during filtering.
    /// node_names_to_remove: Optional[List[&str]]
    ///     List of node names to remove during filtering.
    /// node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept.
    /// node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed.
    /// node_prefixes_to_keep: Optional[List[str]]
    ///     List of node prefixes to keep during filtering.
    /// node_prefixes_to_remove: Optional[List[str]]
    ///     List of node prefixes to remove during filtering.
    /// node_type_names_to_keep: Optional[List[Optional[List[&str]]]]
    ///     List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_names_to_remove: Optional[List[Optional[List[&str]]]]
    ///     List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_name_to_keep: Optional[&List[Optional[&str]]]
    ///     List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_name_to_remove: Optional[&List[Optional[&str]]]
    ///     List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// source_node_names_to_keep: Optional[List[&str]]
    ///     List of source node names to keep during filtering.
    /// source_node_names_to_remove: Optional[List[&str]]
    ///     List of source node names to remove during filtering.
    /// source_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are source nodes.
    /// source_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are source nodes.
    /// source_node_prefixes_to_keep: Optional[List[str]]
    ///     List of source node prefixes to keep during filtering.
    /// source_node_prefixes_to_remove: Optional[List[str]]
    ///     List of source node prefixes to remove during filtering.
    /// source_node_type_names_to_keep: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of source nodes to keep during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_names_to_remove: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of source nodes to remove during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_name_to_keep: Optional[&List[Optional[&str]]]
    ///     List of node type name of source nodes to keep during filtering. Any of node types must match with one of the node types given.
    /// source_node_type_name_to_remove: Optional[&List[Optional[&str]]]
    ///     List of node type name of source nodes to remove during filtering. Any of node types must match with one of the node types given.
    /// destination_node_names_to_keep: Optional[List[&str]]
    ///     List of destination node names to keep during filtering.
    /// destination_node_names_to_remove: Optional[List[&str]]
    ///     List of destination node names to remove during filtering.
    /// destination_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are destination nodes.
    /// destination_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are destination nodes.
    /// destination_node_prefixes_to_keep: Optional[List[str]]
    ///     List of destination node prefixes to keep during filtering.
    /// destination_node_prefixes_to_remove: Optional[List[str]]
    ///     List of destination node prefixes to remove during filtering.
    /// destination_node_type_names_to_keep: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of destination nodes to keep during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_names_to_remove: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of destination nodes to remove during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_name_to_keep: Optional[&List[Optional[&str]]]
    ///     List of node type name of destination nodes to keep during filtering. Any of node types must match with one of the node types given.
    /// destination_node_type_name_to_remove: Optional[&List[Optional[&str]]]
    ///     List of node type name of destination nodes to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_node_names_to_keep: Optional[List[Tuple[str, str]]]
    ///     List of tuple of node names to keep during filtering.
    /// edge_node_names_to_remove: Optional[List[Tuple[str, str]]]
    ///     List of tuple of node names to remove during filtering.
    /// edge_type_names_to_keep: Optional[&List[Optional[&str]]]
    ///     List of edge type names to keep during filtering.
    /// edge_type_names_to_remove: Optional[&List[Optional[&str]]]
    ///     List of edge type names to remove during filtering.
    /// min_edge_weight: Optional[float]
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float]
    ///     Maximum edge weight. Values higher than this are removed.
    /// min_node_degree: Optional[int]
    ///     Minimum node degree. Values lower than this are removed.
    /// max_node_degree: Optional[int]
    ///     Maximum node degree. Values higher than this are removed.
    /// filter_singleton_nodes: Optional[bool]
    ///     Whether to filter out singletons.
    /// filter_singleton_nodes_with_selfloop: Optional[bool]
    ///     Whether to filter out singleton nodes with selfloops.
    /// filter_selfloops: Optional[bool]
    ///     Whether to filter out selfloops.
    /// filter_parallel_edges: Optional[bool]
    ///     Whether to filter out parallel edges.
    /// verbose: Optional[bool]
    ///     Whether to show loading bar while building the graphs.
    ///
    pub fn filter_from_names(
        &self,
        node_names_to_keep: Option<Vec<&str>>,
        node_names_to_remove: Option<Vec<&str>>,
        node_names_to_keep_from_graph: Option<&Graph>,
        node_names_to_remove_from_graph: Option<&Graph>,
        node_prefixes_to_keep: Option<Vec<String>>,
        node_prefixes_to_remove: Option<Vec<String>>,
        node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        node_type_names_to_remove: Option<Vec<Option<Vec<&str>>>>,
        node_type_name_to_keep: Option<Vec<Option<&str>>>,
        node_type_name_to_remove: Option<Vec<Option<&str>>>,
        source_node_names_to_keep: Option<Vec<&str>>,
        source_node_names_to_remove: Option<Vec<&str>>,
        source_node_names_to_keep_from_graph: Option<&Graph>,
        source_node_names_to_remove_from_graph: Option<&Graph>,
        source_node_prefixes_to_keep: Option<Vec<String>>,
        source_node_prefixes_to_remove: Option<Vec<String>>,
        source_node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        source_node_type_names_to_remove: Option<Vec<Option<Vec<&str>>>>,
        source_node_type_name_to_keep: Option<Vec<Option<&str>>>,
        source_node_type_name_to_remove: Option<Vec<Option<&str>>>,
        destination_node_names_to_keep: Option<Vec<&str>>,
        destination_node_names_to_remove: Option<Vec<&str>>,
        destination_node_names_to_keep_from_graph: Option<&Graph>,
        destination_node_names_to_remove_from_graph: Option<&Graph>,
        destination_node_prefixes_to_keep: Option<Vec<String>>,
        destination_node_prefixes_to_remove: Option<Vec<String>>,
        destination_node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        destination_node_type_names_to_remove: Option<Vec<Option<Vec<&str>>>>,
        destination_node_type_name_to_keep: Option<Vec<Option<&str>>>,
        destination_node_type_name_to_remove: Option<Vec<Option<&str>>>,
        edge_node_names_to_keep: Option<Vec<(&str, &str)>>,
        edge_node_names_to_remove: Option<Vec<(&str, &str)>>,
        edge_type_names_to_keep: Option<Vec<Option<&str>>>,
        edge_type_names_to_remove: Option<Vec<Option<&str>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        min_node_degree: Option<NodeT>,
        max_node_degree: Option<NodeT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.filter_from_names(
            node_names_to_keep,
            node_names_to_remove,
            node_names_to_keep_from_graph.map(|sg| &sg.inner),
            node_names_to_remove_from_graph.map(|sg| &sg.inner),
            node_prefixes_to_keep,
            node_prefixes_to_remove,
            node_type_names_to_keep,
            node_type_names_to_remove,
            node_type_name_to_keep.as_ref().map(|x| x.as_slice()),
            node_type_name_to_remove.as_ref().map(|x| x.as_slice()),
            source_node_names_to_keep,
            source_node_names_to_remove,
            source_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            source_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            source_node_prefixes_to_keep,
            source_node_prefixes_to_remove,
            source_node_type_names_to_keep,
            source_node_type_names_to_remove,
            source_node_type_name_to_keep.as_ref().map(|x| x.as_slice()),
            source_node_type_name_to_remove
                .as_ref()
                .map(|x| x.as_slice()),
            destination_node_names_to_keep,
            destination_node_names_to_remove,
            destination_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            destination_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            destination_node_prefixes_to_keep,
            destination_node_prefixes_to_remove,
            destination_node_type_names_to_keep,
            destination_node_type_names_to_remove,
            destination_node_type_name_to_keep
                .as_ref()
                .map(|x| x.as_slice()),
            destination_node_type_name_to_remove
                .as_ref()
                .map(|x| x.as_slice()),
            edge_node_names_to_keep,
            edge_node_names_to_remove,
            edge_type_names_to_keep.as_ref().map(|x| x.as_slice()),
            edge_type_names_to_remove.as_ref().map(|x| x.as_slice()),
            min_edge_weight,
            max_edge_weight,
            min_node_degree,
            max_node_degree,
            filter_singleton_nodes,
            filter_singleton_nodes_with_selfloop,
            filter_selfloops,
            filter_parallel_edges
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without unknown node types and relative nodes.
    ///
    /// Note that this method will remove ALL nodes labeled with unknown node
    /// type!
    pub fn remove_unknown_node_types(&self) -> Graph {
        self.inner.remove_unknown_node_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without unknown edge types and relative edges.
    ///
    /// Note that this method will remove ALL edges labeled with unknown edge
    /// type!
    pub fn remove_unknown_edge_types(&self) -> Graph {
        self.inner.remove_unknown_edge_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without singleton nodes.
    ///
    /// A node is singleton when does not have neither incoming or outgoing edges.
    pub fn remove_singleton_nodes(&self) -> Graph {
        self.inner.remove_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without tendrils
    pub fn remove_tendrils(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_tendrils())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without tendrils
    pub fn remove_dendritic_trees(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_dendritic_trees())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree, number_of_neighbours_for_hash)")]
    /// Returns new graph without isomorphic nodes, only keeping the smallest node ID of each group.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default equal to 5.
    /// number_of_neighbours_for_hash: Optional[int]
    ///     The number of neighbours to consider for the hash. By default 10.
    ///
    pub fn remove_isomorphic_nodes(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .remove_isomorphic_nodes(minimum_node_degree, number_of_neighbours_for_hash))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without singleton nodes with selfloops.
    ///
    /// A node is singleton with selfloop when does not have neither incoming or outgoing edges.
    pub fn remove_singleton_nodes_with_selfloops(&self) -> Graph {
        self.inner.remove_singleton_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without disconnected nodes.
    ///
    /// A disconnected node is a node with no connection to any other node.
    pub fn remove_disconnected_nodes(&self) -> Graph {
        self.inner.remove_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without selfloops.
    pub fn remove_selfloops(&self) -> Graph {
        self.inner.remove_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without parallel edges
    pub fn remove_parallel_edges(&self) -> Graph {
        self.inner.remove_parallel_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state, undesired_edge_types, verbose)")]
    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// The spanning tree is NOT minimal.
    /// The given random_state is NOT the root of the tree.
    ///
    /// This method, additionally, allows for undesired edge types to be
    /// used to build the spanning tree only in extremis when it is utterly
    /// necessary in order to complete the spanning arborescence.
    ///
    /// The quintuple returned contains:
    /// - Set of the edges used in order to build the spanning arborescence.
    /// - Vector of the connected component of each node.
    /// - Number of connected components.
    /// - Minimum component size.
    /// - Maximum component size.
    ///
    /// Parameters
    /// ----------
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    /// undesired_edge_types: Optional[Set[Optional[int]]]
    ///     Which edge types id to try to avoid.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn random_spanning_arborescence_kruskal(
        &self,
        random_state: Option<EdgeT>,
        undesired_edge_types: Option<HashSet<Option<EdgeTypeT>>>,
        verbose: Option<bool>,
    ) -> (
        HashSet<(NodeT, NodeT)>,
        Py<PyArray1<NodeT>>,
        NodeT,
        NodeT,
        NodeT,
    ) {
        let (subresult_0, subresult_1, subresult_2, subresult_3, subresult_4) = self
            .inner
            .random_spanning_arborescence_kruskal(random_state, undesired_edge_types, verbose);
        (
            subresult_0.into(),
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            },
            subresult_2.into(),
            subresult_3.into(),
            subresult_4.into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns consistent spanning arborescence using Kruskal.
    ///
    /// The spanning tree is NOT minimal.
    ///
    /// The quintuple returned contains:
    /// - Set of the edges used in order to build the spanning arborescence.
    /// - Vector of the connected component of each node.
    /// - Number of connected components.
    /// - Minimum component size.
    /// - Maximum component size.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn spanning_arborescence_kruskal(
        &self,
        verbose: Option<bool>,
    ) -> (
        HashSet<(NodeT, NodeT)>,
        Py<PyArray1<NodeT>>,
        NodeT,
        NodeT,
        NodeT,
    ) {
        let (subresult_0, subresult_1, subresult_2, subresult_3, subresult_4) =
            self.inner.spanning_arborescence_kruskal(verbose);
        (
            subresult_0.into(),
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            },
            subresult_2.into(),
            subresult_3.into(),
            subresult_4.into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns vector of predecessors composing a RANDOM spanning tree.
    ///
    /// This is the implementaiton of [A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors (SMPs)](https://smartech.gatech.edu/bitstream/handle/1853/14355/GT-CSE-06-01.pdf)
    /// by David A. Bader and Guojing Cong.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the system configuration does not allow for the creation of the thread pool.
    /// ValueError
    ///     If the current graph instance is directed.
    ///
    pub fn get_random_spanning_tree(&self, verbose: Option<bool>) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_random_spanning_tree(verbose))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Compute the connected components building in parallel a spanning tree using [bader's algorithm](https://www.sciencedirect.com/science/article/abs/pii/S0743731505000882).
    ///
    /// **This works only for undirected graphs.**
    ///
    /// This method is **not thread save and not deterministic** but by design of the algorithm this
    /// shouldn't matter but if we will encounter non-detemristic bugs here is where we want to look.
    ///
    /// The returned quadruple contains:
    /// - Vector of the connected component for each node.
    /// - Number of connected components.
    /// - Minimum connected component size.
    /// - Maximum connected component size.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given graph is directed.
    /// ValueError
    ///     If the system configuration does not allow for the creation of the thread pool.
    ///
    pub fn get_connected_components(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, NodeT, NodeT, NodeT)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2, subresult_3) =
                pe!(self.inner.get_connected_components(verbose))?.into();
            (
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_0, NodeT)
                },
                subresult_1.into(),
                subresult_2.into(),
                subresult_3.into(),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, vector_sources, vector_reciprocal_sqrt_degrees)")]
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// Parameters
    /// ----------
    /// vector_sources: Optional[bool]
    ///     Whether to cache sources into a vector for faster walks.
    /// vector_reciprocal_sqrt_degrees: Optional[bool]
    ///     Whether to cache reciprocal_sqrt_degrees into a vector for faster laplacian kernel computation.
    ///
    pub fn enable(
        &mut self,
        vector_sources: Option<bool>,
        vector_reciprocal_sqrt_degrees: Option<bool>,
    ) {
        self.inner
            .enable(vector_sources, vector_reciprocal_sqrt_degrees);
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Disable all extra perks, reducing memory impact but incresing time requirements
    pub fn disable_all(&mut self) {
        self.inner.disable_all();
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, precision, bits)")]
    /// Returns an approximation of the total distances centrality for all nodes in the graph.
    ///
    /// This method applies the HyperBall algorithm to compute an approximation of the total distances
    /// from each node. The precision parameter indicates the number of bits to use to
    /// represent the HyperLogLog registers. The higher the precision, the more accurate the
    /// results, but also the more memory required and the slower the algorithm.
    ///
    /// Parameters
    /// ----------
    /// precision: Optional[int]
    ///     The number of bits to use to represent the HyperLogLog registers. By default 6.
    /// bits: Optional[int]
    ///     The number of bits to use for the HyperLogLog counters. It must be either 4, 5 or 6, and by default 6.
    ///
    pub fn get_approximated_total_distances(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_approximated_total_distances(precision, bits))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, precision, bits)")]
    /// Returns an approximation of the closeness centrality for all nodes in the graph.
    ///
    /// This method applies the HyperBall algorithm to compute an approximation of the closeness
    /// centrality of each node. The precision parameter indicates the number of bits to use to
    /// represent the HyperLogLog registers. The higher the precision, the more accurate the
    /// results, but also the more memory required and the slower the algorithm.
    ///
    /// Closeness centrality is a metric that measures the importance of a node in a
    /// graph based on how close it is to all other nodes in the graph.
    /// This is determined by taking the reciprocal of the sum of the shortest
    /// path distances between a node and all other nodes in the graph.
    /// Closeness centrality is a measure of how quickly information can spread through a network,
    /// as nodes that are closer to other nodes can transmit information more efficiently.
    /// Nodes with higher closeness centrality are therefore considered more important
    /// in terms of their ability to communicate with other nodes in the network.
    /// However, closeness centrality is sensitive to disconnected nodes and may not
    /// provide a reliable measure of importance in graphs with multiple connected components.
    ///
    /// Parameters
    /// ----------
    /// precision: Optional[int]
    ///     The number of bits to use to represent the HyperLogLog registers. By default 6.
    /// bits: Optional[int]
    ///     The number of bits to use for the HyperLogLog counters. It must be either 4, 5 or 6, and by default 6.
    ///
    pub fn get_approximated_closeness_centrality(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_approximated_closeness_centrality(precision, bits))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, precision, bits)")]
    /// Returns an approximation of the harmonic centrality for all nodes in the graph.
    ///
    /// This method applies the HyperBall algorithm to compute an approximation of the harmonic
    /// centrality of each node. The precision parameter indicates the number of bits to use to
    /// represent the HyperLogLog registers. The higher the precision, the more accurate the
    /// results, but also the more memory required and the slower the algorithm.
    ///
    /// Harmonic centrality is another metric that measures the importance of a node
    /// in a graph based on its ability to reach other nodes.
    /// It is defined as the sum of the harmonic mean of the distances between a node
    /// and all other nodes in the graph. The harmonic mean is used instead of the
    /// arithmetic mean as it gives greater weight to shorter distances.
    /// Harmonic centrality is also more robust to disconnected nodes than closeness centrality,
    /// as it assigns higher centrality scores to nodes that are closer to other nodes
    /// within their own connected component. However, harmonic centrality does not
    /// provide an accurate measure of importance in terms of communication efficiency,
    /// as it does not take into account the actual distances between nodes.
    /// It is therefore most useful for measuring the ability of a node to reach
    /// other nodes within a connected component.
    ///
    /// Parameters
    /// ----------
    /// precision: Optional[int]
    ///     The number of bits to use to represent the HyperLogLog registers. By default 6.
    /// bits: Optional[int]
    ///     The number of bits to use for the HyperLogLog counters. It must be either 4, 5 or 6, and by default 6.
    ///
    pub fn get_approximated_harmonic_centrality(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_approximated_harmonic_centrality(precision, bits))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, precision, bits, dtype)")]
    /// Returns an approximation of the graph diameter.
    ///
    /// Parameters
    /// ----------
    /// precision: Optional[int]
    ///     The number of bits to use to represent the HyperLogLog registers. By default 6.
    /// bits: Optional[int]
    ///     The number of bits to use for the HyperLogLog counters. It must be either 4, 5 or 6, and by default 6.
    /// dtype: Optional[&str]
    ///     The data type to use for the HyperLogLog counters. It must be either "u8", "u16", "u32" or "u64" and by default "u16".
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the data type is not supported.
    /// ValueError
    ///     If the combination of precision and bits is not supported.
    ///
    pub fn get_approximated_diameter(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
        dtype: Option<&str>,
    ) -> PyResult<usize> {
        Ok(pe!(self.inner.get_approximated_diameter(precision, bits, dtype))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns whether the graphs share the same nodes.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_compatible_node_vocabularies(&self, other: &Graph) -> bool {
        self.inner
            .has_compatible_node_vocabularies(&other.inner)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns whether the graphs share the same node types or absence thereof.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_compatible_node_types_vocabularies(&self, other: &Graph) -> bool {
        self.inner
            .has_compatible_node_types_vocabularies(&other.inner)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns whether the graphs share the same edge types or absence thereof.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_compatible_edge_types_vocabularies(&self, other: &Graph) -> bool {
        self.inner
            .has_compatible_edge_types_vocabularies(&other.inner)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Return true if the graphs are compatible.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If a graph is directed and the other is undirected.
    /// ValueError
    ///     If one of the two graphs has edge weights and the other does not.
    /// ValueError
    ///     If one of the two graphs has node types and the other does not.
    /// ValueError
    ///     If one of the two graphs has edge types and the other does not.
    ///
    pub fn is_compatible(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.is_compatible(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Return true if the graphs share the same adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_same_adjacency_matrix(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.has_same_adjacency_matrix(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, approach, sequential, insert_only_source, random_seed)")]
    /// Returns 2-approximated verted cover bitvec using greedy algorithm.
    ///
    /// Parameters
    /// ----------
    /// approach: Optional[&str]
    ///     The approach name to be used. By default, the edge list order is used.
    /// sequential: Optional[bool]
    ///     Whether to proceed sequantially or concurrently. By default, sequential.
    /// insert_only_source: Optional[bool]
    ///     Whether to insert only the source node or both source and destination.
    /// random_seed: Optional[int]
    ///     The random seed to be used for the stocastic approaches.
    ///
    pub fn get_vertex_cover(
        &self,
        approach: Option<&str>,
        sequential: Option<bool>,
        insert_only_source: Option<bool>,
        random_seed: Option<u64>,
    ) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_vertex_cover(
                    approach,
                    sequential,
                    insert_only_source,
                    random_seed
                ))?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_node_type(&self, random_state: u64) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_random_node_type(random_state.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random edge type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_edge_type(&self, random_state: u64) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_random_edge_type(random_state.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random scale_free edge type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    ///
    /// Safety
    /// ------
    /// * If the graph does not have edge types, the method will always return None.
    pub unsafe fn get_unchecked_random_scale_free_edge_type(
        &self,
        random_state: u64,
    ) -> Option<EdgeTypeT> {
        self.inner
            .get_unchecked_random_scale_free_edge_type(random_state.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random scale_free edge type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_random_scale_free_edge_type(
        &self,
        random_state: u64,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self
            .inner
            .get_random_scale_free_edge_type(random_state.clone()))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random scale free edge weight.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    ///
    /// Safety
    /// ------
    /// * If the graph does not have edge types, the method will always return None.
    pub unsafe fn get_unchecked_random_scale_free_edge_weight(
        &self,
        random_state: u64,
    ) -> Option<WeightT> {
        self.inner
            .get_unchecked_random_scale_free_edge_weight(random_state.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random scale free edge weight.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_random_scale_free_edge_weight(
        &self,
        random_state: u64,
    ) -> PyResult<Option<WeightT>> {
        Ok(pe!(self
            .inner
            .get_random_scale_free_edge_weight(random_state.clone()))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_node(&self, random_state: u64) -> NodeT {
        self.inner.get_random_node(random_state.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random edge ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_edge_id(&self, random_state: u64) -> EdgeT {
        self.inner.get_random_edge_id(random_state.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node ID following outbounds degree distribution of the graph.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_outbounds_scale_free_node(&self, random_state: u64) -> NodeT {
        self.inner
            .get_random_outbounds_scale_free_node(random_state.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node ID following inbounds degree distribution of the graph.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_inbounds_scale_free_node(&self, random_state: u64) -> NodeT {
        self.inner
            .get_random_inbounds_scale_free_node(random_state.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes_to_sample, random_state)")]
    /// Return random unique sorted numbers.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_sorted_unique_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_sorted_unique_random_nodes(
                    number_of_nodes_to_sample.clone(),
                    random_state.clone()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes_to_sample, root_node)")]
    /// Return nodes sampled from the neighbourhood of given root nodes.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// root_node: int
    ///     The root node from .
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of requested nodes is higher than the number of nodes in the graph.
    /// ValueError
    ///     If the given root node does not exist in the curret graph instance.
    ///
    pub fn get_breadth_first_search_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        root_node: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_breadth_first_search_random_nodes(
                    number_of_nodes_to_sample.clone(),
                    root_node.clone()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node, random_state, walk_length, unique)")]
    /// Returns unique nodes sampled from uniform random walk.
    ///
    /// Parameters
    /// ----------
    /// node: int
    ///     Node from where to start the random walks.
    /// random_state: int
    ///     the random_state to use for extracting the nodes and edges.
    /// walk_length: int
    ///     Length of the random walk.
    /// unique: Optional[bool]
    ///     Whether to make the sampled nodes unique.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node does not exist in the current slack.
    ///
    pub fn get_uniform_random_walk_random_nodes(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
        unique: Option<bool>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_uniform_random_walk_random_nodes(
                    node.clone(),
                    random_state.clone(),
                    walk_length.clone(),
                    unique
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return list of the supported node sampling methods
    pub fn get_node_sampling_methods(&self) -> Vec<String> {
        self.inner
            .get_node_sampling_methods()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, node_sampling_method, root_node, unique)"
    )]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// root_node: Optional[int]
    ///     The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// unique: Optional[bool]
    ///     Whether to make the sampled nodes unique.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node sampling method is not supported.
    ///
    pub fn get_subsampled_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        node_sampling_method: &str,
        root_node: Option<NodeT>,
        unique: Option<bool>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_subsampled_nodes(
                    number_of_nodes_to_sample.clone(),
                    random_state.clone(),
                    node_sampling_method,
                    root_node,
                    unique
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, features, iterations, maximal_distance, k1, b, include_central_node, verbose)"
    )]
    /// Returns okapi node features propagation within given maximal distance.
    ///
    /// Parameters
    /// ----------
    /// features: List[Optional[List[float]]]
    ///     The features to propagate. Use None to represent eventual unknown features.
    /// iterations: Optional[int]
    ///     The number of iterations to execute. By default one.
    /// maximal_distance: Optional[int]
    ///     The distance to consider for the cooccurrences. The default value is 3.
    /// k1: Optional[float]
    ///     The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// b: Optional[float]
    ///     The b parameter from okapi. Tipicaly 0.75.
    /// include_central_node: Optional[bool]
    ///     Whether to include the central node. By default true.
    /// verbose: Optional[bool]
    ///     Whether to show loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_okapi_bm25_node_feature_propagation(
        &self,
        features: Vec<Vec<f64>>,
        iterations: Option<usize>,
        maximal_distance: Option<usize>,
        k1: Option<f64>,
        b: Option<f64>,
        include_central_node: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_okapi_bm25_node_feature_propagation(
                    features,
                    iterations,
                    maximal_distance,
                    k1,
                    b,
                    include_central_node,
                    verbose
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, maximal_distance, k1, b, verbose)")]
    /// Returns okapi node label propagation within given maximal distance.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int]
    ///     The number of iterations to execute. By default one.
    /// maximal_distance: Optional[int]
    ///     The distance to consider for the cooccurrences. The default value is 3.
    /// k1: Optional[float]
    ///     The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// b: Optional[float]
    ///     The b parameter from okapi. Tipicaly 0.75.
    /// verbose: Optional[bool]
    ///     Whether to show loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_okapi_bm25_node_label_propagation(
        &self,
        iterations: Option<usize>,
        maximal_distance: Option<usize>,
        k1: Option<f64>,
        b: Option<f64>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_okapi_bm25_node_label_propagation(
                    iterations,
                    maximal_distance,
                    k1,
                    b,
                    verbose
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if graph has name that is not the default one.
    ///
    /// TODO: use a default for the default graph name
    pub fn has_default_graph_name(&self) -> bool {
        self.inner.has_default_graph_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if the graph has any nodes.
    pub fn has_nodes(&self) -> bool {
        self.inner.has_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if the graph has any edges.
    pub fn has_edges(&self) -> bool {
        self.inner.has_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has trap nodes.
    pub fn has_trap_nodes(&self) -> bool {
        self.inner.has_trap_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has trap nodes with self-loops.
    pub fn has_trap_nodes_with_selfloops(&self) -> bool {
        self.inner.has_trap_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.inner.is_directed().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether graph is a directed acyclic graph.
    pub fn is_directed_acyclic(&self) -> bool {
        self.inner.is_directed_acyclic().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing whether graph has weights.
    pub fn has_edge_weights(&self) -> bool {
        self.inner.has_edge_weights().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether graph has weights that can represent probabilities
    pub fn has_edge_weights_representing_probabilities(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_edge_weights_representing_probabilities())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether a graph has one or more weighted singleton nodes.
    ///
    /// A weighted singleton node is a node whose weighted node degree is 0.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn has_weighted_singleton_nodes(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_weighted_singleton_nodes())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the graph has constant weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn has_constant_edge_weights(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_constant_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing whether graph has negative weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain weights.
    ///
    pub fn has_negative_edge_weights(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_negative_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing whether graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.inner.has_edge_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.inner.has_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if nodes which are nor singletons nor
    /// singletons with selfloops.
    pub fn has_disconnected_nodes(&self) -> bool {
        self.inner.has_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes(&self) -> bool {
        self.inner.has_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has singletons
    pub fn has_singleton_nodes_with_selfloops(&self) -> bool {
        self.inner.has_singleton_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns whether the graph is connected.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar while computing the connected components, if necessary.
    ///
    pub fn is_connected(&self, verbose: Option<bool>) -> bool {
        self.inner.is_connected(verbose).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has node types
    pub fn has_node_types(&self) -> bool {
        self.inner.has_node_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has multilabel node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_multilabel_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_multilabel_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether there are unknown node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_unknown_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_unknown_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether there are known node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_known_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_known_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether there are unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_unknown_edge_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_unknown_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether there are known edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_known_edge_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_known_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the nodes have an homogenous node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_homogeneous_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_homogeneous_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the nodes have exclusively homogenous node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_exclusively_homogeneous_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_exclusively_homogeneous_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the nodes have an homogenous node ontology
    pub fn has_homogeneous_node_ontologies(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_homogeneous_node_ontologies())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the edges have an homogenous edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_homogeneous_edge_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_homogeneous_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the edges have exclusively homogeneous edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_exclusively_homogeneous_edge_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_exclusively_homogeneous_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether there is at least singleton node type, that is a node type that only appears once.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_singleton_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_singleton_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the graph has exclusively singleton node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_exclusively_singleton_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_exclusively_singleton_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has any known node-related graph oddities
    pub fn has_node_oddities(&self) -> bool {
        self.inner.has_node_oddities().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has any known node type-related graph oddities.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_node_types_oddities(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_node_types_oddities())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether there is at least singleton edge type, that is a edge type that only appears once.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_singleton_edge_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_singleton_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the graph has exclusively singleton edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_exclusively_singleton_edge_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_exclusively_singleton_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has any known edge type-related graph oddities.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_edge_types_oddities(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_edge_types_oddities())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if there are multiple edges between two node
    pub fn is_multigraph(&self) -> bool {
        self.inner.is_multigraph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether at least a node has a known ontology
    pub fn has_node_ontologies(&self) -> bool {
        self.inner.has_node_ontologies().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether at least a node has an unknown ontology
    pub fn has_unknown_node_ontologies(&self) -> bool {
        self.inner.has_unknown_node_ontologies().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_decreasing_outbound_node_degree(&self) -> bool {
        self.inner
            .has_nodes_sorted_by_decreasing_outbound_node_degree()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_lexicographic_order(&self) -> bool {
        self.inner.has_nodes_sorted_by_lexicographic_order().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the graph contains the identity matrix.
    pub fn contains_identity_matrix(&self) -> bool {
        self.inner.contains_identity_matrix().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the node IDs are sorted by increasing outbound node degree.
    pub fn has_nodes_sorted_by_increasing_outbound_node_degree(&self) -> bool {
        self.inner
            .has_nodes_sorted_by_increasing_outbound_node_degree()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the sources time-memory tradeoff is enabled
    pub fn has_sources_tradeoff_enabled(&self) -> bool {
        self.inner.has_sources_tradeoff_enabled().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the reciprocal_sqrt_degrees time-memory tradeoff is enabled
    pub fn has_reciprocal_sqrt_degrees_tradeoff_enabled(&self) -> bool {
        self.inner
            .has_reciprocal_sqrt_degrees_tradeoff_enabled()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of detected dentritic trees
    pub fn get_dendritic_trees(&self) -> PyResult<Vec<DendriticTree>> {
        Ok(pe!(self.inner.get_dendritic_trees())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, verbose)")]
    /// Returns graph to the i-th transitivity closure iteration.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int]
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn get_transitive_closure(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Graph {
        self.inner
            .get_transitive_closure(iterations, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, verbose)")]
    /// Returns graph with unweighted shortest paths computed up to the given depth.
    ///
    /// The returned graph will have no selfloops.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int]
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn get_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Graph {
        self.inner
            .get_all_shortest_paths(iterations, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, use_edge_weights_as_probabilities, verbose)")]
    /// Returns graph with weighted shortest paths computed up to the given depth.
    ///
    /// The returned graph will have no selfloops.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int]
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have weights.
    /// ValueError
    ///     If the graph contains negative weights.
    /// ValueError
    ///     If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    pub fn get_weighted_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.get_weighted_all_shortest_paths(
            iterations,
            use_edge_weights_as_probabilities,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns option with the weight of the given edge id.
    ///
    /// This method will raise a panic if the given edge ID is higher than
    /// the number of edges in the graph. Additionally, it will simply
    /// return None if there are no graph weights.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge whose edge weight is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exists in the graph this method will panic.
    pub unsafe fn get_unchecked_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.inner
            .get_unchecked_edge_weight_from_edge_id(edge_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns option with the weight of the given node ids.
    ///
    /// This method will raise a panic if the given node IDs are higher than
    /// the number of nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node ID.
    /// dst: int
    ///     The destination node ID.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the two given node IDs does not exists in the graph.
    pub unsafe fn get_unchecked_edge_weight_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> WeightT {
        self.inner
            .get_unchecked_edge_weight_from_node_ids(src.clone(), dst.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns node id from given node name raising a panic if used unproperly.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name whose node ID is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node name does not exists in the considered graph the method will panic.
    pub unsafe fn get_unchecked_node_id_from_node_name(&self, node_name: &str) -> NodeT {
        self.inner
            .get_unchecked_node_id_from_node_name(node_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Return edge type ID corresponding to the given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name whose edge type ID is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge type name does not exists in the considered graph the method will panic.
    pub unsafe fn get_unchecked_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: &str,
    ) -> Option<EdgeTypeT> {
        self.inner
            .get_unchecked_edge_type_id_from_edge_type_name(edge_type_name)
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Return edge type ID corresponding to the given edge type name
    /// raising panic if edge type ID does not exists in current graph.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     The edge type naIDme whose edge type name is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge type ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Option<String> {
        self.inner
            .get_unchecked_edge_type_name_from_edge_type_id(edge_type_id)
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type)")]
    /// Return number of edges of the given edge type without checks.
    ///
    /// Parameters
    /// ----------
    /// edge_type: Optional[int]
    ///     The edge type to retrieve count of.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge type ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_count_from_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.inner
            .get_unchecked_edge_count_from_edge_type_id(edge_type)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type)")]
    /// Return number of nodes of the given node type without checks.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Safety
    /// ------
    /// If the provided value is not within the graph's vocabulary
    ///  the method will panic.
    pub unsafe fn get_unchecked_node_count_from_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> NodeT {
        self.inner
            .get_unchecked_node_count_from_node_type_id(node_type)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
    /// Return edge ID without any checks for given tuple of nodes and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node of the edge.
    /// dst: int
    ///     Destination node of the edge.
    /// edge_type: Optional[int]
    ///     Edge Type of the edge.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node IDs or edge type does not exists in the graph this method will panic.
    pub unsafe fn get_unchecked_edge_id_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.inner
            .get_unchecked_edge_id_from_node_ids_and_edge_type_id(
                src.clone(),
                dst.clone(),
                edge_type,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node.
    /// dst: int
    ///     Destination node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node type IDs do not exist in the graph this method will panic.
    pub unsafe fn get_unchecked_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_minmax_edge_ids_from_node_ids(src.clone(), dst.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns node IDs corresponding to given edge ID.
    ///
    /// The method will panic if the given edge ID does not exists in the
    /// current graph instance.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_node_ids_from_edge_id(edge_id.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_names_from_edge_id(&self, edge_id: EdgeT) -> (String, String) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_node_names_from_edge_id(edge_id.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns the source of given edge id without making any boundary check.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will cause an out of bounds.
    pub unsafe fn get_unchecked_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.inner
            .get_unchecked_source_node_id_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns the destination of given edge id without making any boundary check.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose destination is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will cause an out of bounds.
    pub unsafe fn get_unchecked_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.inner
            .get_unchecked_destination_node_id_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns source node ID corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source node ID is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge ID does not exist in the current graph.
    ///
    pub fn get_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_source_node_id_from_edge_id(edge_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns destination node ID corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose destination node ID is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge ID does not exist in the current graph.
    ///
    pub fn get_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_destination_node_id_from_edge_id(edge_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns number of self-loops associated to the provided node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID for which to retrieve the number of self-loops.
    ///
    ///
    /// Safety
    /// ------
    /// This method may panic if the provided node ID is outside
    ///  the number of nodes in the graph.
    pub unsafe fn get_unchecked_number_of_selfloops_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.inner
            .get_unchecked_number_of_selfloops_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns number of self-loops associated to the provided node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID for which to retrieve the number of self-loops.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_number_of_selfloops_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_selfloops_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns number of self-loops associated to the provided node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name for which to retrieve the number of self-loops.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_number_of_selfloops_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_selfloops_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns source node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source node name is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_source_node_name_from_edge_id(&self, edge_id: EdgeT) -> String {
        self.inner
            .get_unchecked_source_node_name_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns destination node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose destination node name is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_destination_node_name_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> String {
        self.inner
            .get_unchecked_destination_node_name_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns source node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source node name is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_source_node_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_source_node_name_from_edge_id(edge_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns destination node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose destination node name is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_destination_node_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_destination_node_name_from_edge_id(edge_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_names_from_edge_id(&self, edge_id: EdgeT) -> PyResult<(String, String)> {
        Ok({
            let (subresult_0, subresult_1) =
                pe!(self.inner.get_node_names_from_edge_id(edge_id.clone()))?.into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> PyResult<(NodeT, NodeT)> {
        Ok({
            let (subresult_0, subresult_1) =
                pe!(self.inner.get_node_ids_from_edge_id(edge_id.clone()))?.into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// The method will panic if the given source and destination node IDs do
    /// not correspond to an edge in this graph instance.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node ID.
    /// dst: int
    ///     The destination node ID.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs do not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.inner
            .get_unchecked_edge_id_from_node_ids(src.clone(), dst.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node ID.
    /// dst: int
    ///     The destination node ID.
    ///
    pub fn get_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_edge_id_from_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_id)")]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_id: int
    ///     The source node ID.
    ///
    ///
    /// Safety
    /// ------
    /// If the given source node ID does not exist in the current graph the method will panic.
    pub unsafe fn get_unchecked_unique_source_node_id(&self, source_id: NodeT) -> NodeT {
        self.inner
            .get_unchecked_unique_source_node_id(source_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source, destination and edge type are to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>) {
        let (subresult_0, subresult_1, subresult_2) = self
            .inner
            .get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id.clone());
        (
            subresult_0.into(),
            subresult_1.into(),
            subresult_2.map(|x| x.into()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source, destination and edge type are to be retrieved.
    ///
    pub fn get_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_node_ids_and_edge_type_id_from_edge_id(edge_id.clone()))?
            .into();
            (
                subresult_0.into(),
                subresult_1.into(),
                subresult_2.map(|x| x.into()),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
        let (subresult_0, subresult_1, subresult_2, subresult_3) = self
            .inner
            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.clone());
        (
            subresult_0.into(),
            subresult_1.into(),
            subresult_2.map(|x| x.into()),
            subresult_3.map(|x| x.into()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    pub fn get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2, subresult_3) = pe!(self
                .inner
                .get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.clone()))?
            .into();
            (
                subresult_0.into(),
                subresult_1.into(),
                subresult_2.map(|x| x.into()),
                subresult_3.map(|x| x.into()),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return vector with unweighted top k central node Ids.
    ///
    /// If the k passed is bigger than the number of nodes this method will return
    /// all the nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     Number of central nodes to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given value k is zero.
    /// ValueError
    ///     If the graph has no nodes.
    ///
    pub fn get_top_k_central_node_ids(&self, k: NodeT) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_top_k_central_node_ids(k.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return vector with weighted top k central node Ids.
    ///
    /// If the k passed is bigger than the number of nodes this method will return
    /// all the nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     Number of central nodes to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph instance does not contain edge weights.
    /// ValueError
    ///     If the given value k is zero.
    ///
    pub fn get_weighted_top_k_central_node_ids(&self, k: NodeT) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_top_k_central_node_ids(k.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the number of outbound neighbours of given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_degree_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.inner
            .get_unchecked_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns number of outbound nodes for a given node ID, adjusted by removing the number of selfloops.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_selfloop_excluded_node_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> NodeT {
        self.inner
            .get_unchecked_selfloop_excluded_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns number of outbound nodes for a given node ID, adjusted by removing the number of selfloops.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     ValueError - If the given node ID does not exist in the current graph the method will raise a panic.
    ///
    pub fn get_selfloop_adjusted_node_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_selfloop_adjusted_node_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns number of outbound nodes for a given node name, adjusted by removing the number of selfloops.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Integer name of the node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     ValueError - If the given node name does not exist in the current graph the method will raise a panic.
    ///
    pub fn get_selfloop_adjusted_node_degree_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_selfloop_adjusted_node_degree_from_node_name(node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the weighted sum of outbound neighbours of given node.
    ///
    /// The method will panic if the given node id is higher than the number of
    /// nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_weighted_node_degree_from_node_id(&self, node_id: NodeT) -> f64 {
        self.inner
            .get_unchecked_weighted_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the number of outbound neighbours of given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    pub fn get_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degree_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the comulative node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_comulative_node_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> EdgeT {
        self.inner
            .get_unchecked_comulative_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the comulative node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    pub fn get_comulative_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_comulative_node_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the reciprocal squared root node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_reciprocal_sqrt_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> WeightT {
        self.inner
            .get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the reciprocal squared root node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    pub fn get_reciprocal_sqrt_degree_from_node_id(&self, node_id: NodeT) -> PyResult<WeightT> {
        Ok(pe!(self
            .inner
            .get_reciprocal_sqrt_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Return vector with reciprocal squared root degree of the provided nodes.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The vector of node IDs whose reciprocal squared root degree is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// This method makes the assumption that the provided node IDs exist in the graph, that is
    ///  they are not higher than the number of nodes in the graph.
    pub unsafe fn get_unchecked_reciprocal_sqrt_degrees_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> Py<PyArray1<WeightT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_unchecked_reciprocal_sqrt_degrees_from_node_ids(&node_ids),
            WeightT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the weighted sum of outbound neighbours of given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    pub fn get_weighted_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_weighted_node_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns the number of outbound neighbours of given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Integer ID of the node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node name does not exist in the graph.
    ///
    pub fn get_node_degree_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degree_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return vector with top k central node names.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     Number of central nodes to extract.
    ///
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_top_k_central_node_names(k.clone()))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns option with vector of node types of given node.
    ///
    /// This method will panic if the given node ID is greater than
    /// the number of nodes in the graph.
    /// Furthermore, if the graph does NOT have node types, it will NOT
    /// return neither an error or a panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     node whose node type is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// Even though the method will return an option when the node types are
    ///  not available for the current graph, the behaviour is undefined.
    pub unsafe fn get_unchecked_node_type_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Py<PyArray1<NodeTypeT>>> {
        self.inner
            .get_unchecked_node_type_ids_from_node_id(node_id.clone())
            .map(|x| {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, x.to_vec(), NodeTypeT)
            })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns node type of given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     node whose node type is to be returned.
    ///
    pub fn get_node_type_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Option<Py<PyArray1<NodeTypeT>>>> {
        Ok(
            pe!(self.inner.get_node_type_ids_from_node_id(node_id.clone()))?.map(|x| {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, x.to_vec(), NodeTypeT)
            }),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns edge type of given edge.
    ///
    /// This method will panic if the given edge ID is greater than
    /// the number of edges in the graph.
    /// Furthermore, if the graph does NOT have edge types, it will NOT
    /// return neither an error or a panic.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     edge whose edge type is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Option<EdgeTypeT> {
        self.inner
            .get_unchecked_edge_type_id_from_edge_id(edge_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns edge type name of given edge.
    ///
    /// This method will panic if the given edge ID is greater than
    /// the number of edges in the graph.
    /// Furthermore, if the graph does NOT have edge types, it will NOT
    /// return neither an error or a panic.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     edge whose edge type is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_edge_type_name_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Option<String> {
        self.inner
            .get_unchecked_edge_type_name_from_edge_id(edge_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns edge type of given edge.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     edge whose edge type is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self.inner.get_edge_type_id_from_edge_id(edge_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns edge type from given edge node IDs.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node ID of the node of interest.
    /// dst: int
    ///     Destination node ID of the node of interest.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided nodes do not form an edge.
    ///
    pub fn get_edge_type_id_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self
            .inner
            .get_edge_type_id_from_edge_node_ids(src.clone(), dst.clone()))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns result of option with the node type of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose node types are to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// This method will return an iterator of None values when the graph
    ///  does not contain node types.
    pub unsafe fn get_unchecked_node_type_names_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<String>> {
        self.inner
            .get_unchecked_node_type_names_from_node_id(node_id.clone())
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns result of option with the node type of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose node types are to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the node types are not available for the current graph instance.
    ///
    pub fn get_node_type_names_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Option<Vec<String>>> {
        Ok(
            pe!(self.inner.get_node_type_names_from_node_id(node_id.clone()))?
                .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns result of option with the node type of the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name whose node types are to be returned.
    ///
    pub fn get_node_type_names_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Vec<String>>> {
        Ok(
            pe!(self.inner.get_node_type_names_from_node_name(node_name))?
                .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns option with the edge type of the given edge id.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose edge type is to be returned.
    ///
    pub fn get_edge_type_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<String>> {
        Ok(pe!(self.inner.get_edge_type_name_from_edge_id(edge_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Return edge type name of given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     Id of the edge type.
    ///
    pub fn get_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_edge_type_name_from_edge_type_id(edge_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns weight of the given edge id.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose weight is to be returned.
    ///
    pub fn get_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> PyResult<WeightT> {
        Ok(pe!(self.inner.get_edge_weight_from_edge_id(edge_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns weight of the given node ids.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The node ID of the source node.
    /// dst: int
    ///     The node ID of the destination node.
    ///
    pub fn get_edge_weight_from_node_ids(&self, src: NodeT, dst: NodeT) -> PyResult<WeightT> {
        Ok(pe!(self
            .inner
            .get_edge_weight_from_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
    /// Returns weight of the given node ids and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The node ID of the source node.
    /// dst: int
    ///     The node ID of the destination node.
    /// edge_type: Optional[int]
    ///     The edge type ID of the edge.
    ///
    pub fn get_edge_weight_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> PyResult<WeightT> {
        Ok(
            pe!(self.inner.get_edge_weight_from_node_ids_and_edge_type_id(
                src.clone(),
                dst.clone(),
                edge_type
            ))?
            .into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
    /// Returns weight of the given node names and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: str
    ///     The node name of the source node.
    /// dst: str
    ///     The node name of the destination node.
    /// edge_type: Optional[&str]
    ///     The edge type name of the edge.
    ///
    pub fn get_edge_weight_from_node_names_and_edge_type_name(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<&str>,
    ) -> PyResult<WeightT> {
        Ok(pe!(self
            .inner
            .get_edge_weight_from_node_names_and_edge_type_name(src, dst, edge_type))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name)")]
    /// Returns weight of the given node names.
    ///
    /// Parameters
    /// ----------
    /// src_name: str
    ///     The node name of the source node.
    /// dst_name: str
    ///     The node name of the destination node.
    ///
    pub fn get_edge_weight_from_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> PyResult<WeightT> {
        Ok(pe!(self
            .inner
            .get_edge_weight_from_node_names(src_name, dst_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns result with the node name.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose name is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_name_from_node_id(&self, node_id: NodeT) -> String {
        self.inner
            .get_unchecked_node_name_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns result with the node name.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose name is to be returned.
    ///
    pub fn get_node_name_from_node_id(&self, node_id: NodeT) -> PyResult<String> {
        Ok(pe!(self.inner.get_node_name_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns result with the node ID.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name whose node ID is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When the given node name does not exists in the current graph.
    ///
    pub fn get_node_id_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_id_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names)")]
    /// Returns result with the node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[&str]
    ///     The node names whose node IDs is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node name does not exists in the current graph.
    ///
    pub fn get_node_ids_from_node_names(
        &self,
        node_names: Vec<&str>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_from_node_names(node_names))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns result with the node names.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node ids whose node names are to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node ids does not exists in the current graph.
    ///
    pub fn get_node_names_from_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_node_names_from_node_ids(node_ids))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_node_names)")]
    /// Returns result with the edge node IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_node_names: List[Tuple[str, str]]
    ///     The node names whose node IDs is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node name does not exists in the current graph.
    ///
    pub fn get_edge_node_ids_from_edge_node_names(
        &self,
        edge_node_names: Vec<(&str, &str)>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_from_edge_node_names(edge_node_names))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_node_ids)")]
    /// Returns result with the edge node names.
    ///
    /// Parameters
    /// ----------
    /// edge_node_ids: List[Tuple[int, int]]
    ///     The node names whose node names is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node IDs does not exists in the current graph.
    ///
    pub fn get_edge_node_names_from_edge_node_ids(
        &self,
        edge_node_ids: Vec<(NodeT, NodeT)>,
    ) -> PyResult<Vec<(String, String)>> {
        Ok(pe!(self
            .inner
            .get_edge_node_names_from_edge_node_ids(edge_node_ids))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return node type ID for the given node name if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Name of the node.
    ///
    pub fn get_node_type_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Py<PyArray1<NodeTypeT>>>> {
        Ok(
            pe!(self.inner.get_node_type_ids_from_node_name(node_name))?.map(|x| {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, x.to_vec(), NodeTypeT)
            }),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return node type name for the given node name if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Name of the node.
    ///
    pub fn get_node_type_name_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Vec<String>>> {
        Ok(
            pe!(self.inner.get_node_type_name_from_node_name(node_name))?
                .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Return number of edges with given edge type ID.
    ///
    /// If None is given as an edge type ID, the unknown edge type IDs
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     The edge type ID to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_edge_count_from_edge_type_id(edge_type_id))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Return edge type ID curresponding to given edge type name.
    ///
    /// If None is given as an edge type ID, None is returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[&str]
    ///     The edge type name whose ID is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self
            .inner
            .get_edge_type_id_from_edge_type_name(edge_type_name))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Return number of edges with given edge type name.
    ///
    /// If None is given as an edge type name, the unknown edge types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[&str]
    ///     The edge type name to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_edge_count_from_edge_type_name(edge_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Return node type ID curresponding to given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type name whose ID is to be returned.
    ///
    pub fn get_node_type_id_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> PyResult<NodeTypeT> {
        Ok(pe!(self
            .inner
            .get_node_type_id_from_node_type_name(node_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Return number of nodes with given node type ID.
    ///
    /// If None is given as an node type ID, the unknown node types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: Optional[int]
    ///     The node type ID to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_count_from_node_type_id(node_type_id))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Return number of nodes with given node type name.
    ///
    /// If None is given as an node type name, the unknown node types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: Optional[&str]
    ///     The node type name to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_name(
        &self,
        node_type_name: Option<&str>,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_node_count_from_node_type_name(node_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return vector of destinations for the given source node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node ID whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_neighbour_node_ids_from_node_id(node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return vector of destinations for the given source node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Node ID whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_neighbour_node_ids_from_node_name(node_name))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return vector of destination names for the given source node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Node name whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_names_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_neighbour_node_names_from_node_name(node_name))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node.
    /// dst: int
    ///     Destination node.
    ///
    pub fn get_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<(EdgeT, EdgeT)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_minmax_edge_ids_from_node_ids(src.clone(), dst.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
    /// Return edge ID for given tuple of nodes and edge type.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node of the edge.
    /// dst: int
    ///     Destination node of the edge.
    /// edge_type: Optional[int]
    ///     Edge Type of the edge.
    ///
    pub fn get_edge_id_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_edge_id_from_node_ids_and_edge_type_id(
            src.clone(),
            dst.clone(),
            edge_type
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name)")]
    /// Return edge ID for given tuple of node names.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// Parameters
    /// ----------
    /// src_name: str
    ///     Source node name of the edge.
    /// dst_name: str
    ///     Destination node name of the edge.
    ///
    pub fn get_edge_id_from_node_names(&self, src_name: &str, dst_name: &str) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_edge_id_from_node_names(src_name, dst_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name, edge_type_name)")]
    /// Return edge ID for given tuple of node names and edge type name.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// Parameters
    /// ----------
    /// src_name: str
    ///     Source node name of the edge.
    /// dst_name: str
    ///     Destination node name of the edge.
    /// edge_type_name: Optional[&str]
    ///     Edge type name.
    ///
    pub fn get_edge_id_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> PyResult<EdgeT> {
        Ok(
            pe!(self.inner.get_edge_id_from_node_names_and_edge_type_name(
                src_name,
                dst_name,
                edge_type_name
            ))?
            .into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_names)")]
    /// Return translated edge types from string to internal edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: List[Optional[&str]]
    ///     Vector of edge types to be converted.
    ///
    pub fn get_edge_type_ids_from_edge_type_names(
        &self,
        edge_type_names: Vec<Option<&str>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_edge_type_ids_from_edge_type_names(&edge_type_names))?
        .into_iter()
        .map(|x| x.map(|x| x.into()))
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[&str]]
    ///     Vector of node types to be converted.
    ///
    pub fn get_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<&str>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_node_type_ids_from_node_type_names(&node_type_names))?
        .into_iter()
        .map(|x| x.map(|x| x.into()))
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[List[&str]]]
    ///     Vector of node types to be converted.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If any of the given node type names do not exists in the graph.
    ///
    pub fn get_multiple_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<Vec<&str>>>,
    ) -> PyResult<Vec<Option<Py<PyArray1<NodeTypeT>>>>> {
        Ok(pe!(self
            .inner
            .get_multiple_node_type_ids_from_node_type_names(node_type_names))?
        .into_iter()
        .map(|x| {
            x.map(|x| {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, x, NodeTypeT)
            })
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src)")]
    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// The method will panic if the given source node ID is higher than
    /// the number of nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Node for which we need to compute the cumulative_node_degrees range.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_minmax_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> (EdgeT, EdgeT) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_minmax_edge_ids_from_source_node_id(src.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src)")]
    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Node for which we need to compute the cumulative_node_degrees range.
    ///
    pub fn get_minmax_edge_ids_from_source_node_id(&self, src: NodeT) -> PyResult<(EdgeT, EdgeT)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_minmax_edge_ids_from_source_node_id(src.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Return node type name of given node type.
    ///
    /// There is no need for a unchecked version since we will have to map
    /// on the note_types anyway.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     Id of the node type.
    ///
    pub fn get_node_type_name_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_node_type_name_from_node_type_id(node_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids)")]
    /// Return node type name of given node type.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids: List[int]
    ///     Id of the node type.
    ///
    ///
    /// Safety
    /// ------
    /// The method will panic if the graph does not contain node types.
    pub unsafe fn get_unchecked_node_type_names_from_node_type_ids(
        &self,
        node_type_ids: Vec<NodeTypeT>,
    ) -> Vec<String> {
        self.inner
            .get_unchecked_node_type_names_from_node_type_ids(&node_type_ids)
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Return number of nodes with the provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type to return the number of nodes of.
    ///
    ///
    /// Safety
    /// ------
    /// The method may panic if an invalid node type (one not present in the graph)
    ///  is provided. If the graph does not have node types, zero will be returned.
    pub unsafe fn get_unchecked_number_of_nodes_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> NodeT {
        self.inner
            .get_unchecked_number_of_nodes_from_node_type_id(node_type_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Return number of nodes with the provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type to return the number of nodes of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the provided node type ID does not exist in the graph.
    ///
    pub fn get_number_of_nodes_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_nodes_from_node_type_id(node_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Return number of nodes with the provided node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type to return the number of nodes of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the provided node type name does not exist in the graph.
    ///
    pub fn get_number_of_nodes_from_node_type_name(&self, node_type_name: &str) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_nodes_from_node_type_name(node_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Return number of edges with the provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type to return the number of edges of.
    ///
    ///
    /// Safety
    /// ------
    /// The method may panic if an invalid edge type (one not present in the graph)
    ///  is provided. If the graph does not have edge types, zero will be returned.
    pub unsafe fn get_unchecked_number_of_edges_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> EdgeT {
        self.inner
            .get_unchecked_number_of_edges_from_edge_type_id(edge_type_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Return number of edges with the provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type to return the number of edges of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the provided edge type ID does not exist in the graph.
    ///
    pub fn get_number_of_edges_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_number_of_edges_from_edge_type_id(edge_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Return number of edges with the provided edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type to return the number of edges of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the provided edge type name does not exist in the graph.
    ///
    pub fn get_number_of_edges_from_edge_type_name(&self, edge_type_name: &str) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_number_of_edges_from_edge_type_name(edge_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns node type IDs counts hashmap for the provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node IDs to consider for this count.
    ///
    ///
    /// Safety
    /// ------
    /// Must have node types and the provided node IDs must exit in the graph
    ///  or the result will be undefined and most likely will lead to panic.
    pub unsafe fn get_unchecked_node_type_id_counts_hashmap_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> PyResult<HashMap<NodeTypeT, NodeT>> {
        Ok(pe!(self
            .inner
            .get_unchecked_node_type_id_counts_hashmap_from_node_ids(&node_ids))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns edge type IDs counts hashmap for the provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node IDs to consider for this count.
    ///
    ///
    /// Safety
    /// ------
    /// Must have edge types and the provided node IDs must exit in the graph
    ///  or the result will be undefined and most likely will lead to panic.
    pub unsafe fn get_unchecked_edge_type_id_counts_hashmap_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
        Ok(pe!(self
            .inner
            .get_unchecked_edge_type_id_counts_hashmap_from_node_ids(&node_ids))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed, edge_type_id)")]
    /// Returns vector containing edge node IDs with given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type ID to extract.
    /// directed: bool
    ///     Whether to iterate the edge list as directed or undirected.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type ID does not exist in the graph.
    ///
    pub fn get_edge_node_ids_from_edge_type_id(
        &self,
        directed: bool,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_from_edge_type_id(directed.clone(), edge_type_id))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns vector containing directed edge node IDs with given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type ID to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type ID does not exist in the graph.
    ///
    pub fn get_directed_edge_node_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_directed_edge_node_ids_from_edge_type_id(edge_type_id))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns vector containing directed edge node names with given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type ID to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type ID does not exist in the graph.
    ///
    pub fn get_directed_edge_node_names_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Vec<(String, String)>> {
        Ok(pe!(self
            .inner
            .get_directed_edge_node_names_from_edge_type_id(edge_type_id))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns vector containing directed edge node names with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[int]
    ///     Edge type name to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type name does not exist in the graph.
    ///
    pub fn get_directed_edge_node_names_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Vec<(String, String)>> {
        Ok(pe!(self
            .inner
            .get_directed_edge_node_names_from_edge_type_name(edge_type_name))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns vector containing directed edge IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type id to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type id does not exist in the graph.
    ///
    pub fn get_directed_edge_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_directed_edge_ids_from_edge_type_id(edge_type_id))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed, edge_type_name)")]
    /// Returns vector containing edge node IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[&str]
    ///     Edge type name to extract.
    /// directed: bool
    ///     Whether to iterate the edge list as directed or undirected.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type name does not exist in the graph.
    ///
    pub fn get_edge_node_ids_from_edge_type_name(
        &self,
        directed: bool,
        edge_type_name: Option<&str>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_from_edge_type_name(directed.clone(), edge_type_name))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns vector containing directed edge node IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: Optional[int]
    ///     Edge type names to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type names does not exist in the graph.
    ///
    pub fn get_directed_edge_node_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_directed_edge_node_ids_from_edge_type_name(edge_type_name))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns vector containing directed edge IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: Optional[int]
    ///     Edge type names to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type names does not exist in the graph.
    ///
    pub fn get_directed_edge_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_directed_edge_ids_from_edge_type_name(edge_type_name))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns vector of directed edge node names with given node name prefixes
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_directed_edge_node_names_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> Vec<(String, String)> {
        self.inner
            .get_directed_edge_node_names_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes,
            )
            .into_iter()
            .map(|x| {
                let (subresult_0, subresult_1) = x;
                (subresult_0.into(), subresult_1.into())
            })
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns vector of directed edge node IDs with given node name prefixes
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_directed_edge_node_ids_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> Py<PyArray2<NodeT>> {
        // Warning: this copies the array so it uses double the memory.
        // To avoid this you should directly generate data compatible with a numpy array
        // Which is a flat vector with row-first or column-first unrolling
        let gil = pyo3::Python::acquire_gil();
        let body = self
            .inner
            .get_directed_edge_node_ids_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes,
            );
        let result_array = ThreadDataRaceAware {
            t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
        };
        body.into_par_iter()
            .enumerate()
            .for_each(|(i, (a, b))| unsafe {
                *(result_array.t.uget_mut([i, 0])) = a;
                *(result_array.t.uget_mut([i, 1])) = b;
            });
        result_array.t.to_owned()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns vector of directed edge IDs with given node name prefixes.
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_directed_edge_ids_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> Py<PyArray1<EdgeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_directed_edge_ids_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes
            ),
            EdgeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns number of directed edge IDs with given node name prefixes.
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_number_of_directed_edges_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> EdgeT {
        self.inner
            .get_number_of_directed_edges_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, curie_prefixes)")]
    /// Returns vector with node IDs with given curie prefix.
    ///
    /// Parameters
    /// ----------
    /// curie_prefixes: List[str]
    ///     Prefix of the source node names.
    ///
    pub fn get_node_ids_from_node_curie_prefixes(
        &self,
        curie_prefixes: Vec<&str>,
    ) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_node_ids_from_node_curie_prefixes(&curie_prefixes),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, curie_prefixes)")]
    /// Returns vector with node names with given curie prefix.
    ///
    /// Parameters
    /// ----------
    /// curie_prefixes: List[&str]
    ///     Prefix of the source node names.
    ///
    pub fn get_node_names_from_node_curie_prefixes(
        &self,
        curie_prefixes: Vec<&str>,
    ) -> Vec<String> {
        self.inner
            .get_node_names_from_node_curie_prefixes(curie_prefixes)
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, curie_prefixes)")]
    /// Returns number of nodes with node IDs with given curie prefix.
    ///
    /// Parameters
    /// ----------
    /// curie_prefixes: List[&str]
    ///     Prefix of the source node names.
    ///
    pub fn get_number_of_nodes_from_node_curie_prefixes(&self, curie_prefixes: Vec<&str>) -> NodeT {
        self.inner
            .get_number_of_nodes_from_node_curie_prefixes(&curie_prefixes)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, separator)")]
    /// Returns vector with node names prefixes when the node names include the provided separator.
    ///
    /// Parameters
    /// ----------
    /// separator: Optional[&str]
    ///     The separator to use to determine a prefix. By default, a column
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided separator is empty.
    ///
    pub fn get_node_names_prefixes(&self, separator: Option<&str>) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_node_names_prefixes(separator))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns mapping from the current graph node names to the other provided graph node names.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph to which remap the node names.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not contained in the provided other graph.
    ///
    pub fn get_node_ids_mapping_from_graph(&self, other: &Graph) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_mapping_from_graph(&other.inner))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns the degree of every node in the provided subgraph
    pub fn get_non_zero_subgraph_node_degrees(
        &self,
        subgraph: &Graph,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_non_zero_subgraph_node_degrees(&subgraph.inner))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns edge IDs of multigraph edge ids with same source and destination nodes and different edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node id of the edge.
    /// dst: int
    ///      Destination node id of the edge.
    ///
    pub fn get_multigraph_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_multigraph_edge_ids_from_node_ids(src.clone(), dst.clone()))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns number of multigraph edges with same source and destination nodes and different edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node id of the edge.
    /// dst: int
    ///      Destination node id of the edge.
    ///
    pub fn get_number_of_multigraph_edges_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<usize> {
        Ok(pe!(self
            .inner
            .get_number_of_multigraph_edges_from_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs, first_node_ids, second_node_ids)")]
    /// Returns shared ancestors of the provided node ids.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// first_node_ids: List[int]
    ///     The first node ids to query for.
    /// second_node_ids: List[int]
    ///     The second node ids to query for.
    ///
    pub fn get_ancestors_jaccard_from_node_ids(
        &self,
        bfs: &ShortestPathsResultBFS,
        first_node_ids: Vec<NodeT>,
        second_node_ids: Vec<NodeT>,
    ) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_ancestors_jaccard_from_node_ids(
                    bfs.into(),
                    &first_node_ids,
                    &second_node_ids
                ))?,
                WeightT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs, first_node_names, second_node_names)")]
    /// Returns shared ancestors of the provided node names.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// first_node_names: List[str]
    ///     The first node names to query for.
    /// second_node_names: List[str]
    ///     The second node names to query for.
    ///
    pub fn get_ancestors_jaccard_from_node_names(
        &self,
        bfs: &ShortestPathsResultBFS,
        first_node_names: Vec<String>,
        second_node_names: Vec<String>,
    ) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_ancestors_jaccard_from_node_names(
                    bfs.into(),
                    &first_node_names,
                    &second_node_names
                ))?,
                WeightT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    ///
    pub fn get_heterogeneous_graphlet_ids_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<HashMap<u16, u32>> {
        Ok(pe!(self
            .inner
            .get_heterogeneous_graphlet_ids_from_edge_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    ///
    pub fn get_heterogeneous_graphlet_names_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<HashMap<String, u32>> {
        Ok(pe!(self
            .inner
            .get_heterogeneous_graphlet_names_from_edge_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    ///
    pub fn get_heterogeneous_graphlet_names_from_edge_node_names(
        &self,
        src: &str,
        dst: &str,
    ) -> PyResult<HashMap<String, u32>> {
        Ok(pe!(self
            .inner
            .get_heterogeneous_graphlet_names_from_edge_node_names(src, dst))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_tendril, compute_tendril_nodes)")]
    /// Return vector of Tendrils in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_tendrils(
        &self,
        minimum_number_of_nodes_per_tendril: Option<NodeT>,
        compute_tendril_nodes: Option<bool>,
    ) -> PyResult<Vec<Tendril>> {
        Ok(pe!(self
            .inner
            .get_tendrils(minimum_number_of_nodes_per_tendril, compute_tendril_nodes))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes_above_threshold)")]
    /// Return threshold representing cutuoff point in graph node degree geometric distribution to have the given amount of elements above cutoff.
    ///
    /// Parameters
    /// ----------
    /// number_of_elements_above_threshold: int
    ///     Number of elements expected to be above cutoff threshold.
    ///
    pub fn get_node_degree_geometric_distribution_threshold(
        &self,
        number_of_nodes_above_threshold: NodeT,
    ) -> f64 {
        self.inner
            .get_node_degree_geometric_distribution_threshold(
                number_of_nodes_above_threshold.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return list of the supported sparse edge weighting methods
    pub fn get_sparse_edge_weighting_methods(&self) -> Vec<String> {
        self.inner
            .get_sparse_edge_weighting_methods()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return list of the supported edge weighting methods
    pub fn get_edge_weighting_methods(&self) -> Vec<String> {
        self.inner
            .get_edge_weighting_methods()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name, weight)")]
    /// Returns new graph with added in missing self-loops with given edge type and weight.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge type for the new selfloops is provided but the graph does not have edge types.
    /// ValueError
    ///     If the edge weight for the new selfloops is provided but the graph does not have edge weights.
    /// ValueError
    ///     If the edge weight for the new selfloops is NOT provided but the graph does have edge weights.
    ///
    pub fn add_selfloops(
        &self,
        edge_type_name: Option<&str>,
        weight: Option<WeightT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.add_selfloops(edge_type_name, weight))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of unweighted degree centrality for all nodes
    pub fn get_degree_centrality(&self) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_degree_centrality())?, f32)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of weighted degree centrality for all nodes
    pub fn get_weighted_degree_centrality(&self) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_degree_centrality())?, f32)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return closeness centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose closeness centrality is to be computed.
    /// verbose: Optional[bool]
    ///     Whether to show an indicative progress bar.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_closeness_centrality_from_node_id(&self, node_id: NodeT) -> f32 {
        self.inner
            .get_unchecked_closeness_centrality_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
    /// Return closeness centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose closeness centrality is to be computed.
    /// use_edge_weights_as_probabilities: bool
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_closeness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: bool,
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_closeness_centrality_from_node_id(
                node_id.clone(),
                use_edge_weights_as_probabilities.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return parallel iterator over closeness centrality for all nodes.
    pub fn get_closeness_centrality(&self) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_closeness_centrality(), f32)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, use_edge_weights_as_probabilities, verbose)")]
    /// Return closeness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// use_edge_weights_as_probabilities: bool
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
    ///     Whether to show an indicative progress bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have weights.
    /// ValueError
    ///     If the graph contains negative weights.
    /// ValueError
    ///     If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    pub fn get_weighted_closeness_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_closeness_centrality(
                    use_edge_weights_as_probabilities,
                    verbose
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return harmonic centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose harmonic centrality is to be computed.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_harmonic_centrality_from_node_id(&self, node_id: NodeT) -> f32 {
        self.inner
            .get_unchecked_harmonic_centrality_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
    /// Return harmonic centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose harmonic centrality is to be computed.
    /// use_edge_weights_as_probabilities: bool
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_harmonic_centrality_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: bool,
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_harmonic_centrality_from_node_id(
                node_id.clone(),
                use_edge_weights_as_probabilities.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector of harmonic centrality for all nodes.
    pub fn get_harmonic_centrality(&self) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_harmonic_centrality(), f32)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, use_edge_weights_as_probabilities, verbose)")]
    /// Return harmonic centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_weighted_harmonic_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_weighted_harmonic_centrality(use_edge_weights_as_probabilities, verbose))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns vector of stress centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while computing the stress centrality. By default, true.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn get_stress_centrality(&self, verbose: Option<bool>) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_stress_centrality(verbose))?, f32)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edges_normalization, min_max_normalization, verbose)")]
    /// Returns vector of betweenness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// edges_normalization: Optional[bool]
    ///     Whether to normalize the values by the number of edges of the complete graph. By default, false.
    /// min_max_normalization: Optional[bool]
    ///     Whether to normalize the values between 0 and 1. By default, false.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while computing the betweenness centrality. By default, true.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn get_betweenness_centrality(
        &self,
        edges_normalization: Option<bool>,
        min_max_normalization: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_betweenness_centrality(
                    edges_normalization,
                    min_max_normalization,
                    verbose
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, ant, maximum_samples_number, random_state)")]
    /// Returns the unweighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `number_of_nodes / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node ID does not exist in the current graph instance.
    ///
    pub fn get_approximated_betweenness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        ant: Option<f32>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_approximated_betweenness_centrality_from_node_id(
                node_id.clone(),
                ant,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name, ant, maximum_samples_number, random_state)")]
    /// Returns the unweighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `number_of_nodes / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node name does not exist in the current graph instance.
    ///
    pub fn get_approximated_betweenness_centrality_from_node_name(
        &self,
        node_name: &str,
        ant: Option<f32>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_approximated_betweenness_centrality_from_node_name(
                node_name,
                ant,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_id, ant, use_edge_weights_as_probabilities, maximum_samples_number, random_state)"
    )]
    /// Returns the weighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to consider the edge weights as probabilities.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `number_of_nodes / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node ID does not exist in the current graph instance.
    ///
    pub fn get_weighted_approximated_betweenness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        ant: Option<f32>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_approximated_betweenness_centrality_from_node_id(
                node_id.clone(),
                ant,
                use_edge_weights_as_probabilities,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_name, ant, use_edge_weights_as_probabilities, maximum_samples_number, random_state)"
    )]
    /// Returns the weighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to consider the edge weights as probabilities.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `number_of_nodes / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node name does not exist in the current graph instance.
    ///
    pub fn get_weighted_approximated_betweenness_centrality_from_node_name(
        &self,
        node_name: &str,
        ant: Option<f32>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_approximated_betweenness_centrality_from_node_name(
                node_name,
                ant,
                use_edge_weights_as_probabilities,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, maximum_iterations_number, tollerance)")]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------
    /// maximum_iterations_number: Optional[int]
    ///     The maximum number of iterations to consider.
    /// tollerance: Optional[float]
    ///     The maximum error tollerance for convergence.
    ///
    pub fn get_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f32>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_eigenvector_centrality(maximum_iterations_number, tollerance))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, maximum_iterations_number, tollerance)")]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------
    /// maximum_iterations_number: Optional[int]
    ///     The maximum number of iterations to consider.
    /// tollerance: Optional[float]
    ///     The maximum error tollerance for convergence.
    ///
    pub fn get_weighted_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f32>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_weighted_eigenvector_centrality(maximum_iterations_number, tollerance))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Print the current graph in a format compatible with Graphviz dot's format
    pub fn to_dot(&self) -> String {
        self.inner.to_dot().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 16, i.e. using 16 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// Parameters
    /// ----------
    /// first: int
    ///     The first node ID of the triple.
    /// second: int
    ///     The second node ID of the triple.
    /// third: int
    ///     The third node ID of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_16_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_16_tricodes_from_node_ids(first.clone(), second.clone(), third.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 13, i.e. using 13 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// Parameters
    /// ----------
    /// first: int
    ///     The first node ID of the triple.
    /// second: int
    ///     The second node ID of the triple.
    /// third: int
    ///     The third node ID of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_13_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_13_tricodes_from_node_ids(first.clone(), second.clone(), third.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 30, i.e. using 30 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// Parameters
    /// ----------
    /// first: int
    ///     The first node ID of the triple.
    /// second: int
    ///     The second node ID of the triple.
    /// third: int
    ///     The third node ID of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_30_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_30_tricodes_from_node_ids(first.clone(), second.clone(), third.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 64, i.e. using 64 possible triads, tricodes associated to the provided triple of node IDs.
    ///
    /// Parameters
    /// ----------
    /// first: int
    ///     The first node ID of the triple.
    /// second: int
    ///     The second node ID of the triple.
    /// third: int
    ///     The third node ID of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_64_tricodes_from_node_ids(
        &self,
        first: NodeT,
        second: NodeT,
        third: NodeT,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_64_tricodes_from_node_ids(first.clone(), second.clone(), third.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 16, i.e. using 16 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// Parameters
    /// ----------
    /// first: str
    ///     The first node name of the triple.
    /// second: str
    ///     The second node name of the triple.
    /// third: str
    ///     The third node name of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_16_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_16_tricodes_from_node_names(first, second, third))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 13, i.e. using 13 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// Parameters
    /// ----------
    /// first: str
    ///     The first node name of the triple.
    /// second: str
    ///     The second node name of the triple.
    /// third: str
    ///     The third node name of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_13_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_13_tricodes_from_node_names(first, second, third))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 30, i.e. using 30 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// Parameters
    /// ----------
    /// first: str
    ///     The first node name of the triple.
    /// second: str
    ///     The second node name of the triple.
    /// third: str
    ///     The third node name of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_30_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_30_tricodes_from_node_names(first, second, third))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first, second, third)")]
    /// Returns the base 64, i.e. using 64 possible triads, tricodes associated to the provided triple of node names.
    ///
    /// Parameters
    /// ----------
    /// first: str
    ///     The first node name of the triple.
    /// second: str
    ///     The second node name of the triple.
    /// third: str
    ///     The third node name of the triple.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_base_64_tricodes_from_node_names(
        &self,
        first: &str,
        second: &str,
        third: &str,
    ) -> PyResult<(u8, u8, u8)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_base_64_tricodes_from_node_names(first, second, third))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns slice with graph-wide triad census defined over 16 type of triads.
    pub fn get_base_16_triad_census(&self) -> Py<PyArray1<u64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_base_16_triad_census().to_vec(), u64)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_base_13_triad_census(&self) -> Py<PyArray1<u64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_base_13_triad_census().to_vec(), u64)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_base_30_triad_census(&self) -> Py<PyArray1<u64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_base_30_triad_census().to_vec(), u64)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_star)")]
    /// Return vector of Stars in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_stars(
        &self,
        minimum_number_of_nodes_per_star: Option<NodeT>,
    ) -> PyResult<Vec<Star>> {
        Ok(pe!(self.inner.get_stars(minimum_number_of_nodes_per_star))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, recursion_minimum_improvement, first_phase_minimum_improvement, patience, random_state)"
    )]
    /// Returns vector of vectors of communities for each layer of hierarchy minimizing undirected modularity.
    ///
    /// Parameters
    /// ----------
    /// recursion_minimum_improvement: Optional[float]
    ///     The minimum improvement to warrant another resursion round. By default, zero.
    /// first_phase_minimum_improvement: Optional[float]
    ///     The minimum improvement to warrant another first phase iteration. By default, `0.00001` (not zero because of numerical instability).
    /// patience: Optional[int]
    ///     How many iterations of the first phase to wait for before stopping. By default, `5`.
    /// random_state: Optional[int]
    ///     The random state to use to reproduce this modularity computation. By default, 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not directed.
    /// ValueError
    ///     If the `recursion_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    /// ValueError
    ///     If the `first_phase_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    ///
    pub fn get_undirected_louvain_community_detection(
        &self,
        recursion_minimum_improvement: Option<f64>,
        first_phase_minimum_improvement: Option<f64>,
        patience: Option<usize>,
        random_state: Option<u64>,
    ) -> PyResult<Vec<Vec<usize>>> {
        Ok(pe!(self.inner.get_undirected_louvain_community_detection(
            recursion_minimum_improvement,
            first_phase_minimum_improvement,
            patience,
            random_state
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_community_memberships)")]
    /// Returns the directed modularity of the graph from the given memberships.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of provided memberships does not match the number of nodes of the graph.
    ///
    pub fn get_directed_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: Vec<NodeT>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_directed_modularity_from_node_community_memberships(&node_community_memberships))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_community_memberships)")]
    /// Returns the undirected modularity of the graph from the given memberships.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of provided memberships does not match the number of nodes of the graph.
    ///
    pub fn get_undirected_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: Vec<NodeT>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_undirected_modularity_from_node_community_memberships(
                &node_community_memberships
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, maximal_hop_distance)")]
    /// Returns the structural distance from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// maximal_hop_distance: int
    ///     Maximal hop distance to consider.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_structural_distance_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        maximal_hop_distance: usize,
    ) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_unchecked_structural_distance_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                maximal_hop_distance.clone()
            ),
            f32
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the minumum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_minimum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_minimum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the maximum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_maximum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_maximum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the minumum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_minimum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_weighted_minimum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the maximum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_maximum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_weighted_maximum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
    /// Returns the unweighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> f32 {
        self.inner
            .get_unchecked_preferential_attachment_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                normalize.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
    /// Returns the unweighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_preferential_attachment_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone(),
            normalize.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name, normalize)")]
    /// Returns the unweighted preferential attachment from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    /// normalize: bool
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_preferential_attachment_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
        normalize: bool,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_preferential_attachment_from_node_names(
            first_node_name,
            second_node_name,
            normalize.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
    /// Returns the weighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_preferential_attachment_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                normalize.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
    /// Returns the weighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_weighted_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_preferential_attachment_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                normalize.clone()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name, normalize)")]
    /// Returns the weighted preferential attachment from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    /// normalize: bool
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_preferential_attachment_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
        normalize: bool,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_preferential_attachment_from_node_names(
                first_node_name,
                second_node_name,
                normalize.clone()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the Neighbours intersection size for the two given nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_neighbours_intersection_size_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.inner
            .get_unchecked_neighbours_intersection_size_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the Jaccard index for the two given nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_jaccard_coefficient_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.inner
            .get_unchecked_jaccard_coefficient_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the Jaccard index for the two given nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_jaccard_coefficient_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_jaccard_coefficient_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
    /// Returns the Jaccard index for the two given nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_jaccard_coefficient_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_jaccard_coefficient_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_adamic_adar_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.inner
            .get_unchecked_adamic_adar_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_adamic_adar_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_adamic_adar_index_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_adamic_adar_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_adamic_adar_index_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.inner
            .get_unchecked_resource_allocation_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_resource_allocation_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_resource_allocation_index_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_resource_allocation_index_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_resource_allocation_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_resource_allocation_index_from_node_names(
                first_node_name,
                second_node_name
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of currently subgraphed edge metrics
    pub fn get_number_of_available_edge_metrics(&self) -> usize {
        self.inner.get_number_of_available_edge_metrics().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns names of currently subgraphed edge metrics
    pub fn get_available_edge_metrics_names(&self) -> Vec<String> {
        self.inner
            .get_available_edge_metrics_names()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
    /// Returns all the implemented edge metrics for the two given node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node IDs do not exist in the graph this method will panic.
    pub unsafe fn get_unchecked_all_edge_metrics_from_node_ids_tuple(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_unchecked_all_edge_metrics_from_node_ids_tuple(
                    source_node_id.clone(),
                    destination_node_id.clone(),
                    normalize.clone()
                )
                .to_vec(),
            f32
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
    /// Returns all the implemented edge metrics for the two given node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node IDs do not exist in the current graph instance.
    ///
    pub fn get_all_edge_metrics_from_node_ids_tuple(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_all_edge_metrics_from_node_ids_tuple(
                    source_node_id.clone(),
                    destination_node_id.clone(),
                    normalize.clone()
                ))?
                .to_vec(),
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_ids, destination_node_ids, normalize)")]
    /// Returns all the implemented edge metrics for the vectors source and destination node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// Parameters
    /// ----------
    /// source_node_ids: List[int]
    ///     Node ID of the first node.
    /// destination_node_ids: List[int]
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node IDs do not exist in the graph this method will panic.
    pub fn get_all_edge_metrics_from_node_ids(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
        normalize: bool,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_all_edge_metrics_from_node_ids(
                    source_node_ids,
                    destination_node_ids,
                    normalize.clone()
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns Preferential Attachment for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_preferential_attachment_scores(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_preferential_attachment_scores(normalize, subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns Resource Allocation index for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_resource_allocation_index_scores(
        &self,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_resource_allocation_index_scores(subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns Jaccard Coefficient for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_jaccard_coefficient_scores(
        &self,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_jaccard_coefficient_scores(subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns Adamic-Adar for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_adamic_adar_scores(&self, subgraph: Option<&Graph>) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_adamic_adar_scores(subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns all available edge metrics for all edges, dispatching to the correct implementation based on the graph directionality.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_all_edge_metrics(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self
                    .inner
                    .get_all_edge_metrics(normalize, subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns all available edge metrics for all directed edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_all_directed_edge_metrics(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self
                    .inner
                    .get_all_directed_edge_metrics(normalize, subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns all available edge metrics for all edges in the triangular upper part of the adjacency matrix.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_all_upper_triangular_edge_metrics(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_all_upper_triangular_edge_metrics(
                    normalize,
                    subgraph.map(|sg| &sg.inner)
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns all available edge metrics for all edges in the triangular lower part of the adjacency matrix.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_all_lower_triangular_edge_metrics(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_all_lower_triangular_edge_metrics(
                    normalize,
                    subgraph.map(|sg| &sg.inner)
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups IDs
    pub fn get_isomorphic_node_type_ids_groups(&self) -> PyResult<Vec<Vec<NodeTypeT>>> {
        Ok(pe!(self.inner.get_isomorphic_node_type_ids_groups())?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups names
    pub fn get_isomorphic_node_type_names_groups(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_isomorphic_node_type_names_groups())?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of isomorphic node type groups
    pub fn get_number_of_isomorphic_node_type_groups(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_isomorphic_node_type_groups())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups IDs
    pub fn get_approximated_isomorphic_node_type_ids_groups(
        &self,
    ) -> PyResult<Vec<Vec<NodeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_approximated_isomorphic_node_type_ids_groups())?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups names
    pub fn get_approximated_isomorphic_node_type_names_groups(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self
            .inner
            .get_approximated_isomorphic_node_type_names_groups())?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of isomorphic node type groups
    pub fn get_number_of_approximated_isomorphic_node_type_groups(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self
            .inner
            .get_number_of_approximated_isomorphic_node_type_groups())?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Returns vector with isomorphic edge type groups IDs.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_edge_type_ids_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<Vec<Vec<EdgeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_isomorphic_edge_type_ids_groups(minimum_number_of_edges))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Returns vector with isomorphic edge type groups names.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_edge_type_names_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self
            .inner
            .get_isomorphic_edge_type_names_groups(minimum_number_of_edges))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Returns number of isomorphic edge type groups.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    pub fn get_number_of_isomorphic_edge_type_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<EdgeTypeT> {
        Ok(pe!(self
            .inner
            .get_number_of_isomorphic_edge_type_groups(minimum_number_of_edges))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree)")]
    /// Returns whether the current graph has topological synonims.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims.
    ///
    pub fn has_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> bool {
        self.inner.has_isomorphic_nodes(minimum_node_degree).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns whether the set of provided node IDs have isomorphic node types.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     Node IDs to check for.
    ///
    pub unsafe fn has_unchecked_isomorphic_node_types_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> bool {
        self.inner
            .has_unchecked_isomorphic_node_types_from_node_ids(&node_ids)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns whether the set of provided node IDs have isomorphic node types.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     Node IDs to check for.
    ///
    pub fn has_isomorphic_node_types_from_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .has_isomorphic_node_types_from_node_ids(&node_ids))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(directed, node_type_path, node_type_list_separator, node_types_column_number, node_types_column, node_types_ids_column_number, node_types_ids_column, number_of_node_types, numeric_node_type_ids, minimum_node_type_id, node_type_list_header, node_type_list_support_balanced_quotes, node_type_list_rows_to_skip, node_type_list_is_correct, node_type_list_max_rows_number, node_type_list_comment_symbol, load_node_type_list_in_parallel, node_path, node_list_separator, node_list_header, node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, nodes_column_number, nodes_column, node_types_separator, node_list_node_types_column_number, node_list_node_types_column, node_ids_column, node_ids_column_number, number_of_nodes, minimum_node_id, numeric_node_ids, node_list_numeric_node_type_ids, skip_node_types_if_unavailable, load_node_list_in_parallel, edge_type_path, edge_types_column_number, edge_types_column, edge_types_ids_column_number, edge_types_ids_column, number_of_edge_types, numeric_edge_type_ids, minimum_edge_type_id, edge_type_list_separator, edge_type_list_header, edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_path, edge_list_separator, edge_list_header, edge_list_support_balanced_quotes, edge_list_rows_to_skip, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_list_edge_types_column_number, edge_list_edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, edge_ids_column, edge_ids_column_number, edge_list_numeric_edge_type_ids, edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_is_complete, edge_list_may_contain_duplicates, edge_list_is_sorted, edge_list_is_correct, edge_list_max_rows_number, edge_list_comment_symbol, number_of_edges, load_edge_list_in_parallel, remove_chevrons, remove_spaces, verbose, may_have_singletons, may_have_singleton_with_selfloops, name)"
    )]
    /// Return graph renderized from given CSVs or TSVs-like files.
    ///
    /// Parameters
    /// ----------
    /// node_type_path: Optional[str]
    ///     The path to the file with the unique node type names.
    /// node_type_list_separator: Optional[str]
    ///     The separator to use for the node types file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// node_types_column_number: Optional[int]
    ///     The number of the column of the node types file from where to load the node types.
    /// node_types_column: Optional[str]
    ///     The name of the column of the node types file from where to load the node types.
    /// number_of_node_types: Optional[int]
    ///     The number of the unique node types. This will be used in order to allocate the correct size for the data structure.
    /// numeric_node_type_ids: Optional[bool]
    ///     Whether the node type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// minimum_node_type_id: Optional[int]
    ///     The minimum node type ID to be used when using numeric node type IDs.
    /// node_type_list_header: Optional[bool]
    ///     Whether the node type file has an header.
    /// node_type_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes.
    /// node_type_list_rows_to_skip: Optional[int]
    ///     The number of lines to skip in the node types file`: the header is already skipped if it has been specified that the file has an header.
    /// node_type_list_is_correct: Optional[bool]
    ///     Whether the node types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// node_type_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the node types file.
    /// node_type_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the node types file. Lines starting with this symbol will be skipped.
    /// load_node_type_list_in_parallel: Optional[bool]
    ///     Whether to load the node type list in parallel. Note that when loading in parallel, the internal order of the node type IDs may result changed across different iterations. We are working to get this to be stable.
    /// node_path: Optional[str]
    ///     The path to the file with the unique node names.
    /// node_list_separator: Optional[str]
    ///     The separator to use for the nodes file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// node_list_header: Optional[bool]
    ///     Whether the nodes file has an header.
    /// node_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes.
    /// node_list_rows_to_skip: Optional[int]
    ///     Number of rows to skip in the node list file.
    /// node_list_is_correct: Optional[bool]
    ///     Whether the nodes file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// node_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the nodes file.
    /// node_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the nodes file. Lines starting with this symbol will be skipped.
    /// default_node_type: Optional[str]
    ///     The node type to be used when the node type for a given node in the node file is None.
    /// nodes_column_number: Optional[int]
    ///     The number of the column of the node file from where to load the node names.
    /// nodes_column: Optional[str]
    ///     The name of the column of the node file from where to load the node names.
    /// node_types_separator: Optional[str]
    ///     The node types separator.
    /// node_list_node_types_column_number: Optional[int]
    ///     The number of the column of the node file from where to load the node types.
    /// node_list_node_types_column: Optional[str]
    ///     The name of the column of the node file from where to load the node types.
    /// node_ids_column: Optional[str]
    ///     The name of the column of the node file from where to load the node IDs.
    /// node_ids_column_number: Optional[int]
    ///     The number of the column of the node file from where to load the node IDs
    /// number_of_nodes: Optional[int]
    ///     The expected number of nodes. Note that this must be the EXACT number of nodes in the graph.
    /// minimum_node_id: Optional[int]
    ///     The minimum node ID to be used, when loading the node IDs as numerical.
    /// numeric_node_ids: Optional[bool]
    ///     Whether to load the numeric node IDs as numeric.
    /// node_list_numeric_node_type_ids: Optional[bool]
    ///     Whether to load the node types IDs in the node file to be numeric.
    /// skip_node_types_if_unavailable: Optional[bool]
    ///     Whether to skip the node types without raising an error if these are unavailable.
    /// load_node_list_in_parallel: Optional[bool]
    ///     Whether to load the node list in parallel. When loading in parallel, without node IDs, the nodes may not be loaded in a deterministic order.
    /// edge_type_path: Optional[str]
    ///     The path to the file with the unique edge type names.
    /// edge_types_column_number: Optional[int]
    ///     The number of the column of the edge types file from where to load the edge types.
    /// edge_types_column: Optional[str]
    ///     The name of the column of the edge types file from where to load the edge types.
    /// number_of_edge_types: Optional[int]
    ///     The number of the unique edge types. This will be used in order to allocate the correct size for the data structure.
    /// numeric_edge_type_ids: Optional[bool]
    ///     Whether the edge type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// minimum_edge_type_id: Optional[int]
    ///     The minimum edge type ID to be used when using numeric edge type IDs.
    /// edge_type_list_separator: Optional[str]
    ///     The separator to use for the edge type list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// edge_type_list_header: Optional[bool]
    ///     Whether the edge type file has an header.
    /// edge_type_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes while reading the edge type list.
    /// edge_type_list_rows_to_skip: Optional[int]
    ///     Number of rows to skip in the edge type list file.
    /// edge_type_list_is_correct: Optional[bool]
    ///     Whether the edge types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// edge_type_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the edge types file.
    /// edge_type_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the edge types file. Lines starting with this symbol will be skipped.
    /// load_edge_type_list_in_parallel: Optional[bool]
    ///     Whether to load the edge type list in parallel. When loading in parallel, without edge type IDs, the edge types may not be loaded in a deterministic order.
    /// edge_path: Optional[str]
    ///     The path to the file with the edge list.
    /// edge_list_separator: Optional[str]
    ///     The separator to use for the edge list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// edge_list_header: Optional[bool]
    ///     Whether the edges file has an header.
    /// edge_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes while reading the edge list.
    /// edge_list_rows_to_skip: Optional[int]
    ///     Number of rows to skip in the edge list file.
    /// sources_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the source nodes.
    /// sources_column: Optional[str]
    ///     The name of the column of the edges file from where to load the source nodes.
    /// destinations_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the destinaton nodes.
    /// destinations_column: Optional[str]
    ///     The name of the column of the edges file from where to load the destinaton nodes.
    /// edge_list_edge_types_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the edge types.
    /// edge_list_edge_types_column: Optional[str]
    ///     The name of the column of the edges file from where to load the edge types.
    /// default_edge_type: Optional[str]
    ///     The edge type to be used when the edge type for a given edge in the edge file is None.
    /// weights_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the edge weights.
    /// weights_column: Optional[str]
    ///     The name of the column of the edges file from where to load the edge weights.
    /// default_weight: Optional[float]
    ///     The edge weight to be used when the edge weight for a given edge in the edge file is None.
    /// edge_ids_column: Optional[str]
    ///     The name of the column of the edges file from where to load the edge IDs.
    /// edge_ids_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the edge IDs.
    /// edge_list_numeric_edge_type_ids: Optional[bool]
    ///     Whether to load the edge type IDs as numeric from the edge list.
    /// edge_list_numeric_node_ids: Optional[bool]
    ///     Whether to load the edge node IDs as numeric from the edge list.
    /// skip_weights_if_unavailable: Optional[bool]
    ///     Whether to skip the weights without raising an error if these are unavailable.
    /// skip_edge_types_if_unavailable: Optional[bool]
    ///     Whether to skip the edge types without raising an error if these are unavailable.
    /// edge_list_is_complete: Optional[bool]
    ///     Whether to consider the edge list as complete, i.e. the edges are presented in both directions when loading an undirected graph.
    /// edge_list_may_contain_duplicates: Optional[bool]
    ///     Whether the edge list may contain duplicates. If the edge list surely DOES NOT contain duplicates, a validation step may be skipped. By default, it is assumed that the edge list may contain duplicates.
    /// edge_list_is_sorted: Optional[bool]
    ///     Whether the edge list is sorted. Note that a sorted edge list has the minimal memory peak, but requires the nodes number and the edges number.
    /// edge_list_is_correct: Optional[bool]
    ///     Whether the edges file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// edge_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the edges file.
    /// edge_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the edges file. Lines starting with this symbol will be skipped.
    /// number_of_edges: Optional[int]
    ///     The expected number of edges. Note that this must be the EXACT number of edges in the graph.
    /// load_edge_list_in_parallel: Optional[bool]
    ///     Whether to load the edge list in parallel. Note that, if the edge IDs indices are not given, it is NOT possible to load a sorted edge list. Similarly, when loading in parallel, without edge IDs, the edges may not be loaded in a deterministic order.
    /// remove_chevrons: Optional[bool]
    ///     Whether remove chevrons while reading elements.
    /// remove_spaces: Optional[bool]
    ///     Whether remove spaces while reading elements.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while reading the files. Note that, if parallel loading is enabled, loading bars will not be showed because they are a synchronization bottleneck.
    /// may_have_singletons: Optional[bool]
    ///     Whether the graph may be expected to have singleton nodes. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// may_have_singleton_with_selfloops: Optional[bool]
    ///     Whether the graph may be expected to have singleton nodes with selfloops. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// directed: bool
    ///     Whether to load the graph as directed or undirected.
    /// name: Optional[str]
    ///     The name of the graph to be loaded.
    ///
    pub fn from_csv(
        directed: bool,
        node_type_path: Option<String>,
        node_type_list_separator: Option<char>,
        node_types_column_number: Option<usize>,
        node_types_column: Option<String>,
        node_types_ids_column_number: Option<usize>,
        node_types_ids_column: Option<String>,
        number_of_node_types: Option<NodeTypeT>,
        numeric_node_type_ids: Option<bool>,
        minimum_node_type_id: Option<NodeTypeT>,
        node_type_list_header: Option<bool>,
        node_type_list_support_balanced_quotes: Option<bool>,
        node_type_list_rows_to_skip: Option<usize>,
        node_type_list_is_correct: Option<bool>,
        node_type_list_max_rows_number: Option<usize>,
        node_type_list_comment_symbol: Option<String>,
        load_node_type_list_in_parallel: Option<bool>,
        node_path: Option<String>,
        node_list_separator: Option<char>,
        node_list_header: Option<bool>,
        node_list_support_balanced_quotes: Option<bool>,
        node_list_rows_to_skip: Option<usize>,
        node_list_is_correct: Option<bool>,
        node_list_max_rows_number: Option<usize>,
        node_list_comment_symbol: Option<String>,
        default_node_type: Option<String>,
        nodes_column_number: Option<usize>,
        nodes_column: Option<String>,
        node_types_separator: Option<char>,
        node_list_node_types_column_number: Option<usize>,
        node_list_node_types_column: Option<String>,
        node_ids_column: Option<String>,
        node_ids_column_number: Option<usize>,
        number_of_nodes: Option<NodeT>,
        minimum_node_id: Option<NodeT>,
        numeric_node_ids: Option<bool>,
        node_list_numeric_node_type_ids: Option<bool>,
        skip_node_types_if_unavailable: Option<bool>,
        load_node_list_in_parallel: Option<bool>,
        edge_type_path: Option<String>,
        edge_types_column_number: Option<usize>,
        edge_types_column: Option<String>,
        edge_types_ids_column_number: Option<usize>,
        edge_types_ids_column: Option<String>,
        number_of_edge_types: Option<EdgeTypeT>,
        numeric_edge_type_ids: Option<bool>,
        minimum_edge_type_id: Option<EdgeTypeT>,
        edge_type_list_separator: Option<char>,
        edge_type_list_header: Option<bool>,
        edge_type_list_support_balanced_quotes: Option<bool>,
        edge_type_list_rows_to_skip: Option<usize>,
        edge_type_list_is_correct: Option<bool>,
        edge_type_list_max_rows_number: Option<usize>,
        edge_type_list_comment_symbol: Option<String>,
        load_edge_type_list_in_parallel: Option<bool>,
        edge_path: Option<String>,
        edge_list_separator: Option<char>,
        edge_list_header: Option<bool>,
        edge_list_support_balanced_quotes: Option<bool>,
        edge_list_rows_to_skip: Option<usize>,
        sources_column_number: Option<usize>,
        sources_column: Option<String>,
        destinations_column_number: Option<usize>,
        destinations_column: Option<String>,
        edge_list_edge_types_column_number: Option<usize>,
        edge_list_edge_types_column: Option<String>,
        default_edge_type: Option<String>,
        weights_column_number: Option<usize>,
        weights_column: Option<String>,
        default_weight: Option<WeightT>,
        edge_ids_column: Option<String>,
        edge_ids_column_number: Option<usize>,
        edge_list_numeric_edge_type_ids: Option<bool>,
        edge_list_numeric_node_ids: Option<bool>,
        skip_weights_if_unavailable: Option<bool>,
        skip_edge_types_if_unavailable: Option<bool>,
        edge_list_is_complete: Option<bool>,
        edge_list_may_contain_duplicates: Option<bool>,
        edge_list_is_sorted: Option<bool>,
        edge_list_is_correct: Option<bool>,
        edge_list_max_rows_number: Option<usize>,
        edge_list_comment_symbol: Option<String>,
        number_of_edges: Option<EdgeT>,
        load_edge_list_in_parallel: Option<bool>,
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        verbose: Option<bool>,
        may_have_singletons: Option<bool>,
        may_have_singleton_with_selfloops: Option<bool>,
        name: Option<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::from_csv(
            directed.clone(),
            node_type_path,
            node_type_list_separator,
            node_types_column_number,
            node_types_column,
            node_types_ids_column_number,
            node_types_ids_column,
            number_of_node_types,
            numeric_node_type_ids,
            minimum_node_type_id,
            node_type_list_header,
            node_type_list_support_balanced_quotes,
            node_type_list_rows_to_skip,
            node_type_list_is_correct,
            node_type_list_max_rows_number,
            node_type_list_comment_symbol,
            load_node_type_list_in_parallel,
            node_path,
            node_list_separator,
            node_list_header,
            node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol,
            default_node_type,
            nodes_column_number,
            nodes_column,
            node_types_separator,
            node_list_node_types_column_number,
            node_list_node_types_column,
            node_ids_column,
            node_ids_column_number,
            number_of_nodes,
            minimum_node_id,
            numeric_node_ids,
            node_list_numeric_node_type_ids,
            skip_node_types_if_unavailable,
            load_node_list_in_parallel,
            edge_type_path,
            edge_types_column_number,
            edge_types_column,
            edge_types_ids_column_number,
            edge_types_ids_column,
            number_of_edge_types,
            numeric_edge_type_ids,
            minimum_edge_type_id,
            edge_type_list_separator,
            edge_type_list_header,
            edge_type_list_support_balanced_quotes,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            edge_path,
            edge_list_separator,
            edge_list_header,
            edge_list_support_balanced_quotes,
            edge_list_rows_to_skip,
            sources_column_number,
            sources_column,
            destinations_column_number,
            destinations_column,
            edge_list_edge_types_column_number,
            edge_list_edge_types_column,
            default_edge_type,
            weights_column_number,
            weights_column,
            default_weight,
            edge_ids_column,
            edge_ids_column_number,
            edge_list_numeric_edge_type_ids,
            edge_list_numeric_node_ids,
            skip_weights_if_unavailable,
            skip_edge_types_if_unavailable,
            edge_list_is_complete,
            edge_list_may_contain_duplicates,
            edge_list_is_sorted,
            edge_list_is_correct,
            edge_list_max_rows_number,
            edge_list_comment_symbol,
            number_of_edges,
            load_edge_list_in_parallel,
            remove_chevrons,
            remove_spaces,
            verbose,
            may_have_singletons,
            may_have_singleton_with_selfloops,
            name
        ))?
        .into())
    }
}

pub const GRAPH_METHODS_NAMES: &[&str] = &[
    "is_unchecked_connected_from_node_id",
    "is_connected_from_node_id",
    "is_unchecked_disconnected_node_from_node_id",
    "is_unchecked_singleton_from_node_id",
    "is_singleton_from_node_id",
    "is_unchecked_singleton_with_selfloops_from_node_id",
    "is_singleton_with_selfloops_from_node_id",
    "is_unchecked_singleton_from_node_name",
    "is_singleton_from_node_name",
    "has_node_name",
    "has_node_type_id",
    "has_node_type_name",
    "has_edge_type_id",
    "has_edge_type_name",
    "has_edge_from_node_ids",
    "has_selfloop_from_node_id",
    "has_edge_from_node_ids_and_edge_type_id",
    "is_unchecked_trap_node_from_node_id",
    "is_trap_node_from_node_id",
    "is_unchecked_trap_node_with_selfloops_from_node_id",
    "is_trap_node_with_selfloops_from_node_id",
    "are_unchecked_isomorphic_from_node_ids",
    "are_isomorphic_from_node_ids",
    "are_isomorphic_from_node_names",
    "has_node_name_and_node_type_name",
    "has_edge_from_node_names",
    "has_edge_from_node_names_and_edge_type_name",
    "has_edge_from_node_id_and_edge_type_id",
    "has_unchecked_edge_from_node_id_and_edge_type_id",
    "strongly_connected_components",
    "get_jaccard_coo_matrix",
    "get_jaccard_graph",
    "get_neighbours_intersection_size_coo_matrix",
    "get_neighbours_intersection_size_graph",
    "get_shared_ancestors_size_coo_matrix",
    "get_shared_ancestors_size_graph",
    "get_ancestors_jaccard_coo_matrix",
    "get_ancestors_jaccard_graph",
    "get_adamic_adar_coo_matrix",
    "get_adamic_adar_graph",
    "get_laplacian_coo_matrix",
    "get_laplacian_graph",
    "get_left_normalized_laplacian_coo_matrix",
    "get_left_normalized_laplacian_graph",
    "get_right_normalized_laplacian_coo_matrix",
    "get_right_normalized_laplacian_graph",
    "get_symmetric_normalized_laplacian_coo_matrix",
    "get_symmetric_normalized_laplacian_graph",
    "sort_by_increasing_outbound_node_degree",
    "sort_by_decreasing_outbound_node_degree",
    "sort_by_node_lexicographic_order",
    "get_bfs_topological_sorting_from_node_id",
    "get_reversed_bfs_topological_sorting_from_node_id",
    "sort_by_bfs_topological_sorting_from_node_id",
    "get_predictions_histograms",
    "get_filtered_predictions",
    "remove_components",
    "overlaps",
    "contains",
    "get_bipartite_edges",
    "get_bipartite_edge_names",
    "get_star_edges",
    "get_star_edge_names",
    "get_clique_edges",
    "get_clique_edge_names",
    "validate_node_id",
    "validate_node_ids",
    "validate_edge_id",
    "validate_edge_ids",
    "must_not_contain_unknown_node_types",
    "must_not_contain_unknown_edge_types",
    "validate_node_type_id",
    "validate_node_type_ids",
    "validate_edge_type_id",
    "validate_edge_type_ids",
    "must_have_node_ontologies",
    "must_be_undirected",
    "must_be_directed_acyclic",
    "must_not_have_trap_nodes",
    "must_be_multigraph",
    "must_not_be_multigraph",
    "must_contain_identity_matrix",
    "must_not_contain_weighted_singleton_nodes",
    "must_have_edges",
    "must_have_nodes",
    "must_be_connected",
    "must_share_node_vocabulary",
    "get_total_edge_weights",
    "get_mininum_edge_weight",
    "get_maximum_edge_weight",
    "get_unchecked_maximum_node_degree",
    "get_unchecked_minimum_node_degree",
    "get_weighted_maximum_node_degree",
    "get_weighted_minimum_node_degree",
    "get_number_of_weighted_singleton_nodes",
    "get_number_of_selfloops",
    "get_number_of_unique_selfloops",
    "generate_new_edges_from_node_features",
    "set_name",
    "set_inplace_all_edge_types",
    "set_all_edge_types",
    "set_inplace_all_node_types",
    "set_all_node_types",
    "remove_inplace_node_type_ids",
    "remove_inplace_singleton_node_types",
    "add_node_type_id_from_node_name_prefixes_inplace",
    "replace_edge_type_id_from_edge_node_type_ids_inplace",
    "replace_edge_type_id_from_edge_node_type_ids",
    "add_node_type_id_from_node_name_prefixes",
    "add_node_type_name_inplace",
    "add_node_type_name_from_node_name_prefixes_inplace",
    "add_edge_type_name_inplace",
    "replace_edge_type_name_from_edge_node_type_names_inplace",
    "replace_edge_type_name_from_edge_node_type_names",
    "add_node_type_name_from_node_name_prefixes",
    "remove_inplace_homogeneous_node_types",
    "remove_inplace_edge_type_ids",
    "remove_inplace_singleton_edge_types",
    "remove_inplace_node_type_names",
    "remove_inplace_node_type_name",
    "remove_node_type_id",
    "remove_singleton_node_types",
    "remove_homogeneous_node_types",
    "remove_inplace_isomorphic_node_types",
    "remove_isomorphic_node_types",
    "remove_inplace_isomorphic_edge_types",
    "remove_isomorphic_edge_types",
    "remove_node_type_names",
    "remove_node_type_name",
    "remove_inplace_edge_type_name",
    "remove_edge_type_id",
    "remove_singleton_edge_types",
    "remove_edge_type_name",
    "remove_inplace_node_types",
    "remove_node_types",
    "remove_inplace_edge_types",
    "remove_edge_types",
    "remove_inplace_edge_weights",
    "remove_edge_weights",
    "divide_edge_weights_inplace",
    "divide_edge_weights",
    "normalize_edge_weights_inplace",
    "normalize_edge_weights",
    "multiply_edge_weights_inplace",
    "multiply_edge_weights",
    "build_bipartite_graph_from_edge_node_ids",
    "build_clique_graph_from_node_ids",
    "build_bipartite_graph_from_edge_node_names",
    "build_clique_graph_from_node_names",
    "build_bipartite_graph_from_edge_node_prefixes",
    "build_clique_graph_from_node_prefixes",
    "build_bipartite_graph_from_edge_node_types",
    "build_clique_graph_from_node_type_names",
    "get_circles",
    "get_memory_stats",
    "get_total_memory_used",
    "get_nodes_total_memory_requirement",
    "get_nodes_total_memory_requirement_human_readable",
    "get_edges_total_memory_requirement",
    "get_edges_total_memory_requirement_human_readable",
    "get_edge_weights_total_memory_requirements",
    "get_edge_weights_total_memory_requirements_human_readable",
    "get_node_types_total_memory_requirements",
    "get_node_types_total_memory_requirements_human_readable",
    "get_edge_types_total_memory_requirements",
    "get_edge_types_total_memory_requirements_human_readable",
    "get_number_of_triangles",
    "get_number_of_squares",
    "get_number_of_squares_per_node",
    "get_number_of_triads",
    "get_number_of_weighted_triads",
    "get_transitivity",
    "get_number_of_triangles_per_node",
    "get_clustering_coefficient_per_node",
    "get_clustering_coefficient",
    "get_average_clustering_coefficient",
    "are_nodes_remappable",
    "remap_unchecked_from_node_ids",
    "remap_from_node_ids",
    "remap_from_node_names",
    "remap_from_node_names_map",
    "remap_from_graph",
    "sample_negative_graph",
    "sample_positive_graph",
    "connected_holdout",
    "random_holdout",
    "get_node_label_holdout_indices",
    "get_node_label_holdout_labels",
    "get_node_label_holdout_graphs",
    "get_edge_label_holdout_graphs",
    "get_random_subgraph",
    "get_node_label_random_holdout",
    "get_node_label_kfold",
    "get_edge_label_kfold",
    "get_edge_prediction_kfold",
    "get_node_tuples",
    "get_chains",
    "get_unchecked_breadth_first_search_predecessors_parallel_from_node_id",
    "get_unchecked_breadth_first_search_distances_parallel_from_node_ids",
    "get_unchecked_breadth_first_search_distances_parallel_from_node_id",
    "get_unchecked_breadth_first_search_distances_sequential_from_node_id",
    "get_unchecked_breadth_first_search_from_node_ids",
    "get_unchecked_breadth_first_search_from_node_id",
    "get_unchecked_shortest_path_node_ids_from_node_ids",
    "get_unchecked_shortest_path_node_names_from_node_ids",
    "get_shortest_path_node_ids_from_node_ids",
    "get_shortest_path_node_ids_from_node_names",
    "get_shortest_path_node_names_from_node_names",
    "get_unchecked_k_shortest_path_node_ids_from_node_ids",
    "get_k_shortest_path_node_ids_from_node_ids",
    "get_k_shortest_path_node_ids_from_node_names",
    "get_k_shortest_path_node_names_from_node_names",
    "get_unchecked_eccentricity_and_most_distant_node_id_from_node_id",
    "get_unchecked_weighted_eccentricity_from_node_id",
    "get_eccentricity_and_most_distant_node_id_from_node_id",
    "get_weighted_eccentricity_from_node_id",
    "get_eccentricity_from_node_name",
    "get_weighted_eccentricity_from_node_name",
    "get_unchecked_dijkstra_from_node_ids",
    "get_unchecked_dijkstra_from_node_id",
    "get_unchecked_weighted_shortest_path_node_ids_from_node_ids",
    "get_unchecked_weighted_shortest_path_node_names_from_node_ids",
    "get_weighted_shortest_path_node_ids_from_node_ids",
    "get_weighted_shortest_path_node_ids_from_node_names",
    "get_weighted_shortest_path_node_names_from_node_names",
    "get_breadth_first_search_from_node_ids",
    "get_dijkstra_from_node_ids",
    "get_four_sweep",
    "get_diameter_naive",
    "get_diameter",
    "get_breadth_first_search_from_node_names",
    "get_dijkstra_from_node_names",
    "get_distances_histogram_between_node_type_ids",
    "get_distances_histogram_between_node_type_names",
    "get_distances_histogram_between_prefixes",
    "get_number_of_connected_components",
    "get_number_of_connected_nodes",
    "get_number_of_singleton_nodes_with_selfloops",
    "get_number_of_singleton_nodes",
    "get_number_of_disconnected_nodes",
    "get_singleton_node_ids",
    "get_singleton_node_names",
    "get_singleton_with_selfloops_node_ids",
    "get_singleton_with_selfloops_node_names",
    "get_density",
    "get_trap_nodes_rate",
    "get_trap_node_ids",
    "get_node_degrees_mean",
    "get_weighted_node_degrees_mean",
    "get_number_of_undirected_edges",
    "get_number_of_unique_undirected_edges",
    "get_number_of_edges",
    "get_number_of_unique_edges",
    "get_node_degrees_median",
    "get_weighted_node_degrees_median",
    "get_maximum_node_degree",
    "get_unchecked_most_central_node_id",
    "get_most_central_node_id",
    "get_minimum_node_degree",
    "get_node_degrees_mode",
    "get_selfloop_nodes_rate",
    "get_name",
    "get_number_of_trap_nodes",
    "get_number_of_trap_nodes_with_selfloops",
    "get_source_node_ids",
    "get_directed_source_node_ids",
    "get_source_names",
    "get_destination_node_ids",
    "get_directed_destination_node_ids",
    "get_destination_names",
    "get_node_names",
    "get_node_urls",
    "get_node_ontologies",
    "get_unchecked_ontology_from_node_name",
    "get_unchecked_ontology_from_node_id",
    "get_ontology_from_node_name",
    "get_ontology_from_node_id",
    "get_node_ids",
    "get_directed_edge_type_ids",
    "get_upper_triangular_edge_type_ids",
    "get_lower_triangular_edge_type_ids",
    "get_imputed_directed_edge_type_ids",
    "get_imputed_upper_triangular_edge_type_ids",
    "get_imputed_lower_triangular_edge_type_ids",
    "get_directed_known_edge_type_ids",
    "get_upper_triangular_known_edge_type_ids",
    "get_lower_triangular_known_edge_type_ids",
    "get_directed_source_nodes_with_known_edge_types",
    "get_directed_destination_nodes_with_known_edge_types",
    "get_unique_edge_type_ids",
    "get_directed_edge_type_names",
    "get_upper_triangular_edge_type_names",
    "get_lower_triangular_edge_type_names",
    "get_unique_edge_type_names",
    "get_directed_edge_weights",
    "get_undirected_edge_weights",
    "get_weighted_node_indegrees",
    "get_node_type_ids",
    "get_known_node_types_mask",
    "get_unknown_node_types_mask",
    "get_known_edge_types_mask",
    "get_unknown_edge_types_mask",
    "get_one_hot_encoded_node_types",
    "get_one_hot_encoded_known_node_types",
    "get_one_hot_encoded_edge_types",
    "get_one_hot_encoded_known_edge_types",
    "get_node_type_names",
    "get_unique_node_type_ids",
    "get_unique_node_type_names",
    "get_number_of_unique_directed_edges",
    "get_nodes_mapping",
    "get_edge_node_ids",
    "get_directed_edge_node_ids",
    "get_directed_edge_triples_ids",
    "get_edge_node_names",
    "get_directed_edge_node_names",
    "get_directed_edge_triples_names",
    "get_number_of_unknown_node_types",
    "get_number_of_known_node_types",
    "get_unknown_node_types_rate",
    "get_known_node_types_rate",
    "get_minimum_number_of_node_types",
    "get_maximum_number_of_node_types",
    "get_maximum_multilabel_count",
    "get_number_of_singleton_node_types",
    "get_number_of_homogeneous_node_types",
    "get_homogeneous_node_type_ids",
    "get_homogeneous_node_type_names",
    "get_singleton_node_type_ids",
    "get_singleton_node_type_names",
    "get_number_of_unknown_edge_types",
    "get_directed_edge_ids_with_unknown_edge_types",
    "get_upper_triangular_edge_ids_with_unknown_edge_types",
    "get_lower_triangular_edge_ids_with_unknown_edge_types",
    "get_directed_edge_ids_with_known_edge_types",
    "get_upper_triangular_edge_ids_with_known_edge_types",
    "get_lower_triangular_edge_ids_with_known_edge_types",
    "get_edge_node_ids_with_known_edge_types",
    "get_edge_node_names_with_known_edge_types",
    "get_edges_with_unknown_edge_types_mask",
    "get_directed_edges_with_known_edge_types_mask",
    "get_upper_triangular_known_edge_types_mask",
    "get_lower_triangular_known_edge_types_mask",
    "get_node_ids_with_unknown_node_types",
    "get_node_ids_with_known_node_types",
    "get_node_names_with_unknown_node_types",
    "get_node_ids_from_node_type_id",
    "get_node_ids_from_node_type_ids",
    "get_node_ids_from_node_type_names",
    "get_node_names_from_node_type_id",
    "get_node_ids_from_node_type_name",
    "get_node_names_from_node_type_name",
    "get_node_names_with_known_node_types",
    "get_nodes_with_unknown_node_types_mask",
    "get_nodes_with_known_node_types_mask",
    "get_number_of_known_edge_types",
    "get_unknown_edge_types_rate",
    "get_known_edge_types_rate",
    "get_minimum_number_of_edge_types",
    "get_maximum_number_of_edge_types",
    "get_number_of_singleton_edge_types",
    "get_singleton_edge_type_ids",
    "get_singleton_edge_type_names",
    "get_number_of_nodes",
    "get_node_connected_component_ids",
    "get_number_of_directed_edges",
    "get_number_of_edge_types",
    "get_number_of_node_types",
    "get_node_degrees",
    "get_node_indegrees",
    "get_weighted_node_degrees",
    "get_not_singletons_node_ids",
    "get_dense_nodes_mapping",
    "get_number_of_parallel_edges",
    "get_cumulative_node_degrees",
    "get_reciprocal_sqrt_degrees",
    "get_number_of_unique_source_nodes",
    "get_edge_type_id_counts_hashmap",
    "get_edge_type_names_counts_hashmap",
    "get_node_type_id_counts_hashmap",
    "get_node_type_names_counts_hashmap",
    "get_single_label_node_type_ids",
    "get_known_single_label_node_type_ids",
    "get_boolean_node_type_ids",
    "get_known_boolean_node_type_ids",
    "get_root_node_ids",
    "get_root_node_names",
    "to_directed_inplace",
    "to_directed",
    "to_upper_triangular",
    "to_lower_triangular",
    "to_main_diagonal",
    "to_anti_diagonal",
    "to_bidiagonal",
    "to_arrowhead",
    "to_transposed",
    "to_undirected",
    "to_complementary",
    "to_structural_similarity_multi_graph",
    "get_approximated_cliques",
    "get_max_clique",
    "get_approximated_number_of_cliques",
    "report",
    "overlap_textual_report",
    "get_node_report_from_node_id",
    "get_node_report_from_node_name",
    "textual_report",
    "get_isomorphic_nodes_mask",
    "get_isomorphic_node_ids",
    "get_flat_repeated_isomorphic_node_ids",
    "get_isomorphic_node_hashes",
    "get_isomorphic_node_names",
    "get_isomorphic_edge_node_ids",
    "get_isomorphic_edge_hashes",
    "get_isomorphic_edge_node_names",
    "get_isomorphic_tuple_node_ids",
    "get_isomorphic_tuple_hashes",
    "get_isomorphic_tuple_node_names",
    "generate_random_connected_graph",
    "generate_random_spanning_tree",
    "generate_star_graph",
    "generate_wheel_graph",
    "generate_circle_graph",
    "generate_chain_graph",
    "generate_complete_graph",
    "generate_barbell_graph",
    "generate_lollipop_graph",
    "generate_squared_lattice_graph",
    "filter_from_ids",
    "filter_from_names",
    "remove_unknown_node_types",
    "remove_unknown_edge_types",
    "remove_singleton_nodes",
    "remove_tendrils",
    "remove_dendritic_trees",
    "remove_isomorphic_nodes",
    "remove_singleton_nodes_with_selfloops",
    "remove_disconnected_nodes",
    "remove_selfloops",
    "remove_parallel_edges",
    "random_spanning_arborescence_kruskal",
    "spanning_arborescence_kruskal",
    "get_random_spanning_tree",
    "get_connected_components",
    "enable",
    "disable_all",
    "get_approximated_total_distances",
    "get_approximated_closeness_centrality",
    "get_approximated_harmonic_centrality",
    "get_approximated_diameter",
    "has_compatible_node_vocabularies",
    "has_compatible_node_types_vocabularies",
    "has_compatible_edge_types_vocabularies",
    "is_compatible",
    "has_same_adjacency_matrix",
    "get_vertex_cover",
    "get_random_node_type",
    "get_random_edge_type",
    "get_unchecked_random_scale_free_edge_type",
    "get_random_scale_free_edge_type",
    "get_unchecked_random_scale_free_edge_weight",
    "get_random_scale_free_edge_weight",
    "get_random_node",
    "get_random_edge_id",
    "get_random_outbounds_scale_free_node",
    "get_random_inbounds_scale_free_node",
    "get_sorted_unique_random_nodes",
    "get_breadth_first_search_random_nodes",
    "get_uniform_random_walk_random_nodes",
    "get_node_sampling_methods",
    "get_subsampled_nodes",
    "get_okapi_bm25_node_feature_propagation",
    "get_okapi_bm25_node_label_propagation",
    "has_default_graph_name",
    "has_nodes",
    "has_edges",
    "has_trap_nodes",
    "has_trap_nodes_with_selfloops",
    "is_directed",
    "is_directed_acyclic",
    "has_edge_weights",
    "has_edge_weights_representing_probabilities",
    "has_weighted_singleton_nodes",
    "has_constant_edge_weights",
    "has_negative_edge_weights",
    "has_edge_types",
    "has_selfloops",
    "has_disconnected_nodes",
    "has_singleton_nodes",
    "has_singleton_nodes_with_selfloops",
    "is_connected",
    "has_node_types",
    "has_multilabel_node_types",
    "has_unknown_node_types",
    "has_known_node_types",
    "has_unknown_edge_types",
    "has_known_edge_types",
    "has_homogeneous_node_types",
    "has_exclusively_homogeneous_node_types",
    "has_homogeneous_node_ontologies",
    "has_homogeneous_edge_types",
    "has_exclusively_homogeneous_edge_types",
    "has_singleton_node_types",
    "has_exclusively_singleton_node_types",
    "has_node_oddities",
    "has_node_types_oddities",
    "has_singleton_edge_types",
    "has_exclusively_singleton_edge_types",
    "has_edge_types_oddities",
    "is_multigraph",
    "has_node_ontologies",
    "has_unknown_node_ontologies",
    "has_nodes_sorted_by_decreasing_outbound_node_degree",
    "has_nodes_sorted_by_lexicographic_order",
    "contains_identity_matrix",
    "has_nodes_sorted_by_increasing_outbound_node_degree",
    "has_sources_tradeoff_enabled",
    "has_reciprocal_sqrt_degrees_tradeoff_enabled",
    "get_dendritic_trees",
    "get_transitive_closure",
    "get_all_shortest_paths",
    "get_weighted_all_shortest_paths",
    "get_unchecked_edge_weight_from_edge_id",
    "get_unchecked_edge_weight_from_node_ids",
    "get_unchecked_node_id_from_node_name",
    "get_unchecked_edge_type_id_from_edge_type_name",
    "get_unchecked_edge_type_name_from_edge_type_id",
    "get_unchecked_edge_count_from_edge_type_id",
    "get_unchecked_node_count_from_node_type_id",
    "get_unchecked_edge_id_from_node_ids_and_edge_type_id",
    "get_unchecked_minmax_edge_ids_from_node_ids",
    "get_unchecked_node_ids_from_edge_id",
    "get_unchecked_node_names_from_edge_id",
    "get_unchecked_source_node_id_from_edge_id",
    "get_unchecked_destination_node_id_from_edge_id",
    "get_source_node_id_from_edge_id",
    "get_destination_node_id_from_edge_id",
    "get_unchecked_number_of_selfloops_from_node_id",
    "get_number_of_selfloops_from_node_id",
    "get_number_of_selfloops_from_node_name",
    "get_unchecked_source_node_name_from_edge_id",
    "get_unchecked_destination_node_name_from_edge_id",
    "get_source_node_name_from_edge_id",
    "get_destination_node_name_from_edge_id",
    "get_node_names_from_edge_id",
    "get_node_ids_from_edge_id",
    "get_unchecked_edge_id_from_node_ids",
    "get_edge_id_from_node_ids",
    "get_unchecked_unique_source_node_id",
    "get_unchecked_node_ids_and_edge_type_id_from_edge_id",
    "get_node_ids_and_edge_type_id_from_edge_id",
    "get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id",
    "get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id",
    "get_top_k_central_node_ids",
    "get_weighted_top_k_central_node_ids",
    "get_unchecked_node_degree_from_node_id",
    "get_unchecked_selfloop_excluded_node_degree_from_node_id",
    "get_selfloop_adjusted_node_degree_from_node_id",
    "get_selfloop_adjusted_node_degree_from_node_name",
    "get_unchecked_weighted_node_degree_from_node_id",
    "get_node_degree_from_node_id",
    "get_unchecked_comulative_node_degree_from_node_id",
    "get_comulative_node_degree_from_node_id",
    "get_unchecked_reciprocal_sqrt_degree_from_node_id",
    "get_reciprocal_sqrt_degree_from_node_id",
    "get_unchecked_reciprocal_sqrt_degrees_from_node_ids",
    "get_weighted_node_degree_from_node_id",
    "get_node_degree_from_node_name",
    "get_top_k_central_node_names",
    "get_unchecked_node_type_ids_from_node_id",
    "get_node_type_ids_from_node_id",
    "get_unchecked_edge_type_id_from_edge_id",
    "get_unchecked_edge_type_name_from_edge_id",
    "get_edge_type_id_from_edge_id",
    "get_edge_type_id_from_edge_node_ids",
    "get_unchecked_node_type_names_from_node_id",
    "get_node_type_names_from_node_id",
    "get_node_type_names_from_node_name",
    "get_edge_type_name_from_edge_id",
    "get_edge_type_name_from_edge_type_id",
    "get_edge_weight_from_edge_id",
    "get_edge_weight_from_node_ids",
    "get_edge_weight_from_node_ids_and_edge_type_id",
    "get_edge_weight_from_node_names_and_edge_type_name",
    "get_edge_weight_from_node_names",
    "get_unchecked_node_name_from_node_id",
    "get_node_name_from_node_id",
    "get_node_id_from_node_name",
    "get_node_ids_from_node_names",
    "get_node_names_from_node_ids",
    "get_edge_node_ids_from_edge_node_names",
    "get_edge_node_names_from_edge_node_ids",
    "get_node_type_ids_from_node_name",
    "get_node_type_name_from_node_name",
    "get_edge_count_from_edge_type_id",
    "get_edge_type_id_from_edge_type_name",
    "get_edge_count_from_edge_type_name",
    "get_node_type_id_from_node_type_name",
    "get_node_count_from_node_type_id",
    "get_node_count_from_node_type_name",
    "get_neighbour_node_ids_from_node_id",
    "get_neighbour_node_ids_from_node_name",
    "get_neighbour_node_names_from_node_name",
    "get_minmax_edge_ids_from_node_ids",
    "get_edge_id_from_node_ids_and_edge_type_id",
    "get_edge_id_from_node_names",
    "get_edge_id_from_node_names_and_edge_type_name",
    "get_edge_type_ids_from_edge_type_names",
    "get_node_type_ids_from_node_type_names",
    "get_multiple_node_type_ids_from_node_type_names",
    "get_unchecked_minmax_edge_ids_from_source_node_id",
    "get_minmax_edge_ids_from_source_node_id",
    "get_node_type_name_from_node_type_id",
    "get_unchecked_node_type_names_from_node_type_ids",
    "get_unchecked_number_of_nodes_from_node_type_id",
    "get_number_of_nodes_from_node_type_id",
    "get_number_of_nodes_from_node_type_name",
    "get_unchecked_number_of_edges_from_edge_type_id",
    "get_number_of_edges_from_edge_type_id",
    "get_number_of_edges_from_edge_type_name",
    "get_unchecked_node_type_id_counts_hashmap_from_node_ids",
    "get_unchecked_edge_type_id_counts_hashmap_from_node_ids",
    "get_edge_node_ids_from_edge_type_id",
    "get_directed_edge_node_ids_from_edge_type_id",
    "get_directed_edge_node_names_from_edge_type_id",
    "get_directed_edge_node_names_from_edge_type_name",
    "get_directed_edge_ids_from_edge_type_id",
    "get_edge_node_ids_from_edge_type_name",
    "get_directed_edge_node_ids_from_edge_type_name",
    "get_directed_edge_ids_from_edge_type_name",
    "get_directed_edge_node_names_from_node_curie_prefixes",
    "get_directed_edge_node_ids_from_node_curie_prefixes",
    "get_directed_edge_ids_from_node_curie_prefixes",
    "get_number_of_directed_edges_from_node_curie_prefixes",
    "get_node_ids_from_node_curie_prefixes",
    "get_node_names_from_node_curie_prefixes",
    "get_number_of_nodes_from_node_curie_prefixes",
    "get_node_names_prefixes",
    "get_node_ids_mapping_from_graph",
    "get_non_zero_subgraph_node_degrees",
    "get_multigraph_edge_ids_from_node_ids",
    "get_number_of_multigraph_edges_from_node_ids",
    "get_ancestors_jaccard_from_node_ids",
    "get_ancestors_jaccard_from_node_names",
    "get_heterogeneous_graphlet_ids_from_edge_node_ids",
    "get_heterogeneous_graphlet_names_from_edge_node_ids",
    "get_heterogeneous_graphlet_names_from_edge_node_names",
    "get_tendrils",
    "get_node_degree_geometric_distribution_threshold",
    "get_sparse_edge_weighting_methods",
    "get_edge_weighting_methods",
    "add_selfloops",
    "get_degree_centrality",
    "get_weighted_degree_centrality",
    "get_unchecked_closeness_centrality_from_node_id",
    "get_unchecked_weighted_closeness_centrality_from_node_id",
    "get_closeness_centrality",
    "get_weighted_closeness_centrality",
    "get_unchecked_harmonic_centrality_from_node_id",
    "get_unchecked_weighted_harmonic_centrality_from_node_id",
    "get_harmonic_centrality",
    "get_weighted_harmonic_centrality",
    "get_stress_centrality",
    "get_betweenness_centrality",
    "get_approximated_betweenness_centrality_from_node_id",
    "get_approximated_betweenness_centrality_from_node_name",
    "get_weighted_approximated_betweenness_centrality_from_node_id",
    "get_weighted_approximated_betweenness_centrality_from_node_name",
    "get_eigenvector_centrality",
    "get_weighted_eigenvector_centrality",
    "to_dot",
    "get_base_16_tricodes_from_node_ids",
    "get_base_13_tricodes_from_node_ids",
    "get_base_30_tricodes_from_node_ids",
    "get_base_64_tricodes_from_node_ids",
    "get_base_16_tricodes_from_node_names",
    "get_base_13_tricodes_from_node_names",
    "get_base_30_tricodes_from_node_names",
    "get_base_64_tricodes_from_node_names",
    "get_base_16_triad_census",
    "get_base_13_triad_census",
    "get_base_30_triad_census",
    "get_stars",
    "get_undirected_louvain_community_detection",
    "get_directed_modularity_from_node_community_memberships",
    "get_undirected_modularity_from_node_community_memberships",
    "get_unchecked_structural_distance_from_node_ids",
    "get_unchecked_minimum_preferential_attachment",
    "get_unchecked_maximum_preferential_attachment",
    "get_unchecked_weighted_minimum_preferential_attachment",
    "get_unchecked_weighted_maximum_preferential_attachment",
    "get_unchecked_preferential_attachment_from_node_ids",
    "get_preferential_attachment_from_node_ids",
    "get_preferential_attachment_from_node_names",
    "get_unchecked_weighted_preferential_attachment_from_node_ids",
    "get_weighted_preferential_attachment_from_node_ids",
    "get_weighted_preferential_attachment_from_node_names",
    "get_unchecked_neighbours_intersection_size_from_node_ids",
    "get_unchecked_jaccard_coefficient_from_node_ids",
    "get_jaccard_coefficient_from_node_ids",
    "get_jaccard_coefficient_from_node_names",
    "get_unchecked_adamic_adar_index_from_node_ids",
    "get_adamic_adar_index_from_node_ids",
    "get_adamic_adar_index_from_node_names",
    "get_unchecked_resource_allocation_index_from_node_ids",
    "get_unchecked_weighted_resource_allocation_index_from_node_ids",
    "get_resource_allocation_index_from_node_ids",
    "get_resource_allocation_index_from_node_names",
    "get_weighted_resource_allocation_index_from_node_ids",
    "get_weighted_resource_allocation_index_from_node_names",
    "get_number_of_available_edge_metrics",
    "get_available_edge_metrics_names",
    "get_unchecked_all_edge_metrics_from_node_ids_tuple",
    "get_all_edge_metrics_from_node_ids_tuple",
    "get_all_edge_metrics_from_node_ids",
    "get_preferential_attachment_scores",
    "get_resource_allocation_index_scores",
    "get_jaccard_coefficient_scores",
    "get_adamic_adar_scores",
    "get_all_edge_metrics",
    "get_all_directed_edge_metrics",
    "get_all_upper_triangular_edge_metrics",
    "get_all_lower_triangular_edge_metrics",
    "get_isomorphic_node_type_ids_groups",
    "get_isomorphic_node_type_names_groups",
    "get_number_of_isomorphic_node_type_groups",
    "get_approximated_isomorphic_node_type_ids_groups",
    "get_approximated_isomorphic_node_type_names_groups",
    "get_number_of_approximated_isomorphic_node_type_groups",
    "get_isomorphic_edge_type_ids_groups",
    "get_isomorphic_edge_type_names_groups",
    "get_number_of_isomorphic_edge_type_groups",
    "has_isomorphic_nodes",
    "has_unchecked_isomorphic_node_types_from_node_ids",
    "has_isomorphic_node_types_from_node_ids",
    "from_csv",
];

pub const GRAPH_TERMS: &[&str] = &[
    "is",
    "unchecked",
    "connected",
    "from",
    "node",
    "id",
    "disconnected",
    "singleton",
    "with",
    "selfloops",
    "name",
    "has",
    "type",
    "edge",
    "ids",
    "selfloop",
    "and",
    "trap",
    "are",
    "isomorphic",
    "names",
    "strongly",
    "components",
    "get",
    "jaccard",
    "coo",
    "matrix",
    "graph",
    "neighbours",
    "intersection",
    "size",
    "shared",
    "ancestors",
    "adamic",
    "adar",
    "laplacian",
    "left",
    "normalized",
    "right",
    "symmetric",
    "sort",
    "by",
    "increasing",
    "outbound",
    "degree",
    "decreasing",
    "lexicographic",
    "order",
    "bfs",
    "topological",
    "sorting",
    "reversed",
    "predictions",
    "histograms",
    "filtered",
    "remove",
    "overlaps",
    "contains",
    "bipartite",
    "edges",
    "star",
    "clique",
    "validate",
    "must",
    "not",
    "contain",
    "unknown",
    "types",
    "have",
    "ontologies",
    "be",
    "undirected",
    "directed",
    "acyclic",
    "nodes",
    "multigraph",
    "identity",
    "weighted",
    "share",
    "vocabulary",
    "total",
    "weights",
    "mininum",
    "weight",
    "maximum",
    "minimum",
    "number",
    "of",
    "unique",
    "generate",
    "new",
    "features",
    "set",
    "inplace",
    "all",
    "add",
    "prefixes",
    "replace",
    "homogeneous",
    "divide",
    "normalize",
    "multiply",
    "build",
    "circles",
    "memory",
    "stats",
    "used",
    "requirement",
    "human",
    "readable",
    "requirements",
    "triangles",
    "squares",
    "per",
    "triads",
    "transitivity",
    "clustering",
    "coefficient",
    "average",
    "remappable",
    "remap",
    "map",
    "sample",
    "negative",
    "positive",
    "holdout",
    "random",
    "label",
    "indices",
    "labels",
    "graphs",
    "subgraph",
    "kfold",
    "prediction",
    "tuples",
    "chains",
    "breadth",
    "first",
    "search",
    "predecessors",
    "parallel",
    "distances",
    "sequential",
    "shortest",
    "path",
    "k",
    "eccentricity",
    "most",
    "distant",
    "dijkstra",
    "four",
    "sweep",
    "diameter",
    "naive",
    "histogram",
    "between",
    "density",
    "rate",
    "degrees",
    "mean",
    "median",
    "central",
    "mode",
    "source",
    "destination",
    "urls",
    "ontology",
    "upper",
    "triangular",
    "lower",
    "imputed",
    "known",
    "indegrees",
    "mask",
    "one",
    "hot",
    "encoded",
    "mapping",
    "triples",
    "multilabel",
    "count",
    "component",
    "singletons",
    "dense",
    "cumulative",
    "reciprocal",
    "sqrt",
    "counts",
    "hashmap",
    "single",
    "boolean",
    "root",
    "to",
    "main",
    "diagonal",
    "anti",
    "bidiagonal",
    "arrowhead",
    "transposed",
    "complementary",
    "structural",
    "similarity",
    "multi",
    "approximated",
    "cliques",
    "max",
    "report",
    "overlap",
    "textual",
    "flat",
    "repeated",
    "hashes",
    "tuple",
    "spanning",
    "tree",
    "wheel",
    "circle",
    "chain",
    "complete",
    "barbell",
    "lollipop",
    "squared",
    "lattice",
    "filter",
    "tendrils",
    "dendritic",
    "trees",
    "arborescence",
    "kruskal",
    "enable",
    "disable",
    "closeness",
    "centrality",
    "harmonic",
    "compatible",
    "vocabularies",
    "same",
    "adjacency",
    "vertex",
    "cover",
    "scale",
    "free",
    "outbounds",
    "inbounds",
    "sorted",
    "uniform",
    "walk",
    "sampling",
    "methods",
    "subsampled",
    "okapi",
    "bm25",
    "feature",
    "propagation",
    "default",
    "representing",
    "probabilities",
    "constant",
    "exclusively",
    "oddities",
    "sources",
    "tradeoff",
    "enabled",
    "transitive",
    "closure",
    "paths",
    "minmax",
    "top",
    "excluded",
    "adjusted",
    "comulative",
    "neighbour",
    "multiple",
    "curie",
    "non",
    "zero",
    "heterogeneous",
    "graphlet",
    "geometric",
    "distribution",
    "threshold",
    "sparse",
    "weighting",
    "stress",
    "betweenness",
    "eigenvector",
    "dot",
    "base",
    "16",
    "tricodes",
    "13",
    "30",
    "64",
    "triad",
    "census",
    "stars",
    "louvain",
    "community",
    "detection",
    "modularity",
    "memberships",
    "distance",
    "preferential",
    "attachment",
    "index",
    "resource",
    "allocation",
    "available",
    "metrics",
    "scores",
    "groups",
    "csv",
];

pub const GRAPH_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("connected", 0.9867929),
        ("from", 0.24942788),
        ("id", 0.412462),
        ("is", 0.87402433),
        ("node", 0.14584725),
        ("unchecked", 0.5054528),
    ],
    &[
        ("connected", 1.3103347),
        ("from", 0.3312083),
        ("id", 0.5476967),
        ("is", 1.1605924),
        ("node", 0.19366646),
    ],
    &[
        ("disconnected", 0.9408769),
        ("from", 0.19405593),
        ("id", 0.32089716),
        ("is", 0.6799946),
        ("node", 0.21133932),
        ("unchecked", 0.39324442),
    ],
    &[
        ("from", 0.24942788),
        ("id", 0.412462),
        ("is", 0.87402433),
        ("node", 0.14584725),
        ("singleton", 0.73318326),
        ("unchecked", 0.5054528),
    ],
    &[
        ("from", 0.3312083),
        ("id", 0.5476967),
        ("is", 1.1605924),
        ("node", 0.19366646),
        ("singleton", 0.97357357),
    ],
    &[
        ("from", 0.15501031),
        ("id", 0.25633007),
        ("is", 0.5431742),
        ("node", 0.090638734),
        ("selfloops", 0.53541404),
        ("singleton", 0.45564663),
        ("unchecked", 0.31412047),
        ("with", 0.4743904),
    ],
    &[
        ("from", 0.19405593),
        ("id", 0.32089716),
        ("is", 0.6799946),
        ("node", 0.113469765),
        ("selfloops", 0.67027974),
        ("singleton", 0.57041967),
        ("with", 0.5938848),
    ],
    &[
        ("from", 0.24942788),
        ("is", 0.87402433),
        ("name", 0.57414216),
        ("node", 0.14584725),
        ("singleton", 0.73318326),
        ("unchecked", 0.5054528),
    ],
    &[
        ("from", 0.3312083),
        ("is", 1.1605924),
        ("name", 0.7623873),
        ("node", 0.19366646),
        ("singleton", 0.97357357),
    ],
    &[
        ("has", 1.5832652),
        ("name", 1.5329753),
        ("node", 0.38941613),
    ],
    &[
        ("has", 1.0888427),
        ("id", 0.7573753),
        ("node", 0.26780915),
        ("type", 0.7218666),
    ],
    &[
        ("has", 1.0888427),
        ("name", 1.0542574),
        ("node", 0.26780915),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("id", 0.7573753),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("name", 1.0542574),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.39319196),
        ("from", 0.3312083),
        ("has", 0.7873977),
        ("ids", 0.4781446),
        ("node", 0.19366646),
    ],
    &[
        ("from", 0.3312083),
        ("has", 0.7873977),
        ("id", 0.5476967),
        ("node", 0.19366646),
        ("selfloop", 1.5426534),
    ],
    &[
        ("and", 0.45714217),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("has", 0.3008059),
        ("id", 0.20923404),
        ("ids", 0.18266337),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("from", 0.19405593),
        ("id", 0.32089716),
        ("is", 0.6799946),
        ("node", 0.21133932),
        ("trap", 0.76772904),
        ("unchecked", 0.39324442),
    ],
    &[
        ("from", 0.24942788),
        ("id", 0.412462),
        ("is", 0.87402433),
        ("node", 0.2664172),
        ("trap", 0.9867929),
    ],
    &[
        ("from", 0.12652998),
        ("id", 0.20923404),
        ("is", 0.44337574),
        ("node", 0.1411762),
        ("selfloops", 0.4370414),
        ("trap", 0.5005811),
        ("unchecked", 0.25640652),
        ("with", 0.38722974),
    ],
    &[
        ("from", 0.15501031),
        ("id", 0.25633007),
        ("is", 0.5431742),
        ("node", 0.17118385),
        ("selfloops", 0.53541404),
        ("trap", 0.61325574),
        ("with", 0.4743904),
    ],
    &[
        ("are", 1.2093469),
        ("from", 0.24942788),
        ("ids", 0.36008334),
        ("isomorphic", 0.74778455),
        ("node", 0.14584725),
        ("unchecked", 0.5054528),
    ],
    &[
        ("are", 1.6058578),
        ("from", 0.3312083),
        ("ids", 0.4781446),
        ("isomorphic", 0.9929622),
        ("node", 0.19366646),
    ],
    &[
        ("are", 1.6058578),
        ("from", 0.3312083),
        ("isomorphic", 0.9929622),
        ("names", 0.6370931),
        ("node", 0.19366646),
    ],
    &[
        ("and", 0.7011078),
        ("has", 0.46133867),
        ("name", 0.8319583),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("edge", 0.39319196),
        ("from", 0.3312083),
        ("has", 0.7873977),
        ("names", 0.6370931),
        ("node", 0.19366646),
    ],
    &[
        ("and", 0.45714217),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("has", 0.3008059),
        ("name", 0.2912513),
        ("names", 0.24338573),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("and", 0.45714217),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("has", 0.3008059),
        ("id", 0.3992521),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("and", 0.3799311),
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("has", 0.24999994),
        ("id", 0.33441225),
        ("node", 0.061489392),
        ("type", 0.16574168),
        ("unchecked", 0.21309958),
    ],
    &[
        ("components", 3.2289894),
        ("connected", 2.6347642),
        ("strongly", 3.9247627),
    ],
    &[
        ("coo", 1.8951929),
        ("get", 0.15653217),
        ("jaccard", 1.8516018),
        ("matrix", 1.7756627),
    ],
    &[
        ("get", 0.22761041),
        ("graph", 1.9768125),
        ("jaccard", 2.6923785),
    ],
    &[
        ("coo", 1.0321105),
        ("get", 0.085246466),
        ("intersection", 1.2689577),
        ("matrix", 0.96701515),
        ("neighbours", 1.2689577),
        ("size", 1.1617486),
    ],
    &[
        ("get", 0.113196395),
        ("graph", 0.9831187),
        ("intersection", 1.6850134),
        ("neighbours", 1.6850134),
        ("size", 1.5426534),
    ],
    &[
        ("ancestors", 1.1221241),
        ("coo", 1.0321105),
        ("get", 0.085246466),
        ("matrix", 0.96701515),
        ("shared", 1.3487675),
        ("size", 1.1617486),
    ],
    &[
        ("ancestors", 1.4900371),
        ("get", 0.113196395),
        ("graph", 0.9831187),
        ("shared", 1.7909907),
        ("size", 1.5426534),
    ],
    &[
        ("ancestors", 1.4900371),
        ("coo", 1.3705107),
        ("get", 0.113196395),
        ("jaccard", 1.3389878),
        ("matrix", 1.2840723),
    ],
    &[
        ("ancestors", 2.0604784),
        ("get", 0.15653217),
        ("graph", 1.3594929),
        ("jaccard", 1.8516018),
    ],
    &[
        ("adamic", 1.4900371),
        ("adar", 1.4900371),
        ("coo", 1.3705107),
        ("get", 0.113196395),
        ("matrix", 1.2840723),
    ],
    &[
        ("adamic", 2.0604784),
        ("adar", 2.0604784),
        ("get", 0.15653217),
        ("graph", 1.3594929),
    ],
    &[
        ("coo", 1.8951929),
        ("get", 0.15653217),
        ("laplacian", 1.9436368),
        ("matrix", 1.7756627),
    ],
    &[
        ("get", 0.22761041),
        ("graph", 1.9768125),
        ("laplacian", 2.8262048),
    ],
    &[
        ("coo", 1.0321105),
        ("get", 0.085246466),
        ("laplacian", 1.0584928),
        ("left", 1.3487675),
        ("matrix", 0.96701515),
        ("normalized", 1.1221241),
    ],
    &[
        ("get", 0.113196395),
        ("graph", 0.9831187),
        ("laplacian", 1.405543),
        ("left", 1.7909907),
        ("normalized", 1.4900371),
    ],
    &[
        ("coo", 1.0321105),
        ("get", 0.085246466),
        ("laplacian", 1.0584928),
        ("matrix", 0.96701515),
        ("normalized", 1.1221241),
        ("right", 1.3487675),
    ],
    &[
        ("get", 0.113196395),
        ("graph", 0.9831187),
        ("laplacian", 1.405543),
        ("normalized", 1.4900371),
        ("right", 1.7909907),
    ],
    &[
        ("coo", 1.0321105),
        ("get", 0.085246466),
        ("laplacian", 1.0584928),
        ("matrix", 0.96701515),
        ("normalized", 1.1221241),
        ("symmetric", 1.3487675),
    ],
    &[
        ("get", 0.113196395),
        ("graph", 0.9831187),
        ("laplacian", 1.405543),
        ("normalized", 1.4900371),
        ("symmetric", 1.7909907),
    ],
    &[
        ("by", 1.0881811),
        ("degree", 0.7979063),
        ("increasing", 1.3487675),
        ("node", 0.14584725),
        ("outbound", 1.2093469),
        ("sort", 1.2093469),
    ],
    &[
        ("by", 1.0881811),
        ("decreasing", 1.3487675),
        ("degree", 0.7979063),
        ("node", 0.14584725),
        ("outbound", 1.2093469),
        ("sort", 1.2093469),
    ],
    &[
        ("by", 1.4449652),
        ("lexicographic", 1.7909907),
        ("node", 0.19366646),
        ("order", 1.7909907),
        ("sort", 1.6058578),
    ],
    &[
        ("bfs", 0.9872544),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.113469765),
        ("sorting", 0.9872544),
        ("topological", 0.9872544),
    ],
    &[
        ("bfs", 0.7886109),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("reversed", 0.9135099),
        ("sorting", 0.7886109),
        ("topological", 0.7886109),
    ],
    &[
        ("bfs", 0.7886109),
        ("by", 0.6762648),
        ("from", 0.15501031),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("sort", 0.7515649),
        ("sorting", 0.7886109),
        ("topological", 0.7886109),
    ],
    &[
        ("get", 0.22761041),
        ("histograms", 3.9247627),
        ("predictions", 3.6012464),
    ],
    &[
        ("filtered", 3.9247627),
        ("get", 0.22761041),
        ("predictions", 3.6012464),
    ],
    &[("components", 4.979057), ("remove", 2.9084692)],
    &[("overlaps", 9.821147)],
    &[("contains", 9.011596)],
    &[
        ("bipartite", 2.996102),
        ("edges", 2.18216),
        ("get", 0.22761041),
    ],
    &[
        ("bipartite", 2.0604784),
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("names", 0.8809959),
    ],
    &[("edges", 2.18216), ("get", 0.22761041), ("star", 3.388152)],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("names", 0.8809959),
        ("star", 2.3300989),
    ],
    &[
        ("clique", 2.9054735),
        ("edges", 2.18216),
        ("get", 0.22761041),
    ],
    &[
        ("clique", 1.9981514),
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("names", 0.8809959),
    ],
    &[
        ("id", 1.1012849),
        ("node", 0.38941613),
        ("validate", 2.8262048),
    ],
    &[
        ("ids", 0.9614324),
        ("node", 0.38941613),
        ("validate", 2.8262048),
    ],
    &[
        ("edge", 0.79061335),
        ("id", 1.1012849),
        ("validate", 2.8262048),
    ],
    &[
        ("edge", 0.79061335),
        ("ids", 0.9614324),
        ("validate", 2.8262048),
    ],
    &[
        ("contain", 1.2093469),
        ("must", 0.93181056),
        ("node", 0.14584725),
        ("not", 1.1221241),
        ("types", 0.4897214),
        ("unknown", 0.8496751),
    ],
    &[
        ("contain", 1.2093469),
        ("edge", 0.29610685),
        ("must", 0.93181056),
        ("not", 1.1221241),
        ("types", 0.4897214),
        ("unknown", 0.8496751),
    ],
    &[
        ("id", 0.7573753),
        ("node", 0.26780915),
        ("type", 0.7218666),
        ("validate", 1.9436368),
    ],
    &[
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("type", 0.7218666),
        ("validate", 1.9436368),
    ],
    &[
        ("edge", 0.5437204),
        ("id", 0.7573753),
        ("type", 0.7218666),
        ("validate", 1.9436368),
    ],
    &[
        ("edge", 0.5437204),
        ("ids", 0.661196),
        ("type", 0.7218666),
        ("validate", 1.9436368),
    ],
    &[
        ("have", 2.2206397),
        ("must", 1.711019),
        ("node", 0.26780915),
        ("ontologies", 2.1332383),
    ],
    &[
        ("be", 3.1019008),
        ("must", 2.4879599),
        ("undirected", 2.9054735),
    ],
    &[
        ("acyclic", 2.476648),
        ("be", 2.1332383),
        ("directed", 1.3210373),
        ("must", 1.711019),
    ],
    &[
        ("have", 1.6058578),
        ("must", 1.2373252),
        ("nodes", 0.8635923),
        ("not", 1.4900371),
        ("trap", 1.3103347),
    ],
    &[
        ("be", 3.1019008),
        ("multigraph", 3.1019008),
        ("must", 2.4879599),
    ],
    &[
        ("be", 2.1332383),
        ("multigraph", 2.1332383),
        ("must", 1.711019),
        ("not", 2.0604784),
    ],
    &[
        ("contain", 2.2206397),
        ("identity", 2.476648),
        ("matrix", 1.7756627),
        ("must", 1.711019),
    ],
    &[
        ("contain", 1.2093469),
        ("must", 0.93181056),
        ("nodes", 0.65035814),
        ("not", 1.1221241),
        ("singleton", 0.73318326),
        ("weighted", 0.70018625),
    ],
    &[("edges", 2.18216), ("have", 3.2289894), ("must", 2.4879599)],
    &[
        ("have", 3.2289894),
        ("must", 2.4879599),
        ("nodes", 1.736474),
    ],
    &[
        ("be", 3.1019008),
        ("connected", 2.6347642),
        ("must", 2.4879599),
    ],
    &[
        ("must", 1.711019),
        ("node", 0.26780915),
        ("share", 2.6991367),
        ("vocabulary", 2.6991367),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("total", 1.7421427),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("mininum", 2.6991367),
        ("weight", 1.7421427),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("maximum", 1.8951929),
        ("weight", 1.7421427),
    ],
    &[
        ("degree", 1.0595174),
        ("get", 0.113196395),
        ("maximum", 1.3705107),
        ("node", 0.19366646),
        ("unchecked", 0.6711767),
    ],
    &[
        ("degree", 1.0595174),
        ("get", 0.113196395),
        ("minimum", 1.4449652),
        ("node", 0.19366646),
        ("unchecked", 0.6711767),
    ],
    &[
        ("degree", 1.0595174),
        ("get", 0.113196395),
        ("maximum", 1.3705107),
        ("node", 0.19366646),
        ("weighted", 0.9297577),
    ],
    &[
        ("degree", 1.0595174),
        ("get", 0.113196395),
        ("minimum", 1.4449652),
        ("node", 0.19366646),
        ("weighted", 0.9297577),
    ],
    &[
        ("get", 0.085246466),
        ("nodes", 0.65035814),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("singleton", 0.73318326),
        ("weighted", 0.70018625),
    ],
    &[
        ("get", 0.15653217),
        ("number", 1.1264132),
        ("of", 1.1264132),
        ("selfloops", 1.5819814),
    ],
    &[
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("selfloops", 1.1440115),
        ("unique", 1.3103347),
    ],
    &[
        ("edges", 0.81727993),
        ("features", 1.4699334),
        ("from", 0.24942788),
        ("generate", 0.9867929),
        ("new", 1.4699334),
        ("node", 0.14584725),
    ],
    &[("name", 2.3638268), ("set", 4.783088)],
    &[
        ("all", 1.2373252),
        ("edge", 0.39319196),
        ("inplace", 1.0595174),
        ("set", 1.5426534),
        ("types", 0.65028733),
    ],
    &[
        ("all", 1.711019),
        ("edge", 0.5437204),
        ("set", 2.1332383),
        ("types", 0.8992413),
    ],
    &[
        ("all", 1.2373252),
        ("inplace", 1.0595174),
        ("node", 0.19366646),
        ("set", 1.5426534),
        ("types", 0.65028733),
    ],
    &[
        ("all", 1.711019),
        ("node", 0.26780915),
        ("set", 2.1332383),
        ("types", 0.8992413),
    ],
    &[
        ("ids", 0.4781446),
        ("inplace", 1.0595174),
        ("node", 0.19366646),
        ("remove", 0.93804675),
        ("type", 0.5220186),
    ],
    &[
        ("inplace", 1.0595174),
        ("node", 0.19366646),
        ("remove", 0.93804675),
        ("singleton", 0.97357357),
        ("types", 0.65028733),
    ],
    &[
        ("add", 0.5520134),
        ("from", 0.12652998),
        ("id", 0.20923404),
        ("inplace", 0.40476254),
        ("name", 0.2912513),
        ("node", 0.1411762),
        ("prefixes", 0.46466494),
        ("type", 0.19942436),
    ],
    &[
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("id", 0.17389452),
        ("ids", 0.15181161),
        ("inplace", 0.33639833),
        ("node", 0.061489392),
        ("replace", 0.50986224),
        ("type", 0.31873372),
    ],
    &[
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("id", 0.20923404),
        ("ids", 0.18266337),
        ("node", 0.0739855),
        ("replace", 0.6134784),
        ("type", 0.3805336),
    ],
    &[
        ("add", 0.6762648),
        ("from", 0.15501031),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("node", 0.17118385),
        ("prefixes", 0.5692553),
        ("type", 0.24431232),
    ],
    &[
        ("add", 1.4449652),
        ("inplace", 1.0595174),
        ("name", 0.7623873),
        ("node", 0.19366646),
        ("type", 0.5220186),
    ],
    &[
        ("add", 0.5520134),
        ("from", 0.12652998),
        ("inplace", 0.40476254),
        ("name", 0.5557541),
        ("node", 0.1411762),
        ("prefixes", 0.46466494),
        ("type", 0.19942436),
    ],
    &[
        ("add", 1.4449652),
        ("edge", 0.39319196),
        ("inplace", 1.0595174),
        ("name", 0.7623873),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("inplace", 0.33639833),
        ("name", 0.2420591),
        ("names", 0.202278),
        ("node", 0.061489392),
        ("replace", 0.50986224),
        ("type", 0.31873372),
    ],
    &[
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("name", 0.2912513),
        ("names", 0.24338573),
        ("node", 0.0739855),
        ("replace", 0.6134784),
        ("type", 0.3805336),
    ],
    &[
        ("add", 0.6762648),
        ("from", 0.15501031),
        ("name", 0.67388225),
        ("node", 0.17118385),
        ("prefixes", 0.5692553),
        ("type", 0.24431232),
    ],
    &[
        ("homogeneous", 1.3389878),
        ("inplace", 1.0595174),
        ("node", 0.19366646),
        ("remove", 0.93804675),
        ("types", 0.65028733),
    ],
    &[
        ("edge", 0.39319196),
        ("ids", 0.4781446),
        ("inplace", 1.0595174),
        ("remove", 0.93804675),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.39319196),
        ("inplace", 1.0595174),
        ("remove", 0.93804675),
        ("singleton", 0.97357357),
        ("types", 0.65028733),
    ],
    &[
        ("inplace", 1.0595174),
        ("names", 0.6370931),
        ("node", 0.19366646),
        ("remove", 0.93804675),
        ("type", 0.5220186),
    ],
    &[
        ("inplace", 1.0595174),
        ("name", 0.7623873),
        ("node", 0.19366646),
        ("remove", 0.93804675),
        ("type", 0.5220186),
    ],
    &[
        ("id", 0.7573753),
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("type", 0.7218666),
    ],
    &[
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("singleton", 1.3462936),
        ("types", 0.8992413),
    ],
    &[
        ("homogeneous", 1.8516018),
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("types", 0.8992413),
    ],
    &[
        ("inplace", 1.0595174),
        ("isomorphic", 0.9929622),
        ("node", 0.19366646),
        ("remove", 0.93804675),
        ("types", 0.65028733),
    ],
    &[
        ("isomorphic", 1.3731048),
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.39319196),
        ("inplace", 1.0595174),
        ("isomorphic", 0.9929622),
        ("remove", 0.93804675),
        ("types", 0.65028733),
    ],
    &[
        ("edge", 0.5437204),
        ("isomorphic", 1.3731048),
        ("remove", 1.2971658),
        ("types", 0.8992413),
    ],
    &[
        ("names", 0.8809959),
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("type", 0.7218666),
    ],
    &[
        ("name", 1.0542574),
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.39319196),
        ("inplace", 1.0595174),
        ("name", 0.7623873),
        ("remove", 0.93804675),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.5437204),
        ("id", 0.7573753),
        ("remove", 1.2971658),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.5437204),
        ("remove", 1.2971658),
        ("singleton", 1.3462936),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.5437204),
        ("name", 1.0542574),
        ("remove", 1.2971658),
        ("type", 0.7218666),
    ],
    &[
        ("inplace", 1.4651399),
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("types", 0.8992413),
    ],
    &[
        ("node", 0.38941613),
        ("remove", 1.8861839),
        ("types", 1.3075695),
    ],
    &[
        ("edge", 0.5437204),
        ("inplace", 1.4651399),
        ("remove", 1.2971658),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.79061335),
        ("remove", 1.8861839),
        ("types", 1.3075695),
    ],
    &[
        ("edge", 0.5437204),
        ("inplace", 1.4651399),
        ("remove", 1.2971658),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.79061335),
        ("remove", 1.8861839),
        ("weights", 2.3688624),
    ],
    &[
        ("divide", 2.476648),
        ("edge", 0.5437204),
        ("inplace", 1.4651399),
        ("weights", 1.6291134),
    ],
    &[
        ("divide", 3.6012464),
        ("edge", 0.79061335),
        ("weights", 2.3688624),
    ],
    &[
        ("edge", 0.5437204),
        ("inplace", 1.4651399),
        ("normalize", 2.476648),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.79061335),
        ("normalize", 3.6012464),
        ("weights", 2.3688624),
    ],
    &[
        ("edge", 0.5437204),
        ("inplace", 1.4651399),
        ("multiply", 2.476648),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.79061335),
        ("multiply", 3.6012464),
        ("weights", 2.3688624),
    ],
    &[
        ("bipartite", 0.8730172),
        ("build", 0.82351184),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("graph", 0.5760122),
        ("ids", 0.28014633),
        ("node", 0.113469765),
    ],
    &[
        ("build", 1.0584928),
        ("clique", 1.0881811),
        ("from", 0.24942788),
        ("graph", 0.7403716),
        ("ids", 0.36008334),
        ("node", 0.14584725),
    ],
    &[
        ("bipartite", 0.8730172),
        ("build", 0.82351184),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("graph", 0.5760122),
        ("names", 0.37327477),
        ("node", 0.113469765),
    ],
    &[
        ("build", 1.0584928),
        ("clique", 1.0881811),
        ("from", 0.24942788),
        ("graph", 0.7403716),
        ("names", 0.47978503),
        ("node", 0.14584725),
    ],
    &[
        ("bipartite", 0.8730172),
        ("build", 0.82351184),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("graph", 0.5760122),
        ("node", 0.113469765),
        ("prefixes", 0.7126453),
    ],
    &[
        ("build", 1.0584928),
        ("clique", 1.0881811),
        ("from", 0.24942788),
        ("graph", 0.7403716),
        ("node", 0.14584725),
        ("prefixes", 0.91599154),
    ],
    &[
        ("bipartite", 0.8730172),
        ("build", 0.82351184),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("graph", 0.5760122),
        ("node", 0.113469765),
        ("types", 0.3810053),
    ],
    &[
        ("build", 0.82351184),
        ("clique", 0.8466095),
        ("from", 0.19405593),
        ("graph", 0.5760122),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("type", 0.30585226),
    ],
    &[("circles", 6.0519295), ("get", 0.35097212)],
    &[
        ("get", 0.22761041),
        ("memory", 2.5819569),
        ("stats", 3.9247627),
    ],
    &[
        ("get", 0.15653217),
        ("memory", 1.7756627),
        ("total", 1.7421427),
        ("used", 2.6991367),
    ],
    &[
        ("get", 0.113196395),
        ("memory", 1.2840723),
        ("nodes", 0.8635923),
        ("requirement", 1.6058578),
        ("total", 1.2598323),
    ],
    &[
        ("get", 0.06632211),
        ("human", 0.9038453),
        ("memory", 0.7523418),
        ("nodes", 0.5059813),
        ("readable", 0.9038453),
        ("requirement", 0.9408769),
        ("total", 0.7381395),
    ],
    &[
        ("edges", 1.0852431),
        ("get", 0.113196395),
        ("memory", 1.2840723),
        ("requirement", 1.6058578),
        ("total", 1.2598323),
    ],
    &[
        ("edges", 0.6358472),
        ("get", 0.06632211),
        ("human", 0.9038453),
        ("memory", 0.7523418),
        ("readable", 0.9038453),
        ("requirement", 0.9408769),
        ("total", 0.7381395),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("memory", 0.96701515),
        ("requirements", 1.1221241),
        ("total", 0.94876033),
        ("weights", 0.88720524),
    ],
    &[
        ("edge", 0.18401958),
        ("get", 0.052977566),
        ("human", 0.7219843),
        ("memory", 0.6009646),
        ("readable", 0.7219843),
        ("requirements", 0.6973591),
        ("total", 0.5896199),
        ("weights", 0.5513657),
    ],
    &[
        ("get", 0.085246466),
        ("memory", 0.96701515),
        ("node", 0.14584725),
        ("requirements", 1.1221241),
        ("total", 0.94876033),
        ("types", 0.4897214),
    ],
    &[
        ("get", 0.052977566),
        ("human", 0.7219843),
        ("memory", 0.6009646),
        ("node", 0.090638734),
        ("readable", 0.7219843),
        ("requirements", 0.6973591),
        ("total", 0.5896199),
        ("types", 0.30434394),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("memory", 0.96701515),
        ("requirements", 1.1221241),
        ("total", 0.94876033),
        ("types", 0.4897214),
    ],
    &[
        ("edge", 0.18401958),
        ("get", 0.052977566),
        ("human", 0.7219843),
        ("memory", 0.6009646),
        ("readable", 0.7219843),
        ("requirements", 0.6973591),
        ("total", 0.5896199),
        ("types", 0.30434394),
    ],
    &[
        ("get", 0.15653217),
        ("number", 1.1264132),
        ("of", 1.1264132),
        ("triangles", 2.476648),
    ],
    &[
        ("get", 0.15653217),
        ("number", 1.1264132),
        ("of", 1.1264132),
        ("squares", 2.476648),
    ],
    &[
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("per", 1.2689577),
        ("squares", 1.3487675),
    ],
    &[
        ("get", 0.15653217),
        ("number", 1.1264132),
        ("of", 1.1264132),
        ("triads", 2.476648),
    ],
    &[
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("triads", 1.7909907),
        ("weighted", 0.9297577),
    ],
    &[("get", 0.35097212), ("transitivity", 6.0519295)],
    &[
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("per", 1.2689577),
        ("triangles", 1.3487675),
    ],
    &[
        ("clustering", 1.6850134),
        ("coefficient", 1.4449652),
        ("get", 0.113196395),
        ("node", 0.19366646),
        ("per", 1.6850134),
    ],
    &[
        ("clustering", 3.388152),
        ("coefficient", 2.9054735),
        ("get", 0.22761041),
    ],
    &[
        ("average", 2.6991367),
        ("clustering", 2.3300989),
        ("coefficient", 1.9981514),
        ("get", 0.15653217),
    ],
    &[
        ("are", 3.2289894),
        ("nodes", 1.736474),
        ("remappable", 3.9247627),
    ],
    &[
        ("from", 0.3312083),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("remap", 1.5426534),
        ("unchecked", 0.6711767),
    ],
    &[
        ("from", 0.4580071),
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("remap", 2.1332383),
    ],
    &[
        ("from", 0.4580071),
        ("names", 0.8809959),
        ("node", 0.26780915),
        ("remap", 2.1332383),
    ],
    &[
        ("from", 0.3312083),
        ("map", 1.9518834),
        ("names", 0.6370931),
        ("node", 0.19366646),
        ("remap", 1.5426534),
    ],
    &[
        ("from", 0.6659793),
        ("graph", 1.9768125),
        ("remap", 3.1019008),
    ],
    &[
        ("graph", 1.9768125),
        ("negative", 3.6012464),
        ("sample", 3.6012464),
    ],
    &[
        ("graph", 1.9768125),
        ("positive", 3.9247627),
        ("sample", 3.6012464),
    ],
    &[("connected", 4.06277), ("holdout", 4.4802)],
    &[("holdout", 4.4802), ("random", 3.498236)],
    &[
        ("get", 0.113196395),
        ("holdout", 1.4449652),
        ("indices", 1.9518834),
        ("label", 1.3389878),
        ("node", 0.19366646),
    ],
    &[
        ("get", 0.113196395),
        ("holdout", 1.4449652),
        ("label", 1.3389878),
        ("labels", 1.9518834),
        ("node", 0.19366646),
    ],
    &[
        ("get", 0.113196395),
        ("graphs", 1.7909907),
        ("holdout", 1.4449652),
        ("label", 1.3389878),
        ("node", 0.19366646),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("graphs", 1.7909907),
        ("holdout", 1.4449652),
        ("label", 1.3389878),
    ],
    &[
        ("get", 0.22761041),
        ("random", 2.268656),
        ("subgraph", 3.6012464),
    ],
    &[
        ("get", 0.113196395),
        ("holdout", 1.4449652),
        ("label", 1.3389878),
        ("node", 0.19366646),
        ("random", 1.1282598),
    ],
    &[
        ("get", 0.15653217),
        ("kfold", 2.3300989),
        ("label", 1.8516018),
        ("node", 0.26780915),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("kfold", 2.3300989),
        ("label", 1.8516018),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("kfold", 2.3300989),
        ("prediction", 2.6991367),
    ],
    &[
        ("get", 0.22761041),
        ("node", 0.38941613),
        ("tuples", 3.9247627),
    ],
    &[("chains", 6.0519295), ("get", 0.35097212)],
    &[
        ("breadth", 0.43513918),
        ("first", 0.43513918),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.17389452),
        ("node", 0.061489392),
        ("parallel", 0.48979476),
        ("predecessors", 0.6197259),
        ("search", 0.43513918),
        ("unchecked", 0.21309958),
    ],
    &[
        ("breadth", 0.43513918),
        ("distances", 0.4587786),
        ("first", 0.43513918),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("ids", 0.15181161),
        ("node", 0.061489392),
        ("parallel", 0.48979476),
        ("search", 0.43513918),
        ("unchecked", 0.21309958),
    ],
    &[
        ("breadth", 0.43513918),
        ("distances", 0.4587786),
        ("first", 0.43513918),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.17389452),
        ("node", 0.061489392),
        ("parallel", 0.48979476),
        ("search", 0.43513918),
        ("unchecked", 0.21309958),
    ],
    &[
        ("breadth", 0.43513918),
        ("distances", 0.4587786),
        ("first", 0.43513918),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.17389452),
        ("node", 0.061489392),
        ("search", 0.43513918),
        ("sequential", 0.6197259),
        ("unchecked", 0.21309958),
    ],
    &[
        ("breadth", 0.641419),
        ("first", 0.641419),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("node", 0.090638734),
        ("search", 0.641419),
        ("unchecked", 0.31412047),
    ],
    &[
        ("breadth", 0.641419),
        ("first", 0.641419),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("search", 0.641419),
        ("unchecked", 0.31412047),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.34855095),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
        ("unchecked", 0.25640652),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("names", 0.24338573),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
        ("unchecked", 0.25640652),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.4226371),
        ("node", 0.17118385),
        ("path", 0.57908624),
        ("shortest", 0.5600393),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("names", 0.29816887),
        ("node", 0.17118385),
        ("path", 0.57908624),
        ("shortest", 0.5600393),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("names", 0.5631334),
        ("node", 0.17118385),
        ("path", 0.57908624),
        ("shortest", 0.5600393),
    ],
    &[
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("ids", 0.29194516),
        ("k", 0.4587786),
        ("node", 0.11824872),
        ("path", 0.39285257),
        ("shortest", 0.3799311),
        ("unchecked", 0.21309958),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.34855095),
        ("k", 0.5520134),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("k", 0.5520134),
        ("names", 0.24338573),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("k", 0.5520134),
        ("names", 0.46441898),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
    ],
    &[
        ("and", 0.32059008),
        ("distant", 0.4798266),
        ("eccentricity", 0.39919773),
        ("from", 0.08873444),
        ("get", 0.030326592),
        ("id", 0.28388616),
        ("most", 0.43022743),
        ("node", 0.10038262),
        ("unchecked", 0.1798158),
    ],
    &[
        ("eccentricity", 0.8730172),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
        ("weighted", 0.5447478),
    ],
    &[
        ("and", 0.3799311),
        ("distant", 0.5686422),
        ("eccentricity", 0.47308898),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.33441225),
        ("most", 0.50986224),
        ("node", 0.11824872),
    ],
    &[
        ("eccentricity", 1.1221241),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("node", 0.14584725),
        ("weighted", 0.70018625),
    ],
    &[
        ("eccentricity", 1.4900371),
        ("from", 0.3312083),
        ("get", 0.113196395),
        ("name", 0.7623873),
        ("node", 0.19366646),
    ],
    &[
        ("eccentricity", 1.1221241),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("name", 0.57414216),
        ("node", 0.14584725),
        ("weighted", 0.70018625),
    ],
    &[
        ("dijkstra", 1.2093469),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("node", 0.14584725),
        ("unchecked", 0.5054528),
    ],
    &[
        ("dijkstra", 1.2093469),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("node", 0.14584725),
        ("unchecked", 0.5054528),
    ],
    &[
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("ids", 0.29194516),
        ("node", 0.11824872),
        ("path", 0.39285257),
        ("shortest", 0.3799311),
        ("unchecked", 0.21309958),
        ("weighted", 0.29519945),
    ],
    &[
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("ids", 0.15181161),
        ("names", 0.202278),
        ("node", 0.11824872),
        ("path", 0.39285257),
        ("shortest", 0.3799311),
        ("unchecked", 0.21309958),
        ("weighted", 0.29519945),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.34855095),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
        ("weighted", 0.35519102),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("names", 0.24338573),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
        ("weighted", 0.35519102),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("names", 0.46441898),
        ("node", 0.1411762),
        ("path", 0.4726896),
        ("shortest", 0.45714217),
        ("weighted", 0.35519102),
    ],
    &[
        ("breadth", 0.80298626),
        ("first", 0.80298626),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("search", 0.80298626),
    ],
    &[
        ("dijkstra", 1.6058578),
        ("from", 0.3312083),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
    ],
    &[
        ("four", 3.9247627),
        ("get", 0.22761041),
        ("sweep", 3.9247627),
    ],
    &[
        ("diameter", 3.388152),
        ("get", 0.22761041),
        ("naive", 3.9247627),
    ],
    &[("diameter", 5.2244835), ("get", 0.35097212)],
    &[
        ("breadth", 0.80298626),
        ("first", 0.80298626),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("search", 0.80298626),
    ],
    &[
        ("dijkstra", 1.6058578),
        ("from", 0.3312083),
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("node", 0.19366646),
    ],
    &[
        ("between", 0.9872544),
        ("distances", 0.8466095),
        ("get", 0.06632211),
        ("histogram", 0.9872544),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("type", 0.30585226),
    ],
    &[
        ("between", 0.9872544),
        ("distances", 0.8466095),
        ("get", 0.06632211),
        ("histogram", 0.9872544),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("type", 0.30585226),
    ],
    &[
        ("between", 1.6850134),
        ("distances", 1.4449652),
        ("get", 0.113196395),
        ("histogram", 1.6850134),
        ("prefixes", 1.2163196),
    ],
    &[
        ("components", 1.6058578),
        ("connected", 1.3103347),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
    ],
    &[
        ("connected", 1.3103347),
        ("get", 0.113196395),
        ("nodes", 0.8635923),
        ("number", 0.81456685),
        ("of", 0.81456685),
    ],
    &[
        ("get", 0.06632211),
        ("nodes", 0.5059813),
        ("number", 0.47725716),
        ("of", 0.47725716),
        ("selfloops", 0.67027974),
        ("singleton", 0.57041967),
        ("with", 0.5938848),
    ],
    &[
        ("get", 0.113196395),
        ("nodes", 0.8635923),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("singleton", 0.97357357),
    ],
    &[
        ("disconnected", 1.6058578),
        ("get", 0.113196395),
        ("nodes", 0.8635923),
        ("number", 0.81456685),
        ("of", 0.81456685),
    ],
    &[
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("singleton", 1.3462936),
    ],
    &[
        ("get", 0.15653217),
        ("names", 0.8809959),
        ("node", 0.26780915),
        ("singleton", 1.3462936),
    ],
    &[
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("node", 0.14584725),
        ("selfloops", 0.86153746),
        ("singleton", 0.73318326),
        ("with", 0.763344),
    ],
    &[
        ("get", 0.085246466),
        ("names", 0.47978503),
        ("node", 0.14584725),
        ("selfloops", 0.86153746),
        ("singleton", 0.73318326),
        ("with", 0.763344),
    ],
    &[("density", 6.0519295), ("get", 0.35097212)],
    &[
        ("get", 0.15653217),
        ("nodes", 1.1942074),
        ("rate", 2.0604784),
        ("trap", 1.8119793),
    ],
    &[
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("trap", 1.8119793),
    ],
    &[
        ("degrees", 1.7756627),
        ("get", 0.15653217),
        ("mean", 2.476648),
        ("node", 0.26780915),
    ],
    &[
        ("degrees", 1.2840723),
        ("get", 0.113196395),
        ("mean", 1.7909907),
        ("node", 0.19366646),
        ("weighted", 0.9297577),
    ],
    &[
        ("edges", 1.0852431),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("undirected", 1.4449652),
    ],
    &[
        ("edges", 0.81727993),
        ("get", 0.085246466),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("undirected", 1.0881811),
        ("unique", 0.9867929),
    ],
    &[
        ("edges", 1.5007144),
        ("get", 0.15653217),
        ("number", 1.1264132),
        ("of", 1.1264132),
    ],
    &[
        ("edges", 1.0852431),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("unique", 1.3103347),
    ],
    &[
        ("degrees", 1.7756627),
        ("get", 0.15653217),
        ("median", 2.476648),
        ("node", 0.26780915),
    ],
    &[
        ("degrees", 1.2840723),
        ("get", 0.113196395),
        ("median", 1.7909907),
        ("node", 0.19366646),
        ("weighted", 0.9297577),
    ],
    &[
        ("degree", 1.4651399),
        ("get", 0.15653217),
        ("maximum", 1.8951929),
        ("node", 0.26780915),
    ],
    &[
        ("central", 1.1617486),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("most", 1.2093469),
        ("node", 0.14584725),
        ("unchecked", 0.5054528),
    ],
    &[
        ("central", 1.5426534),
        ("get", 0.113196395),
        ("id", 0.5476967),
        ("most", 1.6058578),
        ("node", 0.19366646),
    ],
    &[
        ("degree", 1.4651399),
        ("get", 0.15653217),
        ("minimum", 1.9981514),
        ("node", 0.26780915),
    ],
    &[
        ("degrees", 1.7756627),
        ("get", 0.15653217),
        ("mode", 2.6991367),
        ("node", 0.26780915),
    ],
    &[
        ("get", 0.15653217),
        ("nodes", 1.1942074),
        ("rate", 2.0604784),
        ("selfloop", 2.1332383),
    ],
    &[("get", 0.35097212), ("name", 2.3638268)],
    &[
        ("get", 0.113196395),
        ("nodes", 0.8635923),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("trap", 1.3103347),
    ],
    &[
        ("get", 0.06632211),
        ("nodes", 0.5059813),
        ("number", 0.47725716),
        ("of", 0.47725716),
        ("selfloops", 0.67027974),
        ("trap", 0.76772904),
        ("with", 0.5938848),
    ],
    &[
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("source", 1.7756627),
    ],
    &[
        ("directed", 0.95530945),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("source", 1.2840723),
    ],
    &[
        ("get", 0.22761041),
        ("names", 1.2810392),
        ("source", 2.5819569),
    ],
    &[
        ("destination", 1.9436368),
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
    ],
    &[
        ("destination", 1.405543),
        ("directed", 0.95530945),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
    ],
    &[
        ("destination", 2.8262048),
        ("get", 0.22761041),
        ("names", 1.2810392),
    ],
    &[
        ("get", 0.22761041),
        ("names", 1.2810392),
        ("node", 0.38941613),
    ],
    &[
        ("get", 0.22761041),
        ("node", 0.38941613),
        ("urls", 3.9247627),
    ],
    &[
        ("get", 0.22761041),
        ("node", 0.38941613),
        ("ontologies", 3.1019008),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("name", 0.57414216),
        ("node", 0.14584725),
        ("ontology", 1.2093469),
        ("unchecked", 0.5054528),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("node", 0.14584725),
        ("ontology", 1.2093469),
        ("unchecked", 0.5054528),
    ],
    &[
        ("from", 0.3312083),
        ("get", 0.113196395),
        ("name", 0.7623873),
        ("node", 0.19366646),
        ("ontology", 1.6058578),
    ],
    &[
        ("from", 0.3312083),
        ("get", 0.113196395),
        ("id", 0.5476967),
        ("node", 0.19366646),
        ("ontology", 1.6058578),
    ],
    &[
        ("get", 0.22761041),
        ("ids", 0.9614324),
        ("node", 0.38941613),
    ],
    &[
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("triangular", 0.87402433),
        ("type", 0.3931242),
        ("upper", 1.0321105),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("lower", 1.0321105),
        ("triangular", 0.87402433),
        ("type", 0.3931242),
    ],
    &[
        ("directed", 0.7194289),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("imputed", 1.2689577),
        ("type", 0.3931242),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("imputed", 0.9872544),
        ("triangular", 0.6799946),
        ("type", 0.30585226),
        ("upper", 0.80298626),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("imputed", 0.9872544),
        ("lower", 0.80298626),
        ("triangular", 0.6799946),
        ("type", 0.30585226),
    ],
    &[
        ("directed", 0.7194289),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("known", 0.771524),
        ("type", 0.3931242),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("known", 0.6002489),
        ("triangular", 0.6799946),
        ("type", 0.30585226),
        ("upper", 0.80298626),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("known", 0.6002489),
        ("lower", 0.80298626),
        ("triangular", 0.6799946),
        ("type", 0.30585226),
    ],
    &[
        ("directed", 0.4470988),
        ("edge", 0.18401958),
        ("get", 0.052977566),
        ("known", 0.47947398),
        ("nodes", 0.40417382),
        ("source", 0.6009646),
        ("types", 0.30434394),
        ("with", 0.4743904),
    ],
    &[
        ("destination", 0.6578146),
        ("directed", 0.4470988),
        ("edge", 0.18401958),
        ("get", 0.052977566),
        ("known", 0.47947398),
        ("nodes", 0.40417382),
        ("types", 0.30434394),
        ("with", 0.4743904),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("type", 0.5220186),
        ("unique", 1.3103347),
    ],
    &[
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("names", 0.47978503),
        ("triangular", 0.87402433),
        ("type", 0.3931242),
        ("upper", 1.0321105),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("lower", 1.0321105),
        ("names", 0.47978503),
        ("triangular", 0.87402433),
        ("type", 0.3931242),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("type", 0.5220186),
        ("unique", 1.3103347),
    ],
    &[
        ("directed", 1.3210373),
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("undirected", 1.9981514),
        ("weights", 1.6291134),
    ],
    &[
        ("get", 0.15653217),
        ("indegrees", 2.476648),
        ("node", 0.26780915),
        ("weighted", 1.2857033),
    ],
    &[
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("type", 0.7218666),
    ],
    &[
        ("get", 0.113196395),
        ("known", 1.0244851),
        ("mask", 1.3103347),
        ("node", 0.19366646),
        ("types", 0.65028733),
    ],
    &[
        ("get", 0.113196395),
        ("mask", 1.3103347),
        ("node", 0.19366646),
        ("types", 0.65028733),
        ("unknown", 1.1282598),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("known", 1.0244851),
        ("mask", 1.3103347),
        ("types", 0.65028733),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("mask", 1.3103347),
        ("types", 0.65028733),
        ("unknown", 1.1282598),
    ],
    &[
        ("encoded", 1.2093469),
        ("get", 0.085246466),
        ("hot", 1.2093469),
        ("node", 0.14584725),
        ("one", 1.2093469),
        ("types", 0.4897214),
    ],
    &[
        ("encoded", 0.9408769),
        ("get", 0.06632211),
        ("hot", 0.9408769),
        ("known", 0.6002489),
        ("node", 0.113469765),
        ("one", 0.9408769),
        ("types", 0.3810053),
    ],
    &[
        ("edge", 0.29610685),
        ("encoded", 1.2093469),
        ("get", 0.085246466),
        ("hot", 1.2093469),
        ("one", 1.2093469),
        ("types", 0.4897214),
    ],
    &[
        ("edge", 0.23037235),
        ("encoded", 0.9408769),
        ("get", 0.06632211),
        ("hot", 0.9408769),
        ("known", 0.6002489),
        ("one", 0.9408769),
        ("types", 0.3810053),
    ],
    &[
        ("get", 0.15653217),
        ("names", 0.8809959),
        ("node", 0.26780915),
        ("type", 0.7218666),
    ],
    &[
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("type", 0.5220186),
        ("unique", 1.3103347),
    ],
    &[
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("node", 0.19366646),
        ("type", 0.5220186),
        ("unique", 1.3103347),
    ],
    &[
        ("directed", 0.7194289),
        ("edges", 0.81727993),
        ("get", 0.085246466),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("unique", 0.9867929),
    ],
    &[
        ("get", 0.22761041),
        ("mapping", 3.388152),
        ("nodes", 1.736474),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
    ],
    &[
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
    ],
    &[
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("triples", 1.7909907),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("names", 0.8809959),
        ("node", 0.26780915),
    ],
    &[
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("node", 0.19366646),
    ],
    &[
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("triples", 1.7909907),
    ],
    &[
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
        ("unknown", 0.8496751),
    ],
    &[
        ("get", 0.085246466),
        ("known", 0.771524),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("get", 0.113196395),
        ("node", 0.19366646),
        ("rate", 1.4900371),
        ("types", 0.65028733),
        ("unknown", 1.1282598),
    ],
    &[
        ("get", 0.113196395),
        ("known", 1.0244851),
        ("node", 0.19366646),
        ("rate", 1.4900371),
        ("types", 0.65028733),
    ],
    &[
        ("get", 0.085246466),
        ("minimum", 1.0881811),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("get", 0.085246466),
        ("maximum", 1.0321105),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("count", 1.9981514),
        ("get", 0.15653217),
        ("maximum", 1.8951929),
        ("multilabel", 2.476648),
    ],
    &[
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("singleton", 0.73318326),
        ("types", 0.4897214),
    ],
    &[
        ("get", 0.085246466),
        ("homogeneous", 1.0083711),
        ("node", 0.14584725),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("get", 0.113196395),
        ("homogeneous", 1.3389878),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("type", 0.5220186),
    ],
    &[
        ("get", 0.113196395),
        ("homogeneous", 1.3389878),
        ("names", 0.6370931),
        ("node", 0.19366646),
        ("type", 0.5220186),
    ],
    &[
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("singleton", 0.97357357),
        ("type", 0.5220186),
    ],
    &[
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("node", 0.19366646),
        ("singleton", 0.97357357),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
        ("unknown", 0.8496751),
    ],
    &[
        ("directed", 0.4470988),
        ("edge", 0.34754658),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("types", 0.30434394),
        ("unknown", 0.5280421),
        ("with", 0.4743904),
    ],
    &[
        ("edge", 0.28662342),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("triangular", 0.44337574),
        ("types", 0.24842626),
        ("unknown", 0.43102387),
        ("upper", 0.5235698),
        ("with", 0.38722974),
    ],
    &[
        ("edge", 0.28662342),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("lower", 0.5235698),
        ("triangular", 0.44337574),
        ("types", 0.24842626),
        ("unknown", 0.43102387),
        ("with", 0.38722974),
    ],
    &[
        ("directed", 0.4470988),
        ("edge", 0.34754658),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("known", 0.47947398),
        ("types", 0.30434394),
        ("with", 0.4743904),
    ],
    &[
        ("edge", 0.28662342),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("known", 0.3913793),
        ("triangular", 0.44337574),
        ("types", 0.24842626),
        ("upper", 0.5235698),
        ("with", 0.38722974),
    ],
    &[
        ("edge", 0.28662342),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("known", 0.3913793),
        ("lower", 0.5235698),
        ("triangular", 0.44337574),
        ("types", 0.24842626),
        ("with", 0.38722974),
    ],
    &[
        ("edge", 0.34754658),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("known", 0.47947398),
        ("node", 0.090638734),
        ("types", 0.30434394),
        ("with", 0.4743904),
    ],
    &[
        ("edge", 0.34754658),
        ("get", 0.052977566),
        ("known", 0.47947398),
        ("names", 0.29816887),
        ("node", 0.090638734),
        ("types", 0.30434394),
        ("with", 0.4743904),
    ],
    &[
        ("edge", 0.23037235),
        ("edges", 0.6358472),
        ("get", 0.06632211),
        ("mask", 0.76772904),
        ("types", 0.3810053),
        ("unknown", 0.6610508),
        ("with", 0.5938848),
    ],
    &[
        ("directed", 0.4470988),
        ("edge", 0.18401958),
        ("edges", 0.50790966),
        ("get", 0.052977566),
        ("known", 0.47947398),
        ("mask", 0.61325574),
        ("types", 0.30434394),
        ("with", 0.4743904),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("known", 0.6002489),
        ("mask", 0.76772904),
        ("triangular", 0.6799946),
        ("types", 0.3810053),
        ("upper", 0.80298626),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("known", 0.6002489),
        ("lower", 0.80298626),
        ("mask", 0.76772904),
        ("triangular", 0.6799946),
        ("types", 0.3810053),
    ],
    &[
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.21133932),
        ("types", 0.3810053),
        ("unknown", 0.6610508),
        ("with", 0.5938848),
    ],
    &[
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("known", 0.6002489),
        ("node", 0.21133932),
        ("types", 0.3810053),
        ("with", 0.5938848),
    ],
    &[
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("types", 0.3810053),
        ("unknown", 0.6610508),
        ("with", 0.5938848),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("ids", 0.28014633),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.5217772),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("name", 0.446685),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("get", 0.06632211),
        ("known", 0.6002489),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("types", 0.3810053),
        ("with", 0.5938848),
    ],
    &[
        ("get", 0.06632211),
        ("mask", 0.76772904),
        ("node", 0.113469765),
        ("nodes", 0.5059813),
        ("types", 0.3810053),
        ("unknown", 0.6610508),
        ("with", 0.5938848),
    ],
    &[
        ("get", 0.06632211),
        ("known", 0.6002489),
        ("mask", 0.76772904),
        ("node", 0.113469765),
        ("nodes", 0.5059813),
        ("types", 0.3810053),
        ("with", 0.5938848),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("known", 0.771524),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("rate", 1.4900371),
        ("types", 0.65028733),
        ("unknown", 1.1282598),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("known", 1.0244851),
        ("rate", 1.4900371),
        ("types", 0.65028733),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("minimum", 1.0881811),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("maximum", 1.0321105),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("types", 0.4897214),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("singleton", 0.73318326),
        ("types", 0.4897214),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("singleton", 0.97357357),
        ("type", 0.5220186),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("names", 0.6370931),
        ("singleton", 0.97357357),
        ("type", 0.5220186),
    ],
    &[
        ("get", 0.15653217),
        ("nodes", 1.1942074),
        ("number", 1.1264132),
        ("of", 1.1264132),
    ],
    &[
        ("component", 1.9518834),
        ("connected", 1.3103347),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
    ],
    &[
        ("directed", 0.95530945),
        ("edges", 1.0852431),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("types", 0.65028733),
    ],
    &[
        ("get", 0.113196395),
        ("node", 0.19366646),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("types", 0.65028733),
    ],
    &[
        ("degrees", 2.5819569),
        ("get", 0.22761041),
        ("node", 0.38941613),
    ],
    &[
        ("get", 0.22761041),
        ("indegrees", 3.6012464),
        ("node", 0.38941613),
    ],
    &[
        ("degrees", 1.7756627),
        ("get", 0.15653217),
        ("node", 0.26780915),
        ("weighted", 1.2857033),
    ],
    &[
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("not", 1.4900371),
        ("singletons", 1.9518834),
    ],
    &[
        ("dense", 2.6991367),
        ("get", 0.15653217),
        ("mapping", 2.3300989),
        ("nodes", 1.1942074),
    ],
    &[
        ("edges", 1.0852431),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
        ("parallel", 1.5426534),
    ],
    &[
        ("cumulative", 2.6991367),
        ("degrees", 1.7756627),
        ("get", 0.15653217),
        ("node", 0.26780915),
    ],
    &[
        ("degrees", 1.7756627),
        ("get", 0.15653217),
        ("reciprocal", 2.1332383),
        ("sqrt", 2.1332383),
    ],
    &[
        ("get", 0.085246466),
        ("nodes", 0.65035814),
        ("number", 0.6134378),
        ("of", 0.6134378),
        ("source", 0.96701515),
        ("unique", 0.9867929),
    ],
    &[
        ("counts", 1.1221241),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("hashmap", 1.1221241),
        ("id", 0.412462),
        ("type", 0.3931242),
    ],
    &[
        ("counts", 1.1221241),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("hashmap", 1.1221241),
        ("names", 0.47978503),
        ("type", 0.3931242),
    ],
    &[
        ("counts", 1.1221241),
        ("get", 0.085246466),
        ("hashmap", 1.1221241),
        ("id", 0.412462),
        ("node", 0.14584725),
        ("type", 0.3931242),
    ],
    &[
        ("counts", 1.1221241),
        ("get", 0.085246466),
        ("hashmap", 1.1221241),
        ("names", 0.47978503),
        ("node", 0.14584725),
        ("type", 0.3931242),
    ],
    &[
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("label", 1.0083711),
        ("node", 0.14584725),
        ("single", 1.3487675),
        ("type", 0.3931242),
    ],
    &[
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("known", 0.6002489),
        ("label", 0.78451693),
        ("node", 0.113469765),
        ("single", 1.0493468),
        ("type", 0.30585226),
    ],
    &[
        ("boolean", 1.7909907),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("node", 0.19366646),
        ("type", 0.5220186),
    ],
    &[
        ("boolean", 1.3487675),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("known", 0.771524),
        ("node", 0.14584725),
        ("type", 0.3931242),
    ],
    &[
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("node", 0.26780915),
        ("root", 2.476648),
    ],
    &[
        ("get", 0.15653217),
        ("names", 0.8809959),
        ("node", 0.26780915),
        ("root", 2.476648),
    ],
    &[
        ("directed", 1.9208949),
        ("inplace", 2.1304317),
        ("to", 2.5332162),
    ],
    &[("directed", 2.9619932), ("to", 3.9061842)],
    &[
        ("to", 2.5332162),
        ("triangular", 2.333669),
        ("upper", 2.7557635),
    ],
    &[
        ("lower", 2.7557635),
        ("to", 2.5332162),
        ("triangular", 2.333669),
    ],
    &[
        ("diagonal", 3.6012464),
        ("main", 3.9247627),
        ("to", 2.5332162),
    ],
    &[
        ("anti", 3.9247627),
        ("diagonal", 3.6012464),
        ("to", 2.5332162),
    ],
    &[("bidiagonal", 6.0519295), ("to", 3.9061842)],
    &[("arrowhead", 6.0519295), ("to", 3.9061842)],
    &[("to", 3.9061842), ("transposed", 6.0519295)],
    &[("to", 3.9061842), ("undirected", 4.4802)],
    &[("complementary", 6.0519295), ("to", 3.9061842)],
    &[
        ("graph", 0.9831187),
        ("multi", 1.9518834),
        ("similarity", 1.9518834),
        ("structural", 1.7909907),
        ("to", 1.2598323),
    ],
    &[
        ("approximated", 2.5332162),
        ("cliques", 3.6012464),
        ("get", 0.22761041),
    ],
    &[
        ("clique", 2.9054735),
        ("get", 0.22761041),
        ("max", 3.9247627),
    ],
    &[
        ("approximated", 1.2598323),
        ("cliques", 1.7909907),
        ("get", 0.113196395),
        ("number", 0.81456685),
        ("of", 0.81456685),
    ],
    &[("report", 7.7620554)],
    &[
        ("overlap", 3.9247627),
        ("report", 3.1019008),
        ("textual", 3.6012464),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("node", 0.2664172),
        ("report", 1.1617486),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("name", 0.57414216),
        ("node", 0.2664172),
        ("report", 1.1617486),
    ],
    &[("report", 4.783088), ("textual", 5.553072)],
    &[
        ("get", 0.15653217),
        ("isomorphic", 1.3731048),
        ("mask", 1.8119793),
        ("nodes", 1.1942074),
    ],
    &[
        ("get", 0.15653217),
        ("ids", 0.661196),
        ("isomorphic", 1.3731048),
        ("node", 0.26780915),
    ],
    &[
        ("flat", 1.4699334),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("isomorphic", 0.74778455),
        ("node", 0.14584725),
        ("repeated", 1.4699334),
    ],
    &[
        ("get", 0.15653217),
        ("hashes", 2.3300989),
        ("isomorphic", 1.3731048),
        ("node", 0.26780915),
    ],
    &[
        ("get", 0.15653217),
        ("isomorphic", 1.3731048),
        ("names", 0.8809959),
        ("node", 0.26780915),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("isomorphic", 0.9929622),
        ("node", 0.19366646),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("hashes", 2.3300989),
        ("isomorphic", 1.3731048),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("isomorphic", 0.9929622),
        ("names", 0.6370931),
        ("node", 0.19366646),
    ],
    &[
        ("get", 0.113196395),
        ("ids", 0.4781446),
        ("isomorphic", 0.9929622),
        ("node", 0.19366646),
        ("tuple", 1.5426534),
    ],
    &[
        ("get", 0.15653217),
        ("hashes", 2.3300989),
        ("isomorphic", 1.3731048),
        ("tuple", 2.1332383),
    ],
    &[
        ("get", 0.113196395),
        ("isomorphic", 0.9929622),
        ("names", 0.6370931),
        ("node", 0.19366646),
        ("tuple", 1.5426534),
    ],
    &[
        ("connected", 1.8119793),
        ("generate", 1.8119793),
        ("graph", 1.3594929),
        ("random", 1.5601994),
    ],
    &[
        ("generate", 1.8119793),
        ("random", 1.5601994),
        ("spanning", 2.2206397),
        ("tree", 2.476648),
    ],
    &[
        ("generate", 2.6347642),
        ("graph", 1.9768125),
        ("star", 3.388152),
    ],
    &[
        ("generate", 2.6347642),
        ("graph", 1.9768125),
        ("wheel", 3.9247627),
    ],
    &[
        ("circle", 3.9247627),
        ("generate", 2.6347642),
        ("graph", 1.9768125),
    ],
    &[
        ("chain", 3.9247627),
        ("generate", 2.6347642),
        ("graph", 1.9768125),
    ],
    &[
        ("complete", 3.9247627),
        ("generate", 2.6347642),
        ("graph", 1.9768125),
    ],
    &[
        ("barbell", 3.9247627),
        ("generate", 2.6347642),
        ("graph", 1.9768125),
    ],
    &[
        ("generate", 2.6347642),
        ("graph", 1.9768125),
        ("lollipop", 3.9247627),
    ],
    &[
        ("generate", 1.8119793),
        ("graph", 1.3594929),
        ("lattice", 2.6991367),
        ("squared", 2.6991367),
    ],
    &[
        ("filter", 3.6012464),
        ("from", 0.6659793),
        ("ids", 0.9614324),
    ],
    &[
        ("filter", 3.6012464),
        ("from", 0.6659793),
        ("names", 1.2810392),
    ],
    &[
        ("node", 0.26780915),
        ("remove", 1.2971658),
        ("types", 0.8992413),
        ("unknown", 1.5601994),
    ],
    &[
        ("edge", 0.5437204),
        ("remove", 1.2971658),
        ("types", 0.8992413),
        ("unknown", 1.5601994),
    ],
    &[
        ("nodes", 1.736474),
        ("remove", 1.8861839),
        ("singleton", 1.9576195),
    ],
    &[("remove", 2.9084692), ("tendrils", 5.553072)],
    &[
        ("dendritic", 3.6012464),
        ("remove", 1.8861839),
        ("trees", 3.6012464),
    ],
    &[
        ("isomorphic", 1.9966053),
        ("nodes", 1.736474),
        ("remove", 1.8861839),
    ],
    &[
        ("nodes", 0.8635923),
        ("remove", 0.93804675),
        ("selfloops", 1.1440115),
        ("singleton", 0.97357357),
        ("with", 1.0136231),
    ],
    &[
        ("disconnected", 3.2289894),
        ("nodes", 1.736474),
        ("remove", 1.8861839),
    ],
    &[("remove", 2.9084692), ("selfloops", 3.547075)],
    &[
        ("edges", 2.18216),
        ("parallel", 3.1019008),
        ("remove", 1.8861839),
    ],
    &[
        ("arborescence", 2.476648),
        ("kruskal", 2.476648),
        ("random", 1.5601994),
        ("spanning", 2.2206397),
    ],
    &[
        ("arborescence", 3.6012464),
        ("kruskal", 3.6012464),
        ("spanning", 3.2289894),
    ],
    &[
        ("get", 0.15653217),
        ("random", 1.5601994),
        ("spanning", 2.2206397),
        ("tree", 2.476648),
    ],
    &[
        ("components", 3.2289894),
        ("connected", 2.6347642),
        ("get", 0.22761041),
    ],
    &[("enable", 9.821147)],
    &[("all", 3.8363996), ("disable", 6.0519295)],
    &[
        ("approximated", 1.7421427),
        ("distances", 1.9981514),
        ("get", 0.15653217),
        ("total", 1.7421427),
    ],
    &[
        ("approximated", 1.7421427),
        ("centrality", 1.5601994),
        ("closeness", 2.1332383),
        ("get", 0.15653217),
    ],
    &[
        ("approximated", 1.7421427),
        ("centrality", 1.5601994),
        ("get", 0.15653217),
        ("harmonic", 2.1332383),
    ],
    &[
        ("approximated", 2.5332162),
        ("diameter", 3.388152),
        ("get", 0.22761041),
    ],
    &[
        ("compatible", 2.2206397),
        ("has", 1.0888427),
        ("node", 0.26780915),
        ("vocabularies", 2.3300989),
    ],
    &[
        ("compatible", 1.6058578),
        ("has", 0.7873977),
        ("node", 0.19366646),
        ("types", 0.65028733),
        ("vocabularies", 1.6850134),
    ],
    &[
        ("compatible", 1.6058578),
        ("edge", 0.39319196),
        ("has", 0.7873977),
        ("types", 0.65028733),
        ("vocabularies", 1.6850134),
    ],
    &[("compatible", 4.979057), ("is", 3.5984852)],
    &[
        ("adjacency", 2.6991367),
        ("has", 1.0888427),
        ("matrix", 1.7756627),
        ("same", 2.6991367),
    ],
    &[
        ("cover", 3.9247627),
        ("get", 0.22761041),
        ("vertex", 3.9247627),
    ],
    &[
        ("get", 0.15653217),
        ("node", 0.26780915),
        ("random", 1.5601994),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("random", 1.5601994),
        ("type", 0.7218666),
    ],
    &[
        ("edge", 0.23037235),
        ("free", 0.8730172),
        ("get", 0.06632211),
        ("random", 0.6610508),
        ("scale", 0.8730172),
        ("type", 0.30585226),
        ("unchecked", 0.39324442),
    ],
    &[
        ("edge", 0.29610685),
        ("free", 1.1221241),
        ("get", 0.085246466),
        ("random", 0.8496751),
        ("scale", 1.1221241),
        ("type", 0.3931242),
    ],
    &[
        ("edge", 0.23037235),
        ("free", 0.8730172),
        ("get", 0.06632211),
        ("random", 0.6610508),
        ("scale", 0.8730172),
        ("unchecked", 0.39324442),
        ("weight", 0.7381395),
    ],
    &[
        ("edge", 0.29610685),
        ("free", 1.1221241),
        ("get", 0.085246466),
        ("random", 0.8496751),
        ("scale", 1.1221241),
        ("weight", 0.94876033),
    ],
    &[
        ("get", 0.22761041),
        ("node", 0.38941613),
        ("random", 2.268656),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("id", 0.7573753),
        ("random", 1.5601994),
    ],
    &[
        ("free", 1.1221241),
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("outbounds", 1.4699334),
        ("random", 0.8496751),
        ("scale", 1.1221241),
    ],
    &[
        ("free", 1.1221241),
        ("get", 0.085246466),
        ("inbounds", 1.4699334),
        ("node", 0.14584725),
        ("random", 0.8496751),
        ("scale", 1.1221241),
    ],
    &[
        ("get", 0.113196395),
        ("nodes", 0.8635923),
        ("random", 1.1282598),
        ("sorted", 1.6058578),
        ("unique", 1.3103347),
    ],
    &[
        ("breadth", 1.0321105),
        ("first", 1.0321105),
        ("get", 0.085246466),
        ("nodes", 0.65035814),
        ("random", 0.8496751),
        ("search", 1.0321105),
    ],
    &[
        ("get", 0.085246466),
        ("nodes", 0.65035814),
        ("random", 1.5520903),
        ("uniform", 1.4699334),
        ("walk", 1.4699334),
    ],
    &[
        ("get", 0.15653217),
        ("methods", 2.3300989),
        ("node", 0.26780915),
        ("sampling", 2.6991367),
    ],
    &[
        ("get", 0.22761041),
        ("nodes", 1.736474),
        ("subsampled", 3.9247627),
    ],
    &[
        ("bm25", 1.3487675),
        ("feature", 1.4699334),
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("okapi", 1.3487675),
        ("propagation", 1.3487675),
    ],
    &[
        ("bm25", 1.3487675),
        ("get", 0.085246466),
        ("label", 1.0083711),
        ("node", 0.14584725),
        ("okapi", 1.3487675),
        ("propagation", 1.3487675),
    ],
    &[
        ("default", 2.6991367),
        ("graph", 1.3594929),
        ("has", 1.0888427),
        ("name", 1.0542574),
    ],
    &[("has", 2.441373), ("nodes", 2.677619)],
    &[("edges", 3.3648603), ("has", 2.441373)],
    &[("has", 1.5832652), ("nodes", 1.736474), ("trap", 2.6347642)],
    &[
        ("has", 0.7873977),
        ("nodes", 0.8635923),
        ("selfloops", 1.1440115),
        ("trap", 1.3103347),
        ("with", 1.0136231),
    ],
    &[("directed", 2.9619932), ("is", 3.5984852)],
    &[
        ("acyclic", 3.6012464),
        ("directed", 1.9208949),
        ("is", 2.333669),
    ],
    &[
        ("edge", 0.79061335),
        ("has", 1.5832652),
        ("weights", 2.3688624),
    ],
    &[
        ("edge", 0.39319196),
        ("has", 0.7873977),
        ("probabilities", 1.9518834),
        ("representing", 1.9518834),
        ("weights", 1.178095),
    ],
    &[
        ("has", 1.0888427),
        ("nodes", 1.1942074),
        ("singleton", 1.3462936),
        ("weighted", 1.2857033),
    ],
    &[
        ("constant", 2.6991367),
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("negative", 2.476648),
        ("weights", 1.6291134),
    ],
    &[
        ("edge", 0.79061335),
        ("has", 1.5832652),
        ("types", 1.3075695),
    ],
    &[("has", 2.441373), ("selfloops", 3.547075)],
    &[
        ("disconnected", 3.2289894),
        ("has", 1.5832652),
        ("nodes", 1.736474),
    ],
    &[
        ("has", 1.5832652),
        ("nodes", 1.736474),
        ("singleton", 1.9576195),
    ],
    &[
        ("has", 0.7873977),
        ("nodes", 0.8635923),
        ("selfloops", 1.1440115),
        ("singleton", 0.97357357),
        ("with", 1.0136231),
    ],
    &[("connected", 4.06277), ("is", 3.5984852)],
    &[
        ("has", 1.5832652),
        ("node", 0.38941613),
        ("types", 1.3075695),
    ],
    &[
        ("has", 1.0888427),
        ("multilabel", 2.476648),
        ("node", 0.26780915),
        ("types", 0.8992413),
    ],
    &[
        ("has", 1.0888427),
        ("node", 0.26780915),
        ("types", 0.8992413),
        ("unknown", 1.5601994),
    ],
    &[
        ("has", 1.0888427),
        ("known", 1.416696),
        ("node", 0.26780915),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("types", 0.8992413),
        ("unknown", 1.5601994),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("known", 1.416696),
        ("types", 0.8992413),
    ],
    &[
        ("has", 1.0888427),
        ("homogeneous", 1.8516018),
        ("node", 0.26780915),
        ("types", 0.8992413),
    ],
    &[
        ("exclusively", 1.6058578),
        ("has", 0.7873977),
        ("homogeneous", 1.3389878),
        ("node", 0.19366646),
        ("types", 0.65028733),
    ],
    &[
        ("has", 1.0888427),
        ("homogeneous", 1.8516018),
        ("node", 0.26780915),
        ("ontologies", 2.1332383),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("homogeneous", 1.8516018),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.39319196),
        ("exclusively", 1.6058578),
        ("has", 0.7873977),
        ("homogeneous", 1.3389878),
        ("types", 0.65028733),
    ],
    &[
        ("has", 1.0888427),
        ("node", 0.26780915),
        ("singleton", 1.3462936),
        ("types", 0.8992413),
    ],
    &[
        ("exclusively", 1.6058578),
        ("has", 0.7873977),
        ("node", 0.19366646),
        ("singleton", 0.97357357),
        ("types", 0.65028733),
    ],
    &[
        ("has", 1.5832652),
        ("node", 0.38941613),
        ("oddities", 3.388152),
    ],
    &[
        ("has", 1.0888427),
        ("node", 0.26780915),
        ("oddities", 2.3300989),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("singleton", 1.3462936),
        ("types", 0.8992413),
    ],
    &[
        ("edge", 0.39319196),
        ("exclusively", 1.6058578),
        ("has", 0.7873977),
        ("singleton", 0.97357357),
        ("types", 0.65028733),
    ],
    &[
        ("edge", 0.5437204),
        ("has", 1.0888427),
        ("oddities", 2.3300989),
        ("types", 0.8992413),
    ],
    &[("is", 3.5984852), ("multigraph", 4.783088)],
    &[
        ("has", 1.5832652),
        ("node", 0.38941613),
        ("ontologies", 3.1019008),
    ],
    &[
        ("has", 1.0888427),
        ("node", 0.26780915),
        ("ontologies", 2.1332383),
        ("unknown", 1.5601994),
    ],
    &[
        ("by", 0.6762648),
        ("decreasing", 0.83820975),
        ("degree", 0.4958696),
        ("has", 0.3685136),
        ("node", 0.090638734),
        ("nodes", 0.40417382),
        ("outbound", 0.7515649),
        ("sorted", 0.7515649),
    ],
    &[
        ("by", 1.0881811),
        ("has", 0.5929771),
        ("lexicographic", 1.3487675),
        ("nodes", 0.65035814),
        ("order", 1.3487675),
        ("sorted", 1.2093469),
    ],
    &[
        ("contains", 3.6012464),
        ("identity", 3.6012464),
        ("matrix", 2.5819569),
    ],
    &[
        ("by", 0.6762648),
        ("degree", 0.4958696),
        ("has", 0.3685136),
        ("increasing", 0.83820975),
        ("node", 0.090638734),
        ("nodes", 0.40417382),
        ("outbound", 0.7515649),
        ("sorted", 0.7515649),
    ],
    &[
        ("enabled", 2.476648),
        ("has", 1.0888427),
        ("sources", 2.6991367),
        ("tradeoff", 2.476648),
    ],
    &[
        ("degrees", 0.96701515),
        ("enabled", 1.3487675),
        ("has", 0.5929771),
        ("reciprocal", 1.1617486),
        ("sqrt", 1.1617486),
        ("tradeoff", 1.3487675),
    ],
    &[
        ("dendritic", 3.6012464),
        ("get", 0.22761041),
        ("trees", 3.6012464),
    ],
    &[
        ("closure", 3.9247627),
        ("get", 0.22761041),
        ("transitive", 3.9247627),
    ],
    &[
        ("all", 1.711019),
        ("get", 0.15653217),
        ("paths", 2.476648),
        ("shortest", 1.6547412),
    ],
    &[
        ("all", 1.2373252),
        ("get", 0.113196395),
        ("paths", 1.7909907),
        ("shortest", 1.1966277),
        ("weighted", 0.9297577),
    ],
    &[
        ("edge", 0.42907235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("unchecked", 0.39324442),
        ("weight", 0.7381395),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
        ("weight", 0.7381395),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("name", 0.446685),
        ("node", 0.21133932),
        ("unchecked", 0.39324442),
    ],
    &[
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("name", 0.2912513),
        ("type", 0.3805336),
        ("unchecked", 0.25640652),
    ],
    &[
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("name", 0.2912513),
        ("type", 0.3805336),
        ("unchecked", 0.25640652),
    ],
    &[
        ("count", 0.6762648),
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("type", 0.24431232),
        ("unchecked", 0.31412047),
    ],
    &[
        ("count", 0.6762648),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.17118385),
        ("type", 0.24431232),
        ("unchecked", 0.31412047),
    ],
    &[
        ("and", 0.32059008),
        ("edge", 0.20380214),
        ("from", 0.08873444),
        ("get", 0.030326592),
        ("id", 0.28388616),
        ("ids", 0.12810032),
        ("node", 0.05188543),
        ("type", 0.13985467),
        ("unchecked", 0.1798158),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.4226371),
        ("minmax", 0.7515649),
        ("node", 0.090638734),
        ("unchecked", 0.31412047),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.48411497),
        ("node", 0.090638734),
        ("source", 0.6009646),
        ("unchecked", 0.31412047),
    ],
    &[
        ("destination", 0.6578146),
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.48411497),
        ("node", 0.090638734),
        ("unchecked", 0.31412047),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.5976763),
        ("node", 0.113469765),
        ("source", 0.7523418),
    ],
    &[
        ("destination", 0.82351184),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.5976763),
        ("node", 0.113469765),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("selfloops", 0.53541404),
        ("unchecked", 0.31412047),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.113469765),
        ("number", 0.47725716),
        ("of", 0.47725716),
        ("selfloops", 0.67027974),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("node", 0.113469765),
        ("number", 0.47725716),
        ("of", 0.47725716),
        ("selfloops", 0.67027974),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("node", 0.090638734),
        ("source", 0.6009646),
        ("unchecked", 0.31412047),
    ],
    &[
        ("destination", 0.6578146),
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("node", 0.090638734),
        ("unchecked", 0.31412047),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("name", 0.446685),
        ("node", 0.113469765),
        ("source", 0.7523418),
    ],
    &[
        ("destination", 0.82351184),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("name", 0.446685),
        ("node", 0.113469765),
    ],
    &[
        ("edge", 0.29610685),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("names", 0.47978503),
        ("node", 0.14584725),
    ],
    &[
        ("edge", 0.29610685),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("ids", 0.36008334),
        ("node", 0.14584725),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
    ],
    &[
        ("edge", 0.29610685),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("ids", 0.36008334),
        ("node", 0.14584725),
    ],
    &[
        ("get", 0.085246466),
        ("id", 0.412462),
        ("node", 0.14584725),
        ("source", 0.96701515),
        ("unchecked", 0.5054528),
        ("unique", 0.9867929),
    ],
    &[
        ("and", 0.32059008),
        ("edge", 0.20380214),
        ("from", 0.08873444),
        ("get", 0.030326592),
        ("id", 0.28388616),
        ("ids", 0.12810032),
        ("node", 0.05188543),
        ("type", 0.13985467),
        ("unchecked", 0.1798158),
    ],
    &[
        ("and", 0.3799311),
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.33441225),
        ("ids", 0.15181161),
        ("node", 0.061489392),
        ("type", 0.16574168),
    ],
    &[
        ("and", 0.40469912),
        ("edge", 0.19530523),
        ("from", 0.057226434),
        ("get", 0.019558163),
        ("id", 0.18523084),
        ("ids", 0.0826142),
        ("node", 0.033461846),
        ("type", 0.09019479),
        ("unchecked", 0.11596644),
        ("weight", 0.21767482),
    ],
    &[
        ("and", 0.46223167),
        ("edge", 0.22241054),
        ("from", 0.065564714),
        ("get", 0.022407921),
        ("id", 0.2115635),
        ("ids", 0.094651654),
        ("node", 0.038337465),
        ("type", 0.10333678),
        ("weight", 0.24939153),
    ],
    &[
        ("central", 1.1617486),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("k", 1.0881811),
        ("node", 0.14584725),
        ("top", 1.2689577),
    ],
    &[
        ("central", 0.9038453),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("k", 0.8466095),
        ("node", 0.113469765),
        ("top", 0.9872544),
        ("weighted", 0.5447478),
    ],
    &[
        ("degree", 0.6207744),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.21133932),
        ("unchecked", 0.39324442),
    ],
    &[
        ("degree", 0.40476254),
        ("excluded", 0.745669),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("node", 0.1411762),
        ("selfloop", 0.58933276),
        ("unchecked", 0.25640652),
    ],
    &[
        ("adjusted", 0.83820975),
        ("degree", 0.4958696),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.17118385),
        ("selfloop", 0.7219843),
    ],
    &[
        ("adjusted", 0.83820975),
        ("degree", 0.4958696),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("name", 0.35680836),
        ("node", 0.17118385),
        ("selfloop", 0.7219843),
    ],
    &[
        ("degree", 0.4958696),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.17118385),
        ("unchecked", 0.31412047),
        ("weighted", 0.43514016),
    ],
    &[
        ("degree", 0.7979063),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("node", 0.2664172),
    ],
    &[
        ("comulative", 0.83820975),
        ("degree", 0.4958696),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.17118385),
        ("unchecked", 0.31412047),
    ],
    &[
        ("comulative", 1.0493468),
        ("degree", 0.6207744),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.21133932),
    ],
    &[
        ("degree", 0.4958696),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("reciprocal", 0.7219843),
        ("sqrt", 0.7219843),
        ("unchecked", 0.31412047),
    ],
    &[
        ("degree", 0.6207744),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.113469765),
        ("reciprocal", 0.9038453),
        ("sqrt", 0.9038453),
    ],
    &[
        ("degrees", 0.6009646),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("node", 0.090638734),
        ("reciprocal", 0.7219843),
        ("sqrt", 0.7219843),
        ("unchecked", 0.31412047),
    ],
    &[
        ("degree", 0.6207744),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.21133932),
        ("weighted", 0.5447478),
    ],
    &[
        ("degree", 0.7979063),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("name", 0.57414216),
        ("node", 0.2664172),
    ],
    &[
        ("central", 1.1617486),
        ("get", 0.085246466),
        ("k", 1.0881811),
        ("names", 0.47978503),
        ("node", 0.14584725),
        ("top", 1.2689577),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("ids", 0.22377864),
        ("node", 0.17118385),
        ("type", 0.24431232),
        ("unchecked", 0.31412047),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("ids", 0.28014633),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.48411497),
        ("type", 0.24431232),
        ("unchecked", 0.31412047),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("type", 0.24431232),
        ("unchecked", 0.31412047),
    ],
    &[
        ("edge", 0.42907235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.5976763),
        ("type", 0.30585226),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("ids", 0.22377864),
        ("node", 0.090638734),
        ("type", 0.24431232),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("names", 0.29816887),
        ("node", 0.17118385),
        ("type", 0.24431232),
        ("unchecked", 0.31412047),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("edge", 0.42907235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("name", 0.446685),
        ("type", 0.30585226),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("type", 0.4614178),
    ],
    &[
        ("edge", 0.54089445),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("weight", 0.94876033),
    ],
    &[
        ("edge", 0.29610685),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("node", 0.14584725),
        ("weight", 0.94876033),
    ],
    &[
        ("and", 0.3799311),
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.17389452),
        ("ids", 0.15181161),
        ("node", 0.061489392),
        ("type", 0.16574168),
        ("weight", 0.3999986),
    ],
    &[
        ("and", 0.3799311),
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("name", 0.2420591),
        ("names", 0.202278),
        ("node", 0.061489392),
        ("type", 0.16574168),
        ("weight", 0.3999986),
    ],
    &[
        ("edge", 0.29610685),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("names", 0.47978503),
        ("node", 0.14584725),
        ("weight", 0.94876033),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("name", 0.446685),
        ("node", 0.21133932),
        ("unchecked", 0.39324442),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("name", 0.57414216),
        ("node", 0.2664172),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("name", 0.57414216),
        ("node", 0.2664172),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("names", 0.47978503),
        ("node", 0.2664172),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("names", 0.47978503),
        ("node", 0.2664172),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("names", 0.29816887),
        ("node", 0.17118385),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("names", 0.29816887),
        ("node", 0.17118385),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("name", 0.446685),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.8319583),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("count", 0.8466095),
        ("edge", 0.42907235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("type", 0.30585226),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("type", 0.4614178),
    ],
    &[
        ("count", 0.8466095),
        ("edge", 0.42907235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("node", 0.17118385),
        ("type", 0.4614178),
    ],
    &[
        ("count", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("count", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("node", 0.21133932),
        ("type", 0.30585226),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("ids", 0.28014633),
        ("neighbour", 0.9872544),
        ("node", 0.21133932),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("name", 0.446685),
        ("neighbour", 0.9872544),
        ("node", 0.21133932),
    ],
    &[
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("names", 0.37327477),
        ("neighbour", 0.9872544),
        ("node", 0.21133932),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.5217772),
        ("minmax", 0.9408769),
        ("node", 0.113469765),
    ],
    &[
        ("and", 0.3799311),
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.33441225),
        ("ids", 0.15181161),
        ("node", 0.061489392),
        ("type", 0.16574168),
    ],
    &[
        ("edge", 0.29610685),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("id", 0.412462),
        ("names", 0.47978503),
        ("node", 0.14584725),
    ],
    &[
        ("and", 0.3799311),
        ("edge", 0.24007486),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("id", 0.17389452),
        ("name", 0.2420591),
        ("names", 0.202278),
        ("node", 0.061489392),
        ("type", 0.16574168),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("names", 0.29816887),
        ("type", 0.4614178),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("names", 0.29816887),
        ("node", 0.17118385),
        ("type", 0.4614178),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("multiple", 0.745669),
        ("names", 0.24338573),
        ("node", 0.1411762),
        ("type", 0.3805336),
    ],
    &[
        ("edge", 0.15020931),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("ids", 0.18266337),
        ("minmax", 0.6134784),
        ("node", 0.0739855),
        ("source", 0.4905482),
        ("unchecked", 0.25640652),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("ids", 0.22377864),
        ("minmax", 0.7515649),
        ("node", 0.090638734),
        ("source", 0.6009646),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("name", 0.35680836),
        ("node", 0.17118385),
        ("type", 0.4614178),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("names", 0.24338573),
        ("node", 0.1411762),
        ("type", 0.3805336),
        ("unchecked", 0.25640652),
    ],
    &[
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("node", 0.0739855),
        ("nodes", 0.32991418),
        ("number", 0.3111852),
        ("of", 0.3111852),
        ("type", 0.19942436),
        ("unchecked", 0.25640652),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("nodes", 0.40417382),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("type", 0.24431232),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("name", 0.35680836),
        ("node", 0.090638734),
        ("nodes", 0.40417382),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("type", 0.24431232),
    ],
    &[
        ("edge", 0.15020931),
        ("edges", 0.41459042),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("number", 0.3111852),
        ("of", 0.3111852),
        ("type", 0.19942436),
        ("unchecked", 0.25640652),
    ],
    &[
        ("edge", 0.18401958),
        ("edges", 0.50790966),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("type", 0.24431232),
    ],
    &[
        ("edge", 0.18401958),
        ("edges", 0.50790966),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("name", 0.35680836),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("type", 0.24431232),
    ],
    &[
        ("counts", 0.47308898),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("hashmap", 0.47308898),
        ("id", 0.17389452),
        ("ids", 0.15181161),
        ("node", 0.11824872),
        ("type", 0.16574168),
        ("unchecked", 0.21309958),
    ],
    &[
        ("counts", 0.47308898),
        ("edge", 0.12483904),
        ("from", 0.105159126),
        ("get", 0.035940025),
        ("hashmap", 0.47308898),
        ("id", 0.17389452),
        ("ids", 0.15181161),
        ("node", 0.061489392),
        ("type", 0.16574168),
        ("unchecked", 0.21309958),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("ids", 0.22377864),
        ("node", 0.090638734),
        ("type", 0.24431232),
    ],
    &[
        ("directed", 0.36495247),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("ids", 0.18266337),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("directed", 0.36495247),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("id", 0.20923404),
        ("names", 0.24338573),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("directed", 0.36495247),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("name", 0.2912513),
        ("names", 0.24338573),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("directed", 0.4470988),
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("ids", 0.22377864),
        ("type", 0.24431232),
    ],
    &[
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("name", 0.35680836),
        ("node", 0.090638734),
        ("type", 0.24431232),
    ],
    &[
        ("directed", 0.36495247),
        ("edge", 0.28662342),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("name", 0.2912513),
        ("node", 0.0739855),
        ("type", 0.19942436),
    ],
    &[
        ("directed", 0.4470988),
        ("edge", 0.34754658),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("name", 0.35680836),
        ("type", 0.24431232),
    ],
    &[
        ("curie", 0.5520134),
        ("directed", 0.36495247),
        ("edge", 0.15020931),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("names", 0.24338573),
        ("node", 0.1411762),
        ("prefixes", 0.46466494),
    ],
    &[
        ("curie", 0.5520134),
        ("directed", 0.36495247),
        ("edge", 0.15020931),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("node", 0.1411762),
        ("prefixes", 0.46466494),
    ],
    &[
        ("curie", 0.6762648),
        ("directed", 0.4470988),
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("node", 0.090638734),
        ("prefixes", 0.5692553),
    ],
    &[
        ("curie", 0.5520134),
        ("directed", 0.36495247),
        ("edges", 0.41459042),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("node", 0.0739855),
        ("number", 0.3111852),
        ("of", 0.3111852),
        ("prefixes", 0.46466494),
    ],
    &[
        ("curie", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.21133932),
        ("prefixes", 0.7126453),
    ],
    &[
        ("curie", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.21133932),
        ("prefixes", 0.7126453),
    ],
    &[
        ("curie", 0.6762648),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("node", 0.090638734),
        ("nodes", 0.40417382),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("prefixes", 0.5692553),
    ],
    &[
        ("get", 0.15653217),
        ("names", 0.8809959),
        ("node", 0.26780915),
        ("prefixes", 1.6819717),
    ],
    &[
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("graph", 0.7403716),
        ("ids", 0.36008334),
        ("mapping", 1.2689577),
        ("node", 0.14584725),
    ],
    &[
        ("degrees", 0.96701515),
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("non", 1.4699334),
        ("subgraph", 1.3487675),
        ("zero", 1.4699334),
    ],
    &[
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.5217772),
        ("multigraph", 0.9038453),
        ("node", 0.113469765),
    ],
    &[
        ("edges", 0.50790966),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("multigraph", 0.7219843),
        ("node", 0.090638734),
        ("number", 0.3812292),
        ("of", 0.3812292),
    ],
    &[
        ("ancestors", 1.1221241),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("jaccard", 1.0083711),
        ("node", 0.14584725),
    ],
    &[
        ("ancestors", 1.1221241),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("jaccard", 1.0083711),
        ("names", 0.47978503),
        ("node", 0.14584725),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("graphlet", 0.7886109),
        ("heterogeneous", 0.7886109),
        ("ids", 0.4226371),
        ("node", 0.090638734),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("graphlet", 0.7886109),
        ("heterogeneous", 0.7886109),
        ("ids", 0.22377864),
        ("names", 0.29816887),
        ("node", 0.090638734),
    ],
    &[
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("graphlet", 0.7886109),
        ("heterogeneous", 0.7886109),
        ("names", 0.5631334),
        ("node", 0.090638734),
    ],
    &[("get", 0.35097212), ("tendrils", 5.553072)],
    &[
        ("degree", 0.7979063),
        ("distribution", 1.4699334),
        ("geometric", 1.4699334),
        ("get", 0.085246466),
        ("node", 0.14584725),
        ("threshold", 1.4699334),
    ],
    &[
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("methods", 1.6850134),
        ("sparse", 1.9518834),
        ("weighting", 1.7909907),
    ],
    &[
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("methods", 2.3300989),
        ("weighting", 2.476648),
    ],
    &[("add", 4.4802), ("selfloops", 3.547075)],
    &[
        ("centrality", 2.268656),
        ("degree", 2.1304317),
        ("get", 0.22761041),
    ],
    &[
        ("centrality", 1.5601994),
        ("degree", 1.4651399),
        ("get", 0.15653217),
        ("weighted", 1.2857033),
    ],
    &[
        ("centrality", 0.6610508),
        ("closeness", 0.9038453),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
    ],
    &[
        ("centrality", 0.5280421),
        ("closeness", 0.7219843),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("unchecked", 0.31412047),
        ("weighted", 0.43514016),
    ],
    &[
        ("centrality", 2.268656),
        ("closeness", 3.1019008),
        ("get", 0.22761041),
    ],
    &[
        ("centrality", 1.5601994),
        ("closeness", 2.1332383),
        ("get", 0.15653217),
        ("weighted", 1.2857033),
    ],
    &[
        ("centrality", 0.6610508),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("harmonic", 0.9038453),
        ("id", 0.32089716),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
    ],
    &[
        ("centrality", 0.5280421),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("harmonic", 0.7219843),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("unchecked", 0.31412047),
        ("weighted", 0.43514016),
    ],
    &[
        ("centrality", 2.268656),
        ("get", 0.22761041),
        ("harmonic", 3.1019008),
    ],
    &[
        ("centrality", 1.5601994),
        ("get", 0.15653217),
        ("harmonic", 2.1332383),
        ("weighted", 1.2857033),
    ],
    &[
        ("centrality", 2.268656),
        ("get", 0.22761041),
        ("stress", 3.9247627),
    ],
    &[
        ("betweenness", 3.1019008),
        ("centrality", 2.268656),
        ("get", 0.22761041),
    ],
    &[
        ("approximated", 0.7381395),
        ("betweenness", 0.9038453),
        ("centrality", 0.6610508),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("id", 0.32089716),
        ("node", 0.113469765),
    ],
    &[
        ("approximated", 0.7381395),
        ("betweenness", 0.9038453),
        ("centrality", 0.6610508),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("name", 0.446685),
        ("node", 0.113469765),
    ],
    &[
        ("approximated", 0.5896199),
        ("betweenness", 0.7219843),
        ("centrality", 0.5280421),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("id", 0.25633007),
        ("node", 0.090638734),
        ("weighted", 0.43514016),
    ],
    &[
        ("approximated", 0.5896199),
        ("betweenness", 0.7219843),
        ("centrality", 0.5280421),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("name", 0.35680836),
        ("node", 0.090638734),
        ("weighted", 0.43514016),
    ],
    &[
        ("centrality", 2.268656),
        ("eigenvector", 3.6012464),
        ("get", 0.22761041),
    ],
    &[
        ("centrality", 1.5601994),
        ("eigenvector", 2.476648),
        ("get", 0.15653217),
        ("weighted", 1.2857033),
    ],
    &[("dot", 6.0519295), ("to", 3.9061842)],
    &[
        ("16", 0.9872544),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("13", 0.9872544),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("30", 0.9872544),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("64", 1.0493468),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("16", 0.9872544),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("13", 0.9872544),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("30", 0.9872544),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("64", 1.0493468),
        ("base", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("tricodes", 0.82351184),
    ],
    &[
        ("16", 1.6850134),
        ("base", 1.3103347),
        ("census", 1.6850134),
        ("get", 0.113196395),
        ("triad", 1.6850134),
    ],
    &[
        ("13", 1.6850134),
        ("base", 1.3103347),
        ("census", 1.6850134),
        ("get", 0.113196395),
        ("triad", 1.6850134),
    ],
    &[
        ("30", 1.6850134),
        ("base", 1.3103347),
        ("census", 1.6850134),
        ("get", 0.113196395),
        ("triad", 1.6850134),
    ],
    &[("get", 0.35097212), ("stars", 6.0519295)],
    &[
        ("community", 1.6850134),
        ("detection", 1.9518834),
        ("get", 0.113196395),
        ("louvain", 1.9518834),
        ("undirected", 1.4449652),
    ],
    &[
        ("community", 0.9872544),
        ("directed", 0.55971867),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("memberships", 1.0493468),
        ("modularity", 1.0493468),
        ("node", 0.113469765),
    ],
    &[
        ("community", 0.9872544),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("memberships", 1.0493468),
        ("modularity", 1.0493468),
        ("node", 0.113469765),
        ("undirected", 0.8466095),
    ],
    &[
        ("distance", 1.1436143),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("structural", 1.0493468),
        ("unchecked", 0.39324442),
    ],
    &[
        ("attachment", 1.3103347),
        ("get", 0.113196395),
        ("minimum", 1.4449652),
        ("preferential", 1.3103347),
        ("unchecked", 0.6711767),
    ],
    &[
        ("attachment", 1.3103347),
        ("get", 0.113196395),
        ("maximum", 1.3705107),
        ("preferential", 1.3103347),
        ("unchecked", 0.6711767),
    ],
    &[
        ("attachment", 0.9867929),
        ("get", 0.085246466),
        ("minimum", 1.0881811),
        ("preferential", 0.9867929),
        ("unchecked", 0.5054528),
        ("weighted", 0.70018625),
    ],
    &[
        ("attachment", 0.9867929),
        ("get", 0.085246466),
        ("maximum", 1.0321105),
        ("preferential", 0.9867929),
        ("unchecked", 0.5054528),
        ("weighted", 0.70018625),
    ],
    &[
        ("attachment", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("preferential", 0.76772904),
        ("unchecked", 0.39324442),
    ],
    &[
        ("attachment", 0.9867929),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("node", 0.14584725),
        ("preferential", 0.9867929),
    ],
    &[
        ("attachment", 0.9867929),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("names", 0.47978503),
        ("node", 0.14584725),
        ("preferential", 0.9867929),
    ],
    &[
        ("attachment", 0.61325574),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("node", 0.090638734),
        ("preferential", 0.61325574),
        ("unchecked", 0.31412047),
        ("weighted", 0.43514016),
    ],
    &[
        ("attachment", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("node", 0.113469765),
        ("preferential", 0.76772904),
        ("weighted", 0.5447478),
    ],
    &[
        ("attachment", 0.76772904),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("preferential", 0.76772904),
        ("weighted", 0.5447478),
    ],
    &[
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("intersection", 0.7886109),
        ("neighbours", 0.7886109),
        ("node", 0.090638734),
        ("size", 0.7219843),
        ("unchecked", 0.31412047),
    ],
    &[
        ("coefficient", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("jaccard", 0.78451693),
        ("node", 0.113469765),
        ("unchecked", 0.39324442),
    ],
    &[
        ("coefficient", 1.0881811),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("ids", 0.36008334),
        ("jaccard", 1.0083711),
        ("node", 0.14584725),
    ],
    &[
        ("coefficient", 1.0881811),
        ("from", 0.24942788),
        ("get", 0.085246466),
        ("jaccard", 1.0083711),
        ("names", 0.47978503),
        ("node", 0.14584725),
    ],
    &[
        ("adamic", 0.6973591),
        ("adar", 0.6973591),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("index", 0.62666583),
        ("node", 0.090638734),
        ("unchecked", 0.31412047),
    ],
    &[
        ("adamic", 0.8730172),
        ("adar", 0.8730172),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("index", 0.78451693),
        ("node", 0.113469765),
    ],
    &[
        ("adamic", 0.8730172),
        ("adar", 0.8730172),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("index", 0.78451693),
        ("names", 0.37327477),
        ("node", 0.113469765),
    ],
    &[
        ("allocation", 0.6762648),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("index", 0.62666583),
        ("node", 0.090638734),
        ("resource", 0.6762648),
        ("unchecked", 0.31412047),
    ],
    &[
        ("allocation", 0.5520134),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("index", 0.5115273),
        ("node", 0.0739855),
        ("resource", 0.5520134),
        ("unchecked", 0.25640652),
        ("weighted", 0.35519102),
    ],
    &[
        ("allocation", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("index", 0.78451693),
        ("node", 0.113469765),
        ("resource", 0.8466095),
    ],
    &[
        ("allocation", 0.8466095),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("index", 0.78451693),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("resource", 0.8466095),
    ],
    &[
        ("allocation", 0.6762648),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("index", 0.62666583),
        ("node", 0.090638734),
        ("resource", 0.6762648),
        ("weighted", 0.43514016),
    ],
    &[
        ("allocation", 0.6762648),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("index", 0.62666583),
        ("names", 0.29816887),
        ("node", 0.090638734),
        ("resource", 0.6762648),
        ("weighted", 0.43514016),
    ],
    &[
        ("available", 1.3487675),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("metrics", 1.0321105),
        ("number", 0.6134378),
        ("of", 0.6134378),
    ],
    &[
        ("available", 1.7909907),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("metrics", 1.3705107),
        ("names", 0.6370931),
    ],
    &[
        ("all", 0.4726896),
        ("edge", 0.15020931),
        ("from", 0.12652998),
        ("get", 0.043243896),
        ("ids", 0.18266337),
        ("metrics", 0.5235698),
        ("node", 0.0739855),
        ("tuple", 0.58933276),
        ("unchecked", 0.25640652),
    ],
    &[
        ("all", 0.57908624),
        ("edge", 0.18401958),
        ("from", 0.15501031),
        ("get", 0.052977566),
        ("ids", 0.22377864),
        ("metrics", 0.641419),
        ("node", 0.090638734),
        ("tuple", 0.7219843),
    ],
    &[
        ("all", 0.7249525),
        ("edge", 0.23037235),
        ("from", 0.19405593),
        ("get", 0.06632211),
        ("ids", 0.28014633),
        ("metrics", 0.80298626),
        ("node", 0.113469765),
    ],
    &[
        ("attachment", 1.8119793),
        ("get", 0.15653217),
        ("preferential", 1.8119793),
        ("scores", 2.2206397),
    ],
    &[
        ("allocation", 1.4449652),
        ("get", 0.113196395),
        ("index", 1.3389878),
        ("resource", 1.4449652),
        ("scores", 1.6058578),
    ],
    &[
        ("coefficient", 1.9981514),
        ("get", 0.15653217),
        ("jaccard", 1.8516018),
        ("scores", 2.2206397),
    ],
    &[
        ("adamic", 2.0604784),
        ("adar", 2.0604784),
        ("get", 0.15653217),
        ("scores", 2.2206397),
    ],
    &[
        ("all", 1.711019),
        ("edge", 0.5437204),
        ("get", 0.15653217),
        ("metrics", 1.8951929),
    ],
    &[
        ("all", 1.2373252),
        ("directed", 0.95530945),
        ("edge", 0.39319196),
        ("get", 0.113196395),
        ("metrics", 1.3705107),
    ],
    &[
        ("all", 0.93181056),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("metrics", 1.0321105),
        ("triangular", 0.87402433),
        ("upper", 1.0321105),
    ],
    &[
        ("all", 0.93181056),
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("lower", 1.0321105),
        ("metrics", 1.0321105),
        ("triangular", 0.87402433),
    ],
    &[
        ("get", 0.085246466),
        ("groups", 1.0321105),
        ("ids", 0.36008334),
        ("isomorphic", 0.74778455),
        ("node", 0.14584725),
        ("type", 0.3931242),
    ],
    &[
        ("get", 0.085246466),
        ("groups", 1.0321105),
        ("isomorphic", 0.74778455),
        ("names", 0.47978503),
        ("node", 0.14584725),
        ("type", 0.3931242),
    ],
    &[
        ("get", 0.06632211),
        ("groups", 0.80298626),
        ("isomorphic", 0.5817795),
        ("node", 0.113469765),
        ("number", 0.47725716),
        ("of", 0.47725716),
        ("type", 0.30585226),
    ],
    &[
        ("approximated", 0.7381395),
        ("get", 0.06632211),
        ("groups", 0.80298626),
        ("ids", 0.28014633),
        ("isomorphic", 0.5817795),
        ("node", 0.113469765),
        ("type", 0.30585226),
    ],
    &[
        ("approximated", 0.7381395),
        ("get", 0.06632211),
        ("groups", 0.80298626),
        ("isomorphic", 0.5817795),
        ("names", 0.37327477),
        ("node", 0.113469765),
        ("type", 0.30585226),
    ],
    &[
        ("approximated", 0.5896199),
        ("get", 0.052977566),
        ("groups", 0.641419),
        ("isomorphic", 0.4647208),
        ("node", 0.090638734),
        ("number", 0.3812292),
        ("of", 0.3812292),
        ("type", 0.24431232),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("groups", 1.0321105),
        ("ids", 0.36008334),
        ("isomorphic", 0.74778455),
        ("type", 0.3931242),
    ],
    &[
        ("edge", 0.29610685),
        ("get", 0.085246466),
        ("groups", 1.0321105),
        ("isomorphic", 0.74778455),
        ("names", 0.47978503),
        ("type", 0.3931242),
    ],
    &[
        ("edge", 0.23037235),
        ("get", 0.06632211),
        ("groups", 0.80298626),
        ("isomorphic", 0.5817795),
        ("number", 0.47725716),
        ("of", 0.47725716),
        ("type", 0.30585226),
    ],
    &[
        ("has", 1.5832652),
        ("isomorphic", 1.9966053),
        ("nodes", 1.736474),
    ],
    &[
        ("from", 0.15501031),
        ("has", 0.3685136),
        ("ids", 0.22377864),
        ("isomorphic", 0.4647208),
        ("node", 0.17118385),
        ("types", 0.30434394),
        ("unchecked", 0.31412047),
    ],
    &[
        ("from", 0.19405593),
        ("has", 0.46133867),
        ("ids", 0.28014633),
        ("isomorphic", 0.5817795),
        ("node", 0.21133932),
        ("types", 0.3810053),
    ],
    &[("csv", 6.0519295), ("from", 1.0269308)],
];

#[pymethods]
impl Graph {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Graph {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = GRAPH_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = GRAPH_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, GRAPH_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", GRAPH_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct GraphBuilder {
    pub inner: graph::GraphBuilder,
}

impl From<graph::GraphBuilder> for GraphBuilder {
    fn from(val: graph::GraphBuilder) -> GraphBuilder {
        GraphBuilder { inner: val }
    }
}

impl From<GraphBuilder> for graph::GraphBuilder {
    fn from(val: GraphBuilder) -> graph::GraphBuilder {
        val.inner
    }
}

impl<'a> From<&'a GraphBuilder> for &'a graph::GraphBuilder {
    fn from(val: &'a GraphBuilder) -> &'a graph::GraphBuilder {
        &val.inner
    }
}

#[pymethods]
impl GraphBuilder {
    #[new]
    #[automatically_generated_binding]

    /// Create a graph NetworkX style.
    ///
    /// This is **NOT** the most efficient way because it will have to duplicate
    /// the memory. The most efficient way to build a graph is to create an
    /// appropriate CSV that can be loaded directly. This building will use MORE
    /// memory than the loaded graph.
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     The name of the graph
    /// directed: bool
    ///     the generated graph will be directed if this is true, by default it's `false`
    ///
    pub fn new(name: Option<String>, directed: Option<bool>) -> Self {
        graph::GraphBuilder::new(name, directed).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, name)")]
    /// Set the name of the graph that will be created
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     The name of the graph
    ///
    pub fn set_name(&mut self, name: &str) {
        self.inner.set_name(name);
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, is_directed)")]
    /// Set if the graph will be directed or undirected
    ///
    /// Parameters
    /// ----------
    /// is_directed: bool
    ///     the generated graph will be directed if this is true
    ///
    pub fn set_directed(&mut self, is_directed: bool) {
        self.inner.set_directed(is_directed.clone());
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, default_weight)")]
    /// Set a default missing weight to be used if only some edges have weights
    ///
    /// Parameters
    /// ----------
    /// default_weight: float
    ///     set the weight to assign by default at edges
    ///
    pub fn set_default_weight(&mut self, default_weight: WeightT) {
        self.inner.set_default_weight(default_weight.clone());
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type, weight)")]
    /// Add an edge to the graph
    ///
    /// Parameters
    /// ----------
    /// src: str
    ///     The name of the source node
    /// dst: str
    ///     The name of the destination node
    /// edge_type: Optional[str]
    ///     The name of the edge_type, if present
    /// weight: Optional[float]
    ///     The weight of the edge, if present
    ///
    pub fn add_edge(
        &mut self,
        src: String,
        dst: String,
        edge_type: Option<String>,
        weight: Option<WeightT>,
    ) -> PyResult<()> {
        Ok(pe!(self.inner.add_edge(
            src.into(),
            dst.into(),
            edge_type,
            weight
        ))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type, weight)")]
    /// Remove an edge to the graph, if the edge is not present this will do nothing.
    ///
    /// Parameters
    /// ----------
    /// src: str
    ///     The name of the source node
    /// dst: str
    ///     The name of the destination node
    /// edge_type: Optional[str]
    ///     The name of the edge_type, if present
    /// weight: Optional[float]
    ///     The weight of the edge, if present
    ///
    pub fn remove_edge(
        &mut self,
        src: String,
        dst: String,
        edge_type: Option<String>,
        weight: Option<WeightT>,
    ) -> PyResult<()> {
        Ok(pe!(self.inner.remove_edge(
            src.into(),
            dst.into(),
            edge_type,
            weight
        ))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, name, node_type)")]
    /// Add a node to the graph, if the node is already present in the graph it will be overwritten
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     The name of the node
    /// node_type: Optional[List[str]]
    ///     List of node type names, if present
    ///
    pub fn add_node(&mut self, name: String, node_type: Option<Vec<String>>) -> PyResult<()> {
        Ok(pe!(self.inner.add_node(name.into(), node_type))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, name)")]
    /// Remove a node from the graph, if the node does not exist, this method does nothing
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     The name of the node
    ///
    pub fn remove_node(&mut self, name: String) -> PyResult<()> {
        Ok(pe!(self.inner.remove_node(name.into()))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Consume the edges and nodes to create a new graph
    pub fn build(&mut self) -> PyResult<Graph> {
        Ok(pe!(self.inner.build())?.into())
    }
}

pub const GRAPHBUILDER_METHODS_NAMES: &[&str] = &[
    "new",
    "set_name",
    "set_directed",
    "set_default_weight",
    "add_edge",
    "remove_edge",
    "add_node",
    "remove_node",
    "build",
];

pub const GRAPHBUILDER_TERMS: &[&str] = &[
    "new", "set", "name", "directed", "default", "weight", "add", "edge", "remove", "node", "build",
];

pub const GRAPHBUILDER_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[("new", 2.406794)],
    &[("name", 1.1477238), ("set", 0.63512367)],
    &[("directed", 1.1477238), ("set", 0.63512367)],
    &[
        ("default", 0.63361573),
        ("set", 0.3506282),
        ("weight", 0.63361573),
    ],
    &[("add", 0.8386834), ("edge", 0.8386834)],
    &[("edge", 0.8386834), ("remove", 0.8386834)],
    &[("add", 0.8386834), ("node", 0.8386834)],
    &[("node", 0.8386834), ("remove", 0.8386834)],
    &[("build", 2.406794)],
];

#[pymethods]
impl GraphBuilder {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl GraphBuilder {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = GRAPHBUILDER_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = GRAPHBUILDER_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, GRAPHBUILDER_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", GRAPHBUILDER_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug)]
pub struct GraphCSVBuilder {
    pub inner: graph::GraphCSVBuilder,
}

impl From<graph::GraphCSVBuilder> for GraphCSVBuilder {
    fn from(val: graph::GraphCSVBuilder) -> GraphCSVBuilder {
        GraphCSVBuilder { inner: val }
    }
}

impl From<GraphCSVBuilder> for graph::GraphCSVBuilder {
    fn from(val: GraphCSVBuilder) -> graph::GraphCSVBuilder {
        val.inner
    }
}

impl<'a> From<&'a GraphCSVBuilder> for &'a graph::GraphCSVBuilder {
    fn from(val: &'a GraphCSVBuilder) -> &'a graph::GraphCSVBuilder {
        &val.inner
    }
}

#[pymethods]
impl GraphCSVBuilder {
    #[new]
    #[automatically_generated_binding]

    /// Write a csv file loadable from GRAPE like a NetworkX graph.
    ///
    /// This is optional, but might help some users.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The name of the graph
    ///
    pub fn new(path: &str) -> PyResult<Self> {
        Ok(pe!(graph::GraphCSVBuilder::new(path))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type, weight)")]
    /// Add an edge to the graph
    ///
    /// Parameters
    /// ----------
    /// src: str
    ///     The name of the source node
    /// dst: str
    ///     The name of the destination node
    /// edge_type: Optional[str]
    ///     The name of the edge_type, if present
    /// weight: Optional[float]
    ///     The weight of the edge, if present
    ///
    pub fn add_edge(
        &mut self,
        src: String,
        dst: String,
        edge_type: Option<String>,
        weight: Option<WeightT>,
    ) -> PyResult<()> {
        Ok(pe!(self.inner.add_edge(
            src.into(),
            dst.into(),
            edge_type,
            weight
        ))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, name, node_type)")]
    /// Add a node to the graph, if the node is already present in the graph it will be overwritten
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     The name of the node
    /// node_type: Optional[List[str]]
    ///     List of node type names, if present
    ///
    pub fn add_node(&mut self, name: String, node_type: Option<Vec<String>>) -> PyResult<()> {
        Ok(pe!(self.inner.add_node(name.into(), node_type))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Flush the changes to the files and print the example code on how the
    /// graph can be loaded using `Graph.from_csv
    pub fn finish(&mut self) -> PyResult<String> {
        Ok(pe!(self.inner.finish())?.into())
    }
}

pub const GRAPHCSVBUILDER_METHODS_NAMES: &[&str] = &["new", "add_edge", "add_node", "finish"];

pub const GRAPHCSVBUILDER_TERMS: &[&str] = &["new", "add", "edge", "node", "finish"];

pub const GRAPHCSVBUILDER_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[("new", 1.4164387)],
    &[("add", 0.3648143), ("edge", 0.6336699)],
    &[("add", 0.3648143), ("node", 0.6336699)],
    &[("finish", 1.4164387)],
];

#[pymethods]
impl GraphCSVBuilder {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl GraphCSVBuilder {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = GRAPHCSVBUILDER_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = GRAPHCSVBUILDER_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, GRAPHCSVBUILDER_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        GRAPHCSVBUILDER_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct NodeTuple {
    pub inner: graph::NodeTuple,
}

impl From<graph::NodeTuple> for NodeTuple {
    fn from(val: graph::NodeTuple) -> NodeTuple {
        NodeTuple { inner: val }
    }
}

impl From<NodeTuple> for graph::NodeTuple {
    fn from(val: NodeTuple) -> graph::NodeTuple {
        val.inner
    }
}

impl<'a> From<&'a NodeTuple> for &'a graph::NodeTuple {
    fn from(val: &'a NodeTuple) -> &'a graph::NodeTuple {
        &val.inner
    }
}

#[pymethods]
impl NodeTuple {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the tuple
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the tuple
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the tuple
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the tuple
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }
}

pub const NODETUPLE_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_node_ids",
];

pub const NODETUPLE_TERMS: &[&str] = &["get", "root", "node", "id", "name", "len", "ids"];

pub const NODETUPLE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.1049044),
        ("id", 0.35410967),
        ("node", 0.1049044),
        ("root", 0.20386682),
    ],
    &[
        ("get", 0.1049044),
        ("name", 0.35410967),
        ("node", 0.1049044),
        ("root", 0.20386682),
    ],
    &[("len", 1.7199612)],
    &[
        ("get", 0.16212498),
        ("ids", 0.5472604),
        ("node", 0.16212498),
    ],
];

#[pymethods]
impl NodeTuple {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl NodeTuple {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = NODETUPLE_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = NODETUPLE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, NODETUPLE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", NODETUPLE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct ShortestPathsDjkstra {
    pub inner: graph::ShortestPathsDjkstra,
}

impl From<graph::ShortestPathsDjkstra> for ShortestPathsDjkstra {
    fn from(val: graph::ShortestPathsDjkstra) -> ShortestPathsDjkstra {
        ShortestPathsDjkstra { inner: val }
    }
}

impl From<ShortestPathsDjkstra> for graph::ShortestPathsDjkstra {
    fn from(val: ShortestPathsDjkstra) -> graph::ShortestPathsDjkstra {
        val.inner
    }
}

impl<'a> From<&'a ShortestPathsDjkstra> for &'a graph::ShortestPathsDjkstra {
    fn from(val: &'a ShortestPathsDjkstra) -> &'a graph::ShortestPathsDjkstra {
        &val.inner
    }
}

#[pymethods]
impl ShortestPathsDjkstra {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn has_path_to_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.has_path_to_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> PyResult<f32> {
        Ok(pe!(self.inner.get_distance_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> PyResult<Option<NodeT>> {
        Ok(pe!(self.inner.get_parent_from_node_id(node_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id, distance)")]
    /// Returns node at just before given distance on minimum path to given destination node.
    ///
    /// Parameters
    /// ----------
    /// dst_node_id: int
    ///     The node to start computing predecessors from.
    /// distance: float
    ///     The distance to aim for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the predecessors vector was not requested.
    ///
    pub fn get_point_at_given_distance_on_shortest_path(
        &self,
        dst_node_id: NodeT,
        distance: f32,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_point_at_given_distance_on_shortest_path(dst_node_id.clone(), distance.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id)")]
    ///
    pub fn get_median_point(&self, dst_node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point(dst_node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_eccentricity(&self) -> f32 {
        self.inner.get_eccentricity().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_total_distance(&self) -> f32 {
        self.inner.get_total_distance().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_log_total_distance(&self) -> f32 {
        self.inner.get_log_total_distance().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_most_distant_node(&self) -> NodeT {
        self.inner.get_most_distant_node().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of shortest paths starting from the root node
    pub fn get_number_of_shortest_paths(&self) -> NodeT {
        self.inner.get_number_of_shortest_paths().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the number of shortest paths passing through the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If neither predecessors nor distances were computed for this BFS.
    /// ValueError
    ///     If the given node ID does not exist in the current graph instance.
    ///
    pub fn get_number_of_shortest_paths_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_shortest_paths_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id)")]
    /// Return list of successors of a given node.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     The node for which to return the successors.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_successors_from_node_id(
        &self,
        source_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_successors_from_node_id(source_node_id.clone()))?,
                NodeT
            )
        })
    }
}

pub const SHORTESTPATHSDJKSTRA_METHODS_NAMES: &[&str] = &[
    "has_path_to_node_id",
    "get_distance_from_node_id",
    "get_parent_from_node_id",
    "get_point_at_given_distance_on_shortest_path",
    "get_median_point",
    "get_eccentricity",
    "get_total_distance",
    "get_log_total_distance",
    "get_most_distant_node",
    "get_number_of_shortest_paths",
    "get_number_of_shortest_paths_from_node_id",
    "get_successors_from_node_id",
];

pub const SHORTESTPATHSDJKSTRA_TERMS: &[&str] = &[
    "has",
    "path",
    "to",
    "node",
    "id",
    "get",
    "distance",
    "from",
    "parent",
    "point",
    "at",
    "given",
    "on",
    "shortest",
    "median",
    "eccentricity",
    "total",
    "log",
    "most",
    "distant",
    "number",
    "of",
    "paths",
    "successors",
];

pub const SHORTESTPATHSDJKSTRA_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("has", 0.61376506),
        ("id", 0.244485),
        ("node", 0.19700517),
        ("path", 0.46857908),
        ("to", 0.61376506),
    ],
    &[
        ("distance", 0.3015193),
        ("from", 0.3015193),
        ("get", 0.034845833),
        ("id", 0.244485),
        ("node", 0.19700517),
    ],
    &[
        ("from", 0.3015193),
        ("get", 0.034845833),
        ("id", 0.244485),
        ("node", 0.19700517),
        ("parent", 0.61376506),
    ],
    &[
        ("at", 0.2818008),
        ("distance", 0.13843796),
        ("get", 0.01599893),
        ("given", 0.2818008),
        ("on", 0.2818008),
        ("path", 0.2151409),
        ("point", 0.2151409),
        ("shortest", 0.1712331),
    ],
    &[
        ("get", 0.07200755),
        ("median", 1.2683215),
        ("point", 0.96830034),
    ],
    &[("eccentricity", 2.0014732), ("get", 0.11363142)],
    &[
        ("distance", 0.62307787),
        ("get", 0.07200755),
        ("total", 0.96830034),
    ],
    &[
        ("distance", 0.42168552),
        ("get", 0.04873314),
        ("log", 0.8583724),
        ("total", 0.6553246),
    ],
    &[
        ("distant", 0.8583724),
        ("get", 0.04873314),
        ("most", 0.8583724),
        ("node", 0.27551875),
    ],
    &[
        ("get", 0.034845833),
        ("number", 0.46857908),
        ("of", 0.46857908),
        ("paths", 0.46857908),
        ("shortest", 0.37294748),
    ],
    &[
        ("from", 0.13843796),
        ("get", 0.01599893),
        ("id", 0.11225154),
        ("node", 0.090451896),
        ("number", 0.2151409),
        ("of", 0.2151409),
        ("paths", 0.2151409),
        ("shortest", 0.1712331),
    ],
    &[
        ("from", 0.3015193),
        ("get", 0.034845833),
        ("id", 0.244485),
        ("node", 0.19700517),
        ("successors", 0.61376506),
    ],
];

#[pymethods]
impl ShortestPathsDjkstra {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl ShortestPathsDjkstra {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = SHORTESTPATHSDJKSTRA_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = SHORTESTPATHSDJKSTRA_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, SHORTESTPATHSDJKSTRA_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        SHORTESTPATHSDJKSTRA_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct ShortestPathsResultBFS {
    pub inner: graph::ShortestPathsResultBFS,
}

impl From<graph::ShortestPathsResultBFS> for ShortestPathsResultBFS {
    fn from(val: graph::ShortestPathsResultBFS) -> ShortestPathsResultBFS {
        ShortestPathsResultBFS { inner: val }
    }
}

impl From<ShortestPathsResultBFS> for graph::ShortestPathsResultBFS {
    fn from(val: ShortestPathsResultBFS) -> graph::ShortestPathsResultBFS {
        val.inner
    }
}

impl<'a> From<&'a ShortestPathsResultBFS> for &'a graph::ShortestPathsResultBFS {
    fn from(val: &'a ShortestPathsResultBFS) -> &'a graph::ShortestPathsResultBFS {
        &val.inner
    }
}

#[pymethods]
impl ShortestPathsResultBFS {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn has_path_to_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.has_path_to_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_distance_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_parent_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id, k)")]
    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// Parameters
    /// ----------
    /// dst_node_id: int
    ///     The node to start computing predecessors from.
    /// k: int
    ///     Steps to go back.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the predecessors vector was not requested.
    ///
    pub unsafe fn get_unchecked_kth_point_on_shortest_path(
        &self,
        dst_node_id: NodeT,
        k: NodeT,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_unchecked_kth_point_on_shortest_path(dst_node_id.clone(), k.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id, k)")]
    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// Parameters
    /// ----------
    /// dst_node_id: int
    ///     The node to start computing predecessors from.
    /// k: int
    ///     Steps to go back.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the predecessors vector was not requested.
    ///
    pub fn get_kth_point_on_shortest_path(&self, dst_node_id: NodeT, k: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_kth_point_on_shortest_path(dst_node_id.clone(), k.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id)")]
    ///
    pub fn get_median_point(&self, dst_node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point(dst_node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_median_point_to_most_distant_node(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point_to_most_distant_node())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_eccentricity(&self) -> NodeT {
        self.inner.get_eccentricity().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_most_distant_node(&self) -> NodeT {
        self.inner.get_most_distant_node().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of shortest paths starting from the root node.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If neither predecessors nor distances were computed for this BFS.
    ///
    pub fn get_number_of_shortest_paths(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_shortest_paths())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the number of shortest paths passing through the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If neither predecessors nor distances were computed for this BFS.
    /// ValueError
    ///     If the given node ID does not exist in the current graph instance.
    ///
    pub fn get_number_of_shortest_paths_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_shortest_paths_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id)")]
    /// Return list of successors of a given node.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     The node for which to return the successors.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_successors_from_node_id(
        &self,
        source_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_successors_from_node_id(source_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id)")]
    /// Return list of predecessors of a given node.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     The node for which to return the predecessors.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_predecessors_from_node_id(
        &self,
        source_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_predecessors_from_node_id(source_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_id, second_node_id)")]
    /// Return Shared Ancestors number.
    ///
    /// Parameters
    /// ----------
    /// first_node_id: int
    ///     The first node for which to compute the predecessors Jaccard index.
    /// second_node_id: int
    ///     The second node for which to compute the predecessors Jaccard index.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs do not exist in the graph.
    ///
    pub fn get_shared_ancestors_size(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_shared_ancestors_size(first_node_id.clone(), second_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_id, second_node_id)")]
    /// Return Ancestors Jaccard Index.
    ///
    /// Parameters
    /// ----------
    /// first_node_id: int
    ///     The first node for which to compute the predecessors Jaccard index.
    /// second_node_id: int
    ///     The second node for which to compute the predecessors Jaccard index.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs do not exist in the graph.
    ///
    pub fn get_ancestors_jaccard_index(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_ancestors_jaccard_index(first_node_id.clone(), second_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_distances(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_distances())?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_predecessors(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_predecessors())?, NodeT)
        })
    }
}

pub const SHORTESTPATHSRESULTBFS_METHODS_NAMES: &[&str] = &[
    "has_path_to_node_id",
    "get_distance_from_node_id",
    "get_parent_from_node_id",
    "get_unchecked_kth_point_on_shortest_path",
    "get_kth_point_on_shortest_path",
    "get_median_point",
    "get_median_point_to_most_distant_node",
    "get_eccentricity",
    "get_most_distant_node",
    "get_number_of_shortest_paths",
    "get_number_of_shortest_paths_from_node_id",
    "get_successors_from_node_id",
    "get_predecessors_from_node_id",
    "get_shared_ancestors_size",
    "get_ancestors_jaccard_index",
    "get_distances",
    "get_predecessors",
];

pub const SHORTESTPATHSRESULTBFS_TERMS: &[&str] = &[
    "has",
    "path",
    "to",
    "node",
    "id",
    "get",
    "distance",
    "from",
    "parent",
    "unchecked",
    "kth",
    "point",
    "on",
    "shortest",
    "median",
    "most",
    "distant",
    "eccentricity",
    "number",
    "of",
    "paths",
    "successors",
    "predecessors",
    "shared",
    "ancestors",
    "size",
    "jaccard",
    "index",
    "distances",
];

pub const SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("has", 0.6958796),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("path", 0.4586001),
        ("to", 0.55282664),
    ],
    &[
        ("distance", 0.6958796),
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
    ],
    &[
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("parent", 0.6958796),
    ],
    &[
        ("get", 0.014045565),
        ("kth", 0.31866044),
        ("on", 0.31866044),
        ("path", 0.26434636),
        ("point", 0.22377864),
        ("shortest", 0.22377864),
        ("unchecked", 0.40111902),
    ],
    &[
        ("get", 0.01818011),
        ("kth", 0.4124634),
        ("on", 0.4124634),
        ("path", 0.34216106),
        ("point", 0.28965157),
        ("shortest", 0.28965157),
    ],
    &[
        ("get", 0.05054337),
        ("median", 1.1467087),
        ("point", 0.8052739),
    ],
    &[
        ("distant", 0.31866044),
        ("get", 0.014045565),
        ("median", 0.31866044),
        ("most", 0.31866044),
        ("node", 0.12111595),
        ("point", 0.22377864),
        ("to", 0.31866044),
    ],
    &[("eccentricity", 2.2853043), ("get", 0.08002211)],
    &[
        ("distant", 0.7743416),
        ("get", 0.03413058),
        ("most", 0.7743416),
        ("node", 0.2943105),
    ],
    &[
        ("get", 0.024366887),
        ("number", 0.55282664),
        ("of", 0.55282664),
        ("paths", 0.55282664),
        ("shortest", 0.38822138),
    ],
    &[
        ("from", 0.15205239),
        ("get", 0.011158927),
        ("id", 0.13062824),
        ("node", 0.096224256),
        ("number", 0.25316948),
        ("of", 0.25316948),
        ("paths", 0.25316948),
        ("shortest", 0.17778775),
    ],
    &[
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("successors", 0.6958796),
    ],
    &[
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("predecessors", 0.55282664),
    ],
    &[
        ("ancestors", 0.7743416),
        ("get", 0.03413058),
        ("shared", 0.9747151),
        ("size", 0.9747151),
    ],
    &[
        ("ancestors", 0.7743416),
        ("get", 0.03413058),
        ("index", 0.9747151),
        ("jaccard", 0.9747151),
    ],
    &[("distances", 2.2853043), ("get", 0.08002211)],
    &[("get", 0.08002211), ("predecessors", 1.8155112)],
];

#[pymethods]
impl ShortestPathsResultBFS {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl ShortestPathsResultBFS {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = SHORTESTPATHSRESULTBFS_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, SHORTESTPATHSRESULTBFS_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        SHORTESTPATHSRESULTBFS_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct Star {
    pub inner: graph::Star,
}

impl From<graph::Star> for Star {
    fn from(val: graph::Star) -> Star {
        Star { inner: val }
    }
}

impl From<Star> for graph::Star {
    fn from(val: Star) -> graph::Star {
        val.inner
    }
}

impl<'a> From<&'a Star> for &'a graph::Star {
    fn from(val: &'a Star) -> &'a graph::Star {
        &val.inner
    }
}

#[pymethods]
impl Star {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the central node ID of the Star
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the central node name of the star
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Star
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the Star
    pub fn get_star_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_star_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the star.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_star_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_star_node_ids(k.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the star.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_star_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_star_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node names of the nodes composing the star
    pub fn get_star_node_names(&self) -> Vec<String> {
        self.inner
            .get_star_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const STAR_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_star_node_ids",
    "get_first_k_star_node_ids",
    "get_first_k_star_node_names",
    "get_star_node_names",
];

pub const STAR_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "star", "ids", "first", "k", "names",
];

pub const STAR_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
        ("star", 0.21014561),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
        ("star", 0.110427275),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
        ("star", 0.110427275),
    ],
    &[
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
        ("star", 0.21014561),
    ],
];

#[pymethods]
impl Star {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Star {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = STAR_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = STAR_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, STAR_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", STAR_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
#[derive(Debug, Clone)]
pub struct Tendril {
    pub inner: graph::Tendril,
}

impl From<graph::Tendril> for Tendril {
    fn from(val: graph::Tendril) -> Tendril {
        Tendril { inner: val }
    }
}

impl From<Tendril> for graph::Tendril {
    fn from(val: Tendril) -> graph::Tendril {
        val.inner
    }
}

impl<'a> From<&'a Tendril> for &'a graph::Tendril {
    fn from(val: &'a Tendril) -> &'a graph::Tendril {
        &val.inner
    }
}

#[pymethods]
impl Tendril {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the Tendril
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the Tendril
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Tendril
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the Tendril
    pub fn get_tendril_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_tendril_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the Tendril.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_tendril_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_tendril_node_ids(k.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the Tendril.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_tendril_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_tendril_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node names of the nodes composing the Tendril
    pub fn get_tendril_node_names(&self) -> Vec<String> {
        self.inner
            .get_tendril_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const TENDRIL_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_tendril_node_ids",
    "get_first_k_tendril_node_ids",
    "get_first_k_tendril_node_names",
    "get_tendril_node_names",
];

pub const TENDRIL_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "tendril", "ids", "first", "k", "names",
];

pub const TENDRIL_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
        ("tendril", 0.21014561),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
        ("tendril", 0.110427275),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
        ("tendril", 0.110427275),
    ],
    &[
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
        ("tendril", 0.21014561),
    ],
];

#[pymethods]
impl Tendril {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Tendril {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = TENDRIL_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = TENDRIL_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, TENDRIL_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", TENDRIL_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

pub fn register_edge_list_utils(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_wrapped(wrap_pyfunction!(convert_edge_list_to_numeric))?;
    _m.add_wrapped(wrap_pyfunction!(densify_sparse_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(are_there_selfloops_in_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(get_rows_number))?;
    _m.add_wrapped(wrap_pyfunction!(convert_directed_edge_list_to_undirected))?;
    _m.add_wrapped(wrap_pyfunction!(add_numeric_id_to_csv))?;
    _m.add_wrapped(wrap_pyfunction!(build_optimal_lists_files))?;
    _m.add_wrapped(wrap_pyfunction!(filter_duplicates_from_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(convert_undirected_edge_list_to_directed))?;
    _m.add_wrapped(wrap_pyfunction!(get_minmax_node_from_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(parse_wikipedia_graph))?;
    _m.add_wrapped(wrap_pyfunction!(is_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(convert_node_list_node_types_to_numeric))?;
    _m.add_wrapped(wrap_pyfunction!(has_duplicated_edges_in_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list_inplace))?;
    _m.add_wrapped(wrap_pyfunction!(get_number_of_selfloops_from_edge_list))?;
    Ok(())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, directed, original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, original_nodes_column_number, original_nodes_column, number_of_nodes, original_minimum_node_id, original_numeric_node_ids, original_load_node_list_in_parallel, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, number_of_edge_types, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, remove_chevrons, remove_spaces, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, number_of_edges, skip_edge_types_if_unavailable, skip_weights_if_unavailable, numeric_rows_are_surely_smaller_than_original, verbose, name)"
)]
/// Create a new edge list starting from given one with node IDs densified.
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn convert_edge_list_to_numeric(
    original_edge_path: &str,
    target_edge_path: &str,
    directed: bool,
    original_node_path: Option<String>,
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    number_of_nodes: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,
    original_edge_type_path: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    number_of_edge_types: Option<EdgeTypeT>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_separator: Option<char>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_types_column: Option<String>,
    target_edge_list_edge_types_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    target_node_path: Option<&str>,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_header: Option<bool>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::convert_edge_list_to_numeric(
            original_edge_path,
            target_edge_path,
            directed.clone(),
            original_node_path,
            original_node_list_separator,
            original_node_list_header,
            original_node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol,
            original_nodes_column_number,
            original_nodes_column,
            number_of_nodes,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_load_node_list_in_parallel,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            number_of_edge_types,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            original_edge_type_list_support_balanced_quotes,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            original_edge_list_separator,
            original_edge_list_header,
            original_edge_list_support_balanced_quotes,
            original_sources_column_number,
            original_sources_column,
            original_destinations_column_number,
            original_destinations_column,
            original_edge_list_edge_types_column,
            original_edge_list_edge_types_column_number,
            original_weights_column,
            original_weights_column_number,
            target_edge_list_separator,
            target_edge_list_header,
            target_sources_column,
            target_sources_column_number,
            target_destinations_column,
            target_destinations_column_number,
            target_edge_list_edge_types_column,
            target_edge_list_edge_types_column_number,
            target_weights_column,
            target_weights_column_number,
            target_node_path,
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator,
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            remove_chevrons,
            remove_spaces,
            comment_symbol,
            default_edge_type,
            default_weight,
            max_rows_number,
            rows_to_skip,
            number_of_edges,
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            numeric_rows_are_surely_smaller_than_original,
            verbose,
            name
        ))?
        .into();
        (subresult_0.into(), subresult_1.map(|x| x.into()))
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, directed, maximum_node_id, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, original_edge_list_support_balanced_quotes, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, number_of_edge_types, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, number_of_edges, skip_edge_types_if_unavailable, skip_weights_if_unavailable, numeric_rows_are_surely_smaller_than_original, verbose, name)"
)]
/// Create a new edge list starting from given numeric one with node IDs densified and returns the number of unique nodes.
///
/// This method is meant as a solution to parse very large sparse numeric graphs,
/// like for instance ClueWeb.
///
/// Safety
/// ------
/// This method will panic if the node IDs are not numeric.
///  TODO: In the future we may handle this case as a normal error.
///
/// Parameters
/// ----------
/// maximum_node_id: Optional[int]
///     The maximum node ID present in this graph. If available, optimal memory allocation will be used.
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_types_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_types_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header.
/// target_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list.
/// target_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list.
/// target_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list.
/// target_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_types_column: Optional[str]
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_types_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list.
/// target_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list.
/// target_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// number_of_edges: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn densify_sparse_numeric_edge_list(
    original_edge_path: &str,
    target_edge_path: &str,
    directed: bool,
    maximum_node_id: Option<EdgeT>,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_edge_type_path: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    number_of_edge_types: Option<EdgeTypeT>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_separator: Option<char>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_types_column: Option<String>,
    target_edge_list_edge_types_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    target_node_path: Option<&str>,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_header: Option<bool>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::densify_sparse_numeric_edge_list(
            original_edge_path,
            target_edge_path,
            directed.clone(),
            maximum_node_id,
            original_edge_list_separator,
            original_edge_list_header,
            original_sources_column,
            original_sources_column_number,
            original_destinations_column,
            original_destinations_column_number,
            original_edge_list_edge_types_column,
            original_edge_list_edge_types_column_number,
            original_weights_column,
            original_weights_column_number,
            original_edge_list_support_balanced_quotes,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            number_of_edge_types,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            original_edge_type_list_support_balanced_quotes,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            target_edge_list_separator,
            target_edge_list_header,
            target_sources_column,
            target_sources_column_number,
            target_destinations_column,
            target_destinations_column_number,
            target_edge_list_edge_types_column,
            target_edge_list_edge_types_column_number,
            target_weights_column,
            target_weights_column_number,
            target_node_path,
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator,
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            comment_symbol,
            default_edge_type,
            default_weight,
            max_rows_number,
            rows_to_skip,
            number_of_edges,
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            numeric_rows_are_surely_smaller_than_original,
            verbose,
            name
        ))?
        .into();
        (subresult_0.into(), subresult_1.map(|x| x.into()))
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, support_balanced_quotes, max_rows_number, rows_to_skip, number_of_edges, load_edge_list_in_parallel, verbose, name)"
)]
/// Return whether there are selfloops in the edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// number_of_edges: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn are_there_selfloops_in_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    support_balanced_quotes: Option<bool>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    Ok(pe!(graph::are_there_selfloops_in_edge_list(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        support_balanced_quotes,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        load_edge_list_in_parallel,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(text_signature = "(file_path)")]
/// Return number of rows in given CSV path.
///
/// Parameters
/// ----------
/// file_path: str
///     The path from where to load the original CSV.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the file.
///
pub fn get_rows_number(file_path: &str) -> PyResult<usize> {
    Ok(pe!(graph::get_rows_number(file_path))?.into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_list_separator, target_edge_list_header, target_sources_column_number, target_sources_column, target_destinations_column_number, target_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, number_of_edges, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
/// Create a new undirected edge list from a given directed one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the edge list.
/// original_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// number_of_edges: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn convert_directed_edge_list_to_undirected(
    original_edge_path: &str,
    target_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column_number: Option<usize>,
    target_sources_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    Ok(pe!(graph::convert_directed_edge_list_to_undirected(
        original_edge_path,
        target_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_support_balanced_quotes,
        original_sources_column,
        original_sources_column_number,
        original_destinations_column,
        original_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_weights_column,
        original_weights_column_number,
        target_edge_list_separator,
        target_edge_list_header,
        target_sources_column_number,
        target_sources_column,
        target_destinations_column_number,
        target_destinations_column,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_weights_column,
        target_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_csv_path, target_csv_path, original_csv_separator, original_csv_header, target_csv_separator, target_csv_header, target_csv_ids_column, target_csv_ids_column_number, comment_symbol, support_balanced_quotes, max_rows_number, rows_to_skip, lines_number, verbose)"
)]
/// Create a new CSV with the lines number added to it.
///
/// Parameters
/// ----------
/// original_csv_path: str
///     The path from where to load the original CSV.
/// original_csv_separator: Optional[str]
///     Separator to use for the original CSV.
/// original_csv_header: Optional[bool]
///     Whether the original CSV has an header.
/// target_csv_path: str
///     The path from where to load the target CSV. This cannot be the same as the original CSV.
/// target_csv_separator: Optional[str]
///     Separator to use for the target CSV. If None, the one provided from the original CSV will be used.
/// target_csv_header: Optional[bool]
///     Whether the target CSV has an header. If None, the one provided from the original CSV will be used.
/// target_csv_ids_column: Optional[str]
///     The column name to use for the ids in the target list.
/// target_csv_ids_column_number: Optional[int]
///     The column number to use for the ids in the target list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original CSV.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original CSV.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original CSV.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn add_numeric_id_to_csv(
    original_csv_path: &str,
    target_csv_path: &str,
    original_csv_separator: Option<char>,
    original_csv_header: Option<bool>,
    target_csv_separator: Option<char>,
    target_csv_header: Option<bool>,
    target_csv_ids_column: Option<String>,
    target_csv_ids_column_number: Option<usize>,
    comment_symbol: Option<String>,
    support_balanced_quotes: Option<bool>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    lines_number: Option<usize>,
    verbose: Option<bool>,
) -> PyResult<usize> {
    Ok(pe!(graph::add_numeric_id_to_csv(
        original_csv_path,
        target_csv_path,
        original_csv_separator,
        original_csv_header,
        target_csv_separator,
        target_csv_header,
        target_csv_ids_column,
        target_csv_ids_column_number,
        comment_symbol,
        support_balanced_quotes,
        max_rows_number,
        rows_to_skip,
        lines_number,
        verbose
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, directed, original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_support_balanced_quotes, original_node_type_list_rows_to_skip, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, original_node_type_list_is_correct, number_of_node_types, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_node_types_column_number, target_node_type_list_node_types_column, target_node_type_list_header, original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, number_of_nodes, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, original_load_node_list_in_parallel, maximum_node_id, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_node_types_separator, target_node_list_node_types_column, target_node_list_node_types_column_number, original_edge_type_path, original_edge_type_list_separator, original_edge_types_column_number, original_edge_types_column, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_type_list_is_correct, number_of_edge_types, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_edge_types_column_number, target_edge_type_list_edge_types_column, target_edge_type_list_header, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column_number, original_edge_list_edge_types_column, default_edge_type, original_weights_column_number, original_weights_column, default_weight, original_edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_comment_symbol, edge_list_max_rows_number, edge_list_rows_to_skip, load_edge_list_in_parallel, number_of_edges, target_edge_list_separator, remove_chevrons, remove_spaces, numeric_rows_are_surely_smaller_than_original, sort_temporary_directory, verbose, name)"
)]
/// TODO: write the docstrin
pub fn build_optimal_lists_files(
    original_edge_path: String,
    target_edge_path: String,
    directed: bool,
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<char>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_support_balanced_quotes: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    original_node_type_list_is_correct: Option<bool>,
    number_of_node_types: Option<NodeTypeT>,
    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<char>,
    target_node_type_list_node_types_column_number: Option<usize>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_header: Option<bool>,
    original_node_path: Option<String>,
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<char>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    number_of_nodes: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,
    maximum_node_id: Option<EdgeT>,
    target_node_path: Option<String>,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_node_types_separator: Option<char>,
    target_node_list_node_types_column: Option<String>,
    target_node_list_node_types_column_number: Option<usize>,
    original_edge_type_path: Option<String>,
    original_edge_type_list_separator: Option<char>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    edge_type_list_is_correct: Option<bool>,
    number_of_edge_types: Option<EdgeTypeT>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_header: Option<bool>,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    default_edge_type: Option<String>,
    original_weights_column_number: Option<usize>,
    original_weights_column: Option<String>,
    default_weight: Option<WeightT>,
    original_edge_list_numeric_node_ids: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    edge_list_comment_symbol: Option<String>,
    edge_list_max_rows_number: Option<usize>,
    edge_list_rows_to_skip: Option<usize>,
    load_edge_list_in_parallel: Option<bool>,
    number_of_edges: Option<EdgeT>,
    target_edge_list_separator: Option<char>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    sort_temporary_directory: Option<String>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(Option<NodeTypeT>, NodeT, Option<EdgeTypeT>, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2, subresult_3) =
            pe!(graph::build_optimal_lists_files(
                original_edge_path.into(),
                target_edge_path.into(),
                directed.clone(),
                original_node_type_path,
                original_node_type_list_separator,
                original_node_types_column_number,
                original_node_types_column,
                original_numeric_node_type_ids,
                original_minimum_node_type_id,
                original_node_type_list_header,
                original_node_type_list_support_balanced_quotes,
                original_node_type_list_rows_to_skip,
                original_node_type_list_max_rows_number,
                original_node_type_list_comment_symbol,
                original_load_node_type_list_in_parallel,
                original_node_type_list_is_correct,
                number_of_node_types,
                target_node_type_list_path,
                target_node_type_list_separator,
                target_node_type_list_node_types_column_number,
                target_node_type_list_node_types_column,
                target_node_type_list_header,
                original_node_path,
                original_node_list_separator,
                original_node_list_header,
                original_node_list_support_balanced_quotes,
                node_list_rows_to_skip,
                node_list_is_correct,
                node_list_max_rows_number,
                node_list_comment_symbol,
                default_node_type,
                original_nodes_column_number,
                original_nodes_column,
                original_node_types_separator,
                original_node_list_node_types_column_number,
                original_node_list_node_types_column,
                number_of_nodes,
                original_minimum_node_id,
                original_numeric_node_ids,
                original_node_list_numeric_node_type_ids,
                original_skip_node_types_if_unavailable,
                original_load_node_list_in_parallel,
                maximum_node_id,
                target_node_path,
                target_node_list_separator,
                target_node_list_header,
                target_nodes_column,
                target_nodes_column_number,
                target_node_types_separator,
                target_node_list_node_types_column,
                target_node_list_node_types_column_number,
                original_edge_type_path,
                original_edge_type_list_separator,
                original_edge_types_column_number,
                original_edge_types_column,
                original_numeric_edge_type_ids,
                original_minimum_edge_type_id,
                original_edge_type_list_header,
                original_edge_type_list_support_balanced_quotes,
                edge_type_list_rows_to_skip,
                edge_type_list_max_rows_number,
                edge_type_list_comment_symbol,
                load_edge_type_list_in_parallel,
                edge_type_list_is_correct,
                number_of_edge_types,
                target_edge_type_list_path,
                target_edge_type_list_separator,
                target_edge_type_list_edge_types_column_number,
                target_edge_type_list_edge_types_column,
                target_edge_type_list_header,
                original_edge_list_separator,
                original_edge_list_header,
                original_edge_list_support_balanced_quotes,
                original_sources_column_number,
                original_sources_column,
                original_destinations_column_number,
                original_destinations_column,
                original_edge_list_edge_types_column_number,
                original_edge_list_edge_types_column,
                default_edge_type,
                original_weights_column_number,
                original_weights_column,
                default_weight,
                original_edge_list_numeric_node_ids,
                skip_weights_if_unavailable,
                skip_edge_types_if_unavailable,
                edge_list_comment_symbol,
                edge_list_max_rows_number,
                edge_list_rows_to_skip,
                load_edge_list_in_parallel,
                number_of_edges,
                target_edge_list_separator,
                remove_chevrons,
                remove_spaces,
                numeric_rows_are_surely_smaller_than_original,
                sort_temporary_directory,
                verbose,
                name
            ))?
            .into();
        (
            subresult_0.map(|x| x.into()),
            subresult_1.into(),
            subresult_2.map(|x| x.into()),
            subresult_3.into(),
        )
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_edge_list_sources_column, original_edge_list_sources_column_number, original_edge_list_destinations_column, original_edge_list_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_edge_list_weights_column, original_edge_list_weights_column_number, target_edge_list_separator, target_edge_list_header, target_edge_list_sources_column_number, target_edge_list_sources_column, target_edge_list_destinations_column_number, target_edge_list_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_edge_list_weights_column, target_edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, number_of_edges, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
/// Create a new edge list from a given one filtering duplicates.
///
/// Parameters
/// ----------
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// original_edge_list_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_edge_list_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_edge_list_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_edge_list_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_edge_list_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_edge_list_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header.
/// target_edge_list_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list.
/// target_edge_list_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list.
/// target_edge_list_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list.
/// target_edge_list_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list.
/// target_edge_list_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list.
/// target_edge_list_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// number_of_edges: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn filter_duplicates_from_edge_list(
    original_edge_path: &str,
    target_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_edge_list_weights_column: Option<String>,
    original_edge_list_weights_column_number: Option<usize>,
    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_edge_list_sources_column_number: Option<usize>,
    target_edge_list_sources_column: Option<String>,
    target_edge_list_destinations_column_number: Option<usize>,
    target_edge_list_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_edge_list_weights_column: Option<String>,
    target_edge_list_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<()> {
    Ok(pe!(graph::filter_duplicates_from_edge_list(
        original_edge_path,
        target_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_support_balanced_quotes,
        original_edge_list_sources_column,
        original_edge_list_sources_column_number,
        original_edge_list_destinations_column,
        original_edge_list_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_edge_list_weights_column,
        original_edge_list_weights_column_number,
        target_edge_list_separator,
        target_edge_list_header,
        target_edge_list_sources_column_number,
        target_edge_list_sources_column,
        target_edge_list_destinations_column_number,
        target_edge_list_destinations_column,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_edge_list_weights_column,
        target_edge_list_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?)
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, number_of_edges, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
/// Create a new directed edge list from a given undirected one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the edge list.
/// original_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// number_of_edges: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn convert_undirected_edge_list_to_directed(
    original_edge_path: &str,
    target_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    Ok(pe!(graph::convert_undirected_edge_list_to_directed(
        original_edge_path,
        target_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_support_balanced_quotes,
        original_sources_column,
        original_sources_column_number,
        original_destinations_column,
        original_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_weights_column,
        original_weights_column_number,
        target_edge_list_separator,
        target_edge_list_header,
        target_sources_column,
        target_sources_column_number,
        target_destinations_column,
        target_destinations_column_number,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_weights_column,
        target_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, number_of_edges, load_edge_list_in_parallel, remove_chevrons, remove_spaces, verbose, name)"
)]
/// Return minimum and maximum node number from given numeric edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// number_of_edges: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// remove_chevrons: Optional[bool]
///     Whether remove chevrons while reading elements.
/// remove_spaces: Optional[bool]
///     Whether remove spaces while reading elements.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with the edge list file.
/// ValueError
///     If the elements in the edge list are not numeric.
/// ValueError
///     If the edge list is empty.
///
pub fn get_minmax_node_from_numeric_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    support_balanced_quotes: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(EdgeT, EdgeT, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2) =
            pe!(graph::get_minmax_node_from_numeric_edge_list(
                path,
                separator,
                header,
                support_balanced_quotes,
                sources_column,
                sources_column_number,
                destinations_column,
                destinations_column_number,
                comment_symbol,
                max_rows_number,
                rows_to_skip,
                number_of_edges,
                load_edge_list_in_parallel,
                remove_chevrons,
                remove_spaces,
                verbose,
                name
            ))?
            .into();
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(source_path, edge_path, node_path, node_type_path, edge_type_path, node_list_separator, node_type_list_separator, edge_type_list_separator, node_types_separator, nodes_column, node_types_column, node_list_node_types_column, edge_types_column, node_descriptions_column, edge_list_separator, directed, sort_temporary_directory, compute_node_description, keep_nodes_without_descriptions, keep_nodes_without_categories, keep_interwikipedia_nodes, keep_external_nodes, verbose)"
)]
/// TODO: write the docstrin
pub fn parse_wikipedia_graph(
    source_path: &str,
    edge_path: &str,
    node_path: &str,
    node_type_path: &str,
    edge_type_path: &str,
    node_list_separator: char,
    node_type_list_separator: char,
    edge_type_list_separator: char,
    node_types_separator: &str,
    nodes_column: &str,
    node_types_column: &str,
    node_list_node_types_column: &str,
    edge_types_column: &str,
    node_descriptions_column: &str,
    edge_list_separator: char,
    directed: bool,
    sort_temporary_directory: Option<String>,
    compute_node_description: Option<bool>,
    keep_nodes_without_descriptions: Option<bool>,
    keep_nodes_without_categories: Option<bool>,
    keep_interwikipedia_nodes: Option<bool>,
    keep_external_nodes: Option<bool>,
    verbose: Option<bool>,
) -> PyResult<(NodeTypeT, NodeT, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2) = pe!(graph::parse_wikipedia_graph(
            source_path,
            edge_path,
            node_path,
            node_type_path,
            edge_type_path,
            node_list_separator.clone(),
            node_type_list_separator.clone(),
            edge_type_list_separator.clone(),
            node_types_separator,
            nodes_column,
            node_types_column,
            node_list_node_types_column,
            edge_types_column,
            node_descriptions_column,
            edge_list_separator.clone(),
            directed.clone(),
            sort_temporary_directory,
            compute_node_description,
            keep_nodes_without_descriptions,
            keep_nodes_without_categories,
            keep_interwikipedia_nodes,
            keep_external_nodes,
            verbose
        ))?
        .into();
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, number_of_edges, load_edge_list_in_parallel, remove_chevrons, remove_spaces, verbose, name)"
)]
/// Return whether the given edge list is numeric.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// number_of_edges: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// remove_chevrons: Optional[bool]
///     Whether remove chevrons while reading elements.
/// remove_spaces: Optional[bool]
///     Whether remove spaces while reading elements.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn is_numeric_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    support_balanced_quotes: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    Ok(pe!(graph::is_numeric_edge_list(
        path,
        separator,
        header,
        support_balanced_quotes,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        load_edge_list_in_parallel,
        remove_chevrons,
        remove_spaces,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_node_path, target_node_path, original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, number_of_node_types, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_support_balanced_quotes, original_node_type_list_rows_to_skip, original_node_type_list_is_correct, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_header, target_node_type_list_node_types_column, target_node_type_list_node_types_column_number, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, remove_chevrons, remove_spaces, target_node_list_separator, target_node_list_header, target_nodes_column_number, target_nodes_column, target_node_types_separator, target_node_list_node_types_column_number, target_node_list_node_types_column, number_of_nodes)"
)]
/// Converts the node list at given path to numeric saving in stream to file. Furthermore, returns the number of nodes that were written and their node types if any.
///
/// Parameters
/// ----------
/// original_node_type_path: Optional[str]
///     Path to the original list of node types.
/// original_node_type_list_separator: Optional[str]
///     Separator to be used for the original node type list.
/// original_node_types_column_number: Optional[int]
///     Number of the node types column to be used for the original node types list.
/// original_node_types_column: Optional[str]
///     Name of the node types column to be used for the original node types list.
/// number_of_node_types: Optional[int]
///     Number of node types present in the provided original list of node types. If provided, it will allow to make assumptions and to load the node types faster.
/// original_numeric_node_type_ids: Optional[bool]
///     Whether to load the node types as numeric.
/// original_minimum_node_type_id: Optional[int]
///     The minimum numeric node type ID. If provided, it will allow for additional assumptions in the creation of the node types vocabulary.
/// original_node_type_list_header: Optional[bool]
///     Whether the provided node type list has a header.
/// original_node_type_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the node type list.
/// original_node_type_list_rows_to_skip: Optional[int]
///     Number of rows to skip before starting to parse the file for the provided node type list file.
/// original_node_type_list_is_correct: Optional[bool]
///     Whether is it safe to assume that the provided node type list is correct.
/// original_node_type_list_max_rows_number: Optional[int]
///     Maximum number of rows to be read in the provided original node type list.
/// original_node_type_list_comment_symbol: Optional[str]
///     Symbol to be used to skip rows starting with it.
/// original_load_node_type_list_in_parallel: Optional[bool]
///     Whether to load the node type list in parallel.
/// target_node_type_list_path: Optional[str]
///     Path where to store the parsed node types (if any).
/// target_node_type_list_separator: Optional[str]
///     Separator to be used for the target node type list.
/// target_node_type_list_header: Optional[bool]
///     Whether to add header when writing the target node type list.
/// target_node_type_list_node_types_column: Optional[str]
///     Name of the column of the node types in the target node type list.
/// target_node_type_list_node_types_column_number: Optional[int]
///     Number of the column of the node types in the target node type list.
/// original_node_path: str
///     Path to the original list of nodes.
/// original_node_list_separator: Optional[str]
///     Separator to be used for rows of the original node list.
/// original_node_list_header: Optional[bool]
///     Whether to expect a header in the original node list file.
/// original_node_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the node list.
/// node_list_rows_to_skip: Optional[int]
///     Number of rows to skip before starting to parse the original node list.
/// node_list_max_rows_number: Optional[int]
///     Maximum number of rows to read from the origina node list.
/// node_list_comment_symbol: Optional[str]
///     Symbol to use to skip rows starting with it in the original node list.
/// default_node_type: Optional[str]
///     Default node type to be used when none are provided or are missing for some nodes.
/// original_nodes_column_number: Optional[int]
///     Number of the column for the node name in the original node list.
/// original_nodes_column: Optional[str]
///     Name of the column for the node name in the original node list.
/// original_node_types_separator: Optional[str]
///     Separator to be used for the node types within the original node list.
/// original_node_list_node_types_column_number: Optional[int]
///     Number of the column for the node types in the original node list.
/// original_node_list_node_types_column: Optional[str]
///     Name of the column for the node types in the original node list.
/// original_minimum_node_id: Optional[int]
///     The minimum numeric node ID. If provided, it will allow for additional assumptions in the creation of the nodes vocabulary.
/// original_numeric_node_ids: Optional[bool]
///     Whether to load the node names as numeric.
/// original_node_list_numeric_node_type_ids: Optional[bool]
///     Whether to load the node type names from the original node list as numeric.
/// original_skip_node_types_if_unavailable: Optional[bool]
///     Whether to skip the node types if the provided node types column is not provided.
/// remove_chevrons: Optional[bool]
///     Whether remove chevrons while reading elements.
/// remove_spaces: Optional[bool]
///     Whether remove spaces while reading elements.
/// target_node_path: str
///     Path where to store the target node paths.
/// target_node_list_separator: Optional[str]
///     Separator to be used for the target node list.
/// target_node_list_header: Optional[bool]
///     Whether to add an header to the target node list.
/// target_nodes_column_number: Optional[int]
///     Number of the column where to store the node names.
/// target_nodes_column: Optional[str]
///     Name of the column where to store the node names.
/// target_node_types_separator: Optional[str]
///     Separator to be used for the node types within the target node list.
/// target_node_list_node_types_column_number: Optional[int]
///     Number for the column with the node type names within the target node list.
/// target_node_list_node_types_column: Optional[str]
///     Name for the column with the node type names within the target node list.
/// number_of_nodes: Optional[int]
///     Number of the nodes in the original node list.
///
pub fn convert_node_list_node_types_to_numeric(
    original_node_path: String,
    target_node_path: String,
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<char>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    number_of_node_types: Option<NodeTypeT>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_support_balanced_quotes: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_is_correct: Option<bool>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<char>,
    target_node_type_list_header: Option<bool>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_node_types_column_number: Option<usize>,
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<char>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column_number: Option<usize>,
    target_nodes_column: Option<String>,
    target_node_types_separator: Option<char>,
    target_node_list_node_types_column_number: Option<usize>,
    target_node_list_node_types_column: Option<String>,
    number_of_nodes: Option<NodeT>,
) -> PyResult<(NodeT, Option<NodeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::convert_node_list_node_types_to_numeric(
            original_node_path.into(),
            target_node_path.into(),
            original_node_type_path,
            original_node_type_list_separator,
            original_node_types_column_number,
            original_node_types_column,
            number_of_node_types,
            original_numeric_node_type_ids,
            original_minimum_node_type_id,
            original_node_type_list_header,
            original_node_type_list_support_balanced_quotes,
            original_node_type_list_rows_to_skip,
            original_node_type_list_is_correct,
            original_node_type_list_max_rows_number,
            original_node_type_list_comment_symbol,
            original_load_node_type_list_in_parallel,
            target_node_type_list_path,
            target_node_type_list_separator,
            target_node_type_list_header,
            target_node_type_list_node_types_column,
            target_node_type_list_node_types_column_number,
            original_node_list_separator,
            original_node_list_header,
            original_node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_max_rows_number,
            node_list_comment_symbol,
            default_node_type,
            original_nodes_column_number,
            original_nodes_column,
            original_node_types_separator,
            original_node_list_node_types_column_number,
            original_node_list_node_types_column,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_node_list_numeric_node_type_ids,
            original_skip_node_types_if_unavailable,
            remove_chevrons,
            remove_spaces,
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column_number,
            target_nodes_column,
            target_node_types_separator,
            target_node_list_node_types_column_number,
            target_node_list_node_types_column,
            number_of_nodes
        ))?
        .into();
        (subresult_0.into(), subresult_1.map(|x| x.into()))
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(edge_path, edge_list_separator, edge_list_header, edge_list_support_balanced_quotes, edge_list_sources_column, edge_list_sources_column_number, edge_list_destinations_column, edge_list_destinations_column_number, edge_list_edge_type_column, edge_list_edge_type_column_number, edge_list_weights_column, edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, number_of_edges, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
/// Return whether the provided edge list contains duplicated edges.
///
/// Parameters
/// ----------
/// edge_path: str
///     The path from where to load the edge list.
/// edge_list_separator: Optional[str]
///     Separator to use for the edge list.
/// edge_list_header: Optional[bool]
///     Whether the edge list has an header.
/// edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// edge_list_sources_column: Optional[str]
///     The column name to use to load the sources in the edges list.
/// edge_list_sources_column_number: Optional[int]
///     The column number to use to load the sources in the edges list.
/// edge_list_destinations_column: Optional[str]
///     The column name to use to load the destinations in the edges list.
/// edge_list_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the edges list.
/// edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the edges list.
/// edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the edges list.
/// edge_list_weights_column: Optional[str]
///     The column name to use for the weights in the edges list.
/// edge_list_weights_column_number: Optional[int]
///     The column number to use for the weights in the edges list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the edge list.
/// default_weight: Optional[float]
///     The default weight to use within the edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the edge list.
/// number_of_edges: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn has_duplicated_edges_in_edge_list(
    edge_path: &str,
    edge_list_separator: Option<char>,
    edge_list_header: Option<bool>,
    edge_list_support_balanced_quotes: Option<bool>,
    edge_list_sources_column: Option<String>,
    edge_list_sources_column_number: Option<usize>,
    edge_list_destinations_column: Option<String>,
    edge_list_destinations_column_number: Option<usize>,
    edge_list_edge_type_column: Option<String>,
    edge_list_edge_type_column_number: Option<usize>,
    edge_list_weights_column: Option<String>,
    edge_list_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    Ok(pe!(graph::has_duplicated_edges_in_edge_list(
        edge_path,
        edge_list_separator,
        edge_list_header,
        edge_list_support_balanced_quotes,
        edge_list_sources_column,
        edge_list_sources_column_number,
        edge_list_destinations_column,
        edge_list_destinations_column_number,
        edge_list_edge_type_column,
        edge_list_edge_type_column_number,
        edge_list_weights_column,
        edge_list_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, target_path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable, sort_temporary_directory)"
)]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// target_path: str
///     The where to store the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str]
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int]
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// sort_temporary_directory: Optional[str]
///     Where to store the temporary files that are created during parallel sorting.
///
pub fn sort_numeric_edge_list(
    path: &str,
    target_path: &str,
    separator: Option<char>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    sort_temporary_directory: Option<String>,
) -> PyResult<()> {
    Ok(pe!(graph::sort_numeric_edge_list(
        path,
        target_path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable,
        sort_temporary_directory
    ))?)
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable, sort_temporary_directory)"
)]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str]
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int]
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// sort_temporary_directory: Optional[str]
///     Where to store the temporary files that are created during parallel sorting.
///
pub fn sort_numeric_edge_list_inplace(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    sort_temporary_directory: Option<String>,
) -> PyResult<()> {
    Ok(pe!(graph::sort_numeric_edge_list_inplace(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable,
        sort_temporary_directory
    ))?)
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, number_of_edges, load_edge_list_in_parallel, verbose, name)"
)]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// number_of_edges: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn get_number_of_selfloops_from_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    support_balanced_quotes: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    Ok(pe!(graph::get_number_of_selfloops_from_edge_list(
        path,
        separator,
        header,
        support_balanced_quotes,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        number_of_edges,
        load_edge_list_in_parallel,
        verbose,
        name
    ))?
    .into())
}

pub fn register_utils(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
