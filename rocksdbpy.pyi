from typing import Iterator, List, Optional, Tuple


def open_default(path: str) -> DB:
    """
    Opens a database with default options.

    :param str path: The database path
    :return: active database
    :rtype: rocksdbpy.DB
    """
    ...


def open(path: str, opts: Optional[Option]) -> DB:
    """
    Opens the database with the specified options.

    :param str path: The database path
    :param rocksdbpy.Option or None opts: The options
    :return: active database
    :rtype: rocksdbpy.DB
    """
    ...


def open_with_ttl(path: str, ttl: int, opts: Optional[Option]) -> DB:
    """
    Opens the database with TTL compaction filter.

    :param str path: The database path
    :param int ttl: The TTL duration in seconds
    :param rocksdbpy.Option or None opts: The options
    :return: active database
    :rtype: rocksdbpy.DB

    """
    ...


def destroy(path: str, opts: Optional[Option]) -> None:
    """
    Destroy database and it's files.

    :param str path: The database path
    :param rocksdbpy.Option or None opts: The options
    """
    ...


class DB:
    def get(self, key: bytes) -> Optional[bytes]:
        """
        Return the value associated with a "key".

        :param bytes key: The entry key
        :return: The entry value if exists, NULL otherwise
        :rtype: bytes or None
        """
        ...

    def set(self, key: bytes, value: bytes) -> None:
        """
        Sets records by "key" and "value".

        :param bytes key: The entry key
        :param bytes value: The entry value
        """
        ...

    def delete(self, key: bytes) -> None:
        """
        Removes existing records by "key".

        :param bytes key: The entry key
        """
        ...

    def write(self, batch: WriteBatch) -> None:
        """
        Sets database entries for list of key and values as a batch.

        :param rocksdbpy.WriteBatch batch: The batch writer
        """
        ...

    def multi_get(self, keys: List[bytes], skip_missing: Optional[bool]) -> List[bytes]:
        """
        Returns entries according to given list of key and values.

        :param list[bytes] keys: The list of entry keys
        :param bool or None skip_missing: Skips missing records if it's True
        :return: The list of entry values
        :rtype: list[bytes]
        """
        ...

    def iterator(self) -> Iterator[DBIterator]:
        """
        Returns a heap-allocated iterator over the contents of the database.

        :return: The database iterator
        :rtype: iterator[rocksdbpy.DBIterator]
        """
        ...

    def flush(self):
        """
        Flushes database memtables to SST files on the disk using default options.
        """
        ...

    def close(self):
        """
        Close active database
        """
        ...


class Option:
    def create_if_missing(self, create_if_missing: bool) -> None:
        """
        If true, the database will be created if it is missing.

        :param bool create_if_missing: Create or not the database
        """
        ...


class WriteBatch:
    def add(self, key: bytes, value: bytes) -> None:
        """
        Append new "key" and "value" in the batch.

        :param bytes key: The entry key
        :param bytes value: The entry value
        """
        ...

    def delete(self, key: bytes) -> None:
        """
        Remove "key" from the batch.

        :param bytes key: The entry key
        """
        ...

    def len(self) -> int:
        """
        Returns element count of the batch.

        :return: The number of entries in the batch
        :rtype: int
        """
        ...

    def clear(self) -> None:
        """
        Clear the batch.
        """
        ...


class DBIterator:
    def __next__(self) -> Tuple[bytes, bytes]:
        """
        Returns next database entry

        :return: The database entry
        :rtype: (bytes, bytes)
        """


class RocksDBException(Exception):
    ...
