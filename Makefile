.PHONY: run-service build-service

run-service:
	cd service_registry && cargo run
build-service:
	cd service_registry && cargo build



run-bff:
	cd text_analysis_bff && cargo run
build-bff:
	cd text_analysis_bff && cargo build

