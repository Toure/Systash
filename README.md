# Systash

![N|Systash](https://upload.wikimedia.org/wikipedia/commons/4/43/Stash_Logo.png)

Backup solution for distributed compute platforms.

  - Ansible
  - Rust
  - Magic, just a little

### Distributed Backups

Systash looks to solve an important task facing current and future deployed cloud
infrastructure, which is data protection. Just as we rely on journal operations on our
filesystem to provide data security, there should be that same effort taken for deployed
systems through the use of system imaging and continuos backups.


### Tech

Systash uses a number of open source projects to work properly:

* [Ansible] - software provisioning, configuration management, and application-deployment tool.
* [Rust] - fast memory safe programming language.

### Installation


Install:

```sh
$ git clone https://github.com/Toure/Systash.git
$ cd Systash
$ cargo build --relase
$ cargo install
```
Edit the configuration file: (with your favortie editor)

### Example Content [systash.conf]:

```
[Server]
host = "undercloud"
ip = "192.168.0.1"
system_group = "FO_North_01"

[[client]]
#client list defined here.

[client.node1]
host = "controller_0"
ip = "192.168.2.20"
system_group = "FO_North_01"

[client.node2]
host = "controller_1"
ip = "192.168.2.21"
system_group = "FO_Noth_02"

[storage]
archives = "/foo/storage"

[nfs_server]
host = "central_store"
ip = "192.168.2.254"

```
**nfs_server ip should correspond with the network which is addressable
by the controller nodes.

### Install packages on all nodes:
```sh
$ systash --install
```

### Configure NFS Server on defined server node:
```sh
$ systash --init storage --profile server
```

### Configure NFS Clients:
```sh
$ systash --init storage --profile client
``` 

### Perform the first systems backup:
```sh
$ systash --backup
```

### Generate a rescue image:
```sh
$ systash --rescue
```

### Development

Want to contribute? Great! Please submit PRs with patches or
open a bug as a feature request.

### Todos

 - Write Tests
 - Add features which I couldn't think about.

License
----

Apache v2


**Free Software!**

[//]: # (These are reference links used in the body of this note and get stripped out when the markdown processor does its job. There is no need to format nicely because it shouldn't be seen. Thanks SO - http://stackoverflow.com/questions/4823468/store-comments-in-markdown-syntax)

[toure]: https://github.com/Toure/Systash.git
