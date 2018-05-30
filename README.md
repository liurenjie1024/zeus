Zeus is a time-series database designed written in rust.

## Features 

This project is designed to implement features:

* SQL support
* Materialized View
* Real time ingestion with eventual consistency
* Exactly-once near real time ingestion with strong consistency
* Hadoop integration


## Roadmap

### Milestone 1

In this milestone, we will implement a storage engine for low latency analytics of offline data. 
The following features will be included:

- [ ] Integration with apache drill.
- [ ] An efficient storage format.
- [ ] Query api.
- [ ] Supports filter pushdown.
- [ ] Supports aggregation pushdown.

### Milestone 2

In this milestone, our storage engine will be able to accept insertions. The following features 
will be includes:

- [ ] Insertion api.
- [ ] WAL log.
- [ ] Support query of realtime data.
- [ ] Distributed replication.
- [ ] Online schema management.
- [ ] Materialized view.

### Milestone 3

In this milestone, our storage engine will be able to accept exactly once near real time 
insertion. The following features will be implemented.

- [ ] MVCC.
- [ ] Use distributed filesystem as deep storage, such as hdfs, s3.

This is project is still under heavy development...


## How to build

* Install protoc
* Install rust protobuf 1.7.3
* Install rust grpc
* make -s
