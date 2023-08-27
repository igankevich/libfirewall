# Introduction

Libfirewall is a library that aims to protect your CI/CD pipelines from supply chain attacks.
The best way to protect your pipelines is to run runners on your premises behind a restrictive firewall and HTTP proxy server, however,
this is not realisitc for small teams.

Libfirewall offers easy-to-use alternative for the teams,
that do not have the infrastructure to run their own pipelines or
a DevOps engineer to setup firewalls and proxy servers.

Libfirewall restricts DNS resolution to a list of known DNS names
and a list of known DNS servers.

Currently libfirewall is in proof-of-concept stage and mitigations are easy to circumvent.
It is beneficial to use the library to understand
what hosts your pipelines access to build your projects.


# Usage

Add the following lines to your `Dockerfile`.

```dockerfile
RUN . /etc/os-release \
    && curl \
    --silent \
    --fail \
    --location \
    --output /usr/local/lib/libfirewall.so \
    https://github.com/igankevich/libfirewall/releases/download/0.1.0/libfirewall-$ID-$VERSION_ID.so
ENV LD_PRELOAD=/usr/local/lib/libfirewall.so
```

Then in your CI/CD pipeline define a list of allowed domain names.
By default all domain names are blocked!

```bash
export LIBFIREWALL_ALLOW='index.crates.io github.com'
```

If everything is well-configured you will see something like this in your CI/CD job output.

    $ cargo build
    Updating crates.io index
    libfirewall: allow index.crates.io
    ...

The same goes for any other command, not just `cargo`.


# Supported Linux distributions

Currently `libfirewall` is built for the following distributions.

| Distribution | Release | Glibc version |
|--------------|---------|---------------|
| debian | 11 | 2.31 |
| debian | 12 | 2.36 |
| ubuntu | 20.04 | 2.31 |
| ubuntu | 22.04 | 2.35 |

If your distribution is not in the list please file the issue,
or choose the one with the matching (or lower) glibc version.
Use `getconf GNU_LIBC_VERSION` to print your Glibc version.
In the end Glibc version is what matters, and distribution is irrelevant.
