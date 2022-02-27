import unittest
import rocksdbpy
from rocksdbpy import WriteBatch


class TestIterator(unittest.TestCase):
    def setUp(self):
        wb = WriteBatch()

        # add couple of keys and values
        wb.add(b'test_add_1', b'test_value')
        wb.add(b'test_add_2', b'test_value')
        wb.add(b'test_add_3', b'test_value')

        self.db = rocksdbpy.open_default('/tmp/test_iterator')

        self.db.write(wb)

    def test_simple(self):
        # get iterator in default mode which is forward
        itr = self.db.iterator()

        i = 1

        for k, v in itr:
            self.assertEqual(b'test_value', v)
            self.assertEqual(f'test_add_{i}'.encode('ascii'), k)

            i += 1

    def test_end(self):
        # get iterator in end mode which is reverse
        itr = self.db.iterator(mode='end')

        i = 3

        for k, v in itr:
            self.assertEqual(b'test_value', v)
            self.assertEqual(f'test_add_{i}'.encode('ascii'), k)

            i -= 1

    def test_from(self):
        # get iterator in from mode which is skips some keys
        itr = self.db.iterator(mode='from', key=b'test_add_2')

        i = 2

        for k, v in itr:
            self.assertEqual(b'test_value', v)
            self.assertEqual(f'test_add_{i}'.encode('ascii'), k)

            i += 1

    def test_from_reverse(self):
        # get iterator in from mode which is skips some keys and reverse
        itr = self.db.iterator(mode='from', key=b'test_add_2', direction=-1)

        i = 2

        for k, v in itr:
            self.assertEqual(b'test_value', v)
            self.assertEqual(f'test_add_{i}'.encode('ascii'), k)

            i -= 1

    def test_invalid_reverse(self):
        # get invalid iterator
        itr = self.db.iterator(mode='from', direction=-1)

        self.assertEqual(0, len(list(itr)))