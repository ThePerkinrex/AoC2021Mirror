#! /bin/bash

DIR="Dia$(printf "%02d" $1)"
echo $DIR

echo "d$1p1:
	@echo ::: Rust
	cargo run -p d$1p1 -q || echo Rust not installed
	@echo ::: Python
	cd $DIR/d$1p1/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd $DIR/d$1p1/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd $DIR/d$1p1/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd $DIR/d$1p1/src && dotnet run && cd ../../..) || echo dotnet not installed

d$1p2:
	@echo ::: Rust
	cargo run -p d$1p2 -q || echo Rust not installed
	@echo ::: Python
	cd $DIR/d$1p2/src && python main.py && cd ../../.. || echo Python not installed
	@echo ::: JavaScipt
	cd $DIR/d$1p2/src && node main.js && cd ../../.. || echo NodeJS not installed
	@echo ::: Java
	(cd $DIR/d$1p2/src && javac Main.java && java Main && rm Main.class && cd ../../..) || echo Java not installed
	@echo ::: C#
	(cd $DIR/d$1p2/src && dotnet run && cd ../../..) || echo dotnet not installed

" >> Makefile

mkdir -p $DIR
cd $DIR
touch input.txt input2.txt
cargo new "d$1p1"
cd "d$1p1/src"
dotnet new console
touch Main.java main.js main.py
cd ../..
cargo new "d$1p2"
cd "d$1p2/src"
dotnet new console
touch Main.java main.js main.py
cd ../../..

sed -i "$ s/.$/, \"$DIR\/d$1p1\", \"$DIR\/d$1p2\"]/" Cargo.toml