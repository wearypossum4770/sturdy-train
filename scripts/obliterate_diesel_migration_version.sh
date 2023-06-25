#! /bin/bash

read -p "Which Diesel Migration version would you like to remove: "  version

echo "you are obliterating migration version $version"
read -p "This operation cannot be undo Continue? (Y/N): " confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1
PGPASSWORD=manic
psql -U eve_end -d diesel_demo -c "delete from __diesel_schema_migrations where version='$version'"
