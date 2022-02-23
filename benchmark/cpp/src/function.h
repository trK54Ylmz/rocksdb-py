#include <iostream>
#include <chrono>
#include <assert.h>
#include <rocksdb/db.h>

using namespace std;
using namespace std::chrono;

using ROCKSDB_NAMESPACE::DB;
using ROCKSDB_NAMESPACE::ReadOptions;
using ROCKSDB_NAMESPACE::Status;
using ROCKSDB_NAMESPACE::WriteOptions;

const char *PREFIX = "test_";

// Measure running time of the given function
template <typename Function>
int timeit(Function f, int size, DB *db)
{
    time_point<high_resolution_clock> start_time, end_time;

    start_time = high_resolution_clock::now();

    for (int i = 0; i < size; i++)
    {
        f(db, i);
    }

    end_time = high_resolution_clock::now();

    // Get start time diff from epoch
    auto start = time_point_cast<microseconds>(start_time).time_since_epoch().count();

    // Get end time diff from epoch
    auto end = time_point_cast<microseconds>(end_time).time_since_epoch().count();

    return end - start;
}

string get_key(DB *db, string key)
{
    string value;

    // Get value by given key
    Status status = db->Get(ReadOptions(), key, &value);

    if (!status.ok())
    {
        return NULL;
    }

    return value;
}

void put_key(DB *db, string key, string value)
{
    Status status;
    WriteOptions opts;

    // Set key and value
    status = db->Put(opts, key, value);

    assert(status.ok());
}
