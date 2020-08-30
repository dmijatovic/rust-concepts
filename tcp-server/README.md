# TCP server

This is basic (single threaded synchron) http server made from scratch. The basis is based on [Udemy training project](https://udemy.com/course/rust-fundamentals/learn/lecture/20695672#overview)

## Lessons

- Struct
- Enum

## Modules

When making modules structure with sub modules, the parent module can be a folder having mod.rs file at the root. In our example http module is parent to method and request, response modules.

All methods and modules are by default private. To be able to access module and methods ise keyword `pub`. For example `pub mod request`, `pub fn new()` etc.

## Net module

The [standard net module](https://doc.rust-lang.org/std/net/index.html) provides basics tcp functionality.

We use TcpListener to listen on a port.

### Taking incoming connections

We can label loops and break

```rs
//infinite labeled loop
'outer: loop{
  'inner: loop{
    break 'outer';
  }
}
```

## Returning multiple values from function

In rust we use tuple to return multiple values from fn. Tuple makes possible to return different types.

```rs

fn run()->(i32,&str,std::net::TcpListener){
  retrun (234,"abc",socket);
}

```

## TryFrom trait for conversion

Standard trait for converting between types is From or TryFrom (if it can fail). We use TryFrom in this project to convert stream of bytes into a request structure. See Request TryFrom implementation and use in server.rs

TryFrom requires implementation of custom error. As it might fail TryFrom returns Result<Self,Self::Error> with itself or custom self::error. The custom error can be defined as String (message) or as more complex struct.

## Custom Errors

The custom errors need to implement standard error. The standard Error trait can only implemented for types that also implement Debug + Display (see trait signiture).

For implementation see http/error.rs module which implements ParseError.

## Lifetime

Then working with references it is required to define lifetime for references in order to avoid dangling references.

```rs
// example lifetime definition on struct
// because it uses string pointers (&str)
#[derive(Debug)]
pub struct Request<'buf>{
  path: &'buf str,
  params: Option<Params<'buf>>,
  method: Method,
  body: &'buf str,
}
```

## Writing to stream

In the std::io library Read and Write traits are implemented for number of different usecases, file, stream etc. As stream also implements write we can use write! macro to write to a stream. This info can be seen at implementors section on [std docs site](https://doc.rust-lang.org/std/io/trait.Write.html#implementors).

## Traits

Traits are Rust specific way for implementing interfaces (part of functionality that can be implemented later or by foreign code/app).

### Passing traits as parameters of method

Passing trait as parameter of an method makes it possible to provide custom implementation of specific functionality. In this app for response we provide stream parameter in 'naive' implementation. We can replace this with more generic Write trait.

```rs
//naive implementation with direct link to TcpStream
pub fn send(&self, stream: &mut TcpStream)->IoResult<()>{
  unimplemented!
}

//dynamic dispatch trait implementation
pub fn send(&self, stream: &mut dyn Write)->IoResult<()>{
  unimplemented!
}

//static trait (more efficient that dynamic but app size might grow)
pub fn send(&self, stream: &mut impl Write)->IoResult<()>{
  unimplemented!
}
```

## Environment variables

Environment variables in rust can be accessed by std::env

```rs
// get env var
let public_path = env::var("PUBLIC_PATH").unwrap();

```

Passing env variable from bash (command prompt)

```bash
# from bash
PUBLIC_PATH=$(pwd)/public caro run
```

Cargo sets number of default env variables.

```rs
let app_path = env!("CARGO_MANIFEST_DIR");
```
