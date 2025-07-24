//! provide tree structure data type and its manipulation functionality

/// generic parameter `T` can be virtually different between children by using enum
pub struct Tree<'a, N: NodeValue,> {
	value:    N,
	children: &'a [Self],
	parent:   Option<&'a Self,>,
}

pub trait TreeWindow<N: NodeValue,>: TreeWalk<N,> {
	type ChildrenN: NodeValue;
	type Children: TreeWalk<Self::ChildrenN,>;

	type BrothersN: NodeValue;
	type Brothers: TreeWalk<Self::BrothersN,>;
	fn children<WT: WalkTried<T = Self::Children,>,>(&mut self,) -> WT;
	fn brothers<WT: WalkTried<T = Self::Brothers,>,>(&mut self,) -> WT;
}

/// TODO:
/// - [x] consider remove default implementation of nth_ancestor
/// - [-] introduce generic const to represent border condition like position is root, has no more
/// child, first/last brother etc.
pub trait TreeWalk<N: NodeValue,>: Sized + Iterator {
	// NOTE: walk operation
	fn root<WT: WalkTried,>(&mut self,) -> WT;
	/// return tree on current position
	/// there is similar method `node` which returns current **node**
	fn current(&self,) -> impl TreeWalk<N,>;

	fn parent<WT: WalkTried,>(&mut self,) -> WT;
	fn nth_ancestor<WT: WalkTried,>(&mut self, n: usize,) -> WT {
		if n == 0 {
			self.as_walk_tried()
		} else {
			let mut parent = self.parent::<WT>();
			if parent.has_success() {
				TreeWalk::nth_ancestor::<WT,>(parent.current_tree_mut().as_mut().unwrap(), n - 1,)
			} else {
				parent
			}
		}
	}

	fn nth_brother<WT: WalkTried,>(&mut self, n: usize,) -> WT {
		let cur_bro_pos = self.get_pos().last_dimension();

		match cur_bro_pos.cmp(&n,) {
			core::cmp::Ordering::Less => {
				self.next_brother::<WT>().current_tree_mut().as_mut().unwrap().nth_brother(n,)
			},
			core::cmp::Ordering::Equal => self.as_walk_tried(),
			core::cmp::Ordering::Greater => todo!(),
		}
	}
	fn next_brother<WT: WalkTried,>(&mut self,) -> WT {
		todo!()
	}
	fn prev_brother<WT: WalkTried,>(&mut self,) -> WT {
		todo!()
	}
	fn first_brother<WT: WalkTried,>(&mut self,) -> WT;
	fn last_brother<WT: WalkTried,>(&mut self,) -> WT;

	fn nth_child<WT: WalkTried,>(&mut self, n: usize,) -> WT;
	fn first_child<WT: WalkTried,>(&mut self,) -> WT;
	fn last_child<WT: WalkTried,>(&mut self,) -> WT;

	/// set current position specified by `coordinate`
	fn set_pos<WT: WalkTried,>(&mut self, coordinate: impl Coordinate,) -> WT;

	// NOTE: current position info
	fn has_child(&self,) -> bool;
	fn has_parent(&self,) -> bool;

	fn child_count(&self,) -> usize;
	fn brother_count(&self,) -> usize;
	fn generation_count(&self,) -> usize;
	fn get_pos_in_brother() -> usize;

	fn get_pos(&self,) -> impl Coordinate;
	fn as_walk_tried<WT: WalkTried,>(&self,) -> WT;

	fn value(&self,) -> N::Output;
	/// return node on current position
	/// there is similar method `current` which returns current **tree** that contains positon infos
	fn node(&self,) -> N;
}

pub trait WalkTried {
	type N: NodeValue;
	type T: TreeWalk<Self::N,>;
	type C: Coordinate;

	fn has_success(&self,) -> bool;
	fn has_failed(&self,) -> bool {
		!self.has_success()
	}

	fn last_valid_coordinate(&self,) -> &Self::C;
	fn current_tree(&self,) -> &Option<Self::T,>;
	fn current_tree_mut(&mut self,) -> &mut Option<Self::T,>;

	fn from(tn: Self::T, coord: Self::C,) -> Self;
}

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

pub struct WalkRslt<N: NodeValue, T: TreeWalk<N,>, C: Coordinate,> {
	__constraint: core::marker::PhantomData<N,>,
	tree:         Option<T,>,
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
