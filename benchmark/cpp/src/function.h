#include <iostream>
#include <chrono>
#include <assert.h>
#include <rocksdb/db.h>

using namespace std;
using namespace std::chrono;

using ROCKSDB_NAMESPACE::DB;
using ROCKSDB_NAMESPACE::ReadOptions;
using ROCKSDB_NAMESPACE::Status;
using ROCKSDB_NAMESPACE::WriteBatch;
using ROCKSDB_NAMESPACE::WriteOptions;

const char *PREFIX = "test";

// Measure running time of the given function
int timeit(function<void(DB *, int)> const &f, int size, DB *db)
{
    time_point<high_resolution_clock> start_time, end_time;

    start_time = high_resolution_clock::now();

    for (int i = 0; i < size; i++)
    {
        f(db, i);
    }

    end_time = high_resolution_clock::now();

    // Get start time diff from epoch
    auto diff = duration_cast<microseconds>(end_time - start_time).count();

    return diff;
}

// Get value by given key
string get(DB *db, string key)
{
    string value;
    Status status;

    // Get value by given key
    status = db->Get(ReadOptions(), key, &value);

    if (!status.ok())
    {
        return NULL;
    }

    return value;
}

// Set entry for given key and value
void put(DB *db, string key, string value)
{
    Status status;

    // Set key and value
    status = db->Put(WriteOptions(), key, value);

    assert(status.ok());
}

// Set multiple entries for given group of keys and values
void put_multi(DB *db, string **keys, string **values, int *len)
{
    Status status;
    WriteBatch b;

    for (int i = 0; i < *len; i++)
    {
        b.Put((*keys)[i], (*values)[i]);
    }

    status = db->Write(WriteOptions(), &b);

    assert(status.ok());
}