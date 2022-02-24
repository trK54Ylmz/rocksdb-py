import unittest
from rocksdbpy import Option


class TestOption(unittest.TestCase):
    def test_create_missing(self):
        o = Option()

        self.assertIsNone(o.create_if_missing(True))
        self.assertRaises(TypeError, o.create_if_missing, 1)

    def test_max_open_files(self):
        o = Option()

        self.assertIsNone(o.set_max_open_files(1))
        self.assertRaises(TypeError, o.set_max_open_files, 1e100)

    def test_use_fsync(self):
        o = Option()

        self.assertIsNone(o.set_use_fsync(True))
        self.assertRaises(TypeError, o.set_use_fsync, 1)

    def test_bytes_per_sync(self):
        o = Option()

        self.assertIsNone(o.set_bytes_per_sync(1024 * 2))
        self.assertRaises(TypeError, o.set_bytes_per_sync, 1.024)

    def test_optimize_for_point_lookup(self):
        o = Option()

        self.assertIsNone(o.optimize_for_point_lookup(1024 * 2))
        self.assertRaises(TypeError, o.optimize_for_point_lookup, 1.024)

    def test_table_cache_num_shard_bits(self):
        o = Option()

        self.assertIsNone(o.set_table_cache_num_shard_bits(2))
        self.assertRaises(TypeError, o.set_table_cache_num_shard_bits, 1.1)

    def test_max_write_buffer_number(self):
        o = Option()

        self.assertIsNone(o.set_max_write_buffer_number(2))
        self.assertRaises(TypeError, o.set_max_write_buffer_number, 1.024)

    def test_write_buffer_size(self):
        o = Option()

        self.assertIsNone(o.set_write_buffer_size(0x40000))
        self.assertRaises(TypeError, o.set_write_buffer_size, 1.024)

    def test_target_file_size_base(self):
        o = Option()

        self.assertIsNone(o.set_target_file_size_base(1024 * 2))
        self.assertRaises(TypeError, o.set_target_file_size_base, 1.024)

    def test_disable_auto_compactions(self):
        o = Option()

        self.assertIsNone(o.set_disable_auto_compactions(True))
        self.assertRaises(TypeError, o.set_disable_auto_compactions, None)

    def test_compaction_style(self):
        o = Option()

        self.assertIsNone(o.set_compaction_style(""))
        self.assertIsNone(o.set_compaction_style("level"))
        self.assertRaises(TypeError, o.set_compaction_style, 10)
        self.assertRaises(TypeError, o.set_compaction_style, None)

    def test_min_write_buffer_number_to_merge(self):
        o = Option()

        self.assertIsNone(o.set_min_write_buffer_number_to_merge(1))
        self.assertRaises(TypeError, o.set_min_write_buffer_number_to_merge, 1.2)

    def test_level_zero_stop_writes_trigger(self):
        o = Option()

        self.assertIsNone(o.set_level_zero_stop_writes_trigger(10))
        self.assertRaises(TypeError, o.set_level_zero_stop_writes_trigger, None)

    def test_level_zero_stop_writes_trigger(self):
        o = Option()

        self.assertIsNone(o.set_level_zero_stop_writes_trigger(10))
        self.assertRaises(TypeError, o.set_level_zero_stop_writes_trigger, None)

    def test_level_zero_slowdown_writes_trigger(self):
        o = Option()

        self.assertIsNone(o.set_level_zero_slowdown_writes_trigger(10))
        self.assertRaises(TypeError, o.set_level_zero_slowdown_writes_trigger, None)
