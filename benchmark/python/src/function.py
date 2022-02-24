import time


def timeit(f, size, db):
    """
    Measure running time of the given function
    """
    start = time.time_ns()

    for i in range(size):
        f(db, i)

    end = time.time_ns()

    return (end - start) / 1000


def get_key(db, key):
    """
    Get value by given key
    """
    return db.get(key)


def put_key(db, key, value):
    """
    Set entry for given key and value
    """
    db.set(key, value)
