#!/bin/sh

mongoimport --host $DATABASE_HOST --db $DATABASE_NAME --collection users --type json --file /data/seed/users.json
mongoimport --host $DATABASE_HOST --db $DATABASE_NAME --collection organizations --type json --file /data/seed/organizations.json
mongoimport --host $DATABASE_HOST --db $DATABASE_NAME --collection repositories --type json --file /data/seed/repositories.json
