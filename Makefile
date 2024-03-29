include .env

DATABASE_URL=postgres://${DB_USERNAME}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_DBNAME}

diesel-setup:
	rm -rf migrations
	diesel setup --database-url ${DATABASE_URL}
	diesel migration run --database-url ${DATABASE_URL}
clean:
	rm -rf migrations
	rm -rf static/css
	rm -rf db/data
