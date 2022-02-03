### Python bindings for RocksDB

Python bindings for RocksDB written in Rust.

**Note**: This package under active development.

### Usage

```python
import rocksdbpy

db = rocksdbpy.open_default('/tmp/rocksdb')

del db

rocksdb.destroy('/tmp/rocksdb')
```

### Build

You can build PIP package by using `maturin`. The example below is created for MacOS,

```bash
$ git clone https://github.com/trk54ylmz/rocksdb-py.git
$ cd rocksdb-py
$ maturin build
$ pip3 install ./target/wheels/rocksdb_py-0.0.1-cp39-cp39-macosx_11_0_arm64.whl
```
