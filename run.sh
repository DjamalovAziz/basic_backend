sudo rm -rf ./basic_surrealdb

gnome-terminal -- bash -c "surreal start --log trace --user root --pass root --bind 0.0.0.0:7002 file://basic_surrealdb; bash"
sleep 1

surreal import --conn http://localhost:7002 --user=root --pass=root --ns=basic_surrealdb --db=basic_surrealdb ./common/src/schemas/up.surql
sleep 1
cargo run -r