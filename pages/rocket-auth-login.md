# Using Rocket-auth-login

## Rocket-auth-login
User logins are a very common aspect of many web apps. The [Rocket](https://rocket.rs/) framework allows you to design web apps in very cool ways. If you are not familiar with it I would read the [Rocket Guide](https://rocket.rs/guide/).

After trying the [Rocket-simpleauth](https://crates.io/crates/rocket-simpleauth) crate I decided I need more flexibility in the authentication process.  In particular, when the login fails I wanted to know why the it failed (was it an invalid username, a wrong password, a locked account?).  I also wanted to be able to have multiple user types like administrators and regular users.
After using the Rocket-simpleauth crate for some time I decded to create my own login system, so I wrote the  [Rocket-auth-login](https://crates.io/crates/rocket-auth-login) (see also [Rocket-auth-login on Github](https://github.com/vishusandy/rocket-auth-login)) crate.  Hopefully it will give enough flexibility, although it comes at the expense of a more complicated setup.


## Rocket-auth-login Examples
The Rocket-auth-login crate has several examples:
[Rocket-auth-login Examples](https://github.com/vishusandy/rocket-auth-login/tree/master/examples)

## Implementing Rocket-auth-login
Rocket-auth-login requires two structs:
1. A cookie struct - it will hold any information you want instantly available in the cookie - like userid/username
2. A form struct - which will hold the information from the login form

The cookie requires a store_cookie() and a retrieve_cookie() method, and allows you to override the default implementation for delete_cookie().

To use the Rocket-auth-login crate you must implement:
Cookie struct (implemented under the ==AuthorizeCookie== trait):
- `store_cookie(&self) -&gt; String`
- `retrieve_cookie(String) -&gt; Option&lt;Self&gt; where Self: Sized`
- `Optional: delete_cookie()`

Form struct (implemented under the ==AuthorizeForm== trait):
- `uthenticate(&self)` - Determines whether the credentials passed to it are vaild; returns either a new cookie struct or an AuthFail struct indicating which user attempted to login and why the login failed
- `new_form()` - creates a new form struct with the given username, password, and extra fields
- Optional: `fail_url()` - determines the url the user is redirected to when authentication fails
- Optional: `clean_password()` - sanitizes the password
- Optional: `clean_extras()` - executed on each extra field
- Optional: `flash_redirect()` - calls the authenticate() method then determines the page to redirect to depending on successful authentication or failure.
    - This allows you to display error messages indicating why the authentication failed as well as customizing the redirect url to allow the username to persist (by using query strings) as well as redirection to a referrer upon success
- Optional: `redirect()` - same as flash_redirect() except is does not specify a FlashMessage (a temporary one-time-use cookie that is deleted as soon as it is accessed) indicating why the authentication failed.  This only determines the address to redirect to
- Finally the cookie struct should implement `FromRequest` to retrieve the information in the cookie for a route.  This is not in the AuthorizeForm trait.


### Advanced Authentication
My blog uses a more complicated authenticate() method.  I have regular users and admin users, distinguished by an `admin` column.  For security I track the number of incorrect logins before a valid login, and every x failed logins I will lock the user account for y minutes.  I accomplish this by adding an `attempts` field and a `lockout` field to the user table.  This prevents brute force attacks.  If a brute force attack is detected the account is permanently locked and must be unlocked by an admin.

The `attempts` field is simply an intger counting the number of failed logins between successful logins.  When the account is locked the `attempts` column does not reset, it only resets upon succssful login.

The `lockout` column is a datetime stamp indicating when the user can login next.  The query used to determine whether the credentials are valid contains a WHERe clause that checks for a blank `lockout` column.

If the `lockout` column is not empty the first query will fail, proceeding to the next SQL query which will verify the user is in fact an admin (and not a regular user), followed by ensuring the user is an actual user, and finally checking to see if the password is correct.

## Multiple User Types
For multiple user types simply create another cookie and form structs and implement the same methods for them.  Supporting multiple types of users was one of the core reasons I wrote the Rocket-auth-login crate.


## Database
If you are connecting to a database you will need to access the connection from inside the `authenticate()` method.  To accomplish this I used a mutex (since Rocket is multithreaded) inside a lazy_static block.
```rust
lazy_static! {
    static ref PGCONN: Mutex<DbConn> = Mutex::new(
        DbConn(
            init_pg_pool().get().expect("Could not connect to database.")
        )
    );
}
```

See the [Rocket Database Module Example](https://vishus.net/content/database.rs)
And the [Rocket Database Module Usage Example](https://vishus.net/content/rocket-dbconn-example.rs)



