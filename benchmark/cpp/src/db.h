#include <rocksdb/db.h>
#include <stdio.h>

using namespace std;

using ROCKSDB_NAMESPACE::DB;
using ROCKSDB_NAMESPACE::Options;
using ROCKSDB_NAMESPACE::Status;

static DB *get_db()
{
    DB *db;

    Options opts;
    opts.create_if_missing = true;

    Status status = DB::Open(opts, "/tmp/test", &db);

    assert(status.ok());

    return db;
}
