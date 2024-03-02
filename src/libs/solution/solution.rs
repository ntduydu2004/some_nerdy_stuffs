use std::vec;

use crate::libs::{
    dsu::dsu::Dsu, 
    scanner::scanner::Scanner, 
    segment_tree::segment_tree::Segtree, 
    tarjan::tarjan
};

#[derive(Default)]
pub struct Attributes{
    size: usize,
    number_list: Vec<i32>,
    queries_size: usize,
    queries: Vec<(i32, i32, i32)>,
}

impl Attributes{
    pub fn get_size(&self) -> usize{
        self.size
    }
    pub fn get_number_list(&self) -> &Vec<i32>{
        &self.number_list
    }
    pub fn get_queries_size(&self) -> usize{   
        self.queries_size
    }
    pub fn get_queries(&self) -> &Vec<(i32, i32, i32)>{
        &self.queries
    }
}


#[derive(Default)]
pub struct Solution{
    // put the input attributes in here
    test_cases: i32,
    attribute_list: Vec<Attributes>,
    results: Vec<i32>,
}
impl Solution{

    // input data from text files using Scanner
    pub fn input(&mut self){
        self.attribute_list = vec![];
        let mut scanner = Scanner::default();
        let mut input_numbers = scanner.input();
        let mut vec_input_numbers: Vec<i32> = input_numbers.split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect();
        self.test_cases = vec_input_numbers[0];
        let mut index = 1;
        for test_case in (1..self.test_cases) {
            let size =  vec_input_numbers[index];
            index += 1;
            let mut number_list = vec![];
            for i in (1..size) {
                number_list.push(vec_input_numbers[index]);
                index += 1;
            }
            let queries_size = vec_input_numbers[index];
            index += 1;
            let mut queries = vec![];
            for i in (1..queries_size) {
                queries.push((vec_input_numbers[index], vec_input_numbers[index + 1], vec_input_numbers[index + 2]));
                index += 3;
            }
            self.attribute_list.push(Attributes {
                size:size as usize,
                number_list:number_list,
                queries_size:queries_size as usize,
                queries:queries,
            });
        }
        self.results = vec![];
    }

    // output data into text files using Scanner
    pub fn output(&self){
         let scanner = Scanner::default();
         scanner.output(&self.results);
    }

    // solution of the problem
    pub fn solve(&self, attribute: &Attributes) -> Vec<i32>{
        // retrieve data from attribute
        let size = attribute.get_size();
        let number_list = attribute.get_number_list();
        let queries_size = attribute.get_queries_size();
        let queries = attribute.get_queries();
        let mut answers = vec![];

        // initialize the tree
        let mut tree = Segtree::new(number_list.len());
        let n = number_list.len();
        
        // build the tree
        tree.build_tree(&number_list, 0, n - 1, 1); 

        for query in queries{
            if query.0 == 1 {
                tree.update_position(query.1 as usize, query.2, 0, (size - 1) as usize, 1);
            } else {
                answers.push(tree.get(query.1 as usize, query.2 as usize, 0, (size - 1) as usize, 1));
            }
        }
        answers
        // return the max value of range [left_search, right_search]
    }

    // main process
    pub fn process(&mut self){
        self.input();
        for attribute in &self.attribute_list{
            let mut results = self.solve(&attribute);
            self.results.append(&mut results);
        }
        self.output();
    }
}