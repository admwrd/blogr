# Rust Web Apps Using Rocket

## Rocket
I have been using [Rocket](https://rocket.rs/) and I highly recommend using it.  It requires using a nightly build of Rust so it may be a little more unstable, although I have encountered few problems, and the ones I have encountered where mostly fixed by changing the nightly to a newer or older version.

## Design
The design of your web app will vary but there are some common elements that I would recommend:
- User login
- Templates (Rocket supports Tera and Handlebars - I prefer Handlebars)
- Database (My personal preference is Postgresql)
- Pagination (not suitable for all applications)
- Compression
- Setting http expiration headers, and other headers
- Hit Counter

## User Logins
I created a Rust crate named [Rocket-auth-login](https://crates.io/crates/rocket-auth-login) to help with managing logins.

See the article on [Using Rocket-auth-login](https://vishus.net/content/rocket-auth-login.rs)

## Database

[Database Module Usage Example in Rust](https://vishus.net/content/rocket-dbconn-example.rs) 
[Database Module Example](https://vishus.net/content/database.rs)

If you will accessing a database frequently the [R2D2](https://github.com/sfackler/r2d2) crate is wonderful.  R2D2 is a connection pool - it manages multiple open database connections that can be reused as opposed to creating a new connection whenever one is needed and throwing it away afterwards.

In my web app I created a database module which includes a database structure.  The database structure incldues a Request Guard to allow easy access to database connections from your routes' functions.  The Request Guard implements `FromRequest` allowing you to do something like:
```rust
#[get("/database")]
pub fn database(conn: DbConn) -> Html<String> {
    // ...
}
```

Note the DbConn struct which will assign `conn` a database connection from the connection pool.

See the full example:
[Database Module Usage Example in Rust](https://vishus.net/content/rocket-dbconn-example.rs) 

See an exmample of the database module:
[Database Module Example](https://vishus.net/content/database.rs)

The example demonstrated retrieving multiple rows or a single row form a Postgresql database connection.  The example uses the next exmaple's `DbConn` Request Guard:

## Templates
For anything other than simple webpages you should be using templates.  I have adopted handlebars templates, and it has made my life (and my blog) much more simple and powerful.

How you use templates may differ greatly from how I use them.  In the end this is merely an example, that is why there is no example module for using templates.

I created a single templating function.  The function take an enum indicating which template to call and some basic information present on every page. The full parameter list of my template function is:
- `content: TemplateBody` - takes an enum indicating what type of page is being requested and the contents of that page.  
    - Currently I have templates for general pages, a single article, a multiple article pages, search results pages, login pages, create article pages, edit article pages, manage articles pages, and tag cloud pages.
- `msg` - an optional field that when present will display a flash message at the top of the page
- `title` - the title to display in the &lt;title&gt; tag
- `page` - indicates the current url, which is used to check against each menu item to highlight the menu item of the current page
- `admin_opt` - if present provides details of a logged in administrator
- `user_opt` - if present provides details of a logged in regular user
- `javascript` - optional field that displays some javascript to be run at the end of the page
- `gen` - optional field representing the `instant` the route was called, this is used to calculate and display the time it took to generate the page

Each enum variant has its own new() method to generate a context to pass to the template's render() method.  The context is a struct containing all of the information that can be used by the template, such as page information, menu items, user information, generation time, flash messages, etc.

## Compression And HTTP Headers

[Accept Module](https://vishus.net/content/accept.rs)
[Xpress Module](https://vishus.net/content/xpress.rs)

Compressing pages before sending them can result in a nice performance gain, depending on the size and content of the file.  When I created my blog I wanted to be able to send compressed pages, as well as set the expiration headers of static files so client browsers will cache the content and improve performance.  To make setting headers and compressing content easier I created a module I called xpress.

Before I get into the xpress module you first need a module to determine what compression methods (the AcceptEncoding field of the request) that the user accepts.  I copied some code from [Michael Murphy](https://github.com/mmstick) (which no longer seems to be online) and modified it slightly to create my accept module.

[Accept Module](https://vishus.net/content/accept.rs) - determines which compression methods can be used and choses the best available method
[Xpress Module](https://vishus.net/content/xpress.rs) - takes some content (String, byte vector, File, NamedFile, or Template) and provides access to methods to compress the data and modify the headers

```rust
#[get("/compressed/<foo>/<bar>")]
// use the AcceptCompression's Request Guard like:
pub fn compressed(foo: u32, bar: String, encoding: AcceptCompression) -> Express {
    let html = format!("{}: {}", foo, bar);
    // Convert the String (or Template, or byte vector, or file/NamedFile) into an Express object
    let express: Express = html.into();
    // Set compression method to the best available method
    // The available methods are defined in the encoding variable
    // Then the preferred() method is called from the accept module which chooses and executes the best available compression algorithm
    express.compress( encoding )
    // You can also chain further method calls onto it like:
    .set_ttl( 60*60 ) // set cache for one hour
    .set_streamed() // force the content to be streamed in chunks
    .unset_streamed() // force the content to be sent all at once
    .add_extra("X-Powered-By", "Rust and Rocket")
}


```

## Hit Counter
[Counter Module](https://vishus.net/content/counter.rs)

WHen you are running a website it is often beneficial to know how much traffic each pages gets.  I created a counter module for this purpose.  The module will track the number of hits each page gets and every so often write the data to a file.  It will also track the total number of hits the website recieves and write that to a file.  Upon app start the two files are loaded to start off approximately where the app left off.  A few htis may be lost but most of the data should remain intact.

```rust
#[get("/something")]
pub fn something(hits: Hits) -> Html<String> {
    format!();
}

```

See my counter module: [Counter Module](https://vishus.net/content/counter.rs)

## Pagination

[Pagination Module](https://vishus.net/content/pagination.rs)

Pagination can be tricky.  You often want the pagination generalized enough to allow any page to use it, but different pages may need different settings.  If you know there will be many items which will generate many pages you may want the page navigation to display more pages than something with very few items.  You may also want to customize the links.

I created a pagination module that uses two structs:
- One struct containing the current page number, the route's uri(obtained automatically),  as well as an instance of the next struct
- A second struct containing the number of items per page currently selected by the user (or the default items per page)

The first struct is a Request Guard meaning you can list it in your route's list of parameters and it will run the Request Guard to make an instance of the data structure.  The first struct takes a generic parameter that implements the Collate trait.  he second struct implements the Collate struct and is used as the generic parameter for the first.  The second struct contains the currently selected items per page and implements the Collage trait, which define several configuration methods.

Essentially the first method holds basic information about the page as well as a struct that defines certain settings.  You can define new structs that implement Collate in order to change the default settings and define urls.  Currently the HTML is not very flexible but that could be changed without any large changes to the structure of the module.

Usage of the pagination module:

```rust
#[get("/articles")]
// Page is the Request Guard, and Pagination specifies some settings
pub fn show_articles(pagination: Page<Pagination>) -> Html<String> {
pub struct Article { id: u32, title: String,, contents: String }
let articles = vec![ 
    Article{ /* fill in contents here */ }, 
    /* add more articles here */ 
];
for article in &articles {
   let (ipp, cur, num_pages) = pagination.
}

```

The full pagination module [Pagination Module](https://vishus.net/content/pagination.rs)