import rocksdbpy
from rocksdbpy import Option


def get_db(path):
    """
    Open the database by given path
    """
    opts = Option()
    opts.create_if_missing(True)

    return rocksdbpy.open(path, opts)


def destroy(db, path):
    """
    Close active database and destroy
    """
    db.close()

    rocksdbpy.destroy(path)
