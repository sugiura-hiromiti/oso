pub struct LinkedList<'a, T,> {
	node: Node<'a, T,>,
}

impl<'a, T,> LinkedList<'a, T,> {
	pub fn append(&mut self, value: &'a mut Node<'a, T,>,) {
		self.node.append(value,);
	}

	pub fn remove(&mut self, idx: usize,) {
		if idx == 0 {
			todo!()
		} else {
			self.node.remove(idx,);
		}
	}

	pub fn get(&self, idx: usize,) -> Option<&Node<'a, T,>,> {
		if idx == 0 { Some(&self.node,) } else { self.node.get(idx,) }
	}
}

pub struct Node<'a, T,> {
	pub value: T,
	next:      Option<&'a mut Node<'a, T,>,>,
}

impl<'a, T,> Node<'a, T,> {
	pub fn new(value: T,) -> Self {
		Self { value, next: None, }
	}

	pub(in crate::base::util) fn append(&mut self, next: &'a mut Node<'a, T,>,) {
		let mut p = &mut self.next;
		while let Some(next,) = p {
			p = &mut next.next;
		}

		p.replace(next,);
	}

	/// # Panic
	///
	/// if idx is 0, this function will panic
	pub(in crate::base::util) fn remove(&mut self, mut idx: usize,) {
		assert_ne!(0, idx);
		idx -= 1;

		let mut p = &mut self.next;
		for _ in 0..idx {
			if let Some(next,) = p {
				p = &mut next.next;
			} else {
				break;
			}
		}

		// in current situation, variable `p` points to target node  we have to store reference
		// to a node which is next to target node
		let next_to_target = p.as_mut().unwrap().next.as_mut().unwrap();
		let next_to_target = unsafe {
			let raw_pointer_to_next_to_target = *next_to_target as *mut Node<'a, T,>;
			raw_pointer_to_next_to_target.as_mut()
		};
		p.replace(next_to_target.unwrap(),);
	}

	pub(in crate::base::util) fn get(&self, mut idx: usize,) -> Option<&'a Node<'a, T,>,> {
		assert_ne!(0, idx);
		idx -= 1;

		let mut p = &self.next;
		for _ in 0..idx {
			if let Some(next,) = p {
				p = &next.next;
			} else {
				break;
			}
		}

		unsafe {
			let ref_to_next = *p.as_ref().unwrap() as *const Node<'a, T,>;
			ref_to_next.as_ref()
		}
	}
}
