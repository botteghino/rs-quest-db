start-db:
    docker run -p 9000:9000 \
      -p 9009:9009 \
      -p 8812:8812 \
      -p 9003:9003 \
      -v "/Users/fgiordana/Databases/questdb:/root/.questdb/" questdb/questdb

wipe-db:
    rm -rf /Users/fgiordana/Databases/questdb
