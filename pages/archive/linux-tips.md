# Linux Tips And Tricks

I compiled a small list of useful commands and tips for working with ubuntu.  Hopefully these will be of use to you, especially if you are just getting started with linux or webservers.

## Network Commands

#### SSH
`ssh user@123.45.67.89`

#### Download A File
`curl -o output.file https://input.io/`

## Files And Directories
#### List all files (excluding . and ..)
ls -lA /path/to/folder
- -l long listing format
- -a show all items even . prefixed ones
- -A show all items except . and ..

#### Show size of a directory
du -s /path/to/folder
- -s summary, otherwise will show every item in the folder
- -h human readable sizes

#### Copy files
`cp <source> <destination>`
- -r recursive copy
- -u only copy updates (files that are newer in the source than in the destination)
- --preserve preserves the timestamp, mode, ownership, etc
- -n do not overwrite an existing file
- -L follow symbolic links

## Compressed Archives

#### Compress A Folder
The `tar -cvjSf` compresses a folder using bzip2
`sudo tar -cvjSf output_archive.tar.bz2 /path/to/folder`
- c create a new .tar arhcive
- v verbosely show the .tar file progress
- f specify the tar archive name
- j specifies bzip2
    - Instead of S you could specify z to use gzip
    - The j can be omitted to create a .tar archive without compression
- S detect spareness / holes in files

#### Decompress An Archive
`sudo tar -xvjf input_archive.tar.bz2 /path/to/folder
- x extracts files from an archive
- v verbose output of the progress
- j use bzip2
- f specify the tar archive name

## Processes

#### Process Info
`ps -ef | grep service`

#### Kill A Process
`sudo kill -9 <pid>`

#### Run command with different priority
-20 is the highest priority
`nice -n -20 <command>`
19 is the lowest priority
`nice -n 19 <command>`

## Memory

#### Show Memory Usage
- `free -m`

#### Memory Info
- `cat /proc/meminfo`

#### Virtual Memory Stats
- `vmstat -s`

## Disk Space

#### Show Free Disk Space
`df -h`

#### Show Size Of File(s) In Directory
`du -h -s -a`
- -a shows all files not just directories
- -h shows human readable sizes
- -s show total size instead of listing each entry and its size

#### Show Top 10 Largest Directories
To show the top 10 largest directories in a given folder use:
`du -a /path/to/folder | sort -n -r | head -n 10`

## User Management
#### Create user
`sudo adduser <username>`

#### Change password
`sudo passwd <user>`

## Group Management

#### Create user group
`sudo groupadd some-group`

#### Display members of a group
`grep some-group /etc/group`

#### Add user to group
`sudo usermod -a -G <groupname> <username>`
- -a appends the group to the list of groups the user is currently a member of, omitting this removes all groups and replaces them with the specified group
    - For older systems like Ubuntu 14.04 use the following:
    `sudo gpasswd -a <user> <group>`

#### Remove user from group
`sudo deluser <user> <group>`

#### Change a file's group
`sudo chgrp -R www-data /path/to/folder`
or
`sudo chmod -R g+w /path/to/folder`

#### Change owner of a file or directory
`chown <user>:<group> /path/to/file`
`chown <user>:<user> /path/to/file`
`chown <user> /path/to/file`

## Permissions
#### Change file/directory permissions
`sudo chmod 777 -r /path/to/file`


## SSL/TLS
#### Generate a secret key for Rocket
`openssl rand -base64 32`

#### LetsEncrypt Renew Certificate Cron Job
```bash
letsencrypt renew --pre-hook "service nginx stop" --post-hook "service nginx start >> /var/log/letsencrypt.log
```

## NGINX
#### Restart the webserver
`sudo systemctl restart nginx`

#### Reload the configuration without dropping connections
`sudo systemctl reload nginx`

#### Stop the webserver
`sudo systemctl stop nginx`

#### Check the configuration file
`sudo /usr/sbin/nginx -t -c /etc/nginx/nginx.conf`

#### Enable virtualhost site
`sudo ln -s /etc/nginx/sites-available/<your_site> /etc/nginx/sites-enabled/<your_site>`

#### Disable virtualhost site
`sudo rm /etc/nginx/sites-enabled/<your_site>`
> Note: you may need to disable the default virtual host

## Packages
#### List files installed by a package:
`dpkg -L <package>`
Note: this will not list files installed by dependencies.

#### List dependencies installed by a package:
`apt-cache show <package>`

NGINX will install nginx, nginx-core, and nginx-common.  So to g et a full list of files installed by nginx you must run `dpkg -L` on each of the three packages.


## Miscellaneous Commands

#### Reboot
`sudo reboot`

#### Keep SSH command running after disconnect
nohup prog &
- the & is the background operator
- See also [Stackoverflow - How to kill a nohup process](https://stackoverflow.com/questions/17385794/how-to-get-the-process-id-to-kill-a-nohup-process)

## Database - Postgresql
#### Access postgresql
`psql \connect <database>`
or
`psql \c <database>`

#### Change password
`ALTER USER postgres WITH PASSWORD 'password';`

#### Version
`psql --version`

## Make A Program A Service
#### Show all services
`service --status-all`

#### Add service to startup
`update-rc.d <service> defaults`

#### Remove service from startup
`update-rc.d -f <service> remove`

#### Make the service config entry
Create a file:
==/etc/systemd/system/&lt;service_name&gt;.service==

with the conents:
```conf
[Unit]
Description=My Service
After=postgresql.service

[Service]
ExecStart=/bin/bash /path/to/script/to/run
WorkingDirectory=/path/to/script/folder
Restart=always

[Install]
WantedBy=multi-user.target
```
> Note: the `After=postgresql.service` tells the system that the service should be loaded after the specified service (postgresql for example, which could be useful since some apps may require a database)

#### Reload the service daemon
`sudo systemctl daemon-reload`

#### Stop a service
`sudo systemctl stop <service>`

#### Start a service
`sudo systemctl start <service>`

#### Run the service on startup
Create the following file:
==/etc/rc.local==
```bash
#!/bin/sh -e
#
# rc.local
# This script is executed at the end of each multiuser runlevel.
# Make sure that the script will "exit 0" on success or any other
# value on error.
# In order to enable or disable this script just change the execution bits.

/path/to/script.sh

exit 0
```
## Useful Files And Directories
#### Nginx
- /etc/nginx - base nginx folder
- /etc/nginx/nginx.conf - nginx base configuration file
- /etc/nginx/sites-available - configuration for websites
- /etc/nginx/sites-enabled - symbolic links to sites-available entries

