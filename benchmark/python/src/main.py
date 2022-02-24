from db import get_db
from function import get_key, put_key, timeit

if __name__ == '__main__':
    size = 10000

    # get RocksDB connection
    db = get_db()

    get_key_f = lambda db, i: get_key(db, f'test_{i}'.encode('ascii'))
    put_key_f = lambda db, i: put_key(db, f'test_{i}'.encode('ascii'), b"value")

    put_key_diff = timeit(put_key_f, size, db)
    get_key_diff = timeit(get_key_f, size, db)

    print(f'Put key took {put_key_diff}μs')
    print(f'Get key took {get_key_diff}μs')
