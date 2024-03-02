use std::{cmp::max, default};
#[derive(Default)]
pub struct Segtree{
    size: usize,
    tree: Vec<i32>,
}

impl Segtree{

    // create new segment tree
    pub fn new(size: usize) -> Segtree{
        let mut tree = Vec::<i32>::new();
        tree.resize(size * 4 + 5, 0);
        Segtree{
            size:size,
            tree:tree
        }
    }

    // create new segment tree based on a number list
    pub fn build_tree(&mut self, number_list: &Vec<i32>, left: usize, right: usize, id: usize){
        if left >= right{
            self.tree[id] = number_list[left];
            return;
        }
        let mid = (left + right)/2;
        self.build_tree(number_list, left, mid, id*2);
        self.build_tree(number_list, mid + 1, right, id*2 + 1);
        self.tree[id] = max(
            self.tree[id*2],
            self.tree[id*2 + 1]
        );
    }

    // update the value at the index POSITION to VALUE
    pub fn update_position(&mut self, position: usize, value: i32, left: usize, right: usize, id: usize){
        if left == right{
            self.tree[id as usize] = value;
            return;
        }
        let mid = (left + right)/2;
        self.update_position(position, value, left, mid, id*2);
        self.update_position(position, value, mid + 1, right, id*2 + 1);
        self.tree[id as usize] = max(
            self.tree[id*2],
            self.tree[id*2 + 1]
        );
    }

    // retrive the maximum of the given range[left_search, right_search]
    pub fn get(&self, left_search: usize, right_search: usize, left: usize, right: usize, id: usize) -> i32{
        if left_search <= left && right <= right_search {
            return self.tree[id];
        }
        if left_search > right || left > right_search {
            return 0;
        }
        let mid = (left + right)/2;
        max(
            self.get(left_search, right_search, left, mid, id*2),
            self.get(left_search, right_search, mid + 1, right, id*2 + 1)
        )
    }
}
