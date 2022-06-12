# Research

Ideas and approaches to think about and explore.

## Template

Simple & flexible:

- [Git clone particular files](https://stackoverflow.com/a/52269934/152860)
- [Gather user input for config file (see next item)](https://crates.io/crates/enquirer)
- [Shell (bash) templating](https://stackoverflow.com/a/48633756/152860)

## Newtype

- [Newtype template](https://gist.github.com/nyinyithann/84aa6f013c18be8e87440f96f3e6f868)

## Data types

- [Flat-tree](https://github.com/mamcx/tree-flat) and [blog
  post](https://www.elmalabarista.com/blog/2022-flat-tree/): Possibly for use in
  holding data from `Relations` and `Aggregates`?

## Type-tightness

- [Type-tightness](https://www.ecorax.net/tightness/)

## Mutually exclusive traits

- [Exclusive traits implementation](https://geo-ant.github.io/blog/2021/mutually-exclusive-traits-rust/)

## Events

- [Event listener/dispatcher & DB/Web builder](https://willcrichton.net/notes/types-over-strings/)
- Visitor pattern:
  - [Patterns discussion](https://github.com/rust-unofficial/patterns/discussions/236#discussioncomment-393516)
- [Lifeline](https://github.com/austinjones/lifeline-rs): Dependency injection
  library for message-based applications

## Messaging and streaming

For the EventStreaming element of the CQRS-ES pattern.  The
[Compare-NATS](https://docs.nats.io/nats-concepts/overview/compare-nats) content
indicate only Pulsar and NATS provide *"exactly-once"* delivery.

- [NATS-JetStream](https://nats.io/)
- [Pulsar (Go, Apache)](https://github.com/apache/pulsar)

## Visitor over Observer pattern

- [Discussion](https://github.com/rust-unofficial/patterns/discussions/236)
- [Pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
- [Tutorial](https://riptutorial.com/rust/example/24146/visitor-pattern)
- [SO example](https://stackoverflow.com/a/53912993/152860)
- [Yandros example](https://users.rust-lang.org/t/implement-the-visitor-pattern/51166/6?u=taqtiqa-mark)

## Workflow gateways

- +[Cadence (Go, Uber)](https://cadenceworkflow.io/)
- +[Airflow (Go, Apache)]()
- +[Constellation (Rust)](https://github.com/constellation-rs/constellation)
- -[Conductor (Java, Netflix) - complex](https://cadenceworkflow.io/)
- -[Couler (Python, CNCF speculative) - Abstracts across workflow engines](https://github.com/couler-proj/couler)
- -[Argo (k8s)](https://github.com/argoproj/argo-workflows)
- -[Tekton (k8s)](https://github.com/tektoncd)
- -[Dolphin scheduler (Java, Apache - k8s?)](https://github.com/apache/dolphinscheduler)
- -[Zeebe (Java)](https://github.com/camunda/zeebe)
- -[Flowable (Java)](https://github.com/flowable/flowable-engine)
