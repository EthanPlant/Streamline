# Streamline - High Level Architecture

## Overview
Streamine is a high-performance, minimal logging system designed to ingest, store, and query application logs efficiently. The primary goal is to provide fast ingestion, efficient storage, and quick query capabilities without the operational overhead of traditional logging stacks.

This document outlines the core system architecture, data flow, and design decisions.

## Core System Components
### API Layer
Exposes HTTP endpoint for log ingestion:
- POST /logs
```json
{
    timestamp: 1710000000,
    service: "auth",
    "level": "error",
    "message": "database connection failed"
}
```
Responsibilities:
- Validate log payloads
- Accept concurrent requests with minimal latency
- Forward logs to the in-memory buffer

### In-Memory Buffer
Temporary storage for incoming logs before persisting to disk, ensuring low latency for small bursts and high-throughput for bulk ingestion.
- Purpose:
    - Convert small writes into batch writes for efficiency\
    - Enable compression benefits over multiple logs
    - Reduce system calls to disk
- Flush Triggers:
    - Size-based: Flush after N logs (1000)
    - Time-based: Flush every T seconds (2 s)

### Storage Engine
Stores logs as append-only segment files, compressed with Zstandard. Files are sequentially named and rotate on a maximum log count (100k) or maximum file size (100 MB).

This gives us a simple disk structure for sequential access, efficient compression for a reduced storage footprint, and easy deletion of old segments for retention policies.

### Indexing
Maintains a lightweight index for fast query resolution. During the MVP the following fields will be indexed:
- Timestamp
- Service
- Level

Each index entry points to an offset in a segment file, enabling each query to read only relevant portions of the disk. The index is responsible for adding entries on log write, and deleting entries when segments are removed.

### Query Engine
Processes search requests combining structured queries and full-text search. The query engine is responsible for parsing queries, retrieving the relevant segment offsets from indexes, and scanning only the necessary segments to generate results.

### User Interface
During the MVP, this will be a minimal UI for interacting with logs, allowing both querying and live tailing of logs. It's designed primarily for debugging and observability, not complex dashboards.

## Design Decisions
- Buffer flush by time and size
    - Balances latency and throughput.
    - Handles both small bursts and continuous streams efficiently
- Append-only segment files
    - Sequential writes improve disk I/O performance and simplify compressionh
- Zstandard compression
    - Fast compression with low CPU overhead
- Indexed fields: timestamp, service, level
    - Covers most common queries for observability
- Sequential segment naming:
    - Simplifies indexing and rotation logic

## Future Considerations
- Distributed architecture
    - Sharding
    - Replication for durability
- Multi-tenant support
- Metrics, alerting, and dashboards
- Additional indexed fields or log formats
