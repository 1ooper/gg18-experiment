#$1 number
#$2 name
killall gg18_sm_manager $2 >> /dev/null
./target/release/gg18_sm_manager >> /dev/null &
ROCKET_PORT=8002 ./target/release/$2 http://127.0.0.1:8000 $1 &
pid=$(pidof $2)
top -n 7 -p $pid -d 5 -b >> "$2_$1.txt"
killall gg18_sm_manager $2