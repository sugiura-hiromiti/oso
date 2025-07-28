//! # Tree Data Structure Module
//!
//! This module provides a generic tree data structure and associated traits for
//! traversal, manipulation, and navigation. The tree implementation is designed
//! for use in system-level programming where memory efficiency and performance
//! are critical.
//!
//! ## Key Components
//!
//! - `Tree<'a, N>`: The main tree structure with lifetime-managed references
//! - `TreeWalk<N>`: Trait for tree traversal and navigation operations
//! - `TreeWindow<N>`: Trait for windowed views of tree sections
//! - `NodeValue`: Trait for values that can be stored in tree nodes
//! - `Coordinate`: Trait for representing positions within the tree
//!
//! ## Design Principles
//!
//! The tree implementation uses lifetime parameters to ensure memory safety
//! without requiring heap allocation, making it suitable for `no_std` environments.
//! The trait-based design allows for flexible tree operations while maintaining
//! type safety.

/// A generic tree data structure with lifetime-managed references.
///
/// This tree structure uses references to manage parent-child relationships
/// without requiring heap allocation. The generic parameter `N` allows for
/// different node value types, which can be made heterogeneous using enums.
///
/// # Type Parameters
///
/// - `'a`: Lifetime parameter for the tree references
/// - `N`: Type implementing `NodeValue` trait for the node data
///
/// # Examples
///
/// ```rust,ignore
/// use oso_no_std_shared::data::tree::{Tree, Node};
///
/// let root_node = Node("root".to_string());
/// let children = [];
/// let tree = Tree {
///     value: root_node,
///     children: &children,
///     parent: None,
/// };
/// ```
pub struct Tree<'a, N: NodeValue,> {
	/// The value stored in this node
	value:    N,
	/// Array of child trees (slice to avoid heap allocation)
	children: &'a [Self],
	/// Optional reference to the parent tree node
	parent:   Option<&'a Self,>,
}

/// Trait for providing windowed views of tree sections.
///
/// This trait extends `TreeWalk` to provide specialized views of tree sections,
/// allowing for efficient navigation of children and sibling nodes.
///
/// # Type Parameters
///
/// - `N`: The node value type for the current tree level
///
/// # Associated Types
///
/// - `ChildrenN`: Node value type for child nodes
/// - `Children`: Tree walker type for child nodes
/// - `BrothersN`: Node value type for sibling nodes
/// - `Brothers`: Tree walker type for sibling nodes
pub trait TreeWindow<N: NodeValue,>: TreeWalk<N,> {
	/// Node value type for child nodes
	type ChildrenN: NodeValue;
	/// Tree walker type for child nodes
	type Children: TreeWalk<Self::ChildrenN,>;

	/// Node value type for sibling nodes
	type BrothersN: NodeValue;
	/// Tree walker type for sibling nodes
	type Brothers: TreeWalk<Self::BrothersN,>;

	/// Get a walker for the children of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walker that can traverse the child nodes
	fn children<WT: WalkTried<T = Self::Children,>,>(&mut self,) -> WT;

	/// Get a walker for the siblings of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walker that can traverse the sibling nodes
	fn brothers<WT: WalkTried<T = Self::Brothers,>,>(&mut self,) -> WT;
}

/// Core trait for tree traversal and navigation operations.
///
/// This trait provides a comprehensive set of methods for navigating through
/// tree structures, including movement to parents, children, siblings, and
/// arbitrary positions within the tree.
///
/// # Design Notes
///
/// - The `nth_ancestor` method has a default implementation for convenience
/// - Border conditions (like reaching root or leaf nodes) are handled through the `WalkTried`
///   return type
/// - Position tracking is handled through the `Coordinate` trait
///
/// # Type Parameters
///
/// - `N`: Type implementing `NodeValue` for the node data
pub trait TreeWalk<N: NodeValue,>: Sized + Iterator {
	// === Navigation Operations ===

	/// Navigate to the root of the tree.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn root<WT: WalkTried,>(&mut self,) -> WT;

	/// Get the current tree position.
	///
	/// This method returns a tree walker positioned at the current location.
	/// Note: This is different from `node()` which returns just the node value.
	///
	/// # Returns
	///
	/// A tree walker at the current position
	fn current(&self,) -> impl TreeWalk<N,>;

	/// Navigate to the parent of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn parent<WT: WalkTried,>(&mut self,) -> WT;

	/// Navigate to the nth ancestor of the current node.
	///
	/// This method provides a default implementation that recursively calls
	/// `parent()` n times. A value of 0 returns the current position.
	///
	/// # Parameters
	///
	/// - `n`: Number of generations to go up (0 = current, 1 = parent, etc.)
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn nth_ancestor<WT: WalkTried,>(&mut self, n: usize,) -> WT {
		if n == 0 {
			// Base case: return current position
			self.as_walk_tried()
		} else {
			// Recursive case: go to parent and continue
			let mut parent = self.parent::<WT>();
			if parent.has_success() {
				// Successfully found parent, continue recursion
				TreeWalk::nth_ancestor::<WT,>(parent.current_tree_mut().as_mut().unwrap(), n - 1,)
			} else {
				// Failed to find parent, return the failure
				parent
			}
		}
	}

	/// Navigate to the nth sibling of the current node.
	///
	/// # Parameters
	///
	/// - `n`: Index of the target sibling (0-based)
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn nth_brother<WT: WalkTried,>(&mut self, n: usize,) -> WT {
		let cur_bro_pos = self.get_pos().last_dimension();

		match cur_bro_pos.cmp(&n,) {
			// Current position is before target: move forward
			core::cmp::Ordering::Less => {
				self.next_brother::<WT>().current_tree_mut().as_mut().unwrap().nth_brother(n,)
			},
			// Already at target position
			core::cmp::Ordering::Equal => self.as_walk_tried(),
			core::cmp::Ordering::Greater => self.prev_brother::<WT>().current_tree_mut().as_mut().unwrap().nth_brother(n),
		}
	}

	/// Navigate to the next sibling of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn next_brother<WT: WalkTried,>(&mut self,) -> WT {
		todo!()
	}

	/// Navigate to the previous sibling of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn prev_brother<WT: WalkTried,>(&mut self,) -> WT {
		todo!()
	}

	/// Navigate to the first sibling (leftmost child of parent).
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn first_brother<WT: WalkTried,>(&mut self,) -> WT;

	/// Navigate to the last sibling (rightmost child of parent).
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn last_brother<WT: WalkTried,>(&mut self,) -> WT;

	/// Navigate to the nth child of the current node.
	///
	/// # Parameters
	///
	/// - `n`: Index of the target child (0-based)
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn nth_child<WT: WalkTried,>(&mut self, n: usize,) -> WT;

	/// Navigate to the first child of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn first_child<WT: WalkTried,>(&mut self,) -> WT;

	/// Navigate to the last child of the current node.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn last_child<WT: WalkTried,>(&mut self,) -> WT;

	/// Set the current position to the specified coordinate.
	///
	/// # Parameters
	///
	/// - `coordinate`: Target position implementing the `Coordinate` trait
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result indicating success or failure of the operation
	fn set_pos<WT: WalkTried,>(&mut self, coordinate: impl Coordinate,) -> WT;

	// === Position Information Methods ===

	/// Check if the current node has any children.
	///
	/// # Returns
	///
	/// `true` if the node has children, `false` otherwise
	fn has_child(&self,) -> bool;

	/// Check if the current node has a parent.
	///
	/// # Returns
	///
	/// `true` if the node has a parent, `false` if it's the root
	fn has_parent(&self,) -> bool;

	/// Get the number of children of the current node.
	///
	/// # Returns
	///
	/// The count of direct children
	fn child_count(&self,) -> usize;

	/// Get the number of siblings of the current node.
	///
	/// # Returns
	///
	/// The count of sibling nodes (including self)
	fn brother_count(&self,) -> usize;

	/// Get the depth of the tree from the current node.
	///
	/// # Returns
	///
	/// The number of generations in the subtree rooted at this node
	fn generation_count(&self,) -> usize;

	/// Get the position of this node among its siblings.
	///
	/// # Returns
	///
	/// The 0-based index of this node in its parent's children array
	fn get_pos_in_brother() -> usize;

	/// Get the current position as a coordinate.
	///
	/// # Returns
	///
	/// A coordinate representing the current position in the tree
	fn get_pos(&self,) -> impl Coordinate;

	/// Convert the current state to a walk result.
	///
	/// # Type Parameters
	///
	/// - `WT`: Walk result type that must implement `WalkTried`
	///
	/// # Returns
	///
	/// A walk result representing the current state
	fn as_walk_tried<WT: WalkTried,>(&self,) -> WT;

	// === Value Access Methods ===

	/// Get the output value of the current node.
	///
	/// # Returns
	///
	/// The output value as defined by the `NodeValue` trait
	fn value(&self,) -> N::Output;

	/// Get the node value at the current position.
	///
	/// This method returns the node value itself, while `current()` returns
	/// the tree walker with position information.
	///
	/// # Returns
	///
	/// The node value of type `N`
	fn node(&self,) -> N;
}

/// Trait for representing the result of tree walk operations.
///
/// This trait encapsulates the result of tree navigation operations, providing
/// information about success/failure and maintaining the last valid position.
///
/// # Type Parameters
///
/// - `N`: Node value type
/// - `T`: Tree walker type
/// - `C`: Coordinate type for position tracking
pub trait WalkTried {
	/// The node value type
	type N: NodeValue;
	/// The tree walker type
	type T: TreeWalk<Self::N,>;
	/// The coordinate type for position tracking
	type C: Coordinate;
	// type TreeNode: TreeWalk<'a, Self::N,>
	// where Self::N: 'a;

	/// Check if the walk operation was successful.
	///
	/// # Returns
	///
	/// `true` if the operation succeeded, `false` otherwise
	fn has_success(&self,) -> bool;

	/// Check if the walk operation failed.
	///
	/// This is the logical inverse of `has_success()`.
	///
	/// # Returns
	///
	/// `true` if the operation failed, `false` otherwise
	fn has_failed(&self,) -> bool {
		!self.has_success()
	}

	/// Get the last valid coordinate before the operation.
	///
	/// This is useful for error recovery and debugging.
	///
	/// # Returns
	///
	/// Reference to the last valid coordinate
	fn last_valid_coordinate(&self,) -> &Self::C;

	/// Get an immutable reference to the current tree walker.
	///
	/// # Returns
	///
	/// Optional reference to the tree walker (None if operation failed)
	fn current_tree(&self,) -> &Option<Self::T,>;

	/// Get a mutable reference to the current tree walker.
	///
	/// # Returns
	///
	/// Optional mutable reference to the tree walker (None if operation failed)
	fn current_tree_mut(&mut self,) -> &mut Option<Self::T,>;

	/// Create a new walk result from a tree walker and coordinate.
	///
	/// # Parameters
	///
	/// - `tn`: The tree walker
	/// - `coord`: The coordinate position
	///
	/// # Returns
	///
	/// A new walk result instance
	fn from(tn: Self::T, coord: Self::C,) -> Self;
}

/// Trait for representing coordinates/positions within a tree structure.
///
/// This trait provides methods for working with multi-dimensional coordinates
/// that can represent positions in tree hierarchies.
pub trait Coordinate {
	/// Get the value at the nth dimension.
	///
	/// # Parameters
	///
	/// - `n`: The dimension index (0-based)
	///
	/// # Returns
	///
	/// The coordinate value at the specified dimension
	fn nth_dimension(&self, n: usize,) -> usize;

	/// Get the value of the first dimension.
	///
	/// This is a convenience method equivalent to `nth_dimension(0)`.
	///
	/// # Returns
	///
	/// The coordinate value at dimension 0
	fn first_dimension(&self,) -> usize {
		self.nth_dimension(0,)
	}

	/// Get the value of the last dimension.
	///
	/// # Returns
	///
	/// The coordinate value at the highest dimension index
	fn last_dimension(&self,) -> usize {
		let last_dimension_is = self.dimension_count();
		self.nth_dimension(last_dimension_is - 1,)
	}

	/// Get the total number of dimensions in this coordinate.
	///
	/// # Returns
	///
	/// The number of dimensions
	fn dimension_count(&self,) -> usize;

	/// Set the value at a specific dimension.
	///
	/// # Parameters
	///
	/// - `dim`: The dimension index to modify
	/// - `value`: The new value for that dimension
	fn set_at(&mut self, dim: usize, value: usize,);
}

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

/// Concrete implementation of the `WalkTried` trait.
///
/// This struct represents the result of a tree walk operation, containing
/// either a successful tree walker or information about the failure.
///
/// # Type Parameters
///
/// - `N`: Node value type implementing `NodeValue`
/// - `T`: Tree walker type implementing `TreeWalk<N>`
/// - `C`: Coordinate type implementing `Coordinate`
///
/// # Fields
///
/// - `tree`: Optional tree walker (Some if successful, None if failed)
/// - `coord`: The coordinate position associated with this result
/// - `__constraint`: PhantomData to maintain type parameter `N`
pub struct WalkRslt<N: NodeValue, T: TreeWalk<N,>, C: Coordinate,> {
	/// PhantomData to maintain the node value type parameter
	__constraint: core::marker::PhantomData<N,>,
	/// The tree walker result (None if operation failed)
	tree:         Option<T,>,
	/// The coordinate position for this result
	coord:        C,
}

impl<N: NodeValue, T: TreeWalk<N,>, C: Coordinate,> WalkTried for WalkRslt<N, T, C,> {
	type C = C;
	type N = N;
	type T = T;

	fn has_success(&self,) -> bool {
		self.tree.is_some()
	}

	fn last_valid_coordinate(&self,) -> &Self::C {
		&self.coord
	}

	fn current_tree(&self,) -> &Option<T,> {
		&self.tree
	}

	fn current_tree_mut(&mut self,) -> &mut Option<T,> {
		&mut self.tree
	}

	fn from(tn: T, coord: Self::C,) -> Self {
		Self { __constraint: core::marker::PhantomData::<N,>, tree: Some(tn,), coord, }
	}
}
