import unittest
import rocksdbpy
import shutil
import tempfile


class TestBasic(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.mkdtemp()

    def tearDown(self):
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

    def test_multi_get(self):
        db = rocksdbpy.open_default(self.temp)

        keys = [b'test_mget_1', b'test_mget_2']
        values = [b'test_value_1', b'test_value_2']

        db.set(keys[0], values[0])
        db.set(keys[1], values[1])

        got = db.multi_get(keys)

        self.assertEqual(got, values)

        keys = [b'test_mget_1', b'test_mget_2', b'key_not_exist']

        got = db.multi_get(keys, skip_missings=True)

        self.assertEqual(got, values)

        values = [b'test_value_1', b'test_value_2', None]

        got = db.multi_get(keys)

        self.assertEqual(got, values)

    def remove(self):
        db = rocksdbpy.open_default(self.temp)

        db.set(b'test_remove', b'test_value')

        db.remove(b'test_remove')
