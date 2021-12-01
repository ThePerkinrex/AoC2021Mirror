d1p1:
	@echo ::: Rust
	cargo run -p d1p1 -q || echo Rust not installed
	@echo ::: Python
	(cd Dia01/d1p1/src && python main.py && cd ../../..) || echo Python not installed
	@echo ::: JavaScipt
	(cd Dia01/d1p1/src && node main.js && cd ../../..) || echo NodeJS not installed
	@echo ::: Java
	(cd Dia01/d1p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia01/d1p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d1p2:
	@echo ::: Rust
	cargo run -p d1p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia01/d1p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia01/d1p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia01/d1p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia01/d1p2/src && dotnet run && cd ../../..) || echo dotnet not installed

d2p1:
	@echo ::: Rust
	cargo run -p d2p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia02/d2p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia02/d2p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia02/d2p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia02/d2p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d2p2:
	@echo ::: Rust
	cargo run -p d2p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia02/d2p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia02/d2p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia02/d2p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia02/d2p2/src && dotnet run && cd ../../..) || echo dotnet not installed


