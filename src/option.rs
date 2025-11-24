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

    /// If true, allow multi-writers to update mem tables in parallel.
    /// Only some memtable_factory-s support concurrent writes; currently it is implemented only
    /// for SkipListFactory. Concurrent memtable writes are not compatible with
    /// inplace_update_support or filter_deletes. It is strongly recommended
    /// to set `enable_write_thread_adaptive_yield` if you are going to use this feature.
    /// 
    /// Default: `true`
    ///
    /// Examples
    /// ```
    /// opts.set_allow_concurrent_memtable_write(false)
    /// ```
    pub fn set_allow_concurrent_memtable_write(&mut self, allow: bool) {
        self.inner.set_allow_concurrent_memtable_write(allow)
    }

    /// If true, threads synchronizing with the write batch group leader will wait for up to
    /// `write_thread_max_yield_usec` before blocking on a mutex. This can substantially improve
    /// throughput for concurrent workloads, regardless
    /// of whether `allow_concurrent_memtable_write` is enabled.
    /// 
    /// Default: `true`
    ///
    /// Examples
    /// ```
    /// opts.set_enable_write_thread_adaptive_yield(false)
    /// ```
    pub fn set_enable_write_thread_adaptive_yield(&mut self, enabled: bool) {
        self.inner.set_enable_write_thread_adaptive_yield(enabled)
    }

    /// Specifies whether an `iteration->Next()` sequentially skips over keys with the same
    /// user-key or not.
    /// 
    /// This number specifies the number of keys (with the same userkey) that will be sequentially
    /// skipped before a reseek is issued.
    /// 
    /// Default: `8`
    ///
    /// Examples
    /// ```
    /// opts.set_max_sequential_skip_in_iterations(16)
    /// ```
    pub fn set_max_sequential_skip_in_iterations(&mut self, num: u64) {
        self.inner.set_max_sequential_skip_in_iterations(num)
    }

    /// Enable direct I/O mode for reading they may or may not improve performance depending on
    /// the use case
    /// 
    /// Files will be opened in "direct I/O" mode which means that data read from the disk will not
    /// be cached or buffered. The hardware buffer of the devices may however still be used.
    /// Memory mapped files are not impacted by these parameters.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_use_direct_reads(true)
    /// ```
    pub fn set_use_direct_reads(&mut self, enabled: bool) {
        self.inner.set_use_direct_reads(enabled)
    }

    /// Enable direct I/O mode for flush and compaction
    /// 
    /// Files will be opened in "direct I/O" mode which means that data written to the disk will
    /// not be cached or buffered. The hardware buffer of the devices may however still be used.
    /// Memory mapped files are not impacted by these parameters. they may or may not improve
    /// performance depending on the use case
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_use_direct_io_for_flush_and_compaction(true)
    /// ```
    pub fn set_use_direct_io_for_flush_and_compaction(&mut self, enabled: bool) {
        self.inner.set_use_direct_io_for_flush_and_compaction(enabled)
    }

    /// Enable/disable child process inherit open files.
    /// 
    /// Default: `true`
    ///
    /// Examples
    /// ```
    /// opts.set_is_fd_close_on_exec(false)
    /// ```
    pub fn set_is_fd_close_on_exec(&mut self, enabled: bool) {
        self.inner.set_is_fd_close_on_exec(enabled)
    }

    /// By default `target_file_size_multiplier` is 1, which means by default files in different
    /// levels will have similar size.
    /// 
    /// Dynamically changeable through `SetOptions()` API
    /// 
    /// Default: `1`
    ///
    /// Examples
    /// ```
    /// opts.set_target_file_size_multiplier(2)
    /// ```
    pub fn set_target_file_size_multiplier(&mut self, multiplier: i32) {
        self.inner.set_target_file_size_multiplier(multiplier)
    }

    /// Sets the minimum number of write buffers that will be merged before writing to storage.
    /// If set to 1, then all write buffers are flushed to L0 as individual files and this
    /// increases read amplification because a get request has to check in all of these files.
    /// Also, an in-memory merge may result in writing lesser data to storage if there are
    /// duplicate records in each of these individual write buffers.
    /// 
    /// Default: `1`
    ///
    /// Examples
    /// ```
    /// opts.set_min_write_buffer_number(2)
    /// ```
    pub fn set_min_write_buffer_number(&mut self, nbuf: i32) {
        self.inner.set_min_write_buffer_number(nbuf)
    }

    /// Amount of data to build up in memtables across all column families before writing to disk.
    /// 
    /// This is distinct from `write_buffer_size`, which enforces a limit for a single memtable.
    /// 
    /// This feature is disabled by default. Specify a non-zero value to enable it.
    /// 
    /// Default: `0 (disabled)`
    ///
    /// Examples
    /// ```
    /// opts.set_write_buffer_size(128 * 1024 * 1024)
    /// ```
    pub fn set_db_write_buffer_size(&mut self, size: usize) {
        self.inner.set_db_write_buffer_size(size)
    }

    /// Control maximum total data size for a level. `max_bytes_for_level_base` is the max total
    /// for level-1. Maximum number of bytes for level L can be calculated as
    /// (`max_bytes_for_level_base`) * (`max_bytes_for_level_multiplier` ^ (L-1))
    /// For example, if `max_bytes_for_level_base` is 200MB,
    /// and if `max_bytes_for_level_multiplier` is 10, total data size for level-1 will be 200MB,
    /// total file size for level-2 will be 2GB, and total file size for level-3 will be 20GB.
    /// 
    /// Default: `0x10000000 (256MiB).`
    ///
    /// Examples
    /// ```
    /// opts.set_write_buffer_size(512 * 1024 * 1024)
    /// ```
    pub fn set_max_bytes_for_level_base(&mut self, size: u64) {
        self.inner.set_max_bytes_for_level_base(size)
    }

    /// The manifest file is rolled over on reaching this limit. The older manifest file be
    /// deleted. The default value is `MAX_INT` so that roll-over does not take place.
    /// 
    /// Default: `MAX_INT (maximum integer value for architecture, 32 or 64 bit).`
    ///
    /// Examples
    /// ```
    /// opts.set_max_manifest_file_size(20 * 1024 * 1024)
    /// ```
    pub fn set_max_manifest_file_size(&mut self, size: usize) {
        self.inner.set_max_manifest_file_size(size)
    }

    /// Sets the number of files to trigger level-0 compaction. A value < 0 means that level-0
    /// compaction will not be triggered by number of files at all.
    /// 
    /// Dynamically changeable through `SetOptions()` API
    /// 
    /// Default: `4`
    ///
    /// Examples
    /// ```
    /// opts.set_level_zero_file_num_compaction_trigger(8)
    /// ```
    pub fn set_level_zero_file_num_compaction_trigger(&mut self, n: i32) {
        self.inner.set_level_zero_file_num_compaction_trigger(n)
    }

    /// `SetMemtableHugePageSize` sets the page size for huge page for arena used by the memtable.
    /// If <=0, it won't allocate from huge page but from malloc. Users are responsible to reserve
    /// huge pages for it to be allocated. For example: `sysctl -w vm.nr_hugepages=20` See linux
    /// doc `Documentation/vm/hugetlbpage.txt` If there isn't enough free huge page available, it
    /// will fall back to malloc.
    /// 
    /// Dynamically changeable through SetOptions() API
    /// 
    /// Default: `N/A`
    ///
    /// Examples
    /// ```
    /// opts.set_memtable_huge_page_size(20)
    /// ```
    pub fn set_memtable_huge_page_size(&mut self, size: usize) {
        self.inner.set_memtable_huge_page_size(size)
    }

    /// Sets the maximum number of successive merge operations on a key in the memtable.
    /// 
    /// When a merge operation is added to the memtable and the maximum number of successive merges
    /// is reached, the value of the key will be calculated and inserted into the memtable instead
    /// of the merge operation. This will ensure that there are never
    /// more than `max_successive_merges` merge operations in the memtable.
    /// 
    /// Default: `0 (disabled)`
    ///
    /// Examples
    /// ```
    /// opts.set_max_successive_merges(8)
    /// ```
    pub fn set_max_successive_merges(&mut self, num: usize) {
        self.inner.set_max_successive_merges(num)
    }

    /// Enable/disable thread-safe inplace updates.
    /// 
    /// Requires updates if
    /// - key exists in current memtable
    /// - new `sizeof(new_value)` <= `sizeof(old_value)`
    /// - `old_value` for that key is a put i.e. `kTypeValue`
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_inplace_update_support(true)
    /// ```
    pub fn set_inplace_update_support(&mut self, enabled: bool) {
        self.inner.set_inplace_update_support(enabled)
    }

    /// Sets the number of locks used for inplace update.
    /// 
    /// Default: `10000 when inplace_update_support = true, otherwise 0.`
    ///
    /// Examples
    /// ```
    /// opts.set_inplace_update_support(20_000)
    /// ```
    pub fn set_inplace_update_locks(&mut self, num: usize) {
        self.inner.set_inplace_update_locks(num)
    }

    /// The total maximum size(bytes) of write buffers to maintain in memory including copies of
    /// buffers that have already been flushed. This parameter only affects trimming of flushed
    /// buffers and does not affect flushing. This controls the maximum amount of write history
    /// that will be available in memory for conflict checking when Transactions are used. The
    /// actual size of write history (flushed Memtables) might be higher than this limit if
    /// further trimming will reduce write history total size below this limit.
    /// For example, if `max_write_buffer_size_to_maintain` is set to 64MB, and there are three
    /// flushed Memtables, with sizes of 32MB, 20MB, 20MB. Because trimming the next Memtable of
    /// size 20MB will reduce total memory usage to 52MB which is below the limit, RocksDB
    /// will stop trimming.
    /// 
    /// When using an OptimisticTransactionDB: If this value is too low, some transactions may fail
    /// at commit time due to not being able to determine whether there were any write conflicts.
    /// 
    /// When using a TransactionDB: If `Transaction::SetSnapshot` is used, TransactionDB will read
    /// either in-memory write buffers or SST files to do write-conflict checking. Increasing this
    /// value can reduce the number of reads to SST files done for conflict detection.
    /// 
    /// Setting this value to 0 will cause write buffers to be freed immediately after
    /// they are flushed. If this value is set to -1, `max_write_buffer_number * write_buffer_size`
    /// will be used.
    /// 
    /// Default: `If using a TransactionDB/OptimisticTransactionDB, the default value will be set to the value of 'max_write_buffer_number * write_buffer_size' if it is not explicitly set by the user. Otherwise, the default is 0.`
    ///
    /// Examples
    /// ```
    /// opts.set_max_write_buffer_size_to_maintain(20_000)
    /// ```
    pub fn set_max_write_buffer_size_to_maintain(&mut self, size: i64) {
        self.inner.set_max_write_buffer_size_to_maintain(size)
    }

    /// By default, a single write thread queue is maintained. The thread gets to the head of the
    /// queue becomes write batch group leader and responsible for writing to WAL and memtable for
    /// the batch group.
    /// 
    /// If `enable_pipelined_write` is true, separate write thread queue is maintained for WAL
    /// write and memtable write. A write thread first enter WAL writer queue and then memtable
    /// writer queue. Pending thread on the WAL writer queue thus only have to wait for previous
    /// writers to finish their WAL writing but not the memtable writing. Enabling the feature may
    /// improve write throughput and reduce latency of the prepare phase of two-phase commit.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_enable_pipelined_write(true)
    /// ```
    pub fn set_enable_pipelined_write(&mut self, value: bool) {
        self.inner.set_enable_pipelined_write(value)
    }

    /// Measure IO stats in compactions and flushes, if `true`.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_report_bg_io_stats(true)
    /// ```
    pub fn set_report_bg_io_stats(&mut self, enable: bool) {
        self.inner.set_report_bg_io_stats(enable)
    }

    /// Once write-ahead logs exceed this size, we will start forcing the flush of column families
    /// whose memtables are backed by the oldest live WAL file (i.e. the ones that are causing all
    /// the space amplification).
    /// 
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_max_total_wal_size(1 << 30)
    /// ```
    pub fn set_max_total_wal_size(&mut self, size: u64) {
        self.inner.set_max_total_wal_size(size)
    }

    /// If not zero, dump `rocksdb.stats` to LOG every `stats_dump_period_sec`.
    /// 
    /// Default: `600 (10 mins)`
    ///
    /// Examples
    /// ```
    /// opts.set_stats_dump_period_sec(300)
    /// ```
    pub fn set_stats_dump_period_sec(&mut self, period: u32) {
        self.inner.set_stats_dump_period_sec(period)
    }

    /// If not zero, dump `rocksdb.stats` to RocksDB to LOG every `stats_persist_period_sec`.
    /// 
    /// Default: `600 (10 mins)`
    ///
    /// Examples
    /// ```
    /// opts.set_stats_persist_period_sec(5)
    /// ```
    pub fn set_stats_persist_period_sec(&mut self, period: u32) {
        self.inner.set_stats_persist_period_sec(period)
    }

    /// When set to `true`, reading SST files will opt out of the filesystem's readahead.
    /// Setting this to false may improve sequential iteration performance.
    /// 
    /// Default: `true`
    ///
    /// Examples
    /// ```
    /// opts.set_advise_random_on_open(false)
    /// ```
    pub fn set_advise_random_on_open(&mut self, advise: bool) {
        self.inner.set_advise_random_on_open(advise)
    }

    /// Enable/disable adaptive mutex, which spins in the user space before resorting to kernel.
    /// 
    /// This could reduce context switch when the mutex is not heavily contended. However, if
    /// the mutex is hot, we could end up wasting spin time.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_use_adaptive_mutex(true)
    /// ```
    pub fn set_use_adaptive_mutex(&mut self, enabled: bool) {
        self.inner.set_use_adaptive_mutex(enabled)
    }

    /// When a prefix_extractor is defined through `opts.set_prefix_extractor` this creates a
    /// prefix bloom filter for each memtable with the size of
    /// `write_buffer_size * memtable_prefix_bloom_ratio` (capped at 0.25).
    /// 
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_memtable_prefix_bloom_ratio(0.2)
    /// ```
    pub fn set_memtable_prefix_bloom_ratio(&mut self, ratio: f64) {
        self.inner.set_memtable_prefix_bloom_ratio(ratio)
    }

    /// Sets the maximum number of bytes in all compacted files. We try to limit number of bytes in
    /// one compaction to be lower than this threshold. But it's not guaranteed.
    /// 
    /// Value 0 will be sanitized.
    /// 
    /// Default: `target_file_size_base * 25`
    ///
    /// Examples
    /// ```
    /// opts.set_max_compaction_bytes(0)
    /// ```
    pub fn set_max_compaction_bytes(&mut self, nbytes: u64) {
        self.inner.set_max_compaction_bytes(nbytes)
    }

    /// Sets the WAL ttl in seconds.
    /// 
    /// The following two options affect how archived logs will be deleted.
    /// 1. If both set to 0, logs will be deleted asap and will not get into the archive.
    /// 2. If `wal_ttl_seconds` is 0 and `wal_size_limit_mb` is not 0, WAL files will be checked
    /// every 10 min and if total size is greater then `wal_size_limit_mb`, they will be deleted
    /// starting with the earliest until `size_limit` is met. All empty files will be deleted.
    /// 3. If `wal_ttl_seconds` is not 0 and `wal_size_limit_mb` is 0, then WAL files will be
    /// checked every `wal_ttl_seconds` / 2 and those that are older than `wal_ttl_seconds` will
    /// be deleted.
    /// 4. If both are not 0, WAL files will be checked every 10 min and both checks will be
    /// performed with ttl being first.
    /// 
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_wal_ttl_seconds(30)
    /// ```
    pub fn set_wal_ttl_seconds(&mut self, secs: u64) {
        self.inner.set_wal_ttl_seconds(secs)
    }

    /// Sets the WAL size limit in MB.
    /// 
    /// If total size of WAL files is greater then `wal_size_limit_mb`, they will be deleted
    /// starting with the earliest until `size_limit` is met.
    /// 
    /// Default: `0`
    ///
    /// Examples
    /// ```
    /// opts.set_wal_size_limit_mb(64)
    /// ```
    pub fn set_wal_size_limit_mb(&mut self, size: u64) {
        self.inner.set_wal_size_limit_mb(size)
    }

    /// Sets the number of bytes to preallocate (via fallocate) the manifest files.
    /// 
    /// Default is 4MB, which is reasonable to reduce random IO as well as prevent overallocation
    /// for mounts that preallocate large amounts of data (such as xfs's allocsize option).
    /// 
    /// Default: `4MB`
    ///
    /// Examples
    /// ```
    /// opts.set_manifest_preallocation_size(8 * 1024 * 1024)
    /// ```
    pub fn set_manifest_preallocation_size(&mut self, size: usize) {
        self.inner.set_manifest_preallocation_size(size)
    }

    /// If true, then `DB::Open()` will not update the statistics used to optimize compaction
    /// decision by loading table properties from many files. Turning off this feature will
    /// improve DBOpen time especially in disk environment.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_manifest_preallocation_size(true)
    /// ```
    pub fn set_skip_stats_update_on_db_open(&mut self, skip: bool) {
        self.inner.set_skip_stats_update_on_db_open(skip)
    }

    /// Specify the maximal number of info log files to be kept.
    /// 
    /// Default: `1000`
    ///
    /// Examples
    /// ```
    /// opts.set_keep_log_file_num(2_500)
    /// ```
    pub fn set_keep_log_file_num(&mut self, nfiles: usize) {
        self.inner.set_keep_log_file_num(nfiles)
    }

    /// Allow the OS to mmap file for writing.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_allow_mmap_writes(true)
    /// ```
    pub fn set_allow_mmap_writes(&mut self, is_enabled: bool) {
        self.inner.set_allow_mmap_writes(is_enabled)
    }

    /// If enabled, WAL is not flushed automatically after each write. Instead it relies on manual
    /// invocation of `DB::flush_wal()` to write the WAL buffer to its file.
    /// 
    /// Default: `false`
    ///
    /// Examples
    /// ```
    /// opts.set_manual_wal_flush(true)
    /// ```
    pub fn set_manual_wal_flush(&mut self, is_enabled: bool) {
        self.inner.set_manual_wal_flush(is_enabled)
    }
}
