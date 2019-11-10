Graphene
========
Backup for the Cloud platforms, on premise, hybrid, hosted.

Distributed Backups
-------------------
Graphene looks to solve an important task facing current and future deployed cloud
infrastructure, which is data protection. Just as we rely on journal operations on our
filesystem to provide data security, there should be that same effort taken for deployed
systems through the use of snapshots.

Installation
------------
Installation will be handled through the package manager or compiled from source:
 - **> dnf install graphene-server**
 - **> dnf install graphene-client** 
 - **> yum install graphene**

### Or:

 - **> cargo build graphene**


Usage
-----
Graphene will allow users to set environmental option in a configuration file: graphene.conf, which 
will allow for ease of use for single machine archives.



 1. graphene


Supported Platforms
-------------------
Current planed support will be based on **RHEL/CentOS** and **Fedora**, other platforms
are welcomed through contributions.

* Openstack
* Openshift
* Kubernetes
