//! Convert bril programs to RVSDGs.
//!
//! Bril functions are written in terms of basic blocks and jumps/gotos. RVSDGs
//! only support intra-function control flow in the form of switch statements
//! and do-while loops (gamma and theta nodes, respectively). Transforming the
//! native Bril representation to RVSDGs requires the following steps:
//!
//! * Parse to CFG: read the bril program into a graph data-structure where
//! basic blocks are nodes and edges are (conditional) jumps.
//!
//! * Restructure the CFG: Bril programs support irreducible CFGs, but the CFGs
//! corresponding to RVSDGs are all reducible. Before we convert the CFG to an
//! RVSDG, we need to convert the unstructured CFG to a structured one.
//!
//! * RVSDG conversion: Once we have a structured CFG we need to convert the
//! program (still written in terms of gotos) to the structured format for
//! RVSDGs. Part of this conversion process is the discovery of what the
//! "inputs" and "outputs" are for different RVSDG nodes.
pub(crate) mod cfg;

#[cfg(test)]
mod tests;

use thiserror::Error;

/// Errors from the rvsdg module.
#[derive(Debug, Error)]
pub(crate) enum RvsdgError {}

pub(crate) type Result<T = ()> = std::result::Result<T, RvsdgError>;