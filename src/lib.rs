
//!
//! # Neujal
//! 
//! A neural network library written in Rust for learning purposes.
//! 
//! ## Features
//! 
//! - [ ] Feedforward
//! - [ ] Backpropagation
//! - [ ] Convolutional Neural Networks
//! 
//! 
//! ## Examples
//! 
//! ```rust
//! use neujal::nn::layers::fully_connected::FullyConnected;
//! use neujal::nn::sequential::Sequential;
//! 
//! let mut model: Sequential = Sequential::new();
//! 
//! model.add(Box::new(FullyConnected::new(784, 100)));
//! model.add(Box::new(FullyConnected::new(100, 10)));
//! ```
//! 

pub mod nn;
