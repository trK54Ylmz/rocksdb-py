import unittest
import rocksdbpy
import shutil
import tempfile


class TestBasic(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.mkdtemp()

    def tearDown(self) -> None:
        shutil.rmtree(self.temp)

    def test_set(self):
        db = rocksdbpy.open_default(self.temp)

        # set new key and value
        db.set(b'test_set', b'test_value')

    def test_get(self):
        db = rocksdbpy.open_default(self.temp)

        key = b'test_get'
        value = b'test_value'

        db.set(key, value)

        got = db.get(key)

        self.assertEqual(got, value)

        empty = db.get(b'not_exist_key')

        self.assertIsNone(empty)

    def remove(self):
        db = rocksdbpy.open_default(self.temp)

        db.set(b'test_remove', b'test_value')

        db.remove(b'test_remove')
