d1p1:
	@echo ::: Rust
	cargo run -p d1p1 -q || echo Rust not installed
	@echo ::: Python
	(cd Dia01/d1p1/src && python main.py && cd ../../..) || echo Python not installed
	@echo ::: JavaScipt
	(cd Dia01/d1p1/src && node main.js && cd ../../..) || echo NodeJS not installed
	@echo ::: JSFuck
	(cd Dia01/d1p1/src && node main.fuck.js && cd ../../..) || echo NodeJS not installed
	@echo ::: Java
	(cd Dia01/d1p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia01/d1p1/src && dotnet run && cd ../../..) || echo dotnet not installed
	@echo "::: Bash (Applied to example (expected 7), as it takes 20s to execute when applied to original input)"
	(cd Dia01/d1p1/src && bash main.bash && cd ../../..) || echo Bash not installed

d1p2:
	@echo ::: Rust
	cargo run -p d1p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia01/d1p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia01/d1p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: JSFuck
	(cd Dia01/d1p2/src && node main.fuck.js && cd ../../..) || echo NodeJS not installed
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
	@echo ::: V
	(cd Dia02/d2p1/src && v run main.v || v.bat run main.v || v.exe run main.v && cd ../../..) || echo v not installed

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
	@echo ::: V
	(cd Dia02/d2p2/src && v run main.v || v.bat run main.v || v.exe run main.v && cd ../../..) || echo v not installed


d3p1:
	# @echo ::: Rust
	# cargo run -p d3p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia03/d3p1/src && python main.py && cd ../../.. || echo Python not installed
	# @echo ::: JavaScipt
	# cd Dia03/d3p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	# @echo ::: Java
	# (cd Dia03/d3p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	# @echo ::: C#
	# (cd Dia03/d3p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d3p2:
	# @echo ::: Rust
	# cargo run -p d3p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia03/d3p2/src && python main.py && cd ../../.. || echo Python not installed
	# @echo ::: JavaScipt
	# cd Dia03/d3p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	# @echo ::: Java
	# (cd Dia03/d3p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	# @echo ::: C#
	# (cd Dia03/d3p2/src && dotnet run && cd ../../..) || echo dotnet not installed

d4p1:
	@echo ::: Rust
	cargo run -p d4p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia04/d4p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia04/d4p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia04/d4p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia04/d4p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d4p2:
	@echo ::: Rust
	cargo run -p d4p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia04/d4p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia04/d4p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia04/d4p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia04/d4p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d5p1:
	@echo ::: Rust
	cargo run -p d5p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia05/d5p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia05/d5p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia05/d5p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia05/d5p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d5p2:
	@echo ::: Rust
	cargo run -p d5p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia05/d5p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia05/d5p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia05/d5p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia05/d5p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d6p1:
	@echo ::: Rust
	cargo run -p d6p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia06/d6p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia06/d6p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia06/d6p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia06/d6p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d6p2:
	@echo ::: Rust
	cargo run -p d6p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia06/d6p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia06/d6p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia06/d6p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia06/d6p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d7p1:
	@echo ::: Rust
	cargo run -p d7p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia07/d7p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia07/d7p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia07/d7p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia07/d7p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d7p2:
	@echo ::: Rust
	cargo run -p d7p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia07/d7p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia07/d7p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia07/d7p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia07/d7p2/src && dotnet run && cd ../../..) || echo dotnet not installed


