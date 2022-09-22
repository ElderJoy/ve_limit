# dev-1663666936770-68312779187723 has 1000 accounts
# dev-1663750719547-44631546007510 has 800 accounts

ID=dblinov.testnet
# CONTRACT_NAME=dev-1663666936770-68312779187723
CONTRACT_NAME=dev-1663750719547-44631546007510

near call $CONTRACT_NAME calc_ve_order_sum_simple '{"num": 1000000}' --accountId=$ID --gas=300000000000000
near call $CONTRACT_NAME add_user_accounts '{ "started_num": 0, "number_to_add": 500, "rnd_str": "0vr6Ygf7dHoMA8Ch5o0BmkhI42N4QtnIeLf8O4pHOQjF9Pwj27IGSRZQe4RL7JQq"}' --accountId=$ID --gas=300000000000000
near call $CONTRACT_NAME add_user_accounts '{ "started_num": 500, "number_to_add": 400, "rnd_str": "0vr6Ygf7dHoMA8Ch5o0BmkhI42N4QtnIeLf8O4pHOQjF9Pwj27IGSRZQe4RL7JQq"}' --accountId=$ID --gas=300000000000000
near call $CONTRACT_NAME add_user_accounts '{ "started_num": 900, "number_to_add": 100, "rnd_str": "0vr6Ygf7dHoMA8Ch5o0BmkhI42N4QtnIeLf8O4pHOQjF9Pwj27IGSRZQe4RL7JQq"}' --accountId=$ID --gas=300000000000000
near call $CONTRACT_NAME get_users_num --accountId=$ID --gas=300000000000000

near call $CONTRACT_NAME calc_ve_order_sum --accountId=$ID --gas=300000000000000
