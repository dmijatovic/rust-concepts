# TCP server

This is basic http server made from scratch. The basis is based on [Udemy training project](https://udemy.com/course/rust-fundamentals/learn/lecture/20695672#overview)

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


```
