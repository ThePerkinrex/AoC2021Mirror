d1p1:
	@echo ::: Rust
	cargo run -p d1p1 -q
	@echo ::: Python
	cd Dia01/d1p1/src && python main.py && cd ../../..
	@echo ::: JavaScipt
	cd Dia01/d1p1/src && node main.js && cd ../../..

d1p2:
	@echo ::: Rust
	cargo run -p d1p2 -q
	@echo ::: Python
	cd Dia01/d1p2/src && python main.py && cd ../../..
	@echo ::: JavaScipt
	cd Dia01/d1p2/src && node main.js && cd ../../..