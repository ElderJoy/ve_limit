ID=dblinov.testnet
CONTRACT_NAME=dev-1663666936770-68312779187723

near call $CONTRACT_NAME calc_ve_order_sum_simple '{"num": 1000000}' --accountId=$ID --gas=300000000000000
