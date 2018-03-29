#!/bin/bash

echo "$0: trying create user: ${MYSQL_READ_ONLY_USER}"
echo "CREATE USER '${MYSQL_READ_ONLY_USER}'@'%' IDENTIFIED BY '${MYSQL_PASSWORD}'" | mysql --user=root --password=${MYSQL_ROOT_PASSWORD}

SCHEMAS=('reseller')

for schema in ${SCHEMAS[@]}; do
  echo "$0: trying create schema: ${schema}"
  echo "GRANT ALL ON \`${schema}\`.* TO '${MYSQL_USER}'@'%' ;" | mysql --user=root --password=${MYSQL_ROOT_PASSWORD}
  echo "GRANT ALL ON \`${schema}\`.* TO '${MYSQL_READ_ONLY_USER}'@'%' ;" | mysql --user=root --password=${MYSQL_ROOT_PASSWORD}
  mysqladmin --user=${MYSQL_USER} --password=${MYSQL_PASSWORD} create ${schema}
  sed -i -e 's#\bAUTO_INCREMENT=[[:digit:]]\+\b##' -e "s#DEFINER=[0-9a-z\`@%./_]*#DEFINER=\`${MYSQL_USER}\`@\`%\`#" /tmp/${schema}.dump.sql
  mysql --user=${MYSQL_USER} --password=${MYSQL_PASSWORD} ${schema} < /tmp/${schema}.dump.sql
done
