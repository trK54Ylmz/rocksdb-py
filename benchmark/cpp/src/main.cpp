#include <rocksdb/db.h>

#include "db.h"
#include "function.h"

using namespace std;
using ROCKSDB_NAMESPACE::DB;

int main(int argc, char **argv)
{
    const int size = 100000;
    const string path = "/tmp/path";

    // Get RocksDB connection
    DB *db = get_db(path);

    auto get_key_f = [](DB *db, int i)
    {
        get(db, string(PREFIX) + "_" + to_string(i));
    };

    auto put_key_f = [](DB *db, int i)
    {
        put(db, string(PREFIX) + "_" + to_string(i), "value");
    };

    auto put_m_key_f = [](DB *db, int i)
    {
        int len = 10;
        string *keys = new string[len]();
        string *values = new string[len]();

        for (int j = 0; j < len; j++)
        {
            keys[j] = string(PREFIX) + "_" + to_string(i) + "_" + to_string(j);
            values[j] = "value";
        }

        put_multi(db, &keys, &values, &len);
    };

    auto put_key_diff = timeit(ref(put_key_f), size, db);
    auto get_key_diff = timeit(ref(get_key_f), size, db);
    auto put_m_key_diff = timeit(ref(put_m_key_f), size, db);

    cout << "Put key = " << put_key_diff << "μs" << endl;
    cout << "Get key = " << get_key_diff << "μs" << endl;
    cout << "Put multi key = " << put_m_key_diff << "μs" << endl;

    // Destroy database
    destroy(db, path);

    return 1;
}
