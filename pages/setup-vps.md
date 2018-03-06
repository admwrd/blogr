# Configuring A VPS Webserver

This article is intended to help create a linux webserver, especially an Ubuntu webserver using NGINX as a reverse proxy for a web application written in a language like Rust or NodeJS.

The webserver will:
1. Take requests from clients.
2. Determine if the request should be served by the web application or if it is a static file (like js/css/image/etc).
	- If the client is requesting a static file the webserver the file is retrieved by NGINX which becomes the response.
	- Otherwise the request is forwarded to the web application which responds to the request and sends it back to NGINX.
3. The response will be sent back to the client.  If the NGINX configuration allows HTTP 2 the response will be sent using HTTP 2, even if the web application's original response was sent back to NGINX using HTTP 1.x (it will convert the reponse to HTTP 2).

This is based on my experience setting up my Linode VPS.  I have an Ubuntu 17.10 server running an NGINX webserver as a reverse proxy with TLS enabled and a Postgresql database; your server may need a different setup depending on your needs.

> Note: This is not a definitive step by step guide to setting up a linux webserver, there are many great articles available for that purpose; this article will try to focus on how to setup a reverse proxy webserver that is intended for use with a web application written in languages like Rust or Javascript (NodeJS); I will try to post some relevant links where my instructions are lacking or too general to be very useful for someone with little linux/webserver experience.



## Select A Virtual Private Server
First you need to purchase Virtual Private Server (VPS).  I was recomended [Linode](https://www.linode.com/?r=b0738c61551a05bb0e66386a797c23c1cbf49da6) (referral link) by a good friend of mine and have loved them.  They have some very good helpful articles on how to setup your server.  I followed some of those articles when setting up my server and will link to some of the articles I used.

Make sure to edit your host file (/etc/hosts), I added a line like:
```bash
127.0.1.1    vishus.net    www.vishus.net    blog
```
Here are some articles useful articles to get you started:
> Note: The Getting Started guide has instructions for setting up your server using Linode, however many of the other articles should have information that will work on any VPS.

- [Linode - Getting Started Guide](https://linode.com/docs/getting-started/)
- [Linode - Guides and Tutorials](https://www.linode.com/docs/)
- [Linode - Beginner's Guide](https://linode.com/docs/platform/linode-beginners-guide/)
- [Linode - Basic Linux Administration](https://linode.com/docs/tools-reference/linux-system-administration-basics/)
- [Linode - Linux Users and Groups](https://linode.com/docs/tools-reference/linux-users-and-groups/)
- [Linode - How To Secure Your Server](https://www.linode.com/docs/security/securing-your-server/)
- [Linode - Advanced OpenSSH](https://www.linode.com/docs/security/advanced-ssh-server-security/)
- [Linode - SSH on Windows Using PuTTY](https://www.linode.com/docs/networking/ssh/ssh-connections-using-putty-on-windows/)

## Install Packages

Your setup may vary depending on your needs but there are a few programs I would recommend regardless.

##### Compatibility
> Warning: the commands and information on this page may not be 100% accurate depending on your linux distribution, operating system version, software/environment setup and network setup.  If you are not familiar with linux or webservers I would recommend reading some of the links and/or searching on how to do each task.

Once you have chosen an operating system and setup ssh I would recommend installing the following:
> Note: the examples are assuming you are using Ubuntu or a similar operating system.  I am showing the commands I used on my Ubuntu 17.10 server, if you are using an older Ubuntu installation you may have to use apt-get instead of apt or other slight variations in the commands.

- GCC
    - GCC is a very useful program to have installed for many reasons; some other packages will require it, but also as a developer tool it I recommend to have it installed.
- 
``` bash
sudo apt update
sudo apt upgrade
sudo apt install build-essential
```

- Git
    - Version control is an essentail developer tool.  To install Git use:
```bash
sudo apt update
sudo apt install git
```

## NGINX
I decided to use NGINX for performance reasons.  My Rust web app uses the [Rocket Web Framework](https://rocket.rs/) and because it uses Hyper it does not support HTTP2.  To get around this I use NGINX, which is a good idea anyways as the webserver in the Rust apps are fairly basic and not as mature or feature rich as Apache or NGINX.

Using NGINX or Apache as a reverse proxy allows all incoming requests to go to NGINX (or whatever) first; if the file is a CSS/JS/image file (a static file) it can be served directly from NGINX, and all html/other content can be forwarded to the Rust application.
```bash
sudo apt install nginx
```
Your configuration file of NGINX will be customized to match your production environment.  I chose to enable HTTP2, TLS/SSL, and Gzip compression.

My configuration file is fairly simple since I have a very limited number of file types that I will be using, which allows me to tell NGINX to serve files with a JavaScript/CSS/images/etc extension (static files) and all other requests will be sent to the Rust web app.

Compression will be disabled for compressed archvies, images, and movies  due to their high entropy.  

#### NGINX Configuration
Your configuration may vary greatly than the examples I will show here.  For some more examples of NGINX configurations see:
- [H5BP NGINX Configs](https://github.com/h5bp/server-configs-nginx)

I would highly recommend securing your website with HTTPS since you can now get TLS/SSL certificates for free using LetsEncrypt.  If you handle passwords at all you need to use HTTPS, there is no good way to secure the user's credentials without using a public key encryption algorithm like TLS uses.  Hashing a password will only keep others from immediately knowing what the original password was; with enough computing power and/or lookup tables for common hash algorithms the hash can be reversed.

Here is an example of an NGINX configuration similar to mine:

== /etc/nginx/sites-available/blog ==

```conf
# This first server block will forward the users to the https 
# version of the website, ensuring the website is secured
server {
    listen 80; 
    server_name your_domain.com;
    include /etc/nginx/snippetsletsencrypt.conf;
    
    location / {
        return 301 https://vishus.net$request_uri;
    }
}

# This block contains settings for the https version of the website.
server {
    server_name your_domain.com;
    # Enable HTTP2 as well as HTTPS on port 443
    listen 443 ssl http2 default_server;
    # Enable IPv6
    listen [::]:443 ssl http2 default_server ipv6only=on;
    
    # Enable TLS/SSL using LetsEncrypt
    ssl_certificate /etc/letsencrypt/live/your_domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your_domain.com/privkey.pem;
    ssl_trusted_certificate /etc/letsencrypt/live/your_domain.com/fullchain.pem;
    include /etc/nginx/snippets/ssl.conf;
    
    # Specify settings for compressable files (files with low entropy)
    location ~* \.(css|js|ttf|otf|txt)$ {
        # Specify your root folder for web documents
        root /var/www/html;
        # Allow the client to cache the static files
        expires 60d;
        # Enable Gzip compression
        gzip on;
        # Specify a minimum file length when compressing files
          # small files are often not worth compresing as 
          # the resources required do not justify the small
          # amount of space saved by the compression
        gzip_min_length 1000;
        gzip_types
        	text/plain
            application/javascript
            text/javascript
            application/x-font-ttf
            font/opentype
            text/css
    }
    
    # Settings for files that should not be compressed
    location ~* \.(png|gif|jpg|jpeg|zip|tar|bz2|gz|7z|mp4|mkv|m4v) {
        root /var/www/html;
        expires 60d;
        gzip off;
    }
    
    # Send all other requests to the web application
    location / {
       proxy_pass https://localhost:8000;
    }
    

}
```
Then create two snippet config files for an SSL setup (taken from the wonderul tutorial at [The Code Ship](https://www.thecodeship.com/web-development/guide-implementing-free-ssl-certificate-nginx-lets-encrypt/)):

== /etc/nginx/snippets/ssl.conf ==
The `ssl.conf` article will hold some settings to configure SSL/TLS.  The following will enable [OCSP Stapling](http://blog.mozilla.org/security/2013/07/29/ocsp-stapling-in-firefox) which securely checks if the requested certificate has been revoked.  A certificate may be revoked if it was created with inaccurate information, the ownership of the website was transferred, or a hacker gains access to the private key.

```conf
ssl_session_timeout 1d;
ssl_session_cache shared:SSL:50m;
ssl_session_tickets off;

ssl_protocols TLSv1.2;
# Specify the available ciphers
ssl_ciphers EECDH+AESGCM:EECDH+AES;
# Specify the eliptical curve algorithm
ssl_ecdh_curve secp384r1;
ssl_prefer_server_ciphers on;

ssl_stapling on;
ssl_stapling_verify on;

add_header Strict-Transport-Security "max-age=15768000; includeSubdomains; preload";
add_header X-Frame-Options DENY;
add_header X-Content-Type-Options nosniff;

# =================
# OPTIONAL SETTINGS
# =================

# Omit the NGINX version on errors pages and in response headers.
server_tokens off;
add_header X-XSS-Protection "1; mode=block";
# Enable HSTS
add_header Strict-Transport-Security "max-age=63072000; includeSubdomains"; 

```

See [Mozilla - HSTS](https://developer.mozilla.org/en-US/docs/Security/HTTP_Strict_Transport_Security) for more information on HTTP Strict-Transport Security

== /etc/nginx/snippets/letsencrypt.conf ==

The following servers HTTPS challenges (see [The Code Ship article](https://www.thecodeship.com/web-development/guide-implementing-free-ssl-certificate-nginx-lets-encrypt/))
```conf
location ^~ /.well-known/acme-challenge/ {
    default_type "text/plain";
    root /var/www/letsencrypt;
}
```

You should also run the following command to create the hidden directory.  This directory is where the challenges are served from.

```bash
sudo mkdir -p /var/www/letsencrypt/.well-known/acme-challenge
sudo service nginx restart
```

> Note: this is just one way of configuring the server.  This approach serves only files with specific extensions and forwards all other requests to the web app.  While this approach works for me it may not for you.  Another approach would be to send html files (or a custom extension) to the web app and try to serve all other files.  You couuld even send all requests to the web app and have the web app handle static files.  Completely up to you, this configuration should hopefully provide a good example.  See also: 
- [Nginx - Try_files](http://nginx.org/en/docs/http/ngx_http_core_module.html#try_files)
- [Digital Ocean - NGINX Server Blocks](https://www.digitalocean.com/community/tutorials/understanding-nginx-server-and-location-block-selection-algorithms#when-does-location-block-evaluation-jump-to-other-locations)
- [Server Fault Try_files Explanation](https://serverfault.com/a/329970)

Some helpful NGINX articles:
- [Linode - NGINX Reverse Proxy]()
- [Linode - NGINX Setup](https://linode.com/docs/web-servers/nginx/nginx-installation-and-basic-setup/)
- [Linode - NGINX Configuration](https://linode.com/docs/web-servers/nginx/how-to-configure-nginx/)
- [ScaleScale - Securing NGINX](https://www.scalescale.com/tips/nginx/nginx-security-guide/)


## Generating An SSL Certificate
To get HTTPS up and running on your website you will need to generate a certificate for your domain.  I found a great article on The Code Ship that explains the process better than I could.  Instead of trying to summarize the article I used I am just going to refer you to the website:

[The Code Ship - SSL Certificate on NGINX](https://www.thecodeship.com/web-development/guide-implementing-free-ssl-certificate-nginx-lets-encrypt/)

You will most likely want to setup LetsEncrypt in Webroot mode which will serve challenges from the ==.well-known== folder (or another folder depending on your configuration; the above configuration specifies ==.well-known== for serving SSL challenges).  The other mode is standalone which will bind to port 80 to serve requests without having a webserver running like NGINX or Apache.

[Ubuntu - Certificates](https://help.ubuntu.com/lts/serverguide/certificates-and-security.html)

## FTP
FTP is an easy to use tool to help you manage the files on your server.  I use FTP to transfer updates to my program and update static files.  On Ubuntu I use VSFTPD:

```bash
sudo apt install vsftpd
```

Then edit the ==/etc/vsftpd.conf== configuration file to enable uploads from system authenticated users (find the `write_enable` section and change it to `YES`):

```conf
write_enable=YES
```

Also consider enabling SSL by finding the following directives and change them to:

```bash
ssl_enable=YES
rsa_cert_file=/etc/ssl/certs/ssl-cert-snakeoil.pem
rsa_private_key_file=/etc/ssl/private/ssl-cert-snakeoil.key
```

Restart the FTP server:
```bash
sudo restart vsftpd
```

[Ubuntu VSFTPD](https://help.ubuntu.com/lts/serverguide/ftp-server.html)
[Digital Ocean - Install VSFTPD on Ubuntu 16.04](https://www.digitalocean.com/community/tutorials/how-to-set-up-vsftpd-for-a-user-s-directory-on-ubuntu-16-04)


