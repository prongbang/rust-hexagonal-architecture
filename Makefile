test:
	cargo test

run:
	cargo run

test_create:
	curl -d '{"number": 4, "name": "Charmander", "types": ["Fire"]}' \
	-H "Content-Type: application/json" \
	-X POST http://localhost:8000

test_get_all:
	curl --request GET -sL \
	     --url 'http://localhost:8000'

test_get_by_id:
	curl --request GET -sL \
	     --url 'http://localhost:8000/4'

test_del_by_id:
	curl --request DELETE -sL \
	     --url 'http://localhost:8000/4'

cli_help:
	cargo run -- --help

cli:
	cargo run -- --cli

cli_sqlite:
	cargo run -- --sqlite database.sqlite

cli_airtable:
	cargo run -- --airtable <API_KEY> <WORKSPACE_ID>