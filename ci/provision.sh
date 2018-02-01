#!/usr/bin/env bash
set -x -e -o pipefail

apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv E56151BF
DISTRO=$(lsb_release -is | tr '[:upper:]' '[:lower:]')
CODENAME=$(lsb_release -cs)

echo "deb http://repos.mesosphere.io/${DISTRO} ${CODENAME} main" | \
    sudo tee /etc/apt/sources.list.d/mesosphere.list
apt-get -y update

apt-get -y install mesos=1.4.0-2.0.1 zookeeperd

service mesos-master start
service mesos-slave start

# Curl Mesos master to see if it's up and running.
MESOS_MASTER=$(mesos-resolve "$(cat /etc/mesos/zk)")
curl -f "http://$MESOS_MASTER" > /dev/null
