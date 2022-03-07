import unittest
import rocksdbpy
import shutil
import tempfile


class TestBasic(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.mkdtemp()

        self.db = rocksdbpy.open_default(self.temp)

    def tearDown(self):
        del self.db

        shutil.rmtree(self.temp)

    def test_set(self):
        # set new key and value
        self.db.set(b'test_set', b'test_value')

    def test_get(self):
        key = b'test_get'
        value = b'test_value'

        self.db.set(key, value)

        got = self.db.get(key)

        self.assertEqual(got, value)

        empty = self.db.get(b'not_exist_key')

        self.assertIsNone(empty)

    def test_multi_get(self):
        keys = [b'test_mget_1', b'test_mget_2']
        values = [b'test_value_1', b'test_value_2']

        self.db.set(keys[0], values[0])
        self.db.set(keys[1], values[1])

        got = self.db.multi_get(keys)

        self.assertEqual(got, values)

        keys = [b'test_mget_1', b'test_mget_2', b'key_not_exist']

        got = self.db.multi_get(keys, skip_missings=True)

        self.assertEqual(got, values)

        values = [b'test_value_1', b'test_value_2', None]

        got = self.db.multi_get(keys)

        self.assertEqual(got, values)

    def remove(self):
        self.db.set(b'test_remove', b'test_value')

        self.db.remove(b'test_remove')
