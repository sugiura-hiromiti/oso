//! provide tree structure data type and its manipulation functionality

use oso_error::Rslt;

/// generic parameter `T` can be virtually different between children by using enum
pub struct Tree<'a, N: NodeValue,> {
	value:    N,
	children: &'a [Self],
	parent:   Option<&'a Self,>,
}

/// TODO: consider remove default implementation of nth_ancestor
pub trait TreeWalk<N: NodeValue,>: Sized + Iterator {
	type ChildTree;
	// type TreeType: TreeWalk<'a, N,>;

	// NOTE: walk operation
	fn root<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	/// return tree on current position
	/// there is similar method `node` which returns current **node**
	fn current<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> O;

	fn children<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	fn parent<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	// TODO: handle opaque type,  recursive trait method and lifetime at once
	// #[define_opaque(WalkRslt)]
	fn nth_ancestor<N2: NodeValue, O: TreeWalk<N2,>,>(&self, n: usize,) -> impl WalkTried<N2, O,> {
		if n == 0 {
			return self.as_walk_tried();
		}

		self.nth_ancestor(n - 1,)
	}

	fn nth_brother<N2: NodeValue, O: TreeWalk<N2,>,>(&self, n: usize,) -> impl WalkTried<N2, O,>;
	fn next_brother<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	fn prev_brother<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	fn first_brother<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	fn last_brother<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;

	fn nth_child<N2: NodeValue, O: TreeWalk<N2,>,>(&self, n: usize,) -> impl WalkTried<N2, O,>;
	fn first_child<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;
	fn last_child<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;

	/// set current position specified by `coordinate`
	fn set_pos<N2: NodeValue, O: TreeWalk<N2,>,>(
		&self,
		coordinate: impl Coordinate,
	) -> impl WalkTried<N2, O,>;

	// NOTE: current position info
	fn has_child(&self,) -> bool;
	fn has_parent(&self,) -> bool;

	fn child_count(&self,) -> usize;
	fn brother_count(&self,) -> usize;
	fn generation_count(&self,) -> usize;

	fn get_pos(&self,) -> impl Coordinate;
	fn as_walk_tried<N2: NodeValue, O: TreeWalk<N2,>,>(&self,) -> impl WalkTried<N2, O,>;

	fn value(&self,) -> N::Output;
	/// return node on current position
	/// there is similar method `current` which returns current **tree** that contains positon infos
	fn node(&self,) -> N;
}

pub trait WalkTried<N: NodeValue, T: TreeWalk<N,>,> {
	// type TreeNode: TreeWalk<'a, Self::N,>
	// where Self::N: 'a;

	fn has_success(&self,) -> bool;
	fn has_failed(&self,) -> bool {
		!self.has_success()
	}

	fn last_valid_coordinate(&self,) -> impl Coordinate;
	fn current_node(&self,) -> T;

	fn from(tn: T, coord: impl Coordinate,) -> Self;
}

type WalkRslt<T: TreeWalk<N,>, C: Coordinate,> = Rslt<T, C,>;

fn walk_tried_from<N: NodeValue, T: TreeWalk<N,>, WT: WalkTried<N, T,>,>(
	tn: T,
	coord: impl Coordinate,
) -> WT {
	WT::from(tn, coord,)
}

pub trait Coordinate {
	fn nth_dimension(&self,) -> usize;
	fn dimension_count(&self,) -> usize;
}

pub struct Node<T: Clone,>(T,);

/// wrap generic type in associated type. thus, this trait has no generic parameter!
pub trait NodeValue: AsMut<Self::Output,> + AsRef<Self::Output,>
where Self::Output: Clone
{
	type Output;
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
