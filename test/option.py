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

    def test_increase_parallelism(self):
        o = Option()

        self.assertIsNone(o.increase_parallelism(2))
        self.assertRaises(TypeError, o.increase_parallelism, 1.1)
        self.assertRaises(TypeError, o.increase_parallelism, None)

    def test_optimize_level_style_compaction(self):
        o = Option()

        self.assertIsNone(o.optimize_level_style_compaction(0x40000))
        self.assertRaises(TypeError, o.optimize_level_style_compaction, 1.024)

    def test_optimize_universal_style_compaction(self):
        o = Option()

        self.assertIsNone(o.optimize_universal_style_compaction(0x40000))
        self.assertRaises(TypeError, o.optimize_universal_style_compaction, 1.024)

    def test_create_missing_column_families(self):
        o = Option()

        self.assertIsNone(o.create_missing_column_families(True))
        self.assertRaises(TypeError, o.create_missing_column_families, None)

    def test_set_error_if_exists(self):
        o = Option()

        self.assertIsNone(o.set_error_if_exists(True))
        self.assertRaises(TypeError, o.set_error_if_exists, None)

    def test_set_paranoid_checks(self):
        o = Option()

        self.assertIsNone(o.set_paranoid_checks(True))
        self.assertRaises(TypeError, o.set_paranoid_checks, None)

    def test_set_compression_options_parallel_threads(self):
        o = Option()

        self.assertIsNone(o.set_compression_options_parallel_threads(3))
        self.assertRaises(TypeError, o.set_compression_options_parallel_threads, None)

    def test_set_compression_options(self):
        o = Option()

        self.assertIsNone(o.set_compression_options(4, 5, 6, 7))
        self.assertRaises(TypeError, o.set_compression_options, None, None, None, None)

    def test_set_bottommost_compression_options(self):
        o = Option()

        self.assertIsNone(o.set_bottommost_compression_options(4, 5, 6, 7, True))
        self.assertRaises(TypeError, o.set_bottommost_compression_options, None, None, None, None, None)

    def test_set_zstd_max_train_bytes(self):
        o = Option()

        self.assertIsNone(o.set_zstd_max_train_bytes(64 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_zstd_max_train_bytes, None)

    def test_set_bottommost_zstd_max_train_bytes(self):
        o = Option()

        self.assertIsNone(o.set_bottommost_zstd_max_train_bytes(64 * 1024 * 1024, True))
        self.assertRaises(TypeError, o.set_bottommost_zstd_max_train_bytes, None, None)

    def test_set_compaction_readahead_size(self):
        o = Option()

        self.assertIsNone(o.set_compaction_readahead_size(16 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_compaction_readahead_size, None)

    def test_set_level_compaction_dynamic_level_bytes(self):
        o = Option()

        self.assertIsNone(o.set_level_compaction_dynamic_level_bytes(True))
        self.assertRaises(TypeError, o.set_level_compaction_dynamic_level_bytes, None)

    def test_set_optimize_filters_for_hits(self):
        o = Option()

        self.assertIsNone(o.set_optimize_filters_for_hits(True))
        self.assertRaises(TypeError, o.set_optimize_filters_for_hits, None)

    def test_set_delete_obsolete_files_period_micros(self):
        o = Option()

        self.assertIsNone(o.set_delete_obsolete_files_period_micros(300_000_000))
        self.assertRaises(TypeError, o.set_delete_obsolete_files_period_micros, None)

    def test_prepare_for_bulk_load(self):
        o = Option()

        self.assertIsNone(o.prepare_for_bulk_load())
        self.assertRaises(TypeError, o.prepare_for_bulk_load, None)

    def test_set_max_file_opening_threads(self):
        o = Option()

        self.assertIsNone(o.set_max_file_opening_threads(32))
        self.assertRaises(TypeError, o.set_max_file_opening_threads, None)

    def test_set_wal_bytes_per_sync(self):
        o = Option()

        self.assertIsNone(o.set_wal_bytes_per_sync(1 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_wal_bytes_per_sync, None)

    def test_set_writable_file_max_buffer_size(self):
        o = Option()

        self.assertIsNone(o.set_writable_file_max_buffer_size(4096 * 1024))
        self.assertRaises(TypeError, o.set_writable_file_max_buffer_size, None)

    def test_set_allow_concurrent_memtable_write(self):
        o = Option()

        self.assertIsNone(o.set_allow_concurrent_memtable_write(False))
        self.assertRaises(TypeError, o.set_allow_concurrent_memtable_write, None)

    def test_set_enable_write_thread_adaptive_yield(self):
        o = Option()

        self.assertIsNone(o.set_enable_write_thread_adaptive_yield(False))
        self.assertRaises(TypeError, o.set_enable_write_thread_adaptive_yield, None)

    def test_set_max_sequential_skip_in_iterations(self):
        o = Option()

        self.assertIsNone(o.set_max_sequential_skip_in_iterations(16))
        self.assertRaises(TypeError, o.set_max_sequential_skip_in_iterations, None)

    def test_set_use_direct_reads(self):
        o = Option()

        self.assertIsNone(o.set_use_direct_reads(True))
        self.assertRaises(TypeError, o.set_use_direct_reads, None)

    def test_set_use_direct_io_for_flush_and_compaction(self):
        o = Option()

        self.assertIsNone(o.set_use_direct_io_for_flush_and_compaction(True))
        self.assertRaises(TypeError, o.set_use_direct_io_for_flush_and_compaction, None)

    def test_set_is_fd_close_on_exec(self):
        o = Option()

        self.assertIsNone(o.set_is_fd_close_on_exec(False))
        self.assertRaises(TypeError, o.set_is_fd_close_on_exec, None)

    def test_set_target_file_size_multiplier(self):
        o = Option()

        self.assertIsNone(o.set_target_file_size_multiplier(2))
        self.assertRaises(TypeError, o.set_target_file_size_multiplier, None)

    def test_set_min_write_buffer_number(self):
        o = Option()

        self.assertIsNone(o.set_min_write_buffer_number(2))
        self.assertRaises(TypeError, o.set_min_write_buffer_number, None)

    def test_set_db_write_buffer_size(self):
        o = Option()

        self.assertIsNone(o.set_db_write_buffer_size(128 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_db_write_buffer_size, None)

    def test_set_max_bytes_for_level_base(self):
        o = Option()

        self.assertIsNone(o.set_db_write_buffer_size(512 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_db_write_buffer_size, None)

    def test_set_max_manifest_file_size(self):
        o = Option()

        self.assertIsNone(o.set_max_manifest_file_size(20 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_max_manifest_file_size, None)

    def test_set_level_zero_file_num_compaction_trigger(self):
        o = Option()

        self.assertIsNone(o.set_level_zero_file_num_compaction_trigger(8))
        self.assertRaises(TypeError, o.set_level_zero_file_num_compaction_trigger, None)

    def test_set_memtable_huge_page_size(self):
        o = Option()

        self.assertIsNone(o.set_memtable_huge_page_size(20))
        self.assertRaises(TypeError, o.set_memtable_huge_page_size, None)

    def test_set_max_successive_merges(self):
        o = Option()

        self.assertIsNone(o.set_max_successive_merges(8))
        self.assertRaises(TypeError, o.set_max_successive_merges, None)

    def test_set_inplace_update_support(self):
        o = Option()

        self.assertIsNone(o.set_inplace_update_support(True))
        self.assertRaises(TypeError, o.set_inplace_update_support, None)

    def test_set_inplace_update_locks(self):
        o = Option()

        self.assertIsNone(o.set_inplace_update_locks(25_000))
        self.assertRaises(TypeError, o.set_inplace_update_locks, None)

    def test_set_max_write_buffer_size_to_maintain(self):
        o = Option()

        self.assertIsNone(o.set_max_write_buffer_size_to_maintain(20_000))
        self.assertRaises(TypeError, o.set_max_write_buffer_size_to_maintain, None)

    def test_set_enable_pipelined_write(self):
        o = Option()

        self.assertIsNone(o.set_enable_pipelined_write(True))
        self.assertRaises(TypeError, o.set_enable_pipelined_write, None)

    def test_set_report_bg_io_stats(self):
        o = Option()

        self.assertIsNone(o.set_report_bg_io_stats(True))
        self.assertRaises(TypeError, o.set_report_bg_io_stats, None)

    def test_set_max_total_wal_size(self):
        o = Option()

        self.assertIsNone(o.set_max_total_wal_size(1 << 30))
        self.assertRaises(TypeError, o.set_max_total_wal_size, None)

    def test_set_stats_dump_period_sec(self):
        o = Option()

        self.assertIsNone(o.set_stats_dump_period_sec(300))
        self.assertRaises(TypeError, o.set_stats_dump_period_sec, None)

    def test_set_stats_persist_period_sec(self):
        o = Option()

        self.assertIsNone(o.set_stats_persist_period_sec(5))
        self.assertRaises(TypeError, o.set_stats_persist_period_sec, None)

    def test_set_advise_random_on_open(self):
        o = Option()

        self.assertIsNone(o.set_advise_random_on_open(False))
        self.assertRaises(TypeError, o.set_advise_random_on_open, None)

    def test_set_use_adaptive_mutex(self):
        o = Option()

        self.assertIsNone(o.set_use_adaptive_mutex(True))
        self.assertRaises(TypeError, o.set_use_adaptive_mutex, None)

    def test_set_memtable_prefix_bloom_ratio(self):
        o = Option()

        self.assertIsNone(o.set_memtable_prefix_bloom_ratio(0.2))
        self.assertRaises(TypeError, o.set_memtable_prefix_bloom_ratio, None)

    def test_set_max_compaction_bytes(self):
        o = Option()

        self.assertIsNone(o.set_max_compaction_bytes(0))
        self.assertRaises(TypeError, o.set_max_compaction_bytes, None)

    def test_set_wal_ttl_seconds(self):
        o = Option()

        self.assertIsNone(o.set_wal_ttl_seconds(30))
        self.assertRaises(TypeError, o.set_wal_ttl_seconds, None)

    def test_set_wal_size_limit_mb(self):
        o = Option()

        self.assertIsNone(o.set_wal_size_limit_mb(64))
        self.assertRaises(TypeError, o.set_wal_size_limit_mb, None)

    def test_set_manifest_preallocation_size(self):
        o = Option()

        self.assertIsNone(o.set_manifest_preallocation_size(8 * 1024 * 1024))
        self.assertRaises(TypeError, o.set_manifest_preallocation_size, None)

    def test_set_skip_stats_update_on_db_open(self):
        o = Option()

        self.assertIsNone(o.set_skip_stats_update_on_db_open(True))
        self.assertRaises(TypeError, o.set_skip_stats_update_on_db_open, None)

    def test_set_keep_log_file_num(self):
        o = Option()

        self.assertIsNone(o.set_keep_log_file_num(2_500))
        self.assertRaises(TypeError, o.set_keep_log_file_num, None)
