#include <rocksdb/db.h>

#include "db.h"
#include "function.h"

using ROCKSDB_NAMESPACE::DB;

int main(int argc, char **argv)
{
    const int size = 1e4;

    // Get RocksDB connection
    DB *db = get_db();

    auto get_key_f = [](DB* db, int i) { get_key(db, "test_" + to_string(i)); };
    auto put_key_f = [](DB* db, int i) { put_key(db, "test_" + to_string(i), "value"); };

    auto put_key_diff = timeit(put_key_f, size, db);
    auto get_key_diff = timeit(get_key_f, size, db);

    cout << "Put key took " << put_key_diff << "μs" << endl;
    cout << "Get key took " << get_key_diff << "μs" << endl;

    return 1;
}