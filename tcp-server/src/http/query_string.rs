use std::collections::HashMap;

// a=1&b=2&c=1&c=3

#[derive(Debug)]
pub enum Value<'buf>{
  Single(&'buf str),
  Multi(Vec<&'buf str>),
}

pub struct QueryString<'buf>{
  data: HashMap<&'buf str,Value<'buf>>
}

impl<'buf> QueryString<'buf>{
  pub fn get(&self, key: &str) -> Option<&Value>{
    self.data.get(key)
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
  fn from(s: &'buf str) -> Self{
    let mut data = HashMap::new();

    for param in s.split("&"){
      // split on =
      let v:Vec<&str> = param.split("=").collect();

      let e = data.entry(v[0]);
      println!("{:?}", e);
      // e.and_modify(|val: &mut Value| match val{
      //       Value::Single(prev) => {
      //         // let mut vec = vec![prev,v[1]];
      //         *val = Value::Multi(vec![v1]);
      //       }
      //       Value::Multi(vec)=>{vec.push(v[1])}
      //     }
      //   ).or_insert(Value::Single(v[1]))

      // data.insert(v[0],v[1]);
    }
    // return
    QueryString{data}
  }
}

