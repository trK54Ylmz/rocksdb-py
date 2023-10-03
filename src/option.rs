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
}
