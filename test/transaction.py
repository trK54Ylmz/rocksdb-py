import unittest
import rocksdbpy
import shutil
import tempfile
from rocksdbpy import RocksDB, Option


class TestTransaction(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.mkdtemp()

        opts = Option()
        opts.create_if_missing(True)

        self.db: RocksDB = rocksdbpy.open_for_transaction(self.temp, opts)

    def tearDown(self):
        self.db.close()

        shutil.rmtree(self.temp)

    def test_add(self):
        tx = self.db.transaction()

        # add couple of keys and values
        tx.add(b'test_add_1', b'test_value')
        tx.add(b'test_add_2', b'test_value')
        
        tx.commit()

        self.assertEqual(self.db.get(b'test_add_3'), b'test_value')
