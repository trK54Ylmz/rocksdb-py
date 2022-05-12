from db import destroy, get_db
from function import timeit
from bench import get_key_f, put_key_f, put_m_key_f

if __name__ == '__main__':
    size = 100000
    path = '/tmp/test'

    # get RocksDB connection
    db = get_db(path)

    put_key_diff = timeit(put_key_f, size, db)
    get_key_diff = timeit(get_key_f, size, db)
    put_m_key_diff = timeit(put_m_key_f, size, db)

    print(f'Put key = {put_key_diff}μs')
    print(f'Get key = {get_key_diff}μs')
    print(f'Put multi key = {put_m_key_diff}μs')

    # destroy database
    destroy(db, path)
