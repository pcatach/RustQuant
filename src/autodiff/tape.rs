// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the implementation of the `Tape`.
//! The tape is also known as a Wengert List.
//!
//! The tape is an abstract data structure that contains `Vertex`s. These
//! contain the adjoints and indices to the parent vertices.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{variable::Variable, vertex::Vertex, OperationArity};
use std::cell::RefCell;
// use std::{rc::Rc, sync::Arc};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// VERTEX AND TAPE STRUCTS AND IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the tape (Wengert list), as a vector of `Vertex`s.
#[derive(Debug, Clone)]
pub struct Tape {
    /// Vector containing the vertices in the Wengert List.
    pub vertices: RefCell<Vec<Vertex>>,
    // pub vertices: RefCell<Rc<[Vertex]>>,
}

impl Default for Tape {
    #[inline]
    fn default() -> Self {
        Tape {
            vertices: RefCell::new(Vec::new()),
            // vertices: RefCell::new(Rc::new([])),
        }
    }
}

/// Implementation for the `Tape` struct.
impl Tape {
    /// Instantiate a new tape.
    #[inline]
    pub fn new() -> Self {
        Tape {
            vertices: RefCell::new(Vec::new()),
            // vertices: RefCell::new(Rc::new([])),
        }
    }

    /// Add a new variable to to the tape.
    /// Returns a new `Variable` instance (the contents of a vertex).
    #[inline]
    pub fn var(&self, value: f64) -> Variable {
        Variable {
            tape: self,
            value,
            index: self.push_nullary(),
        }
    }

    /// Add a multiple variables (a slice) to the tape.
    /// Useful for larger functions with many inputs.
    #[inline]
    pub fn vars<'v>(&'v self, values: &[f64]) -> Vec<Variable<'v>> {
        values.iter().map(|&val| self.var(val)).collect()
    }

    /// Returns the length of the tape so new vertices can index to the correct position.
    #[inline]
    pub fn len(&self) -> usize {
        self.vertices.borrow().len()
    }

    /// Returns true/false depending on whether the tape is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vertices.borrow().len() == 0
    }

    /// Clears the entire tape.
    #[inline]
    pub fn clear(&self) {
        self.vertices.borrow_mut().clear();
    }

    /// Zeroes the adjoints in the tape.
    #[inline]
    pub fn zero(&self) {
        self.vertices
            .borrow_mut()
            .iter_mut()
            .for_each(|vertex| vertex.partials = [0.0; 2]);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Functions to push values to the tape (Wengert List):
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Pushes a vertex to the tape.
    #[inline]
    pub fn push(
        &self,
        operation: OperationArity,
        parents: &[usize; 2],
        partials: &[f64; 2],
    ) -> usize {
        let mut vertices = self.vertices.borrow_mut();
        let len = vertices.len();

        let vertex = match operation {
            OperationArity::Nullary => Vertex {
                partials: [0.0, 0.0],
                parents: [len, len],
            },
            OperationArity::Unary => Vertex {
                partials: [partials[0], 0.0],
                parents: [parents[0], len],
            },
            OperationArity::Binary => Vertex {
                partials: [partials[0], partials[1]],
                parents: [parents[0], parents[1]],
            },
        };

        vertices.push(vertex);

        len
    }

    /// Nullary operator pushback.
    ///
    /// The vertex pushed to the tape is the result of a **nullary** operation.
    /// e.g. `x.neg()` ($-x$)
    /// Thus no partials and only the current index are added to the new vertex.
    ///
    /// 1. Constructs the vertex,
    /// 2. Pushes the new vertex onto the tape,
    /// 3. Returns the index of the new vertex.
    #[inline]
    pub fn push_nullary(&self) -> usize {
        let mut vertices = self.vertices.borrow_mut();
        let len = vertices.len();
        vertices.push(Vertex {
            partials: [0.0, 0.0],
            parents: [len, len],
        });
        len
    }

    /// Unary operator pushback.
    ///
    /// The vertex pushed to the tape is the result of a **unary** operation.
    /// e.g. `x.sin()` ($sin(x)$)
    /// Thus one partial and one parent are added to the new vertex.
    ///
    /// 1. Constructs the vertex,
    /// 2. Pushes the new vertex onto the tape,
    /// 3. Returns the index of the new vertex.
    #[inline]
    pub fn push_unary(&self, parent0: usize, partial0: f64) -> usize {
        let mut vertices = self.vertices.borrow_mut();
        let len = vertices.len();
        vertices.push(Vertex {
            partials: [partial0, 0.0],
            parents: [parent0, len],
        });
        len
    }

    /// Binary operator pushback.
    ///
    /// The vertex pushed to the tape is the result of a **binary** operation.
    /// e.g. `x + y`
    /// Thus two partials and two parents are added to the new vertex.
    ///
    /// 1. Constructs the vertex,
    /// 2. Pushes the new vertex onto the tape,
    /// 3. Returns the index of the new vertex.
    #[inline]
    pub fn push_binary(
        &self,
        parent0: usize,
        partial0: f64,
        parent1: usize,
        partial1: f64,
    ) -> usize {
        let mut vertices = self.vertices.borrow_mut();
        let len = vertices.len();
        vertices.push(Vertex {
            partials: [partial0, partial1],
            parents: [parent0, parent1],
        });
        len
    }
}