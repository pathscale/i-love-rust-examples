#!/bin/bash

export PGHOST=$(jq '.db.host' -r ../etc/config.json)
export PGPORT=$(jq '.db.port' -r ../etc/config.json)
export PGUSER=$(jq '.db.user' -r ../etc/config.json)
export PGPASSWORD=$(jq '.db.password' -r ../etc/config.json)
export DATABASE=$(jq '.db.dbname' -r ../etc/config.json)
pg_exec() {
    echo "executing psql $@"
    psql $@
}
pg_exec2() {
    echo "executing psql $@"
    PGDATABASE=$DATABASE psql $@
}
# this is special, it will use coldvaults as user with -c option
echo "CREATE DATABASE coldvaults;" | pg_exec

pg_exec2 -f model.sql
# run twice because of wrong dependencies
pg_exec2 -f tbl.sql
pg_exec2 -f tbl.sql
pg_exec2 -f api.sql