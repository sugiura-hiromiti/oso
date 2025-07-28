/// A wrapper struct for node values.
///
/// This struct wraps any cloneable type to make it compatible with the
/// `NodeValue` trait, providing a simple way to store data in tree nodes.
///
/// # Type Parameters
///
/// - `T`: The wrapped type, which must implement `Clone`
pub struct Node<T: Clone,>(T,);

/// Trait for values that can be stored in tree nodes.
///
/// This trait abstracts over different types of node values, providing a
/// consistent interface for accessing and cloning node data. The trait uses
/// an associated type to avoid generic parameters on the trait itself.
///
/// # Associated Types
///
/// - `Output`: The actual type of the value stored in the node
///
/// # Trait Bounds
///
/// The implementing type must also implement `AsMut<Self::Output>` and
/// `AsRef<Self::Output>`, and the output type must be `Clone`.
pub trait NodeValue: AsMut<Self::Output,> + AsRef<Self::Output,>
where Self::Output: Clone
{
	/// The type of value stored in the node
	type Output;

	/// Create a clone of the node's value.
	///
	/// # Returns
	///
	/// A cloned copy of the node's value
	fn clone_value(&self,) -> Self::Output;
}

impl<T: Clone,> NodeValue for Node<T,> {
	type Output = T;

	fn clone_value(&self,) -> Self::Output {
		self.0.clone()
	}
}

impl<T: Clone,> AsRef<T,> for Node<T,> {
	fn as_ref(&self,) -> &T {
		&self.0
	}
}

impl<T: Clone,> AsMut<T,> for Node<T,> {
	fn as_mut(&mut self,) -> &mut T {
		&mut self.0
	}
}
