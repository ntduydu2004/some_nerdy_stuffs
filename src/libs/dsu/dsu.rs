pub struct Dsu{
    size: usize,
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl Dsu{
    pub fn new(&mut self, size:usize){
        self.size = size;
        self.parent = vec![0; size + 2];
        self.rank = vec![0; size + 2];
        for i in (0..size - 1){
            self.parent[i] = i;
        }
    }
    pub fn find(&mut self, u: usize) -> usize{
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]);
        }
        self.parent[u]
    }

    pub fn union(&mut self, u: usize, v: usize){
        let pu = self.find(u);
        let pv = self.find(v);
        if self.rank[pu] < self.rank[pv] {
            self.parent[pu] = pv;
        } else if self.rank[pv] < self.rank[pu] {
            self.parent[pv] = pu;
        } else {
            self.parent[pu] = pv;
            self.rank[pv] += 1;
        }
    }
}