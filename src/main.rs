struct FilterCondition {
    filter_fn: Box<dyn Fn(&i32) -> bool>,
}

impl FilterCondition {
    fn new<F>(filter: F) -> Self 
    where F: 'static + Fn(&i32) -> bool {
        Self { filter_fn: Box::new(filter) }
    }

    fn is_match(&self, item: &i32) -> bool {
        (self.filter_fn)(item)
    }
} 

fn custom_filter(items: Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    items.into_iter().filter(|x| filter.is_match(x)).collect::<Vec<i32>>()
}

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; 
    let filter = FilterCondition::new(|x| x % 2 == 0); 
    let result = custom_filter(items, &filter);

    println!("Filtered items: {:?}", result);
}