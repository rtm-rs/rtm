# Prototype CQRS-04

Intermediate: https://github.com/get-eventually/eventually-rs/tree/main/examples/bank-accounting

## Example: Bank Accounting application

In this folder you can find an example Event-sourced application for a generic (and simple) Bank Accounting bounded context, implemented using the `eventually` crate.

This example application should be useful for people interested in:

1. Having a possible reference as to model a Domain Layer using the crate,
2. Starting out from a reference package and code structure,
3. Modeling Business Processes using Projections/Event Subscriptions.

### Usage

Build then run the service:

```bash
cargo build
```

We'll use [gRPCui](https://github.com/fullstorydev/grpcui) to interact with the
service:

```bash
podman run --interactive \
           --tty \
           --rm \
           --network="host" \
           --volume $(pwd)/proto:/protos \
           fullstorydev/grpcui -plaintext localhost:10437
```
