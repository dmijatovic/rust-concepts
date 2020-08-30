use std::collections::HashMap;

// a=1&b=2&c=1&c=3

// #[derive(Debug)]
// pub enum Value<'buf>{
//   Single(&'buf str),
//   Multi(Vec<&'buf str>),
// }

#[derive(Debug)]
pub struct QueryString<'buf>{
  data: HashMap<&'buf str, &'buf str>
}

impl<'buf> QueryString<'buf>{
  pub fn new()->Self{
    QueryString{
      data: HashMap::new()
    }
  }
  // pub fn get(&self, key: &'buf str) -> Option<&'buf str>{
  //   self.data.get(key)
  // }
  pub fn set(&mut self, key: &'buf str, val: &'buf str){
    self.data.insert(key, val);
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
  fn from(s: &'buf str) -> Self{
    // let mut data = HashMap::new();
    let mut qp = QueryString::new();

    for param in s.split("&"){
      // split on =
      let v:Vec<&str> = param.split("=").collect();
      // it will insert single value type
      // data.entry(v[0]).or_insert(v[1]);
      qp.set(v[0], v[1]);
    }
    // return
    // QueryString{data}
    return qp;
  }
}

//very simple hashmap to save received params

pub type Params<'buf> = HashMap<&'buf str, &'buf str>;

pub fn params_from_str(s: &str)-> Params{

  let mut qp = Params::new();

  for param in s.split("&"){
    // split on =
    let v:Vec<&str> = param.split("=").collect();
    // it will insert single value type
    // data.entry(v[0]).or_insert(v[1]);
    qp.insert(v[0], v[1]);
  }
  // return
  // QueryString{data}
  return qp;
}