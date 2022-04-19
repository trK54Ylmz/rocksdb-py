## rocksdb-py

Python bindings for RocksDB written in Rust.

### Features

* Get, set, delete, multi get
* Destroy
* Batch write
* Database iterator
* Read options

### Install

To install a wheel from PyPI,

```bash
pip install --upgrade rocksdb-py
```

or if you want to build a wheel, see [build](https://github.com/trK54Ylmz/rocksdb-py#Build).

### Usage

#### Open database

Open a database with default options.

```python
import rocksdbpy

db = rocksdbpy.open_default('/tmp/rocksdb')
```

Open a database with the specified options.

```python
opts = Option()
opts.create_if_missing(True)

db = rocksdbpy.open('/tmp/rocksdb', opts)
```

Open a database with TTL compaction filter.

```python
opts = Option()
opts.create_if_missing(True)

db = rocksdbpy.open_with_ttl('/tmp/rocksdb', 5, opts)
```

Destroy the database and it's files.

```python
rocksdbpy.destroy('/tmp/rocksdb')
```

Close active database and release lock.

```python
db.close()
```

#### Simple read, set and delete

Set records by key and value.

```python
db.set(b'key', b'value')
```

Get a value associated with a key.

```python
value = db.get(b'key')
```

Remove existing records by key.

```python
db.delete(b'key')
```

#### Batch write, database iterator and flush

Set database entries for list of key and values as a batch.

```python
from rocksdbpy import WriteBatch

batch = WriteBatch()
batch.add(b'first', b'1')
batch.add(b'second', b'2')

db.write(batch)
```

Extra operations for the batch.

```python
batch.delete(b'first')

batch.clear()

size = batch.len()
```

Return a heap-allocated iterator over the contents of the database.

```python
iterator = db.iterator()

iterator = db.iterator(mode='end')

iterator = db.iterator(mode='from', key=b'test')

iterator = db.iterator(mode='from', key=b'test', direction=-1)

for key, value in iterator:
    print(key, value)
```

Flush database memtables to SST files on the disk using default options.

```python
db.flush()
```

#### Read options

Set database read options.

```python
opts = Option()

opts.create_if_missing(True)

opts.set_max_open_files(10)

opts.set_use_fsync(True)

opts.set_bytes_per_sync(1024 * 1024)

# and more
```

### Build

You can build PIP package by using `maturin`. The example below is created for MacOS,

```bash
$ git clone https://github.com/trk54ylmz/rocksdb-py.git
$ cd rocksdb-py
$ maturin build
$ pip3 install ./target/wheels/rocksdb_py-0.0.1-cp39-cp39-macosx_11_0_arm64.whl
```
