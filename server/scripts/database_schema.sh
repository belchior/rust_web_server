#!/bin/sh

while ! nc -z $POSTGRES_HOST $POSTGRES_PORT; do sleep 1; done;

PGPASSWORD=$POSTGRES_PASSWORD psql -U $POSTGRES_USER -h $POSTGRES_HOST -p $POSTGRES_PORT -d $POSTGRES_DB -f /database/schema.sql
