### Python bindings for RocksDB

Python bindings for RocksDB written in Rust.

### Usage

```python
import rocksdbpy

db = rocksdbpy.open_default('/tmp/rocksdb')

del db

rocksdb.destroy('/tmp/rocksdb')
```
