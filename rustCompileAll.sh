#!/bin/bash

for i in *.rs
do echo "$i"
done

for i in *.rs
do echo "$i" ; rustc "$i"


done
