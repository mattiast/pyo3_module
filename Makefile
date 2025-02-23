test:
	cargo test --release
	maturin develop --uv --release
	pytest test_sample.py
