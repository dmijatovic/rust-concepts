# TLS actix demo with OpenSSL

This example is based on [actix demo](https://github.com/actix/examples/tree/master/rustls) with some additional remarks from my side.

I was not able to use same certificates I used with Go TLS server. This demo uses certificates provided with the demo. Further investigation is needed concerning why I was not able to use same certificates as with Golang app.

## Creating certificate

```bash
# make certificate
mkcert 127.0.0.1

```
