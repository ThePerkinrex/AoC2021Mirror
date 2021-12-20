d1p1: INTERCAL
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

INTERCAL:
	@echo -e "\n################# d1p1 #################"
	@echo "::: INTERCAL (Applied to example, expected 7)"
	(cd Dia01/d1p1/src && ick main.i && cat ../../input2.txt | ./main +wimpmode && rm main && cd ../../..) || echo "INTERCAL (ick) not installed"

d1p2:
	@echo -e "\n################# d1p2 #################"
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
	@echo -e "\n################# d2p1 #################"
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
	@echo -e "\n################# d2p2 #################"
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
	@echo -e "\n################# d3p1 #################"
	@echo ::: Rust
	cargo run -p d3p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia03/d3p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia03/d3p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia03/d3p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia03/d3p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d3p2:
	@echo -e "\n################# d3p2 #################"
	@echo ::: Rust
	cargo run -p d3p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia03/d3p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia03/d3p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia03/d3p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia03/d3p2/src && dotnet run && cd ../../..) || echo dotnet not installed

d4p1:
	@echo -e "\n################# d4p1 #################"
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
	@echo -e "\n################# d4p2 #################"
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
	@echo -e "\n################# d512 #################"
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
	@echo -e "\n################# d5p2 #################"
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
	@echo -e "\n################# d6p1 #################"
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
	@echo -e "\n################# d6p2 #################"
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
	@echo -e "\n################# d7p1 #################"
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
	@echo -e "\n################# d7p2 #################"
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


d8p1:
	@echo -e "\n################# d8p1 #################"
	@echo ::: Rust
	cargo run -p d8p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia08/d8p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia08/d8p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia08/d8p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia08/d8p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d8p2:
	@echo -e "\n################# d8p2 #################"
	@echo ::: Rust
	cargo run -p d8p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia08/d8p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia08/d8p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia08/d8p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia08/d8p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d9p1:
	@echo -e "\n################# d9p1 #################"
	@echo ::: Rust
	cargo run -p d9p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia09/d9p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia09/d9p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia09/d9p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia09/d9p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d9p2:
	@echo -e "\n################# d9p2 #################"
	@echo ::: Rust
	cargo run -p d9p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia09/d9p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia09/d9p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia09/d9p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia09/d9p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d10p1:
	@echo -e "\n################# d10p1 #################"
	@echo ::: Rust
	cargo run -p d10p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia10/d10p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia10/d10p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia10/d10p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia10/d10p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d10p2:
	@echo -e "\n################# d10p2 #################"
	@echo ::: Rust
	cargo run -p d10p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia10/d10p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia10/d10p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia10/d10p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia10/d10p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d11p1:
	@echo -e "\n################# d11p1 #################"
	@echo ::: Rust
	cargo run -p d11p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia11/d11p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia11/d11p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia11/d11p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia11/d11p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d11p2:
	@echo -e "\n################# d11p2 #################"
	@echo ::: Rust
	cargo run -p d11p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia11/d11p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia11/d11p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia11/d11p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia11/d11p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d12p1:
	@echo -e "\n################# d12p1 #################"
	@echo ::: Rust
	cargo run -p d12p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia12/d12p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia12/d12p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia12/d12p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia12/d12p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d12p2:
	@echo -e "\n################# d12p2 #################"
	@echo ::: Rust
	cargo run -p d12p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia12/d12p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia12/d12p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia12/d12p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia12/d12p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d13p1:
	@echo -e "\n################# d13p1 #################"
	@echo ::: Rust
	cargo run -p d13p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia13/d13p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia13/d13p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia13/d13p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia13/d13p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d13p2:
	@echo -e "\n################# d13p2 #################"
	@echo ::: Rust
	cargo run -p d13p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia13/d13p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd Dia13/d13p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd Dia13/d13p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd Dia13/d13p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d14p1:
	@echo -e "\n################# d14p1 #################"
	@echo ::: Rust
	cargo run -p d14p1 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia14/d14p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia14/d14p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia14/d14p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia14/d14p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d14p2:
	@echo -e "\n################# d14p2 #################"
	@echo ::: Rust
	cargo run -p d14p2 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia14/d14p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia14/d14p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia14/d14p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia14/d14p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d15p1:
	@echo -e "\n################# d15p1 #################"
	@# @echo ::: Rust
	@# cargo run -p d15p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia15/d15p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia15/d15p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia15/d15p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia15/d15p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d15p2:
	@echo -e "\n################# d15p2 #################"
	@# @echo ::: Rust
	@# cargo run -p d15p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia15/d15p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia15/d15p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia15/d15p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia15/d15p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d16p1:
	@echo -e "\n################# d16p1 #################"
	@echo ::: Rust
	cargo run -p d16p1 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia16/d16p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia16/d16p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia16/d16p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia16/d16p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d16p2:
	@echo -e "\n################# d16p2 #################"
	@echo ::: Rust
	cargo run -p d16p2 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia16/d16p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia16/d16p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia16/d16p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia16/d16p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d17p1:
	@echo -e "\n################# d17p1 #################"
	@# @echo ::: Rust
	@# cargo run -p d17p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia17/d17p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia17/d17p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia17/d17p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia17/d17p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d17p2:
	@echo -e "\n################# d17p2 #################"
	@# @echo ::: Rust
	@# cargo run -p d17p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia17/d17p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia17/d17p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia17/d17p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia17/d17p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d18p1:
	@echo -e "\n################# d18p1 #################"
	@echo ::: Rust
	cargo run -p d18p1 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia18/d18p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia18/d18p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia18/d18p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia18/d18p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d18p2:
	@echo -e "\n################# d18p2 #################"
	@echo ::: Rust
	cargo run -p d18p2 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia18/d18p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia18/d18p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia18/d18p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia18/d18p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d19p1:
	@echo -e "\n################# d19p1 #################"
	@# @echo ::: Rust
	@# cargo run -p d19p1 -q || echo Rust not installed
	@echo ::: Python
	cd Dia19/d19p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia19/d19p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia19/d19p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia19/d19p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d19p2:
	@echo -e "\n################# d19p2 #################"
	@# @echo ::: Rust
	@# cargo run -p d19p2 -q || echo Rust not installed
	@echo ::: Python
	cd Dia19/d19p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia19/d19p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia19/d19p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia19/d19p2/src && dotnet run && cd ../../..) || echo dotnet not installed


d20p1:
	@echo -e "\n################# d20p1 #################"
	@echo ::: Rust
	cargo run -p d20p1 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia20/d20p1/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia20/d20p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia20/d20p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia20/d20p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d20p2:
	@echo -e "\n################# d20p2 #################"
	@echo ::: Rust
	cargo run -p d20p2 -q || echo Rust not installed
	@# @echo ::: Python
	@# cd Dia20/d20p2/src && python main.py && cd ../../.. || echo Python not installed
	@# @echo ::: JavaScipt
	@# cd Dia20/d20p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@# @echo ::: Java
	@# (cd Dia20/d20p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@# @echo ::: C#
	@# (cd Dia20/d20p2/src && dotnet run && cd ../../..) || echo dotnet not installed

all: d1p1 d1p2 d2p1 d2p2 d3p1 d3p2 d4p1 d4p2 d5p1 d5p2 d6p1 d6p2 d7p1 d7p2 d8p1 d8p2 d9p1 d9p2 d10p1 d10p2 d11p1 d11p2 d12p1 d12p2 d13p1 d13p2 d14p1 d14p2 d15p1 d15p2 d6p1 d6p2 d18p1 d18p2 d19p1 d19p2 d20p1 d20p2
