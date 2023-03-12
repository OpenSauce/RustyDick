run:
	nohup cargo run > rusty.log 2>&1 &
	echo $! > save_pid.txt

stop:
	kill -9 `cat save_pid.txt`
	rm save_pid.txt