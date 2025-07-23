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
pub trait TreeWalk<N: NodeValue,>: Sized + Iterator {
	type ChildTree: Iterator;

	// type TreeType: TreeWalk<'a, N,>;

	// NOTE: walk operation
	fn root<
		const ROOT_IS_BOTTOM: bool,
		N2: NodeValue,
		O: TreeWalk<true, ROOT_IS_BOTTOM, true, true, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,>;
	/// return tree on current position
	/// there is similar method `node` which returns current **node**
	fn current<
		N2: NodeValue,
		O: TreeWalk<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&self,
	) -> O;

	fn children<C: Coordinate,>(&mut self,) -> WalkRslt<Self::ChildTree, C,>;
	fn parent<
		const PARENT_IS_TOP: bool,
		const PARENT_IS_LEFT_MOST: bool,
		const PARENT_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<PARENT_IS_TOP, false, PARENT_IS_LEFT_MOST, PARENT_IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,>;
	// TODO: handle opaque type,  recursive trait method and lifetime at once
	fn nth_ancestor<
		const ANCESTOR_IS_TOP: bool,
		const ANCESTOR_IS_BOTTOM: bool,
		const ANCESTOR_IS_LEFT_MOST: bool,
		const ANCESTOR_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<
				ANCESTOR_IS_TOP,
				ANCESTOR_IS_BOTTOM,
				ANCESTOR_IS_LEFT_MOST,
				ANCESTOR_IS_RIGHT_MOST,
				N2,
			>,
		C: Coordinate,
	>(
		&mut self,
		n: usize,
	) -> WalkRslt<O, C,> {
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

	fn nth_brother<
		const BROTHER_IS_TOP: bool,
		const BROTHER_IS_BOTTOM: bool,
		const BROTHER_IS_LEFT_MOST: bool,
		const BROTHER_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<
				BROTHER_IS_TOP,
				BROTHER_IS_BOTTOM,
				BROTHER_IS_LEFT_MOST,
				BROTHER_IS_RIGHT_MOST,
				N2,
			>,
		C: Coordinate,
	>(
		&mut self,
		n: usize,
	) -> WalkRslt<O, C,> {
		todo!()
	}
	fn next_brother<
		const BROTHER_IS_TOP: bool,
		const BROTHER_IS_BOTTOM: bool,
		const BROTHER_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<BROTHER_IS_TOP, BROTHER_IS_BOTTOM, false, BROTHER_IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,> {
		todo!()
	}
	fn prev_brother<
		const BROTHER_IS_TOP: bool,
		const BROTHER_IS_BOTTOM: bool,
		const BROTHER_IS_LEFT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<BROTHER_IS_TOP, BROTHER_IS_BOTTOM, BROTHER_IS_LEFT_MOST, false, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,> {
		todo!()
	}
	fn first_brother<
		const BROTHER_IS_TOP: bool,
		const BROTHER_IS_BOTTOM: bool,
		const BROTHER_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<BROTHER_IS_TOP, BROTHER_IS_BOTTOM, true, BROTHER_IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,>;
	fn last_brother<
		const BROTHER_IS_TOP: bool,
		const BROTHER_IS_BOTTOM: bool,
		const BROTHER_IS_LEFT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<BROTHER_IS_TOP, BROTHER_IS_BOTTOM, BROTHER_IS_LEFT_MOST, true, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,>;

	fn nth_child<
		const CHILD_IS_BOTTOM: bool,
		const CHILD_IS_LEFT_MOST: bool,
		const CHILD_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<false, CHILD_IS_BOTTOM, CHILD_IS_LEFT_MOST, CHILD_IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&mut self,
		n: usize,
	) -> WalkRslt<O, C,>;
	fn first_child<
		const CHILD_IS_BOTTOM: bool,
		const CHILD_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<false, CHILD_IS_BOTTOM, true, CHILD_IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,>;
	fn last_child<
		const CHILD_IS_BOTTOM: bool,
		const CHILD_IS_LEFT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<false, CHILD_IS_BOTTOM, CHILD_IS_LEFT_MOST, true, N2,>,
		C: Coordinate,
	>(
		&mut self,
	) -> WalkRslt<O, C,>;

	/// set current position specified by `coordinate`
	fn set_pos<
		const POS_IS_TOP: bool,
		const POS_IS_BOTTOM: bool,
		const POS_IS_LEFT_MOST: bool,
		const POS_IS_RIGHT_MOST: bool,
		N2: NodeValue,
		O: TreeWalk<POS_IS_TOP, POS_IS_BOTTOM, POS_IS_LEFT_MOST, POS_IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&mut self,
		coordinate: impl Coordinate,
	) -> WalkRslt<O, C,>;

	// NOTE: current position info
	fn has_child(&self,) -> bool;
	fn has_parent(&self,) -> bool;

	fn child_count(&self,) -> usize;
	fn brother_count(&self,) -> usize;
	fn generation_count(&self,) -> usize;
	fn get_pos_in_brother() -> usize;

	fn get_pos(&self,) -> impl Coordinate;
	fn as_walk_tried<
		N2: NodeValue,
		O: TreeWalk<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N2,>,
		C: Coordinate,
	>(
		&self,
	) -> WalkRslt<O, C,>;

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

pub struct WalkRslt<T, C,> {
	// __constraint: core::marker::PhantomData<N,>,
	tree:  Option<T,>,
	coord: C,
}

impl<
	const IS_TOP: bool,
	const IS_BOTTOM: bool,
	const IS_LEFT_MOST: bool,
	const IS_RIGHT_MOST: bool,
	N: NodeValue,
	T: TreeWalk<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N,>,
	C: Coordinate,
> WalkTried<IS_TOP, IS_BOTTOM, IS_LEFT_MOST, IS_RIGHT_MOST, N, T,> for WalkRslt<T, C,>
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
		Self {
			//__constraint: core::marker::PhantomData::<N,>,
			tree: Some(tn,),
			coord,
		}
	}
}
