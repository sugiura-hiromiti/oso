//! provide tree structure data type and its manipulation functionality

use oso_error::Rslt;

pub type WalkRslt<'a, N, T,> = impl WalkTried<'a, N, T,>;

/// generic parameter `T` can be virtually different between children by using enum
pub struct Tree<'a, N: NodeValue,> {
	value:    N,
	children: &'a [Self],
	parent:   Option<&'a Self,>,
}

pub trait TreeWalk<'a, N: NodeValue,>: Sized + Iterator
where
	N: 'a,
	Self: 'a,
{
	type ChildTree;
	// type TreeType: TreeWalk<'a, N,>;

	// NOTE: walk operation
	fn root(&self,) -> WalkRslt<'a, N, Self,>;
	/// return tree on current position
	/// there is similar method `node` which returns current **node**
	fn current(&self,) -> Self;

	fn children(&self,) -> WalkRslt<'a, N, Self,>;
	fn parent(&self,) -> WalkRslt<'a, N, Self,> + use<'a,>;
	// TODO: handle opaque type,  recursive trait method and lifetime at once
	#[define_opaque(WalkRslt)]
	fn nth_ancestor(&self, n: usize,) -> WalkRslt<'a, N, Self,> {
		if n == 0 || !self.has_parent() {
			return self.as_walk_tried();
		}

		let parent = self.parent();
		let parent = parent.current_node();
		parent.nth_ancestor(n - 1,)
	}

	fn nth_brother(&self, n: usize,) -> WalkRslt<'a, N, Self,>;
	fn next_brother(&self,) -> WalkRslt<'a, N, Self,>;
	fn prev_brother(&self,) -> WalkRslt<'a, N, Self,>;
	fn first_brother(&self,) -> WalkRslt<'a, N, Self,>;
	fn last_brotheer(&self,) -> WalkRslt<'a, N, Self,>;

	fn nth_child(&self, n: usize,) -> WalkRslt<'a, N, Self,>;
	fn first_child(&self,) -> WalkRslt<'a, N, Self,>;
	fn last_child(&self,) -> WalkRslt<'a, N, Self,>;

	/// set current position specified by `coordinate`
	fn set_pos(&self, coordinate: impl Coordinate,) -> WalkRslt<'a, N, Self,>;

	// NOTE: current position info
	fn has_child(&self,) -> bool;
	fn has_parent(&self,) -> bool;

	fn child_count(&self,) -> usize;
	fn brother_count(&self,) -> usize;
	fn generation_count(&self,) -> usize;

	fn get_pos(&self,) -> impl Coordinate;
	fn as_walk_tried(&self,) -> WalkRslt<'a, N, Self,> {
		let cur_tree_node = self.current();
		let coord = self.get_pos();
		todo!()
		// WT::from(cur_tree_node, coord,)
		// walk_tried_from::<_, _, WT,>(cur_tree_node, coord,)
	}

	fn value(&self,) -> N::Output;
	/// return node on current position
	/// there is similar method `current` which returns current **tree** that contains positon infos
	fn node(&self,) -> N;
}

pub trait WalkTried<'a, N: NodeValue + 'a, T: TreeWalk<'a, N,> + 'a,> {
	// type TreeNode: TreeWalk<'a, Self::N,>
	// where Self::N: 'a;

	fn has_success(&self,) -> bool;
	fn has_failed(&self,) -> bool {
		!self.has_success()
	}

	fn last_valid_coordinate(&self,) -> impl Coordinate;
	fn current_node(&'a self,) -> T;

	fn from(tn: T, coord: impl Coordinate,) -> Self;
}

fn walk_tried_from<
	'a,
	N: NodeValue + 'a,
	T: TreeWalk<'a, N,> + 'a,
	WT: WalkTried<'a, N, T,> + 'a,
>(
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
