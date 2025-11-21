use pyo3::prelude::*;
use rocksdb::{DBCompactionStyle, Options};

#[pyclass(name = "Option")]
#[derive(Clone)]
pub struct OptionPy {
    pub inner: Options,
}

#[pymethods]
impl OptionPy {
    #[new]
    pub fn new() -> Self {
        let opts = Options::default();

        OptionPy { inner: opts }
    }

    /// If true, the database will be created if it is missing.
    ///
    /// Default: `false`
    ///
    /// # Example
    ///
    /// ```
    /// opts.create_if_missing(True)
    /// ```
    pub fn create_if_missing(&mut self, create_if_missing: bool) {
        self.inner.create_if_missing(create_if_missing)
    }

    /// Sets the number of open files that can be used by the DB.
    ///
    /// Default: `-1`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_max_open_files(10)
    /// ```
    pub fn set_max_open_files(&mut self, number: i32) {
        self.inner.set_max_open_files(number)
    }

    /// If true, then every store to stable storage will issue a fsync.
    /// If false, then every store to stable storage will issue a fdatasync.
    ///
    /// Default: `false`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_use_fsync(True)
    /// ```
    pub fn set_use_fsync(&mut self, useit: bool) {
        self.inner.set_use_fsync(useit)
    }

    /// Allows OS to incrementally sync files to disk while they are being written,
    /// asynchronously, in the background.
    ///
    /// Default: `0`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_bytes_per_sync(1024 * 1024)
    /// ```
    pub fn set_bytes_per_sync(&mut self, nbytes: u64) {
        self.inner.set_bytes_per_sync(nbytes)
    }

    /// Use this if you don't need to keep the data sorted, i.e. you'll never use
    /// an iterator.
    ///
    /// Default: `0`
    ///
    /// # Example
    ///
    /// ```
    /// opts.optimize_for_point_lookup(1024 * 1024)
    /// ```
    pub fn optimize_for_point_lookup(&mut self, cache_size: u64) {
        self.inner.optimize_for_point_lookup(cache_size)
    }

    /// Sets the number of shards used for table cache.
    ///
    /// Default: `6`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_table_cache_num_shard_bits(12)
    /// ```
    pub fn set_table_cache_num_shard_bits(&mut self, size: i32) {
        self.inner.set_table_cache_num_shard_bits(size)
    }

    /// Sets the maximum number of write buffers that are built up in memory.
    ///
    /// Default: `2`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_max_write_buffer_number(6)
    /// ```
    pub fn set_max_write_buffer_number(&mut self, size: i32) {
        self.inner.set_max_write_buffer_number(size)
    }

    /// Sets the amount of data to build up in memory (backed by an unsorted log on disk)
    /// before converting to a sorted on-disk file.
    ///
    /// Default: `0x4000000`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_write_buffer_size(128 * 1024 * 1024)
    /// ```
    pub fn set_write_buffer_size(&mut self, size: usize) {
        self.inner.set_write_buffer_size(size)
    }

    /// Sets the target file size for compaction. target_file_size_base is per-file size for level-1.
    ///
    /// Default: `0x4000000`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_target_file_size_base(128 * 1024 * 1024)
    /// ```
    pub fn set_target_file_size_base(&mut self, size: u64) {
        self.inner.set_target_file_size_base(size)
    }

    /// Sets the minimum number of write buffers that will be merged together before writing to storage.
    ///
    /// Default: `1`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_min_write_buffer_number_to_merge(2)
    /// ```
    pub fn set_min_write_buffer_number_to_merge(&mut self, size: i32) {
        self.inner.set_min_write_buffer_number_to_merge(size)
    }

    /// Sets the maximum number of level-0 files. We stop writes at this point.
    ///
    /// Default: `24`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_level_zero_stop_writes_trigger(48)
    /// ```
    pub fn set_level_zero_stop_writes_trigger(&mut self, size: i32) {
        self.inner.set_level_zero_stop_writes_trigger(size)
    }

    /// Sets the soft limit on number of level-0 files. We start slowing down writes at this point.
    /// A value < 0 means that no writing slow down will be triggered by number of files in level-0.
    ///
    /// Default: `20`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_level_zero_slowdown_writes_trigger(10)
    /// ```
    pub fn set_level_zero_slowdown_writes_trigger(&mut self, size: i32) {
        self.inner.set_level_zero_slowdown_writes_trigger(size)
    }

    /// Disables automatic compactions. Manual compactions can still be issued on this column family.
    ///
    /// Default: `False`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_disable_auto_compactions(True)
    /// ```
    pub fn set_disable_auto_compactions(&mut self, disable: bool) {
        self.inner.set_disable_auto_compactions(disable)
    }

    /// Sets unordered_write to true trades higher write throughput with relaxing the immutability
    /// guarantee of snapshots.
    ///
    /// Default: `False`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_unordered_write(True)
    /// ```
    pub fn set_unordered_write(&mut self, unordered: bool) {
        self.inner.set_unordered_write(unordered)
    }

    /// Sets maximum number of threads that will concurrently perform a compaction job by breaking
    /// it into multiple, smaller ones that are run simultaneously.
    ///
    /// Default: `1`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_max_subcompactions(4)
    /// ```
    pub fn set_max_subcompactions(&mut self, num: u32) {
        self.inner.set_max_subcompactions(num)
    }

    /// Sets maximum number of concurrent background jobs (compactions and flushes).
    ///
    /// Default: `2`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_max_background_jobs(8)
    /// ```
    pub fn set_max_background_jobs(&mut self, jobs: i32) {
        self.inner.set_max_background_jobs(jobs)
    }

    /// Control locality of bloom filter probes to improve cache miss rate.
    ///
    /// Default: `0`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_bloom_locality(16)
    /// ```
    pub fn set_bloom_locality(&mut self, val: u32) {
        self.inner.set_bloom_locality(val)
    }

    /// Sets the compaction style.
    ///
    /// Default: `level`
    ///
    /// # Example
    ///
    /// ```
    /// opts.set_compaction_style('fifo')
    /// ```
    pub fn set_compaction_style(&mut self, style: &str) {
        // Get compaction style from string
        let c = match style {
            "fifo" => DBCompactionStyle::Fifo,
            "universal" => DBCompactionStyle::Universal,
            _ => DBCompactionStyle::Level,
        };

        self.inner.set_compaction_style(c)
    }

    /// Allow the OS to mmap file for reading ss tables
    ///
    /// Default: false
    ///
    /// Examples
    /// ```
    /// opts.set_allow_mmap_reads(True)
    /// ```
    pub fn set_allow_mmap_reads(&mut self, is_enabled: bool) {
        self.inner.set_allow_mmap_reads(is_enabled)
    }

    /// By default, RocksDB uses only one background thread for flush and compaction. Calling this
    /// function will set it up such that total of total_threads is used. Good value for total_threads
    /// is the number of cores. You almost definitely want to call this function if your system is
    /// bottlenecked by RocksDB.
    ///
    /// Default: `1`
    ///
    /// Examples
    /// ```
    /// opts.increase_parallelism(3)
    /// ```
    pub fn increase_parallelism(&mut self, parallelism: i32) {
        self.inner.increase_parallelism(parallelism)
    }

    /// Optimize level style compaction.
    /// 
    /// Default values for some parameters in `Options`` are not optimized for heavy workloads and
    /// big datasets, which means you might observe write stalls under some conditions.
    /// 
    /// This can be used as one of the starting points for tuning RocksDB options in such cases.
    /// 
    /// Internally, it sets `write_buffer_size`, `min_write_buffer_number_to_merge`,
    /// `max_write_buffer_number`, `level0_file_num_compaction_trigger`, `target_file_size_base`,
    /// `max_bytes_for_level_base`, so it can override if those parameters were set before.
    ///
    /// It sets buffer sizes so that memory consumption would be constrained by `memtable_memory_budget`.
    ///
    /// Default: `0x4000000`
    ///
    /// Examples
    /// ```
    /// opts.optimize_level_style_compaction(64 * 1024 * 1024)
    /// ```
    pub fn optimize_level_style_compaction(&mut self, memtable_memory_budget: usize) {
        self.inner.optimize_level_style_compaction(memtable_memory_budget)
    }

    /// Optimize universal style compaction.
    ///
    /// Default values for some parameters in Options are not optimized for heavy workloads and big
    /// datasets, which means you might observe write stalls under some conditions.
    ///
    /// This can be used as one of the starting points for tuning RocksDB options in such cases.
    ///
    /// Internally, it sets `write_buffer_size`, `min_write_buffer_number_to_merge`,
    /// `max_write_buffer_number`, `level0_file_num_compaction_trigger`, `target_file_size_base`,
    /// `max_bytes_for_level_base`, so it can override if those parameters were set before.
    ///
    /// It sets buffer sizes so that memory consumption would be
    /// constrained by `memtable_memory_budget`.
    ///
    /// Default: `0x4000000`
    ///
    /// Examples
    /// ```
    /// opts.optimize_universal_style_compaction(64 * 1024 * 1024)
    /// ```
    pub fn optimize_universal_style_compaction(&mut self, memtable_memory_budget: usize) {
        self.inner.optimize_universal_style_compaction(memtable_memory_budget)
    }

    /// If true, any column families that didn't exist when opening the database will be created.
    ///
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.create_missing_column_families(true)
    /// ```
    pub fn create_missing_column_families(&mut self, create_missing_cfs: bool) {
        self.inner.create_missing_column_families(create_missing_cfs)
    }

    /// Specifies whether an error should be raised if the database already exists.
    ///
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_error_if_exists(true)
    /// ```
    pub fn set_error_if_exists(&mut self, enabled: bool) {
        self.inner.set_error_if_exists(enabled)
    }

    /// Enable/disable paranoid checks.
    ///
    /// If true, the implementation will do aggressive checking of the data it is processing and
    /// will stop early if it detects any errors. This may have unforeseen ramifications: for
    /// example, a corruption of one DB entry may cause a large number of entries to become
    /// unreadable or for the entire DB to become unopenable. If any of the writes to the database
    /// fails (Put, Delete, Merge, Write), the database will switch to read-only mode and fail all
    /// other Write operations.
    ///
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_paranoid_checks(true)
    /// ```
    pub fn set_paranoid_checks(&mut self, enabled: bool) {
        self.inner.set_paranoid_checks(enabled)
    }

    /// Number of threads for parallel compression. Parallel compression is enabled
    /// only if threads > 1. **THE FEATURE IS STILL EXPERIMENTAL**
    /// 
    /// See [code](https://github.com/facebook/rocksdb/blob/v8.6.7/include/rocksdb/advanced_options.h#L116-L127) for more information.
    ///
    /// Default: `1`
    ///
    /// Examples
    /// ```
    /// opts.set_compression_options_parallel_threads(3)
    /// ```
    pub fn set_compression_options_parallel_threads(&mut self, num: i32) {
        self.inner.set_compression_options_parallel_threads(num)
    }

    /// Maximum size of dictionaries used to prime the compression library. Enabling dictionary can
    /// improve compression ratios when there are repetitions across data blocks.
    ///
    /// The dictionary is created by sampling the SST file data. If `zstd_max_train_bytes` is
    /// nonzero, the samples are passed through zstd's dictionary generator. Otherwise, the random
    /// samples are used directly as the dictionary.
    ///
    /// When compression dictionary is disabled, we compress and write each block before buffering
    /// data for the next one. When compression dictionary is enabled, we buffer all SST file data
    /// in-memory so we can sample it, as data can only be compressed and written after the
    /// dictionary has been finalized. So users of this feature may see increased memory usage.
    ///
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_compression_options(4, 5, 6, 7)
    /// ```
    pub fn set_compression_options(&mut self, w_bits: i32, level: i32, strategy: i32, max_dict_bytes: i32) {
        self.inner.set_compression_options(w_bits, level, strategy, max_dict_bytes)
    }

    /// Sets compression options for blocks at the bottom-most level. Meaning of all settings is
    /// the same as in `set_compression_options` method but affect only the bottom-most compression
    /// which is set using `set_bottommost_compression_type` method.
    ///
    /// Default: `N/A`
    ///
    /// Examples
    /// ```
    /// opts.set_bottommost_compression_options(4, 5, 6, 7, true)
    /// ```
    pub fn set_bottommost_compression_options(&mut self, w_bits: i32, level: i32, strategy: i32, max_dict_bytes: i32, enabled: bool) {
        self.inner.set_bottommost_compression_options(w_bits, level, strategy, max_dict_bytes, enabled)
    }

    /// Sets maximum size of training data passed to zstd's dictionary trainer. Using zstd's
    /// dictionary trainer can achieve even better compression ratio improvements than
    /// using `max_dict_bytes` alone.
    /// 
    /// The training data will be used to generate a dictionary of `max_dict_bytes`.
    ///
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_zstd_max_train_bytes(64 * 1024 * 1024)
    /// ```
    pub fn set_zstd_max_train_bytes(&mut self, value: i32) {
        self.inner.set_zstd_max_train_bytes(value)
    }

    /// Sets maximum size of training data passed to zstd's dictionary trainer when compressing the
    /// bottom-most level. Using zstd's dictionary trainer can achieve even better compression
    /// ratio improvements than using `max_dict_bytes` alone.
    /// 
    /// The training data will be used to generate a dictionary of `max_dict_bytes`.
    ///
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_zstd_max_train_bytes(64 * 1024 * 1024, true)
    /// ```
    pub fn set_bottommost_zstd_max_train_bytes(&mut self, value: i32, enabled: bool) {
        self.inner.set_bottommost_zstd_max_train_bytes(value, enabled)
    }

    /// If non-zero, we perform bigger reads when doing compaction. If you're running RocksDB on
    /// spinning disks, you should set this to at least 2MB. That way RocksDB's compaction is doing
    /// sequential instead of random reads.
    ///
    /// Default: `2 * 1024 * 1024 (2MB)`
    ///
    /// Examples
    /// ```
    /// opts.set_compaction_readahead_size(16 * 1024 * 1024)
    /// ```
    pub fn set_compaction_readahead_size(&mut self, compaction_readahead_size: usize) {
        self.inner.set_compaction_readahead_size(compaction_readahead_size)
    }

    /// Allow RocksDB to pick dynamic base of bytes for levels. With this feature turned on,
    /// RocksDB will automatically adjust max bytes for each level. The goal of this feature is to
    /// have lower bound on size amplification.
    ///
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_level_compaction_dynamic_level_bytes(true)
    /// ```
    pub fn set_level_compaction_dynamic_level_bytes(&mut self, enabled: bool) {
        self.inner.set_level_compaction_dynamic_level_bytes(enabled)
    }

    /// Sets the `optimize_filters_for_hits` flag
    ///
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_optimize_filters_for_hits(true)
    /// ```
    pub fn set_optimize_filters_for_hits(&mut self, optimize_for_hits: bool) {
        self.inner.set_optimize_filters_for_hits(optimize_for_hits)
    }

    /// Sets the periodicity when obsolete files get deleted.
    /// 
    /// The files that get out of scope by compaction process will still get automatically delete
    /// on every compaction, regardless of this setting.
    ///
    /// Default: `6 hours`
    ///
    /// Examples
    /// ```
    /// opts.set_delete_obsolete_files_period_micros(300_000_000)
    /// ```
    pub fn set_delete_obsolete_files_period_micros(&mut self, micros: u64) {
        self.inner.set_delete_obsolete_files_period_micros(micros)
    }

    /// Prepare the DB for bulk loading.
    /// 
    /// All data will be in level 0 without any automatic compaction. It's recommended to manually
    /// call `CompactRange(NULL, NULL)` before reading from the database, because otherwise the
    /// read can be very slow.
    ///
    /// Default: `N/A`
    ///
    /// Examples
    /// ```
    /// opts.prepare_for_bulk_load()
    /// ```
    pub fn prepare_for_bulk_load(&mut self) {
        self.inner.prepare_for_bulk_load()
    }

    /// If `max_open_files` is -1, DB will open all files on `DB::Open()`. You can use this option
    /// to increase the number of threads used to open the files.
    ///
    /// Default: `16`
    ///
    /// Examples
    /// ```
    /// opts.set_max_file_opening_threads(32)
    /// ```
    pub fn set_max_file_opening_threads(&mut self, nthreads: i32) {
        self.inner.set_max_file_opening_threads(nthreads)
    }

    /// Same as bytes_per_sync, but applies to WAL files.
    ///
    /// Dynamically changeable through `SetDBOptions()` API.
    /// 
    /// Default: `0, turned off`
    ///
    /// Examples
    /// ```
    /// opts.set_wal_bytes_per_sync(1 * 1024 * 1024)
    /// ```
    pub fn set_wal_bytes_per_sync(&mut self, nbytes: u64) {
        self.inner.set_wal_bytes_per_sync(nbytes)
    }

    /// Sets the maximum buffer size that is used by `WritableFileWriter`.
    /// 
    /// On Windows, we need to maintain an aligned buffer for writes. We allow the buffer to grow
    /// until it's size hits the limit in buffered IO and fix the buffer size when using direct IO
    /// to ensure alignment of write requests if the logical sector size is unusual
    ///
    /// Dynamically changeable through `SetDBOptions()` API.
    /// 
    /// Default: `1024 * 1024 (1 MB)`
    ///
    /// Examples
    /// ```
    /// opts.set_writable_file_max_buffer_size(4096 * 1024)
    /// ```
    pub fn set_writable_file_max_buffer_size(&mut self, nbytes: u64) {
        self.inner.set_writable_file_max_buffer_size(nbytes)
    }
}
