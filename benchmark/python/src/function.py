import time


def timeit(f, size, db):
    """
    Measure running time of the given function
    """
    start = time.time_ns()

    for i in range(size):
        f(db, i)

    end = time.time_ns()

    return int((end - start) / 1000)


def get_key(db, key):
    """
    Get value by given key
    """
    value = db.get(key)

    if value is None:
        return None

    return value


def put_key(db, key, value):
    """
    Set entry for given key and value
    """
    db.set(key, value)
