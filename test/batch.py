import unittest
import rocksdbpy
import shutil
import tempfile
from rocksdbpy import WriteBatch


class TestBatch(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.mkdtemp()

    def tearDown(self) -> None:
        shutil.rmtree(self.temp)

    def test_add(self):
        wb = WriteBatch()

        # add couple of keys and values
        wb.add(b'test_add_1', b'test_value')
        wb.add(b'test_add_2', b'test_value')

        self.assertEqual(wb.len(), 2)

    def test_clear(self):
        wb = WriteBatch()

        # add couple of keys and values
        wb.add(b'test_add_1', b'test_value')
        wb.add(b'test_add_2', b'test_value')

        self.assertEqual(wb.len(), 2)

        wb.clear()

        self.assertEqual(wb.len(), 0)

    def test_write(self):
        db = rocksdbpy.open_default(self.temp)

        self.assertIsNone(db.get(b'test_add_1'))

        wb = WriteBatch()

        # add couple of keys and values
        wb.add(b'test_add_1', b'test_value')
        wb.add(b'test_add_2', b'test_value')

        self.assertEqual(wb.len(), 2)

        db.write(wb)

        self.assertEqual(db.get(b'test_add_1'), b'test_value')
