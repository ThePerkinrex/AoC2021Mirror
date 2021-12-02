LINES=$(cat ../../input2.txt)
LAST=-1
SUM=0
for LINE in $LINES; do
	CURRENT="$(echo "$LINE" | sed -e 's/\s*$//')"
	if (($LAST != -1)); then
		# echo "NOT FIRST $LAST < $CURRENT = $(($LAST < $CURRENT))"
		if (($LAST < $CURRENT)); then
			# echo "INCR"
			SUM=$(($SUM + 1))
		fi
	# else
		# echo "FIRST"
	fi
	LAST=$CURRENT
done

echo ">>> $SUM"