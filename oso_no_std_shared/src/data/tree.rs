//! provide tree structure data type and its manipulation functionality

/// generic parameter `T` can be virtually different between children by using enum
pub struct Tree<'a, N: NodeValue,> {
	value:    N,
	children: &'a [Self],
	parent:   Option<&'a Self,>,
}

/// TODO:
/// - [x] consider remove default implementation of nth_ancestor
/// - [ ] introduce generic const to represent border condition like position is root, has no more
/// child, first/last brother etc.
pub trait TreeWalk<
	const IS_TOP: bool,
	const IS_BOTTOM: bool,
	const IS_LEFT_MOST: bool,
	const IS_RIGHT_MOST: bool,
	N: NodeValue,
>: Sized + Iterator
{
	type ChildTree;

	// type TreeType: TreeWalk<'a, N,>;

	// NOTE: walk operation
	fn root<
		const ROOT_IS_BOTTOM: bool,
		N2: NodeValue,
		O: TreeWalk<true, ROOT_IS_BOTTOM, true, true, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<N2, O, C,>;
	/// return tree on current position
	/// there is similar method `node` which returns current **node**
	fn current<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(&self,) -> O;

	fn children<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(&mut self,)
	-> WalkRslt<N2, O, C,>;
	fn parent<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(&mut self,) -> WalkRslt<N2, O, C,>;
	// TODO: handle opaque type,  recursive trait method and lifetime at once
	fn nth_ancestor<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
		n: usize,
	) -> WalkRslt<N2, O, C,> {
		if n == 0 {
			self.as_walk_tried()
		} else {
			let mut parent: WalkRslt<N2, O, C,> = self.parent();
			if parent.has_success() {
				parent.current_tree_mut().as_mut().unwrap().nth_ancestor(n - 1,)
			} else {
				parent
			}
		}
	}

	fn nth_brother<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
		n: usize,
	) -> WalkRslt<N2, O, C,> {
	}
	fn next_brother<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
	) -> WalkRslt<N2, O, C,> {
	}
	fn prev_brother<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
	) -> WalkRslt<N2, O, C,> {
	}
	fn first_brother<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
	) -> WalkRslt<N2, O, C,>;
	fn last_brother<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
	) -> WalkRslt<N2, O, C,>;

	fn nth_child<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
		n: usize,
	) -> WalkRslt<N2, O, C,>;
	fn first_child<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
	) -> WalkRslt<N2, O, C,>;
	fn last_child<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
	) -> WalkRslt<N2, O, C,>;

	/// set current position specified by `coordinate`
	fn set_pos<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&mut self,
		coordinate: impl Coordinate,
	) -> WalkRslt<N2, O, C,>;

	// NOTE: current position info
	fn has_child(&self,) -> bool;
	fn has_parent(&self,) -> bool;

	fn child_count(&self,) -> usize;
	fn brother_count(&self,) -> usize;
	fn generation_count(&self,) -> usize;
	fn get_pos_in_brother() -> usize;

	fn get_pos(&self,) -> impl Coordinate;
	fn as_walk_tried<N2: NodeValue, O: TreeWalk<N2,>, C: Coordinate,>(
		&self,
	) -> WalkRslt<N2, O, C,>;

	fn value(&self,) -> N::Output;
	/// return node on current position
	/// there is similar method `current` which returns current **tree** that contains positon infos
	fn node(&self,) -> N;
}

pub trait WalkTried<
	const IS_TOP: bool,
	const IS_BOTTOM: bool,
	const IS_LEFT_MOST: bool,
	const IS_RIGHT_MOST: bool,
	N: NodeValue,
	T: TreeWalk<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N,>,
>
{
	type C: Coordinate;
	// type TreeNode: TreeWalk<'a, Self::N,>
	// where Self::N: 'a;

	fn has_success(&self,) -> bool;
	fn has_failed(&self,) -> bool {
		!self.has_success()
	}

	fn last_valid_coordinate(&self,) -> &Self::C;
	fn current_tree(&self,) -> &Option<T,>;
	fn current_tree_mut(&mut self,) -> &mut Option<T,>;

	fn from(tn: T, coord: Self::C,) -> Self;
}

// fn walk_tried_from<N: NodeValue, T: TreeWalk<N,>, WT: WalkTried<N, T, C = impl Coordinate,>,>(
// 	tn: T,
// 	coord: WT::C,
// ) -> WT {
// 	WT::from(tn, coord,)
// }

pub trait Coordinate {
	fn nth_dimension(&self, n: usize,) -> usize;
	fn first_dimension(&self,) -> usize {
		self.nth_dimension(0,)
	}
	fn last_dimension(&self,) -> usize {
		let last_dimension_is = self.dimension_count();
		self.nth_dimension(last_dimension_is - 1,)
	}

	fn dimension_count(&self,) -> usize;
	fn set_at(&mut self, dim: usize, value: usize,);
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

pub struct WalkRslt<
	const IS_TOP: bool,
	const IS_BOTTOM: bool,
	const IS_LEFT_MOST: bool,
	const IS_RIGHT_MOST: bool,
	N: NodeValue,
	T: TreeWalk<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N,>,
	C: Coordinate,
> {
	__constraint: core::marker::PhantomData<N,>,
	tree:         Option<T,>,
	coord:        C,
}

impl<
	const IS_TOP: bool,
	const IS_BOTTOM: bool,
	const IS_LEFT_MOST: bool,
	const IS_RIGHT_MOST: bool,
	N: NodeValue,
	T: TreeWalk<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N,>,
	C: Coordinate,
> WalkTried<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N, T,>
	for WalkRslt<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N, T, C,>
{
	type C = C;

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
