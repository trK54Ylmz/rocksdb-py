import time


def timeit(f, size, db):
    start = time.time_ns()

    for i in range(size):
        f(db, i)

    end = time.time_ns()

    return (end - start) / 1000


def get_key(db, key):
    return db.get(key)


def put_key(db, key, value):
    db.set(key, value)
