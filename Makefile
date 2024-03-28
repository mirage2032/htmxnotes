diesel-setup:
	diesel migration run --database-url postgres://user:pass@localhost:5432/htmxnotes
clean:
	rm -rf static/css
	rm -rf db/data
