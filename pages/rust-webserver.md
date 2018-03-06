# Rust Virtual Private Server

When I first started playing around with Rust web apps I had difficulty finding good tutorials on how to setup a Rust webserver.  So here is my attempt at getting some information out there.  I will try to keep this relatively short and focus on the general tasks to allow you to customize your server to your needs.  If you have any comments/suggestions/corrections please send them to <script type="text/javascript" language="javascript">show_contact();</script>.

## Getting Started
First you must have a Virtual Private Server with a webserver installed.  This article assumes an Ubuntu linux operating system with an NGINX webserver.  The specific linux distribution and webserver can be changed; your server may use another distribution or software - just modify any commands used here to your distribution's equivalent command.

This is just an example of one way of setting up a Rust webserver, your server may use a different linux distribution or webserver (Apache or Lighttpd instead of NGINX).  This is only intended to give you a starting point to build your server.

## Setup A VPS
I recommend using [Linode](https://www.linode.com/?r=b0738c61551a05bb0e66386a797c23c1cbf49da6)(my referral link).  There are other companies that offer Virtual Private Servers, the choice is yours - do some looking and compare prices and options.  I found Linode to be a fast and cheap option.  Linode has a handy Linode Manager and even a web based shell.

I created an article that lists some basic steps for configuring your webserver.  The article tries to generalize some steps but assumes an NGINX webserver and enabling SSL (since SSL certificates can be obtained for free they are being recommend, and may also boost your search engine ranking).

##### [Configuring A VPS Webserver](https://vishus.net/article/16/)



## Rust
- First install rustup:
```bash
curl https://sh.rustup.rs -sSf | sh
```
- Then install your desired version(s) of Rust.  I personally enjoy using nightlies (and using Rocket have to) as they can compile a wider range of code.  Here is an example of a nightly for linux:
```bash
rustup toolchain install nightly-2017-12-17-x86_64-unknown-linux-gnu
```
Or:
```bash
rustup toolchain install stable-x86_64-unkown-linux-gnu
```


## Database

- [Linode - Postgresql Setup](https://linode.com/docs/databases/postgresql/how-to-install-postgresql-on-ubuntu-16-04/)
- [Linode - Remote Postgresql Management](https://linode.com/docs/databases/postgresql/how-to-access-postgresql-database-remotely-using-pgadmin-on-windows/)

## Rocket.toml

#### Generate secret key
`openssl rand -base64 32`

#### Example configuration
If you are using HTTPS (highly recommended) make sure to change your paths to your certificates to the correct location.
```conf
[global]
workers = 8

[development]
address = "localhost"
port = 8000
log = "normal"
secret_key = "<secret key - see above command>"
limits = { forms = 32768 }
tls = { certs = "/path/to/private/fullchain.pem", key = "/path/to/private/privkey.pem" }

[staging]
address = "localhost"
port = 8000
log = "debug"
secret_key = "<secret key - see above command>"
limits = { forms = 32768 }
tls = { certs = "/path/to/private/fullchain.pem", key = "/path/to/private/privkey.pem" }

[production]
address = "localhost"
port = 8000
log = "critical"
secret_key = "<secret key - see above command>"
limits = { forms = 131072 }
tls = { certs = "/path/to/private/fullchain.pem", key = "/path/to/private/privkey.pem" }



```
> Note: the port is 8000 even in production because with a reverse proxy the webserver (NGINX/Apache/etc) is bound to port 80 and when a request comes in that should be processed by the Rust web app the webserver forwards the request to the web app which will send back a resposne (usually an html file).  Rocket will send a response using HTTP 1.x although NGINX will convert it to HTTP2 (if HTTP2 is enabled).

#### Run Rocket in staging or production mode
Rocket defaults to development mode.  You can change modes and execute the application with:
`sudo ROCKET_ENV=stage cargo run`
You can specify dev/development, stage/staging, or prod/production modes.

## Cargo.toml
When specifying Rocket as a dependency make sure to enable the tls feature if you are using SSL.
```conf
[dependencies.rocket]
version = "0.3.3"
features = ["tls"]
```

If you are using template make sure to include rocket_contrib:
```conf
[dependencies.rocket_contrib]
version = "0.3.3"
default-features = false
features = ["handlebars_templates"]
```

See my [Cargo.toml Example](https://vishus.net/content/Cargo.toml)
The example file shows some versions (maybe not most recent but close) of awesome or popular crates.

> Note: if you are using TLS you must enable the TLS feature in Rocket, see example







