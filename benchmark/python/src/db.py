import rocksdbpy
from rocksdbpy import Option


def get_db():
    opts = Option()
    opts.create_if_missing(True)

    return rocksdbpy.open('/tmp/test', opts)
