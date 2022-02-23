## rocksdb-py

Python bindings for RocksDB written in Rust.

**Note**: This package is under active development.

### Features

* Get/set/delete
* Destroy
* Batch write

### Usage

Basic K-V operations,

```python
import rocksdbpy

db = rocksdbpy.open_default('/tmp/rocksdb')

db.set(b'key', b'value')

value = db.get(b'key')

db.delete(b'key')
```

Remove database,

```python
rocksdb.destroy('/tmp/rocksdb')
```

Write multiple records as batch,

```python
from rocksdbpy import WriteBatch

batch = WriteBatch()
batch.add(b'first', b'1')
batch.add(b'second', b'2')

db.write(batch)
```

Extra operations in the batch,

```python
batch.delete(b'first')

batch.clear()

size = batch.len()
```

Close active database and release lock,

```python
del db
```

### Build

You can build PIP package by using `maturin`. The example below is created for MacOS,

```bash
$ git clone https://github.com/trk54ylmz/rocksdb-py.git
$ cd rocksdb-py
$ maturin build
$ pip3 install ./target/wheels/rocksdb_py-0.0.1-cp39-cp39-macosx_11_0_arm64.whl
```
