#!/usr/bin/env bash
set -x -e -o pipefail

apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv E56151BF
DISTRO=$(lsb_release -is | tr '[:upper:]' '[:lower:]')
CODENAME=$(lsb_release -cs)

echo "deb http://repos.mesosphere.io/${DISTRO} ${CODENAME} main" | \
    sudo tee /etc/apt/sources.list.d/mesosphere.list
apt-get -y update

apt-get -y install mesos=1.4.0-2.0.1 zookeeperd

service zookeeper restart
service mesos-master restart
service mesos-slave restart

# Curl Mesos master.
MESOS_MASTER=$(mesos-resolve `cat /etc/mesos/zk`)
echo $MESOS_MASTER
curl http://localhost:5050
