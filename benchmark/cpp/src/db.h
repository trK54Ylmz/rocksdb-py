#include <rocksdb/db.h>
#include <stdio.h>

using namespace std;

using ROCKSDB_NAMESPACE::DB;
using ROCKSDB_NAMESPACE::Options;
using ROCKSDB_NAMESPACE::Status;

// Open the database by given path
static DB *get_db(string path)
{
    DB *db;

    Options opts;
    opts.create_if_missing = true;

    Status status = DB::Open(opts, path, &db);

    assert(status.ok());

    return db;
}

// Close active database and destroy
static void destroy(DB *db, string path)
{
    delete db;

    Options opts;

    Status status = DestroyDB(path, opts);

    assert(status.ok());
}
