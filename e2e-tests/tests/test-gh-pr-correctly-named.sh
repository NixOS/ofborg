#!/usr/bin/env bash
set -e

configPath=${1}
fixturesPath=${2}

>&2 echo -e "\n[+] Getting rabbitmqadmin."
if [ ! -f "./rabbitmqadmin" ]
  then wget "http://localhost:15672/cli/rabbitmqadmin"
fi

# Kind of a dirty hack... We call evaluation-filter to create
# the exchange/queue routes.
evaluation-filter "${configPath}" &
evalFilterPID=$!
sleep 7
kill ${evalFilterPID}

>&2 echo -e "\n[+] Sending fake PR event."
github-event-faker "${configPath}" "${fixturesPath}/pr-update-correctly-named.json"
sleep 7

# Check event
mrci=$(python rabbitmqadmin -u guest -p guest list queues | grep mass-rebuild-check-inputs | awk '{print $4}')
if [ ${mrci} != 1 ]
  then >&2 echo "[!] Github fake event not routed to mass-rebuild-check-inputs."
       exit 1
fi

>&2 echo -e "\n[+] Starting evaluation-filter."
evaluation-filter "${configPath}" &
evalFilterPID=$!
sleep 7
kill ${evalFilterPID}

mrcj=$(python rabbitmqadmin -u guest -p guest list queues | grep mass-rebuild-check-jobs | awk '{print $4}')
if [ ${mrci} != 1 ]
  then >&2 echo "[!] Github fake event not routed to mass-rebuild-check-jobs."
       exit 1
fi

>&2 echo -e "\n\n======================================"
>&2 echo "[+] test_gh_pr_correctly_named test successful."
