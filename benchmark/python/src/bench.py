from function import get, put, put_multi

PREFIX = 'test'


def get_key_f(db, i):
    """
    Get value by given key
    """
    get(db, f'{PREFIX}_{i}'.encode('ascii'))


def put_key_f(db, i):
    """
    Set entry for given key and value
    """
    put(db, f'{PREFIX}_{i}'.encode('ascii'), b'value')


def put_m_key_f(db, i):
    """
    Set multiple entries for given group of keys and values
    """
    keys = [f'{PREFIX}_{i}_{j}'.encode('ascii') for j in range(10)]
    vals = [b'value' for j in range(10)]

    put_multi(db, keys, vals)
