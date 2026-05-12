#!/bin/sh

sleep 5;
PGPASSWORD=$DATABASE_PASSWORD psql -U $DATABASE_USER -h $DATABASE_HOST -p $DATABASE_PORT -d $DATABASE_DB -f scripts/database/schema.sql
