#!/usr/bin/env bash
set -e

if [ ! $(basename "${PWD}") == "e2e-tests" ]
  then echo -e "\n[!] Please, cd to e2e-tests before running this script"
       exit 1
fi

echo -e "\n[+] Getting rebbitmqadmin"
if [ ! -f "./rabbitmqadmin" ]
  then wget "http://localhost:15672/cli/rabbitmqadmin"
fi

echo -e "\n[+] Purging Queues"
python rabbitmqadmin -u guest -p guest purge queue name=mass-rebuild-check-jobs
python rabbitmqadmin -u guest -p guest purge queue name=mass-rebuild-check-inputs

echo -e "\n[+] Sending fake update github event"
../ofborg/target/debug/github-event-faker ../config.json ./fixtures
sleep 7

# Check event
mrci=$(python rabbitmqadmin -u guest -p guest list queues | grep mass-rebuild-check-inputs | awk '{print $4}')
if [ ${mrci} != 1 ]
	then echo "[!] Github fake event not routed to mass-rebuild-check-inputs"
			 exit 1
fi

echo -e "\n[+] Starting evaluation-filter"
../ofborg/target/debug/evaluation-filter ../config.json &
evalFilterPID=$!
sleep 7
kill ${evalFilterPID}

mrcj=$(python rabbitmqadmin -u guest -p guest list queues | grep mass-rebuild-check-jobs | awk '{print $4}')
if [ ${mrci} != 1 ]
	then echo "[!] Github fake event not routed to mass-rebuild-check-jobs"
			 exit 1
fi
echo -e "\n[+] Message successfully checked and moved to mass-rebuild-check-jobs"
echo -e "\n[+] Purging queue"
python rabbitmqadmin -u guest -p guest purge queue name=mass-rebuild-check-jobs


echo -e "\n\n======================================"
echo "[+] Github integration E2E successful"
