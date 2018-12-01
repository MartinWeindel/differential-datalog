#!/bin/bash
# Shell script to build and run the span program in Java

set -ex

# Build the Java library with the Ddlog API
cd ../java_call
make
cd ../datalog_tests
ln -sf ../java_call/ddlogapi .
# Force linking with the static library by deleting the dynamic library
rm -f span_ddlog/target/release/libspan_ddlog.so
# Compile Span.java
javac -cp ../java_call Span.java
# Create manifest file for jar
mkdir -p META-INF
echo "Main-Class: Span" > META-INF/MANIFEST.MF
# Create jar containing Span and the Ddlog API
jar cmvf META-INF/MANIFEST.MF span.jar Span.class ./ddlogapi/*.class
# Create a shared library containing all the native code: ddlogapi.c, libspan_ddlog.a
gcc -shared -Wl,-soname,ddlogapi -fPIC -I${JAVA_HOME}/include -I${JAVA_HOME}/include/linux ../java_call/ddlogapi.c ../java_call/ddlogapi_DDLogAPI.h -Lspan_ddlog/target/release/ -lspan_ddlog -o libddlogapi.so
# Run the java program pointing to the created shared library
java -Djava.library.path=. -jar span.jar

# Cleanup
rm -rf META-INF Span.class
