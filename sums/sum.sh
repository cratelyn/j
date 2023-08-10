#!/bin/sh
SUM=0
for i in $(seq 100);
do
	SUM=$(expr $SUM + $i)
done
echo $SUM
