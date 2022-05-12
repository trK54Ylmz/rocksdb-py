import time
from rocksdbpy import WriteBatch


def timeit(f, size, db):
    """
    Measure running time of the given function
    """
    start = time.time_ns()

    for i in range(size):
        f(db, i)

    end = time.time_ns()

    return int((end - start) / 1000)


def get(db, key):
    """
    Get value by given key
    """
    value = db.get(key)

    if value is None:
        return None

    return value


def put(db, key, value):
    """
    Set entry for given key and value
    """
    db.set(key, value)


def put_multi(db, keys, values):
    """
    Set multiple entries for given group of keys and values
    """
    b = WriteBatch()

    for i in range(len(keys)):
        b.add(keys[i], values[i])

    db.write(b)
